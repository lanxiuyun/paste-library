//! Pairing Service
//! 
//! Handles device pairing with PIN confirmation

use super::{PairingRequest, Platform, SyncDevice};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::time::{SystemTime, UNIX_EPOCH};

/// Minimum PIN length
const PIN_LENGTH: usize = 6;

/// Pairing service for managing device connections
pub struct PairingService {
    /// Pending pairing requests
    pending_requests: Arc<RwLock<HashMap<String, PairingRequest>>>,
    /// Paired devices
    paired_devices: Arc<RwLock<HashMap<String, SyncDevice>>},
    /// Generated PINs (device_id -> pin)
    generated_pins: Arc<RwLock<HashMap<String, String>>>,
}

impl PairingService {
    pub fn new() -> Self {
        Self {
            pending_requests: Arc::new(RwLock::new(HashMap::new())),
            paired_devices: Arc::new(RwLock::new(HashMap::new())),
            generated_pins: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Generate a random PIN
    pub fn generate_pin(&self, device_id: &str) -> String {
        use std::time::{SystemTime, UNIX_EPOCH};
        
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
