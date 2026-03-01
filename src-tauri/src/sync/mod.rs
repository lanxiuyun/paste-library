//! Sync module for LAN clipboard synchronization
//! 
//! This module handles device discovery, pairing, and P2P communication
//! for clipboard content synchronization across devices on the same LAN.

KX|pub mod discovery;
VY|pub mod pairing;
NS|pub mod connection;
KZ|pub mod protocol;
YJ|pub mod crypto;
VB|pub mod sync_manager;

NV|pub use discovery::DiscoveryService;
VY|pub use pairing::PairingService;
NS|pub use connection::ConnectionManager;
KZ|pub use protocol::SyncProtocol;
YJ|pub use crypto::CryptoManager;
XB|pub use sync_manager::{SyncManager, SyncEvent};

pub use discovery::DiscoveryService;
pub use pairing::PairingService;
pub use connection::ConnectionManager;
pub use protocol::SyncProtocol;
pub use crypto::CryptoManager;

use serde::{Deserialize, Serialize};

/// Device platform
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Platform {
    Windows,
    Macos,
    Linux,
    Android,
}

impl Platform {
    pub fn current() -> Self {
        #[cfg(target_os = "windows")]
        return Platform::Windows;
        #[cfg(target_os = "macos")]
        return Platform::Macos;
        #[cfg(target_os = "linux")]
        return Platform::Linux;
        #[cfg(target_os = "android")]
        return Platform::Android;
        
        #[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux", target_os = "android")))]
        return Platform::Linux;
    }
}

/// Sync device information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncDevice {
    pub id: String,
    pub name: String,
    pub platform: Platform,
    pub last_seen: i64,
    pub is_online: bool,
    pub is_paired: bool,
    pub ip_address: Option<String>,
}

/// Pairing request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PairingRequest {
    pub device_id: String,
    pub device_name: String,
    pub pin: String,
    pub timestamp: i64,
}

/// Sync message types
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "payload")]
pub enum SyncMessage {
    /// Text content sync
    Text { content_hash: String, content: String },
    /// Image metadata (reference sync)
    Image { content_hash: String, thumbnail: Option<String>, path: Option<String> },
    /// File metadata (reference sync)
    File { content_hash: String, name: String, path: Option<String>, size: u64 },
    /// Delete operation
    Delete { content_hash: String },
    /// Acknowledgment
    Ack { content_hash: String },
    /// Request content (on-demand)
    RequestContent { content_hash: String },
    /// Content response
    ContentResponse { content_hash: String, data: Vec<u8> },
}

/// Sync state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncState {
    pub enabled: bool,
    pub auto_sync: bool,
    pub device_id: String,
    pub device_name: String,
}

impl Default for SyncState {
    fn default() -> Self {
        Self {
            enabled: false,
            auto_sync: true,
            device_id: uuid::Uuid::new_v4().to_string(),
            device_name: whoami::fallible::desktop_name()
                .unwrap_or_else(|_| "My Device".to_string()),
        }
    }
}
