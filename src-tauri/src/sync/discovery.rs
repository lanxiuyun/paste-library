//! mDNS Device Discovery Service
//!
//! Implements device discovery on LAN using mDNS (multicast DNS)
//! Service type: _pastelib._tcp

use mdns_sd::*;
use std::collections::HashMap;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::sync::{Arc, RwLock};
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use tokio::sync::mpsc;

use super::{Platform, SyncDevice};

/// Service name for mDNS
const SERVICE_TYPE: &str = "_pastelib._tcp.";
/// Service port (arbitrary, but must be consistent)
const SERVICE_PORT: u16 = 17563;

/// Discovery service for finding devices on LAN
pub struct DiscoveryService {
    /// Service info receiver
    receiver: ServiceDaemon,
    /// Discovered devices cache
    devices: Arc<RwLock<HashMap<String, SyncDevice>>>,
    /// Service info channel
    service_full: Option<ServiceInfo>,
    /// Current device info
    device_id: String,
    device_name: String,
}

impl DiscoveryService {
    /// Create a new discovery service
    pub fn new(device_id: String, device_name: String) -> Result<Self, String> {
        let daemon =
            ServiceDaemon::new().map_err(|e| format!("Failed to create mDNS daemon: {}", e))?;

        Ok(Self {
            receiver: daemon,
            devices: Arc::new(RwLock::new(HashMap::new())),
            service_full: None,
            device_id,
            device_name,
        })
    }

    /// Start advertising this device and discovering others
    pub fn start(&mut self) -> Result<(), String> {
        // Get local IP address
        let local_ip =
            get_local_ip().ok_or_else(|| "Failed to get local IP address".to_string())?;

        // Create service info for this device
        let hostname = format!("pastelib-{}.local", &self.device_id[..8]);
        let service_name = format!("PasteLibrary-{}", &self.device_id[..8]);

        let mut properties = HashMap::new();
        properties.insert("id".to_string(), self.device_id.clone());
        properties.insert("name".to_string(), self.device_name.clone());
        properties.insert(
            "platform".to_string(),
            match Platform::current() {
                Platform::Windows => "windows",
                Platform::Macos => "macos",
                Platform::Linux => "linux",
                Platform::Android => "android",
            }
            .to_string(),
        );
        properties.insert("version".to_string(), "1.0.0".to_string());

        let service_info = ServiceInfo::new(
            SERVICE_TYPE,
            &service_name,
            &hostname,
            local_ip,
            SERVICE_PORT,
            properties,
        )
        .map_err(|e| format!("Failed to create service info: {}", e))?;

        // Register service
        self.receiver
            .register(service_info)
            .map_err(|e| format!("Failed to register service: {}", e))?;

        self.service_full = Some(service_info);

        log::info!("Started mDNS discovery on {}:{}", local_ip, SERVICE_PORT);

        Ok(())
    }

    /// Stop advertising this device
    pub fn stop(&mut self) -> Result<(), String> {
        if let Some(service_info) = self.service_full.take() {
            self.receiver
                .unregister(service_info.get_full_name())
                .map_err(|e| format!("Failed to unregister service: {}", e))?;
        }
        log::info!("Stopped mDNS discovery");
        Ok(())
    }

    /// Get list of discovered devices
    pub fn get_devices(&self) -> Vec<SyncDevice> {
        let devices = self.devices.read().unwrap();
        devices.values().cloned().collect()
    }

    /// Update device online status
    pub fn update_device_status(&self, device_id: &str, is_online: bool) {
        let mut devices = self.devices.write().unwrap();
        if let Some(device) = devices.get_mut(device_id) {
            device.is_online = is_online;
            device.last_seen = current_timestamp();
        }
    }
}

/// Get local IP address (first non-loopback IPv4)
fn get_local_ip() -> Option<IpAddr> {
    let socket = match std::net::UdpSocket::bind("0.0.0.0:0") {
        Ok(s) => s,
        Err(_) => return None,
    };

    match socket.connect("8.8.8.8:80") {
        Ok(_) => {}
        Err(_) => return None,
    }

    match socket.local_addr() {
        Ok(addr) => Some(addr.ip()),
        Err(_) => None,
    }
}

/// Get current timestamp in milliseconds
fn current_timestamp() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis() as i64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_local_ip() {
        let ip = get_local_ip();
        println!("Local IP: {:?}", ip);
    }
}
