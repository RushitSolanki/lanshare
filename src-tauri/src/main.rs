// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::Arc;
use std::time::Duration;
use tauri::Manager;
use log::{info, error};
use anyhow::Result;


mod discovery;
use discovery::{DiscoveryService, PeerRegistry};

// WebSocket server for text sharing


/// Global state for the application
struct AppState {
    discovery_service: Arc<tokio::sync::Mutex<Option<DiscoveryService>>>,
    peer_registry: Arc<PeerRegistry>,
}

#[tauri::command]
async fn get_peers(state: tauri::State<'_, AppState>) -> Result<Vec<discovery::Peer>, String> {
    Ok(state.peer_registry.get_peers().await)
}

#[tauri::command]
async fn get_peer_count(state: tauri::State<'_, AppState>) -> Result<usize, String> {
    Ok(state.peer_registry.peer_count().await)
}

#[tauri::command]
async fn get_peer_id(state: tauri::State<'_, AppState>) -> Result<Option<String>, String> {
    let discovery_service = state.discovery_service.lock().await;
    Ok(discovery_service.as_ref().and_then(|ds| ds.peer_id()))
}

#[tauri::command]
async fn debug_peer_structure(state: tauri::State<'_, AppState>) -> Result<String, String> {
    let peers = state.peer_registry.get_peers().await;
    if let Some(first_peer) = peers.first() {
        Ok(format!("Peer structure: {:?}", first_peer))
    } else {
        Ok("No peers available for debugging".to_string())
    }
}

#[tauri::command]
async fn send_text_to_peer(state: tauri::State<'_, AppState>, peer_id: String, text: String) -> Result<(), String> {
    // Validate size before sending (reserve room for JSON overhead)
    const MAX_TEXT_LEN: usize = 6000; // ~6KB text payload within 8KB UDP buffer
    if text.len() > MAX_TEXT_LEN {
        return Err(format!("Text too large ({} chars). Max allowed: {}", text.len(), MAX_TEXT_LEN));
    }
    let peers = state.peer_registry.get_peers().await;
    
    if let Some(_peer) = peers.iter().find(|p| p.id == peer_id) {
        let discovery_service = state.discovery_service.lock().await;
        if let Some(ds) = discovery_service.as_ref() {
            // Create text message
            let message = discovery::DiscoveryMessage {
                message_type: discovery::MessageType::TextMessage,
                peer_id: ds.peer_id().unwrap_or_default(),
                port: 7878, // Changed from 8080 to 7878
                hostname: None,
                timestamp: chrono::Utc::now(),
                text: Some(text.clone()),
            };
            
            // Send to specific peer
            if let Ok(message_bytes) = serde_json::to_vec(&message) {
                // Send UDP message to peer
                if let Ok(socket) = tokio::net::UdpSocket::bind("0.0.0.0:0").await {
                    let peer_addr = format!("{}:{}", _peer.ip, 7878); // Changed to use 7878
                    if let Ok(addr) = peer_addr.parse::<std::net::SocketAddr>() {
                        if let Err(e) = socket.send_to(&message_bytes, addr).await {
                            error!("Failed to send text to peer {}: {}", peer_id, e);
                        } else {
                            info!("Sent text to peer {}: {}", peer_id, text);
                        }
                    }
                }
            }
        }
        Ok(())
    } else {
        Err(format!("Peer {} not found", peer_id))
    }
}

#[tauri::command]
async fn send_text_to_all_peers(state: tauri::State<'_, AppState>, text: String) -> Result<(), String> {
    // Validate size before sending (reserve room for JSON overhead)
    const MAX_TEXT_LEN: usize = 6000; // ~6KB text payload within 8KB UDP buffer
    if text.len() > MAX_TEXT_LEN {
        return Err(format!("Text too large ({} chars). Max allowed: {}", text.len(), MAX_TEXT_LEN));
    }
    let peers = state.peer_registry.get_peers().await;
    
    if peers.is_empty() {
        info!("No peers available to send text to: {}", text);
        return Ok(()); // Return success instead of error
    }
    
    let discovery_service = state.discovery_service.lock().await;
    if let Some(ds) = discovery_service.as_ref() {
        // Create text message
        let message = discovery::DiscoveryMessage {
            message_type: discovery::MessageType::TextMessage,
            peer_id: ds.peer_id().unwrap_or_default(),
            port: 7878, // Changed from 8080 to 7878
            hostname: None,
            timestamp: chrono::Utc::now(),
            text: Some(text.clone()),
        };
        
        // Send to all peers
        if let Ok(message_bytes) = serde_json::to_vec(&message) {
            // Broadcast UDP message to all peers
            if let Ok(socket) = tokio::net::UdpSocket::bind("0.0.0.0:0").await {
                for peer in peers {
                    let peer_addr = format!("{}:{}", peer.ip, 7878); // Changed to use 7878
                    if let Ok(addr) = peer_addr.parse::<std::net::SocketAddr>() {
                        if let Err(e) = socket.send_to(&message_bytes, addr).await {
                            error!("Failed to send text to peer {}: {}", peer.id, e);
                        } else {
                            info!("Sent text to peer {}: {}", peer.id, text);
                        }
                    }
                }
                info!("Broadcasted text to all peers: {}", text);
            }
        }
    }
    
    Ok(())
}

fn main() -> Result<()> {
    // Initialize logging
    env_logger::init();
    info!("Starting LanShare application...");
    // Create the discovery service
    let discovery_service = DiscoveryService::new(Duration::from_secs(8)); // 8 second timeout for faster cleanup
    let peer_registry = discovery_service.registry();
    let app_state = AppState {
        discovery_service: Arc::new(tokio::sync::Mutex::new(Some(discovery_service))),
        peer_registry,
    };
    tauri::Builder::default()
        .manage(app_state)
        .invoke_handler(tauri::generate_handler![
            get_peers,
            get_peer_count,
            get_peer_id,
            debug_peer_structure,
            send_text_to_peer,
            send_text_to_all_peers
        ])
        .setup(|app| {
            let discovery_service = app.state::<AppState>().discovery_service.clone();
            let app_handle = app.app_handle();
            tauri::async_runtime::block_on(async move {
                let mut discovery_service_guard = discovery_service.lock().await;
                if let Some(ref mut ds) = *discovery_service_guard {
                    ds.app_handle = Some(app_handle.clone());
                    match ds.start(7878).await {
                        Ok(()) => {
                            info!("Discovery service initialized successfully");
                            if let Some(peer_id) = ds.peer_id() {
                                if let Ok(_broadcaster_handle) = ds.get_broadcaster_task(7878) {
                                    info!("Broadcaster task spawned");
                                } else {
                                    error!("Failed to spawn broadcaster task");
                                }
                                if let Ok(_listener_handle) = ds.get_listener_task(peer_id.clone()) {
                                    info!("Listener task spawned");
                                } else {
                                    error!("Failed to spawn listener task");
                                }
                                let _cleanup_handle = ds.get_cleanup_task();
                                info!("Cleanup task spawned");
                            }
                        }
                        Err(e) => {
                            error!("Failed to initialize discovery service: {}", e);
                        }
                    }
                }
            });
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
    Ok(())
} 