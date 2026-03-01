//! Pairing Service
//! 
//! Handles device pairing with PIN confirmation

use super::{PairingRequest, Platform, SyncDevice};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use serde::{Deserialize, Serialize};

/// Minimum PIN length
const PIN_LENGTH: usize = 6;

/// Default pairing port
const PAIRING_PORT: u16 = 52999;

/// Pairing message types
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
enum PairingMessage {
    /// Request pairing
    Request { device_id: String, device_name: String, platform: String },
    /// PIN response
    Pin { pin: String },
    /// Accept pairing
    Accept,
    /// Reject pairing
    Reject,
    /// Error
    Error { message: String },
}

/// Pairing service for managing device connections
pub struct PairingService {
    /// Pending pairing requests
    pending_requests: Arc<RwLock<HashMap<String, PairingRequest>>>,
    /// Paired devices
    paired_devices: Arc<RwLock<HashMap<String, SyncDevice>>>,
    /// Generated PINs (device_id -> pin)
    generated_pins: Arc<RwLock<HashMap<String, String>>>,
    /// Pairing TCP server handle
    server_handle: Arc<RwLock<Option<tokio::task::JoinHandle<()>>>>,
}

impl PairingService {
    pub fn new() -> Self {
        Self {
            pending_requests: Arc::new(RwLock::new(HashMap::new())),
            paired_devices: Arc::new(RwLock::new(HashMap::new())),
            generated_pins: Arc::new(RwLock::new(HashMap::new())),
            server_handle: Arc::new(RwLock::new(None)),
        }
    }

    /// Start TCP pairing server
    pub async fn start_pairing_server(&self) -> Result<(), String> {
        let addr = format!("0.0.0.0:{}", PAIRING_PORT);
        let listener = TcpListener::bind(&addr).await
            .map_err(|e| format!("Failed to bind pairing server: {}", e))?;
        
        log::info!("Pairing server started on {}", addr);
        
        let pending = self.pending_requests.clone();
        let pins = self.generated_pins.clone();
        
        // Spawn the server task
        let handle = tokio::spawn(async move {
            loop {
                match listener.accept().await {
                    Ok((stream, addr)) => {
                        log::info!("Pairing request from {}", addr);
                        let pending = pending.clone();
                        let pins = pins.clone();
                        
                        tokio::spawn(async move {
                            if let Err(e) = Self::handle_pairing_connection(
                                stream, pending, pins
                            ).await {
                                log::error!("Pairing error: {}", e);
                            }
                        });
                    }
                    Err(e) => {
                        log::error!("Pairing accept error: {}", e);
                    }
                }
            }
        });
        
        let mut server = self.server_handle.write().unwrap();
        *server = Some(handle);
        
        Ok(())
    }

    /// Handle a single pairing connection
    async fn handle_pairing_connection(
        mut stream: TcpStream,
        pending: Arc<RwLock<HashMap<String, PairingRequest>>>,
        pins: Arc<RwLock<HashMap<String, String>>>,
    ) -> Result<(), String> {
        let mut buf = [0u8; 1024];
        let n = stream.read(&mut buf).await
            .map_err(|e| format!("Read error: {}", e))?;
        
        if n == 0 {
            return Ok(());
        }
        
        // Parse the message
        let msg: PairingMessage = serde_json::from_slice(&buf[..n])
            .map_err(|e| format!("Parse error: {}", e))?;
        
        match msg {
            PairingMessage::Request { device_id, device_name, platform } => {
                // Generate PIN for this device
                let pin = {
                    let seed = SystemTime::now()
                        .duration_since(UNIX_EPOCH)
                        .unwrap()
                        .as_nanos() as u64;
                    let pin = format!("{:06}", seed % 1_000_000);
                    let mut pins_guard = pins.write().unwrap();
                    pins_guard.insert(device_id.clone(), pin.clone());
                    pin
                };
                
                // Store pending request
                let platform = match platform.as_str() {
                    "windows" => Platform::Windows,
                    "macos" => Platform::Macos,
                    "linux" => Platform::Linux,
                    "android" => Platform::Android,
                    _ => Platform::Linux,
                };
                
                let request = PairingRequest {
                    device_id: device_id.clone(),
                    device_name: device_name.clone(),
                    pin: pin.clone(),
                    timestamp: current_timestamp(),
                };
                
                {
                    let mut pending_guard = pending.write().unwrap();
                    pending_guard.insert(device_id.clone(), request);
                }
                
                // Send PIN back
                let response = PairingMessage::Pin { pin };
                let response_json = serde_json::to_vec(&response)
                    .map_err(|e| format!("Serialize error: {}", e))?;
                stream.write_all(&response_json).await
                    .map_err(|e| format!("Write error: {}", e))?;
                
                log::info!("Sent PIN to {} for device {}", device_name, device_id);
            }
            _ => {
                let response = PairingMessage::Error { 
                    message: "Unknown message type".to_string() 
                };
                let response_json = serde_json::to_vec(&response)
                    .map_err(|e| format!("Serialize error: {}", e))?;
                stream.write_all(&response_json).await
                    .map_err(|e| format!("Write error: {}", e))?;
            }
        }
        
        Ok(())
    }

    /// Stop pairing server
    pub fn stop_pairing_server(&self) {
        let mut handle = self.server_handle.write().unwrap();
        if let Some(h) = handle.take() {
            h.abort();
        }
    }

    /// Generate a random PIN
    pub fn generate_pin(&self, device_id: &str) -> String {
        let seed = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos() as u64;
        
        let pin = format!("{:06}", seed % 1_000_000);
        
        // Store generated PIN
        let mut pins = self.generated_pins.write().unwrap();
        pins.insert(device_id.to_string(), pin.clone());
        
        pin
    }

    /// Verify a PIN
    pub fn verify_pin(&self, device_id: &str, pin: &str) -> bool {
        let pins = self.generated_pins.read().unwrap();
        pins.get(device_id).map(|p| p == pin).unwrap_or(false)
    }

    /// Create a pairing request
    pub fn create_request(&self, device_id: String, device_name: String, platform: Platform) -> PairingRequest {
        let pin = self.generate_pin(&device_id);
        
        let request = PairingRequest {
            device_id: device_id.clone(),
            device_name,
            pin,
            timestamp: current_timestamp(),
        };
        
        let mut requests = self.pending_requests.write().unwrap();
        requests.insert(device_id, request.clone());
        
        request
    }

    /// Confirm pairing
    pub fn confirm_pairing(&self, device_id: &str) -> Option<SyncDevice> {
        let mut requests = self.pending_requests.write().unwrap();
        let request = requests.remove(device_id)?;
        
        let device = SyncDevice {
            id: request.device_id,
            name: request.device_name,
            platform: Platform::Macos, // Will be set from actual device info
            last_seen: current_timestamp(),
            is_online: false,
            is_paired: true,
            ip_address: None,
        };
        
        let mut paired = self.paired_devices.write().unwrap();
        paired.insert(device.id.clone(), device.clone());
        
        // Remove PIN
        let mut pins = self.generated_pins.write().unwrap();
        pins.remove(device_id);
        
        Some(device)
    }

    /// Get paired devices
    pub fn get_paired_devices(&self) -> Vec<SyncDevice> {
        let paired = self.paired_devices.read().unwrap();
        paired.values().cloned().collect()
    }

    /// Unpair a device
    pub fn unpair(&self, device_id: &str) -> bool {
        let mut paired = self.paired_devices.write().unwrap();
        paired.remove(device_id).is_some()
    }

    /// Check if device is paired
    pub fn is_paired(&self, device_id: &str) -> bool {
        let paired = self.paired_devices.read().unwrap();
        paired.contains_key(device_id)
    }
}

impl Default for PairingService {
    fn default() -> Self {
        Self::new()
    }
}

fn current_timestamp() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis() as i64
}
