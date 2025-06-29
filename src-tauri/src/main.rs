// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::Arc;
use std::time::Duration;

use anyhow::Result;
use log::{error, info};
use tauri::Manager;

mod discovery;

use discovery::{DiscoveryService, PeerRegistry};

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

fn main() -> Result<()> {
    // Initialize logging
    env_logger::init();
    
    info!("Starting LanShare application...");

    // Create the discovery service
    let mut discovery_service = DiscoveryService::new(Duration::from_secs(30)); // 30 second timeout
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
        ])
        .setup(|app| {
            let discovery_service = app.state::<AppState>().discovery_service.clone();
            
            // Start the discovery service in a background task using Tauri's runtime
            tauri::async_runtime::spawn(async move {
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