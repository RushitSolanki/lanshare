// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::Arc;
use std::time::Duration;
use tauri::Manager;
use log::{info, error};
use anyhow::Result;
use tokio::sync::broadcast;
use tokio_tungstenite::accept_async;
use tokio_tungstenite::tungstenite::Message;
use futures_util::{SinkExt, StreamExt};

mod discovery;
use discovery::{DiscoveryService, PeerRegistry};

// WebSocket server for text sharing
struct WebSocketServer {
    tx: broadcast::Sender<String>,
}

impl WebSocketServer {
    fn new() -> Self {
        let (tx, _) = broadcast::channel::<String>(100);
        Self { tx }
    }

    async fn start(&self) -> Result<()> {
        let addr = "127.0.0.1:9001";
        let listener = tokio::net::TcpListener::bind(addr).await?;
        info!("WebSocket server listening on {}", addr);

        let tx = self.tx.clone();
        
        tokio::spawn(async move {
            while let Ok((stream, addr)) = listener.accept().await {
                info!("New WebSocket connection from: {}", addr);
                
                let tx = tx.clone();
                tokio::spawn(async move {
                    if let Err(e) = Self::handle_connection(stream, tx).await {
                        error!("WebSocket connection error: {}", e);
                    }
                });
            }
        });

        Ok(())
    }

    async fn handle_connection(
        stream: tokio::net::TcpStream,
        tx: broadcast::Sender<String>,
    ) -> Result<()> {
        let ws_stream = accept_async(stream).await?;
        let (mut ws_sender, mut ws_receiver) = ws_stream.split();
        
        let mut rx = tx.subscribe();

        // Handle incoming messages
        let tx_clone = tx.clone();
        let receive_task = tokio::spawn(async move {
            while let Some(msg) = ws_receiver.next().await {
                match msg {
                    Ok(Message::Text(text)) => {
                        info!("Received text message: {}", text);
                        // Broadcast to all connected clients
                        if let Err(e) = tx_clone.send(text) {
                            error!("Failed to broadcast message: {}", e);
                        }
                    }
                    Ok(Message::Close(_)) => {
                        info!("WebSocket connection closed");
                        break;
                    }
                    Err(e) => {
                        error!("WebSocket error: {}", e);
                        break;
                    }
                    _ => {}
                }
            }
        });

        // Handle outgoing messages
        let send_task = tokio::spawn(async move {
            while let Ok(msg) = rx.recv().await {
                if let Err(e) = ws_sender.send(Message::Text(msg)).await {
                    error!("Failed to send message: {}", e);
                    break;
                }
            }
        });

        // Wait for either task to complete
        tokio::select! {
            _ = receive_task => {},
            _ = send_task => {},
        }

        Ok(())
    }


}

/// Global state for the application
struct AppState {
    discovery_service: Arc<tokio::sync::Mutex<Option<DiscoveryService>>>,
    peer_registry: Arc<PeerRegistry>,
    websocket_server: Arc<tokio::sync::Mutex<Option<WebSocketServer>>>,
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

fn main() -> Result<()> {
    // Initialize logging
    env_logger::init();
    
    info!("Starting LanShare application...");

    // Create the discovery service
    let discovery_service = DiscoveryService::new(Duration::from_secs(30)); // 30 second timeout
    let peer_registry = discovery_service.registry();
    
    // Create the WebSocket server
    let websocket_server = WebSocketServer::new();
    
    let app_state = AppState {
        discovery_service: Arc::new(tokio::sync::Mutex::new(Some(discovery_service))),
        peer_registry,
        websocket_server: Arc::new(tokio::sync::Mutex::new(Some(websocket_server))),
    };

    tauri::Builder::default()
        .manage(app_state)
        .invoke_handler(tauri::generate_handler![
            get_peers,
            get_peer_count,
            get_peer_id,
            debug_peer_structure
        ])
        .setup(|app| {
            let discovery_service = app.state::<AppState>().discovery_service.clone();
            let websocket_server = app.state::<AppState>().websocket_server.clone();
            
            // Start the discovery service and WebSocket server in background tasks
            tauri::async_runtime::spawn(async move {
                // Start WebSocket server
                let ws_server_guard = websocket_server.lock().await;
                if let Some(ref ws_server) = *ws_server_guard {
                    if let Err(e) = ws_server.start().await {
                        error!("Failed to start WebSocket server: {}", e);
                    } else {
                        info!("WebSocket server started successfully");
                    }
                }
                
                // Start the discovery service
                let mut discovery_service_guard = discovery_service.lock().await;
                if let Some(ref mut ds) = *discovery_service_guard {
                    // Initialize the discovery service
                    match ds.start(8080).await {
                        Ok(()) => {
                            info!("Discovery service initialized successfully");
                            
                            // Get the peer ID for spawning tasks
                            if let Some(peer_id) = ds.peer_id() {
                                // Spawn the broadcaster task
                                if let Ok(_broadcaster_handle) = ds.get_broadcaster_task(8080) {
                                    info!("Broadcaster task spawned");
                                } else {
                                    error!("Failed to spawn broadcaster task");
                                }
                                
                                // Spawn the listener task
                                if let Ok(_listener_handle) = ds.get_listener_task(peer_id.clone()) {
                                    info!("Listener task spawned");
                                } else {
                                    error!("Failed to spawn listener task");
                                }
                                
                                // Spawn the cleanup task
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