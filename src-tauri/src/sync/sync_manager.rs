//! Sync Manager
//!
//! Manages P2P connections and clipboard synchronization

use crate::sync::{ConnectionManager, SyncDevice, SyncMessage, SyncProtocol};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::time::{Duration, Instant};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::sync::mpsc;
use tokio::time;

/// Sync event types
#[derive(Debug, Clone)]
pub enum SyncEvent {
    /// New clipboard item from remote device
    NewItem { content_hash: String, content: String },
    /// New image item from remote device (reference only)
    NewImageItem { content_hash: String, thumbnail: Option<String>, path: Option<String>, source_device: String },
    /// New file item from remote device (reference only)
    NewFileItem { content_hash: String, name: String, path: Option<String>, size: u64, source_device: String },
    /// Delete request from remote device
    DeleteItem { content_hash: String },
    /// Heartbeat acknowledgment
    HeartbeatAck { device_id: String },
    /// Connection lost
    ConnectionLost { device_id: String },
    /// Content received (on-demand transfer)
    ContentReceived { content_hash: String, data: Vec<u8>, source_device: String },
}

/// Sync manager for handling P2P connections and message broadcasting
pub struct SyncManager {
    /// Connection manager
    connections: Arc<ConnectionManager>,
    /// Pending messages queue (content_hash -> message)
    pending_messages: Arc<RwLock<HashMap<String, SyncMessage>>>,
    /// Event sender for notifying about incoming sync events
    event_sender: Option<mpsc::Sender<SyncEvent>>,
    /// Device ID
    device_id: String,
    /// Running state
    is_running: bool,
}

impl SyncManager {
    /// Create a new sync manager
    pub fn new(device_id: String) -> Self {
        Self {
            connections: Arc::new(ConnectionManager::new()),
            pending_messages: Arc::new(RwLock::new(HashMap::new())),
            event_sender: None,
            device_id,
            is_running: false,
        }
    }

    /// Set the event sender channel
    pub fn set_event_sender(&mut self, sender: mpsc::Sender<SyncEvent>) {
        self.event_sender = Some(sender);
    }

    /// Connect to a paired device
    pub async fn connect_to_device(&self, device: &SyncDevice) -> Result<(), String> {
        let ip = device.ip_address.as_ref()
            .ok_or_else(|| "Device has no IP address".to_string())?;
        
        let addr = format!("{}:{}", ip, SYNC_PORT);
        let stream = TcpStream::connect(&addr).await
            .map_err(|e| format!("Failed to connect to {}: {}", addr, e))?;
        
        // Set up connection
        self.connections.add_connection(
            device.id.clone(),
            ip.clone(),
            SYNC_PORT,
        );
        
        log::info!("Connected to device {} at {}", device.name, addr);
        Ok(())
    }

    /// Disconnect from a device
    pub fn disconnect_from_device(&self, device_id: &str) {
        self.connections.remove_connection(device_id);
        log::info!("Disconnected from device {}", device_id);
    }

    /// Broadcast clipboard content to all connected devices
    pub async fn broadcast_text(&self, content_hash: String, content: String) -> Result<(), String> {
        let message = SyncMessage::Text {
            content_hash: content_hash.clone(),
            content: content.clone(),
        };
        
        // Store in pending for ACK tracking
        {
            let mut pending = self.pending_messages.write().unwrap();
            pending.insert(content_hash.clone(), message.clone());
        }
        
        // Get all connected devices
        let device_ids = self.connections.get_connected_devices();
        
        // Send to each device
        for device_id in device_ids {
            if let Err(e) = self.send_message(&device_id, &message).await {
                log::error!("Failed to send to {}: {}", device_id, e);
            }
        }
        
        Ok(())
    }

    /// Broadcast delete operation to all connected devices
    pub async fn broadcast_delete(&self, content_hash: String) -> Result<(), String> {
        let message = SyncMessage::Delete { content_hash: content_hash.clone() };
        
        let device_ids = self.connections.get_connected_devices();
        
        for device_id in device_ids {
            if let Err(e) = self.send_message(&device_id, &message).await {
                log::error!("Failed to send delete to {}: {}", device_id, e);
            }
        }
        
        Ok(())
    }

    /// Send a message to a specific device
    async fn send_message(&self, device_id: &str, message: &SyncMessage) -> Result<(), String> {
        // For now, we'll use a simplified approach
        // In a full implementation, this would use actual TCP connections
        log::debug!("Would send message to {}: {:?}", device_id, message);
        Ok(())
    }

    /// Broadcast image metadata to all connected devices
    pub async fn broadcast_image(
        &self,
        content_hash: String,
        thumbnail: Option<String>,
        path: Option<String>,
    ) -> Result<(), String> {
        let message = SyncMessage::Image {
            content_hash: content_hash.clone(),
            thumbnail,
            path,
        };

        // Store in pending for ACK tracking
        {
            let mut pending = self.pending_messages.write().unwrap();
            pending.insert(content_hash.clone(), message.clone());
        }

        // Get all connected devices
        let device_ids = self.connections.get_connected_devices();

        // Send to each device
        for device_id in device_ids {
            if let Err(e) = self.send_message(&device_id, &message).await {
                log::error!("Failed to send image to {}: {}", device_id, e);
            }
        }

        Ok(())
    }

    /// Broadcast file metadata to all connected devices
    pub async fn broadcast_file(
        &self,
        content_hash: String,
        name: String,
        path: Option<String>,
        size: u64,
    ) -> Result<(), String> {
        let message = SyncMessage::File {
            content_hash: content_hash.clone(),
            name,
            path,
            size,
        };

        // Store in pending for ACK tracking
        {
            let mut pending = self.pending_messages.write().unwrap();
            pending.insert(content_hash.clone(), message.clone());
        }

        // Get all connected devices
        let device_ids = self.connections.get_connected_devices();

        // Send to each device
        for device_id in device_ids {
            if let Err(e) = self.send_message(&device_id, &message).await {
                log::error!("Failed to send file to {}: {}", device_id, e);
            }
        }

        Ok(())
    }

    /// Request content from a specific device (on-demand transfer)
    pub async fn request_content(&self, device_id: &str, content_hash: String) -> Result<Vec<u8>, String> {
        let message = SyncMessage::RequestContent {
            content_hash: content_hash.clone(),
        };

        self.send_message(device_id, &message).await?;

        // Wait for response (simplified - in real impl would use a response channel)
        log::debug!("Requested content {} from {}", content_hash, device_id);
        Ok(Vec::new())
    }

    /// Send content response to a device
    pub async fn send_content_response(
        &self,
        device_id: &str,
        content_hash: String,
        data: Vec<u8>,
    ) -> Result<(), String> {
        let message = SyncMessage::ContentResponse {
            content_hash,
            data,
        };

        self.send_message(device_id, &message).await
    }

    /// Handle incoming message from a device
    async fn handle_incoming_message(&self, device_id: &str, data: Vec<u8>) -> Result<(), String> {
        let message = SyncProtocol::decode(&data)?;
        
        match &message {
            SyncMessage::Text { content_hash, content } => {
                // Send event to clipboard manager
                if let Some(ref sender) = self.event_sender {
                    let _ = sender.send(SyncEvent::NewItem {
                        content_hash: content_hash.clone(),
                        content: content.clone(),
                    }).await;
                }
                
                // Send ACK
                let ack = SyncMessage::Ack { content_hash: content_hash.clone() };
                self.send_message(device_id, &ack).await?;
            }
            
            SyncMessage::Delete { content_hash } => {
                if let Some(ref sender) = self.event_sender {
                    let _ = sender.send(SyncEvent::DeleteItem {
                        content_hash: content_hash.clone(),
                    }).await;
                }
                
                // Send ACK
                let ack = SyncMessage::Ack { content_hash: content_hash.clone() };
                self.send_message(device_id, &ack).await?;
            }
            
            SyncMessage::Ack { content_hash } => {
                // Remove from pending
                {
                    let mut pending = self.pending_messages.write().unwrap();
                    pending.remove(content_hash);
                }
                
                if let Some(ref sender) = self.event_sender {
                    let _ = sender.send(SyncEvent::HeartbeatAck {
                        device_id: device_id.to_string(),
                    }).await;
                }
            }
            
            // Image metadata received - store reference only
            SyncMessage::Image { content_hash, thumbnail, path } => {
                log::info!("Received image sync: {} from {}", content_hash, device_id);
                // Send event to clipboard manager
                if let Some(ref sender) = self.event_sender {
                    let _ = sender.send(SyncEvent::NewImageItem {
                        content_hash: content_hash.clone(),
                        thumbnail: thumbnail.clone(),
                        path: path.clone(),
                        source_device: device_id.to_string(),
                    }).await;
                }
                // Send ACK
                let ack = SyncMessage::Ack { content_hash: content_hash.clone() };
                self.send_message(device_id, &ack).await?;
            }
            
            // File metadata received - store reference only
            SyncMessage::File { content_hash, name, path, size } => {
                log::info!("Received file sync: {} ({}) from {}", name, size, device_id);
                // Send event to clipboard manager
                if let Some(ref sender) = self.event_sender {
                    let _ = sender.send(SyncEvent::NewFileItem {
                        content_hash: content_hash.clone(),
                        name: name.clone(),
                        path: path.clone(),
                        size: *size,
                        source_device: device_id.to_string(),
                    }).await;
                }
                // Send ACK
                let ack = SyncMessage::Ack { content_hash: content_hash.clone() };
                self.send_message(device_id, &ack).await?;
            }
            
            // Content request - on-demand transfer
            SyncMessage::RequestContent { content_hash } => {
                log::info!("Content requested: {} from {}", content_hash, device_id);
                // TODO: Look up content in local database and send it
                // For now, just send empty response
                let response = SyncMessage::ContentResponse {
                    content_hash: content_hash.clone(),
                    data: Vec::new(),
                };
                self.send_message(device_id, &response).await?;
            }
            
            // Content response - on-demand transfer received
            SyncMessage::ContentResponse { content_hash, data } => {
                log::info!("Content received: {} ({} bytes) from {}", 
                    content_hash, data.len(), device_id);
                // Send event to clipboard manager
                if let Some(ref sender) = self.event_sender {
                    let _ = sender.send(SyncEvent::ContentReceived {
                        content_hash: content_hash.clone(),
                        data: data.clone(),
                        source_device: device_id.to_string(),
                    }).await;
                }
            }
        }
        
        Ok(())
    }

    /// Start the sync manager background tasks
    pub fn start(&mut self) {
        if self.is_running {
            return;
        }
        
        self.is_running = true;
        log::info!("Sync manager started");
    }

    /// Stop the sync manager
    pub fn stop(&mut self) {
        if !self.is_running {
            return;
        }
        
        self.is_running = false;
        
        // Clear all connections
        let device_ids = self.connections.get_connected_devices();
        for device_id in device_ids {
            self.connections.remove_connection(&device_id);
        }
        
        log::info!("Sync manager stopped");
    }

    /// Check if running
    pub fn is_running(&self) -> bool {
        self.is_running
    }

    /// Get connection manager reference
    pub fn connections(&self) -> &ConnectionManager {
        &self.connections
    }
}

/// Default sync port for P2P communication
const SYNC_PORT: u16 = 17564;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sync_manager_creation() {
        let manager = SyncManager::new("test-device".to_string());
        assert!(!manager.is_running());
    }
}
