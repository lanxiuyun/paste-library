//! Connection Manager
//!
//! Manages P2P connections to paired devices

use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::time::{Duration, Instant};

/// Connection state
#[derive(Debug, Clone, PartialEq)]
pub enum ConnectionState {
    Disconnected,
    Connecting,
    Connected,
    Reconnecting,
}

/// P2P connection info
#[derive(Debug, Clone)]
pub struct Connection {
    pub device_id: String,
    pub ip_address: String,
    pub port: u16,
    pub state: ConnectionState,
    pub last_heartbeat: Instant,
}

/// Connection manager for paired devices
pub struct ConnectionManager {
    /// Active connections
    connections: Arc<RwLock<HashMap<String, Connection>>>,
    /// Heartbeat interval
    heartbeat_interval: Duration,
}

impl ConnectionManager {
    pub fn new() -> Self {
        Self {
            connections: Arc::new(RwLock::new(HashMap::new())),
            heartbeat_interval: Duration::from_secs(30),
        }
    }

    /// Add a connection
    pub fn add_connection(&self, device_id: String, ip_address: String, port: u16) {
        let mut connections = self.connections.write().unwrap();

        connections.insert(
            device_id.clone(),
            Connection {
                device_id,
                ip_address,
                port,
                state: ConnectionState::Connected,
                last_heartbeat: Instant::now(),
            },
        );
    }

    /// Remove a connection
    pub fn remove_connection(&self, device_id: &str) {
        let mut connections = self.connections.write().unwrap();
        connections.remove(device_id);
    }

    /// Get connection state
    pub fn get_state(&self, device_id: &str) -> Option<ConnectionState> {
        let connections = self.connections.read().unwrap();
        connections.get(device_id).map(|c| c.state.clone())
    }

    /// Check if connected
    pub fn is_connected(&self, device_id: &str) -> bool {
        self.get_state(device_id) == Some(ConnectionState::Connected)
    }

    /// Get all connected device IDs
    pub fn get_connected_devices(&self) -> Vec<String> {
        let connections = self.connections.read().unwrap();
        connections
            .iter()
            .filter(|(_, c)| c.state == ConnectionState::Connected)
            .map(|(id, _)| id.clone())
            .collect()
    }

    /// Update heartbeat
    pub fn update_heartbeat(&self, device_id: &str) {
        let mut connections = self.connections.write().unwrap();
        if let Some(conn) = connections.get_mut(device_id) {
            conn.last_heartbeat = Instant::now();
        }
    }

    /// Check for stale connections (no heartbeat)
    pub fn check_stale_connections(&self) -> Vec<String> {
        let mut connections = self.connections.write().unwrap();
        let stale_threshold = self.heartbeat_interval * 2;

        let mut stale = Vec::new();

        for (id, conn) in connections.iter_mut() {
            if conn.last_heartbeat.elapsed() > stale_threshold {
                conn.state = ConnectionState::Reconnecting;
                stale.push(id.clone());
            }
        }

        stale
    }
}

impl Default for ConnectionManager {
    fn default() -> Self {
        Self::new()
    }
}
