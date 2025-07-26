use std::collections::HashMap;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::sync::Arc;
use std::time::Duration;

use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use log::{debug, error, info, warn};
use serde::{Deserialize, Serialize};
use tokio::net::UdpSocket;
use tokio::sync::RwLock;
use tokio::time::{interval, sleep};
use uuid::Uuid;
use tauri::AppHandle;
use tauri::Emitter;

/// Represents a discovered peer on the network
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Peer {
    pub id: String,
    pub ip: IpAddr,
    pub port: u16,
    pub last_seen: DateTime<Utc>,
    pub hostname: Option<String>,
}

impl Peer {
    pub fn new(id: String, ip: IpAddr, port: u16, hostname: Option<String>) -> Self {
        Self {
            id,
            ip,
            port,
            last_seen: Utc::now(),
            hostname,
        }
    }

    #[allow(dead_code)]
    pub fn socket_addr(&self) -> SocketAddr {
        SocketAddr::new(self.ip, self.port)
    }

    pub fn is_stale(&self, timeout_duration: Duration) -> bool {
        let now = Utc::now();
        let last_seen = self.last_seen;
        let duration_since_last_seen = now.signed_duration_since(last_seen);
        duration_since_last_seen.num_seconds() as u64 > timeout_duration.as_secs()
    }
}

/// Message format for UDP broadcasts
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MessageType {
    PeerDiscovery,
    TextMessage,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscoveryMessage {
    pub message_type: MessageType,
    pub peer_id: String,
    pub port: u16,
    pub hostname: Option<String>,
    pub timestamp: DateTime<Utc>,
    pub text: Option<String>, // For text messages
}

/// Registry for managing discovered peers
#[derive(Debug)]
pub struct PeerRegistry {
    peers: Arc<RwLock<HashMap<String, Peer>>>,
    timeout_duration: Duration,
}

impl PeerRegistry {
    pub fn new(timeout_duration: Duration) -> Self {
        Self {
            peers: Arc::new(RwLock::new(HashMap::new())),
            timeout_duration,
        }
    }

    /// Add or update a peer in the registry
    pub async fn add_peer(&self, peer: Peer) {
        let mut peers = self.peers.write().await;
        let existing = peers.get(&peer.id);
        
        if let Some(existing_peer) = existing {
            if existing_peer.ip != peer.ip || existing_peer.port != peer.port {
                info!("Peer {} updated: {}:{} -> {}:{}", 
                    peer.id, existing_peer.ip, existing_peer.port, peer.ip, peer.port);
            }
        } else {
            info!("New peer discovered: {} at {}:{}", peer.id, peer.ip, peer.port);
        }
        
        peers.insert(peer.id.clone(), peer);
    }

    /// Remove a peer from the registry
    #[allow(dead_code)]
    pub async fn remove_peer(&self, peer_id: &str) -> bool {
        let mut peers = self.peers.write().await;
        if let Some(peer) = peers.remove(peer_id) {
            info!("Peer removed: {} at {}:{}", peer_id, peer.ip, peer.port);
            true
        } else {
            false
        }
    }

    /// Get all peers
    pub async fn get_peers(&self) -> Vec<Peer> {
        let peers = self.peers.read().await;
        peers.values().cloned().collect()
    }

    /// Get a specific peer by ID
    #[allow(dead_code)]
    pub async fn get_peer(&self, peer_id: &str) -> Option<Peer> {
        let peers = self.peers.read().await;
        peers.get(peer_id).cloned()
    }

    /// Clean up stale peers
    pub async fn cleanup_stale_peers(&self) -> usize {
        let mut peers = self.peers.write().await;
        let initial_count = peers.len();
        
        peers.retain(|peer_id, peer| {
            if peer.is_stale(self.timeout_duration) {
                warn!("Removing stale peer: {} at {}:{}", peer_id, peer.ip, peer.port);
                false
            } else {
                true
            }
        });
        
        let removed_count = initial_count - peers.len();
        if removed_count > 0 {
            info!("Cleaned up {} stale peers", removed_count);
        }
        
        removed_count
    }

    /// Get the number of peers
    pub async fn peer_count(&self) -> usize {
        self.peers.read().await.len()
    }
}

/// UDP broadcaster for announcing presence on the network
pub struct UdpBroadcaster {
    socket: UdpSocket,
    peer_id: String,
    port: u16,
    hostname: Option<String>,
    broadcast_interval: Duration,
}

impl UdpBroadcaster {
    pub async fn new(port: u16, broadcast_interval: Duration) -> Result<Self> {
        let socket = UdpSocket::bind("0.0.0.0:0")
            .await
            .context("Failed to bind UDP socket for broadcasting")?;
        
        socket.set_broadcast(true)
            .context("Failed to enable broadcast on UDP socket")?;

        let peer_id = Uuid::new_v4().to_string();
        let hostname = hostname::get();

        Ok(Self {
            socket,
            peer_id,
            port,
            hostname,
            broadcast_interval,
        })
    }

    pub fn get_peer_id(&self) -> &str {
        &self.peer_id
    }

    pub fn set_peer_id(&mut self, peer_id: String) {
        self.peer_id = peer_id;
    }

    /// Start broadcasting presence messages
    #[allow(dead_code)]
    pub async fn start_broadcasting(&self) -> Result<()> {
        let mut interval = interval(self.broadcast_interval);
        let broadcast_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::BROADCAST), 7878);

        info!("Starting UDP broadcast on port 7878 with peer ID: {}", self.peer_id);

        loop {
            interval.tick().await;
            
            let message = DiscoveryMessage {
                message_type: MessageType::PeerDiscovery,
                peer_id: self.peer_id.clone(),
                port: self.port,
                hostname: self.hostname.clone(),
                timestamp: Utc::now(),
                text: None,
            };

            let message_bytes = serde_json::to_vec(&message)
                .context("Failed to serialize discovery message")?;

            match self.socket.send_to(&message_bytes, broadcast_addr).await {
                Ok(_) => {
                    debug!("Broadcasted presence message");
                }
                Err(e) => {
                    error!("Failed to broadcast presence message: {}", e);
                }
            }
        }
    }
}

/// UDP listener for discovering other peers on the network
pub struct UdpListener {
    socket: UdpSocket,
    #[allow(dead_code)]
    registry: Arc<PeerRegistry>,
    #[allow(dead_code)]
    own_peer_id: String,
}

impl UdpListener {
    pub async fn new(registry: Arc<PeerRegistry>, own_peer_id: String) -> Result<Self> {
        let socket = UdpSocket::bind("0.0.0.0:7878")
            .await
            .context("Failed to bind UDP socket for listening")?;

        info!("UDP listener started on port 7878");

        Ok(Self { 
            socket, 
            registry,
            own_peer_id,
        })
    }

    /// Start listening for discovery messages
    #[allow(dead_code)]
    pub async fn start_listening(&self) -> Result<()> {
        let mut buf = [0; 1024];

        loop {
            match self.socket.recv_from(&mut buf).await {
                Ok((len, src_addr)) => {
                    let message_bytes = &buf[..len];
                    
                    if let Err(e) = self.handle_message(message_bytes, src_addr).await {
                        error!("Failed to handle discovery message: {}", e);
                    }
                }
                Err(e) => {
                    error!("Failed to receive UDP message: {}", e);
                    sleep(Duration::from_millis(100)).await;
                }
            }
        }
    }

    #[allow(dead_code)]
    async fn handle_message(&self, message_bytes: &[u8], src_addr: SocketAddr) -> Result<()> {
        let message: DiscoveryMessage = serde_json::from_slice(message_bytes)
            .context("Failed to deserialize discovery message")?;

        // Ignore our own messages
        if message.peer_id == self.own_peer_id {
            return Ok(());
        }

        match message.message_type {
            MessageType::PeerDiscovery => {
                let peer = Peer::new(
                    message.peer_id,
                    src_addr.ip(),
                    message.port,
                    message.hostname,
                );
                self.registry.add_peer(peer).await;
            }
            MessageType::TextMessage => {
                if let Some(text) = message.text {
                    info!("Received text message from {}: {}", message.peer_id, text);
                    // TODO: Handle text message (emit to frontend)
                }
            }
        }

        Ok(())
    }
}

/// Main discovery service that coordinates broadcasting and listening
pub struct DiscoveryService {
    registry: Arc<PeerRegistry>,
    peer_id: Option<String>,
    pub app_handle: Option<AppHandle>,
}

impl DiscoveryService {
    pub fn new(timeout_duration: Duration) -> Self {
        Self {
            registry: Arc::new(PeerRegistry::new(timeout_duration)),
            peer_id: None,
            app_handle: None,
        }
    }

    /// Start the discovery service
    pub async fn start(&mut self, port: u16) -> Result<()> {
        // Start the broadcaster
        let _broadcaster = UdpBroadcaster::new(port, Duration::from_secs(5)).await?;
        let peer_id = _broadcaster.get_peer_id().to_string();
        
        // Store the peer ID
        self.peer_id = Some(peer_id.clone());
        
        // Start the listener
        let _listener = UdpListener::new(self.registry.clone(), peer_id.clone()).await?;

        // Return the tasks to be spawned by the caller
        // The caller should spawn these tasks in the appropriate runtime context
        
        info!("Discovery service initialized with peer ID: {}", peer_id);
        Ok(())
    }

    /// Get the broadcaster task for spawning
    pub fn get_broadcaster_task(&self, port: u16) -> Result<tokio::task::JoinHandle<()>> {
        // Use the peer ID that was already generated in start()
        let peer_id = self.peer_id.clone().ok_or_else(|| {
            anyhow::anyhow!("Peer ID not available - call start() first")
        })?;
        
        let broadcaster = UdpBroadcaster::new(port, Duration::from_secs(5));
        
        Ok(tokio::spawn(async move {
            let broadcaster = match broadcaster.await {
                Ok(mut b) => {
                    // Override the peer ID to use the one from start()
                    b.set_peer_id(peer_id);
                    b
                },
                Err(e) => {
                    error!("Failed to create broadcaster: {}", e);
                    return;
                }
            };
            let mut interval = interval(broadcaster.broadcast_interval);
            let broadcast_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::BROADCAST), 7878);
            info!("Starting UDP broadcast on port 7878 with peer ID: {}", broadcaster.get_peer_id());
            loop {
                interval.tick().await;
                            let message = DiscoveryMessage {
                message_type: MessageType::PeerDiscovery,
                peer_id: broadcaster.get_peer_id().to_string(),
                port: broadcaster.port,
                hostname: broadcaster.hostname.clone(),
                timestamp: Utc::now(),
                text: None,
            };
                let message_bytes = match serde_json::to_vec(&message) {
                    Ok(bytes) => bytes,
                    Err(e) => {
                        error!("Failed to serialize discovery message: {}", e);
                        continue;
                    }
                };
                match broadcaster.socket.send_to(&message_bytes, broadcast_addr).await {
                    Ok(_) => {
                        debug!("Broadcasted presence message");
                    }
                    Err(e) => {
                        error!("Failed to broadcast presence message: {}", e);
                    }
                }
            }
        }))
    }

    /// Get the listener task for spawning
    pub fn get_listener_task(&self, own_peer_id: String) -> Result<tokio::task::JoinHandle<()>> {
        let registry = self.registry.clone();
        let app_handle = self.app_handle.clone();
        let listener = UdpListener::new(registry.clone(), own_peer_id.clone());
        
        Ok(tokio::spawn(async move {
            let listener = match listener.await {
                Ok(l) => l,
                Err(e) => {
                    error!("Failed to create listener: {}", e);
                    return;
                }
            };
            let mut buf = [0; 1024];
            loop {
                match listener.socket.recv_from(&mut buf).await {
                    Ok((len, src_addr)) => {
                        let message_bytes = &buf[..len];
                        if let Err(e) = Self::handle_listener_message(
                            message_bytes,
                            src_addr,
                            &registry,
                            &own_peer_id,
                            app_handle.clone(),
                        ).await {
                            error!("Failed to handle discovery message: {}", e);
                        }
                    }
                    Err(e) => {
                        error!("Failed to receive UDP message: {}", e);
                        sleep(Duration::from_millis(100)).await;
                    }
                }
            }
        }))
    }

    /// Get the cleanup task for spawning
    pub fn get_cleanup_task(&self) -> tokio::task::JoinHandle<()> {
        let registry = self.registry.clone();
        tokio::spawn(async move {
            let mut interval = interval(Duration::from_secs(10));
            loop {
                interval.tick().await;
                registry.cleanup_stale_peers().await;
            }
        })
    }

    async fn handle_listener_message(
        message_bytes: &[u8],
        src_addr: SocketAddr,
        registry: &Arc<PeerRegistry>,
        own_peer_id: &str,
        app_handle: Option<AppHandle>,
    ) -> Result<()> {
        // Log raw UDP packet
        info!("Received UDP packet from {}: {:?}", src_addr, message_bytes);

        let message: DiscoveryMessage = match serde_json::from_slice(message_bytes) {
            Ok(msg) => msg,
            Err(e) => {
                error!("Failed to deserialize discovery message from {}: {}", src_addr, e);
                return Ok(());
            }
        };

        // Log after deserialization
        info!("Deserialized message from {}: {:?}", src_addr, message);

        if message.peer_id == own_peer_id {
            return Ok(());
        }
        match message.message_type {
            MessageType::PeerDiscovery => {
                let peer = Peer::new(
                    message.peer_id,
                    src_addr.ip(),
                    message.port,
                    message.hostname,
                );
                registry.add_peer(peer).await;
            }
            MessageType::TextMessage => {
                if let Some(text) = message.text {
                    info!("Received text message from {}: {}", message.peer_id, text);
                    if let Some(app) = app_handle {
                        info!("Emitting text-received event to frontend: {}", text);
                        let _ = app.emit("text-received", text);
                    }
                }
            }
        }
        Ok(())
    }

    /// Get the peer registry
    pub fn registry(&self) -> Arc<PeerRegistry> {
        self.registry.clone()
    }

    /// Get the current peer ID
    pub fn peer_id(&self) -> Option<String> {
        self.peer_id.clone()
    }

    /// Stop the discovery service
    #[allow(dead_code)]
    pub async fn stop(&mut self) -> Result<()> {
        info!("Stopping discovery service");
        self.peer_id = None;
        Ok(())
    }
}

// Helper function to get hostname
mod hostname {
    use std::env;

    pub fn get() -> Option<String> {
        env::var("HOSTNAME")
            .or_else(|_| env::var("COMPUTERNAME"))
            .or_else(|_| env::var("USER"))
            .ok()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::Ipv4Addr;

    #[tokio::test]
    async fn test_peer_registry() {
        let registry = PeerRegistry::new(Duration::from_secs(30));
        
        let peer = Peer::new(
            "test-id".to_string(),
            IpAddr::V4(Ipv4Addr::new(192, 168, 1, 100)),
            8080,
            Some("test-host".to_string()),
        );

        registry.add_peer(peer.clone()).await;
        assert_eq!(registry.peer_count().await, 1);

        let peers = registry.get_peers().await;
        assert_eq!(peers.len(), 1);
        assert_eq!(peers[0].id, "test-id");

        registry.remove_peer("test-id").await;
        assert_eq!(registry.peer_count().await, 0);
    }

    #[tokio::test]
    async fn test_peer_stale_detection() {
        let registry = PeerRegistry::new(Duration::from_secs(1));
        
        let mut peer = Peer::new(
            "test-id".to_string(),
            IpAddr::V4(Ipv4Addr::new(192, 168, 1, 100)),
            8080,
            None,
        );

        // Manually set last_seen to be old
        peer.last_seen = Utc::now() - chrono::Duration::seconds(2);
        
        registry.add_peer(peer).await;
        assert_eq!(registry.peer_count().await, 1);

        // Wait for cleanup
        sleep(Duration::from_millis(1100)).await;
        let removed = registry.cleanup_stale_peers().await;
        assert_eq!(removed, 1);
        assert_eq!(registry.peer_count().await, 0);
    }

    #[tokio::test]
    async fn test_discovery_service_creation() {
        let mut discovery_service = DiscoveryService::new(Duration::from_secs(30));
        
        // Test that the service can be created
        assert!(discovery_service.peer_id().is_none());
        
        // Test that the registry is accessible
        let registry = discovery_service.registry();
        assert_eq!(registry.peer_count().await, 0);
    }
} 