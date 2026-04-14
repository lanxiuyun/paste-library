use std::collections::{HashMap, VecDeque};
use std::fs;
use std::io::{Read, Write};
use std::net::{Ipv4Addr, Shutdown, SocketAddr, TcpListener, TcpStream, UdpSocket};
use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Emitter};

use crate::clipboard::ClipboardManager;
use crate::models::{AppSettings, LanDiscoveredDevice, LanPairingRequest, LanSyncStatus};

const DISCOVERY_INTERVAL_SECS: u64 = 3;
const DISCOVERY_STALE_SECS: i64 = 12;
const SOCKET_TIMEOUT_MS: u64 = 1000;
const RECENT_MESSAGE_CACHE_SIZE: usize = 256;
const PROTOCOL_VERSION: u8 = 1;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
enum LanWireMessage {
    Heartbeat {
        protocol_version: u8,
        device_id: String,
        device_name: String,
        tcp_port: u16,
    },
    PairRequest {
        protocol_version: u8,
        device_id: String,
        device_name: String,
        tcp_port: u16,
    },
    PairResponse {
        protocol_version: u8,
        device_id: String,
        device_name: String,
        tcp_port: u16,
        accepted: bool,
    },
    SyncText {
        protocol_version: u8,
        device_id: String,
        device_name: String,
        tcp_port: u16,
        message_id: String,
        created_at: String,
        text: String,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct StoredTrustedDevice {
    device_id: String,
    device_name: String,
    address: String,
    tcp_port: u16,
    last_seen: DateTime<Utc>,
}

#[derive(Debug, Clone)]
struct LanServiceState {
    device_id: String,
    device_name: String,
    running: bool,
    discovery_enabled: bool,
    tcp_port: u16,
    discovery_port: u16,
    discovered_devices: HashMap<String, LanDiscoveredDevice>,
    trusted_devices: HashMap<String, StoredTrustedDevice>,
    pending_requests: HashMap<String, LanPairingRequest>,
    last_error: Option<String>,
    recent_message_ids: VecDeque<String>,
}

#[derive(Clone)]
pub struct LanSyncService {
    app: AppHandle,
    clipboard_manager: ClipboardManager,
    app_dir: PathBuf,
    state: Arc<Mutex<LanServiceState>>,
    generation: Arc<AtomicU64>,
}

impl LanSyncService {
    pub fn new(
        app: AppHandle,
        clipboard_manager: ClipboardManager,
        app_dir: PathBuf,
        settings: &AppSettings,
    ) -> Self {
        let device_id = load_or_create_device_id(&app_dir);
        let trusted_devices = load_trusted_devices(&app_dir);
        let initial_state = LanServiceState {
            device_id,
            device_name: normalize_device_name(&settings.lan_sync_device_name),
            running: false,
            discovery_enabled: settings.lan_sync_discovery_enabled,
            tcp_port: sanitize_port(settings.lan_sync_tcp_port, 48571),
            discovery_port: sanitize_port(settings.lan_sync_discovery_port, 48572),
            discovered_devices: HashMap::new(),
            trusted_devices,
            pending_requests: HashMap::new(),
            last_error: None,
            recent_message_ids: VecDeque::new(),
        };

        Self {
            app,
            clipboard_manager,
            app_dir,
            state: Arc::new(Mutex::new(initial_state)),
            generation: Arc::new(AtomicU64::new(0)),
        }
    }

    pub fn apply_settings(&self, settings: &AppSettings) {
        let should_run = settings.lan_sync_enabled;
        let should_discover = settings.lan_sync_discovery_enabled;
        let next_tcp_port = sanitize_port(settings.lan_sync_tcp_port, 48571);
        let next_discovery_port = sanitize_port(settings.lan_sync_discovery_port, 48572);

        let (was_running, restart_required) = {
            let mut state = self.state.lock().unwrap();
            let was_running = state.running;
            let restart_required = was_running
                && (state.tcp_port != next_tcp_port
                    || state.discovery_port != next_discovery_port
                    || state.discovery_enabled != should_discover);

            state.device_name = normalize_device_name(&settings.lan_sync_device_name);
            state.discovery_enabled = should_discover;
            state.tcp_port = next_tcp_port;
            state.discovery_port = next_discovery_port;
            state.running = should_run;
            state.last_error = None;

            if !should_run {
                state.discovered_devices.clear();
                state.pending_requests.clear();
            }

            (was_running, restart_required)
        };

        self.emit_status();

        if !should_run {
            self.generation.fetch_add(1, Ordering::SeqCst);
            return;
        }

        if was_running && !restart_required {
            return;
        }

        let new_generation = self.generation.fetch_add(1, Ordering::SeqCst) + 1;
        let startup_delay_ms = if was_running { SOCKET_TIMEOUT_MS + 200 } else { 0 };
        self.schedule_runtime_start(new_generation, startup_delay_ms);
    }

    pub fn get_status(&self) -> LanSyncStatus {
        let state = self.state.lock().unwrap();
        let mut discovered_devices = state
            .discovered_devices
            .values()
            .cloned()
            .collect::<Vec<_>>();
        discovered_devices.sort_by(|a, b| b.last_seen.cmp(&a.last_seen));

        let mut trusted_devices = state
            .trusted_devices
            .values()
            .map(|device| LanDiscoveredDevice {
                device_id: device.device_id.clone(),
                device_name: device.device_name.clone(),
                address: device.address.clone(),
                tcp_port: device.tcp_port,
                trusted: true,
                last_seen: device.last_seen,
            })
            .collect::<Vec<_>>();
        trusted_devices.sort_by(|a, b| a.device_name.cmp(&b.device_name));

        let mut pending_requests = state
            .pending_requests
            .values()
            .cloned()
            .collect::<Vec<_>>();
        pending_requests.sort_by(|a, b| b.requested_at.cmp(&a.requested_at));

        LanSyncStatus {
            running: state.running,
            device_id: state.device_id.clone(),
            device_name: state.device_name.clone(),
            discovery_enabled: state.discovery_enabled,
            tcp_port: state.tcp_port,
            discovery_port: state.discovery_port,
            discovered_devices,
            trusted_devices,
            pending_requests,
            last_error: state.last_error.clone(),
        }
    }

    pub fn request_pairing(&self, device_id: &str) -> Result<(), String> {
        let peer = {
            let state = self.state.lock().unwrap();
            state
                .discovered_devices
                .get(device_id)
                .cloned()
                .ok_or_else(|| "未找到目标设备".to_string())?
        };

        let message = {
            let state = self.state.lock().unwrap();
            LanWireMessage::PairRequest {
                protocol_version: PROTOCOL_VERSION,
                device_id: state.device_id.clone(),
                device_name: state.device_name.clone(),
                tcp_port: state.tcp_port,
            }
        };

        self.send_message(&peer.address, peer.tcp_port, &message)
    }

    pub fn approve_device(&self, device_id: &str) -> Result<(), String> {
        let request = {
            let mut state = self.state.lock().unwrap();
            let request = state
                .pending_requests
                .remove(device_id)
                .ok_or_else(|| "未找到待处理配对请求".to_string())?;

            state.trusted_devices.insert(
                request.device_id.clone(),
                StoredTrustedDevice {
                    device_id: request.device_id.clone(),
                    device_name: request.device_name.clone(),
                    address: request.address.clone(),
                    tcp_port: request.tcp_port,
                    last_seen: Utc::now(),
                },
            );

            request
        };

        self.persist_trusted_devices()?;
        self.emit_status();

        let response = {
            let state = self.state.lock().unwrap();
            LanWireMessage::PairResponse {
                protocol_version: PROTOCOL_VERSION,
                device_id: state.device_id.clone(),
                device_name: state.device_name.clone(),
                tcp_port: state.tcp_port,
                accepted: true,
            }
        };

        self.send_message(&request.address, request.tcp_port, &response)
    }

    pub fn reject_device(&self, device_id: &str) -> Result<(), String> {
        let request = {
            let mut state = self.state.lock().unwrap();
            state
                .pending_requests
                .remove(device_id)
                .ok_or_else(|| "未找到待处理配对请求".to_string())?
        };

        self.emit_status();

        let response = {
            let state = self.state.lock().unwrap();
            LanWireMessage::PairResponse {
                protocol_version: PROTOCOL_VERSION,
                device_id: state.device_id.clone(),
                device_name: state.device_name.clone(),
                tcp_port: state.tcp_port,
                accepted: false,
            }
        };

        self.send_message(&request.address, request.tcp_port, &response)
    }

    pub fn remove_trusted_device(&self, device_id: &str) -> Result<(), String> {
        let removed = {
            let mut state = self.state.lock().unwrap();
            state.trusted_devices.remove(device_id).is_some()
        };

        if !removed {
            return Err("未找到已信任设备".to_string());
        }

        self.persist_trusted_devices()?;
        self.emit_status();
        Ok(())
    }

    pub fn broadcast_text(&self, text: String) {
        let message = {
            let mut state = self.state.lock().unwrap();
            if !state.running {
                return;
            }

            let message_id = generate_message_id(&state.device_id);
            remember_message_id(&mut state, message_id.clone());

            LanWireMessage::SyncText {
                protocol_version: PROTOCOL_VERSION,
                device_id: state.device_id.clone(),
                device_name: state.device_name.clone(),
                tcp_port: state.tcp_port,
                message_id,
                created_at: Utc::now().to_rfc3339(),
                text,
            }
        };

        let peers = {
            let state = self.state.lock().unwrap();
            state
                .trusted_devices
                .values()
                .cloned()
                .collect::<Vec<_>>()
        };

        for peer in peers {
            let send_result = self.send_message(&peer.address, peer.tcp_port, &message);
            if let Err(error) = send_result {
                self.set_last_error(format!("发送同步消息失败: {}", error));
            }
        }
    }

    fn spawn_tcp_listener(&self, generation: u64) {
        let service = self.clone();

        thread::spawn(move || {
            let bind_addr = {
                let state = service.state.lock().unwrap();
                format!("0.0.0.0:{}", state.tcp_port)
            };

            let listener = match TcpListener::bind(&bind_addr) {
                Ok(listener) => listener,
                Err(error) => {
                    service.set_last_error(format!("监听同步端口失败: {}", error));
                    return;
                }
            };

            let _ = listener.set_nonblocking(true);

            while service.is_generation_active(generation) {
                match listener.accept() {
                    Ok((mut stream, address)) => {
                        let _ = stream.set_read_timeout(Some(Duration::from_millis(SOCKET_TIMEOUT_MS)));
                        let _ = stream.set_write_timeout(Some(Duration::from_millis(SOCKET_TIMEOUT_MS)));

                        let mut buffer = String::new();
                        if let Err(error) = stream.read_to_string(&mut buffer) {
                            service.set_last_error(format!("读取同步消息失败: {}", error));
                            continue;
                        }

                        if buffer.trim().is_empty() {
                            continue;
                        }

                        match serde_json::from_str::<LanWireMessage>(&buffer) {
                            Ok(message) => service.handle_incoming_message(message, address.ip().to_string()),
                            Err(error) => {
                                service.set_last_error(format!("解析同步消息失败: {}", error));
                            }
                        }
                    }
                    Err(error) if error.kind() == std::io::ErrorKind::WouldBlock => {
                        thread::sleep(Duration::from_millis(120));
                    }
                    Err(error) => {
                        service.set_last_error(format!("接受同步连接失败: {}", error));
                        thread::sleep(Duration::from_millis(300));
                    }
                }
            }
        });
    }

    fn spawn_udp_listener(&self, generation: u64) {
        let service = self.clone();

        thread::spawn(move || {
            let bind_addr = {
                let state = service.state.lock().unwrap();
                format!("0.0.0.0:{}", state.discovery_port)
            };

            let socket = match UdpSocket::bind(&bind_addr) {
                Ok(socket) => socket,
                Err(error) => {
                    service.set_last_error(format!("监听发现端口失败: {}", error));
                    return;
                }
            };

            let _ = socket.set_read_timeout(Some(Duration::from_millis(SOCKET_TIMEOUT_MS)));

            while service.is_generation_active(generation) {
                let mut buffer = [0_u8; 2048];
                match socket.recv_from(&mut buffer) {
                    Ok((size, address)) => {
                        let Ok(payload) = std::str::from_utf8(&buffer[..size]) else {
                            continue;
                        };

                        let Ok(message) = serde_json::from_str::<LanWireMessage>(payload) else {
                            continue;
                        };

                        if let LanWireMessage::Heartbeat {
                            protocol_version,
                            device_id,
                            device_name,
                            tcp_port,
                        } = message
                        {
                            if protocol_version != PROTOCOL_VERSION {
                                continue;
                            }

                            service.upsert_discovered_device(
                                device_id,
                                device_name,
                                address.ip().to_string(),
                                tcp_port,
                            );
                        }
                    }
                    Err(error)
                        if error.kind() == std::io::ErrorKind::TimedOut
                            || error.kind() == std::io::ErrorKind::WouldBlock =>
                    {
                        service.cleanup_stale_devices();
                    }
                    Err(error) => {
                        service.set_last_error(format!("监听发现广播失败: {}", error));
                        thread::sleep(Duration::from_millis(300));
                    }
                }
            }
        });
    }

    fn spawn_udp_broadcaster(&self, generation: u64) {
        let service = self.clone();

        thread::spawn(move || {
            let socket = match UdpSocket::bind((Ipv4Addr::UNSPECIFIED, 0)) {
                Ok(socket) => socket,
                Err(error) => {
                    service.set_last_error(format!("创建发现广播失败: {}", error));
                    return;
                }
            };

            let _ = socket.set_broadcast(true);

            while service.is_generation_active(generation) {
                let (discovery_port, payload) = {
                    let state = service.state.lock().unwrap();
                    let payload = LanWireMessage::Heartbeat {
                        protocol_version: PROTOCOL_VERSION,
                        device_id: state.device_id.clone(),
                        device_name: state.device_name.clone(),
                        tcp_port: state.tcp_port,
                    };
                    (state.discovery_port, payload)
                };

                if let Ok(json) = serde_json::to_string(&payload) {
                    let target = SocketAddr::from((Ipv4Addr::BROADCAST, discovery_port));
                    if let Err(error) = socket.send_to(json.as_bytes(), target) {
                        service.set_last_error(format!("发送发现广播失败: {}", error));
                    }
                }

                let mut remaining_ms = DISCOVERY_INTERVAL_SECS * 1000;
                while remaining_ms > 0 && service.is_generation_active(generation) {
                    let step_ms = remaining_ms.min(200);
                    thread::sleep(Duration::from_millis(step_ms));
                    remaining_ms -= step_ms;
                }
            }
        });
    }

    fn schedule_runtime_start(&self, generation: u64, delay_ms: u64) {
        let service = self.clone();

        thread::spawn(move || {
            if delay_ms > 0 {
                thread::sleep(Duration::from_millis(delay_ms));
            }

            if !service.is_generation_active(generation) {
                return;
            }

            service.spawn_tcp_listener(generation);

            let discovery_enabled = {
                let state = service.state.lock().unwrap();
                state.discovery_enabled
            };

            if discovery_enabled {
                service.spawn_udp_listener(generation);
                service.spawn_udp_broadcaster(generation);
            }
        });
    }

    fn handle_incoming_message(&self, message: LanWireMessage, address: String) {
        match message {
            LanWireMessage::Heartbeat {
                protocol_version,
                device_id,
                device_name,
                tcp_port,
            } => {
                if protocol_version != PROTOCOL_VERSION {
                    return;
                }

                self.upsert_discovered_device(device_id, device_name, address, tcp_port);
            }
            LanWireMessage::PairRequest {
                protocol_version,
                device_id,
                device_name,
                tcp_port,
            } => {
                if protocol_version != PROTOCOL_VERSION || self.is_local_device(&device_id) {
                    return;
                }

                self.upsert_discovered_device(
                    device_id.clone(),
                    device_name.clone(),
                    address.clone(),
                    tcp_port,
                );

                let should_emit = {
                    let mut state = self.state.lock().unwrap();
                    if state.trusted_devices.contains_key(&device_id) {
                        false
                    } else {
                        state.pending_requests.insert(
                            device_id.clone(),
                            LanPairingRequest {
                                device_id,
                                device_name,
                                address: address.clone(),
                                tcp_port,
                                requested_at: Utc::now(),
                            },
                        );
                        true
                    }
                };

                if should_emit {
                    self.emit_status();
                } else {
                    let response = {
                        let state = self.state.lock().unwrap();
                        LanWireMessage::PairResponse {
                            protocol_version: PROTOCOL_VERSION,
                            device_id: state.device_id.clone(),
                            device_name: state.device_name.clone(),
                            tcp_port: state.tcp_port,
                            accepted: true,
                        }
                    };

                    if let Err(error) = self.send_message(&address, tcp_port, &response) {
                        self.set_last_error(format!("回复已信任设备失败: {}", error));
                    }
                }
            }
            LanWireMessage::PairResponse {
                protocol_version,
                device_id,
                device_name,
                tcp_port,
                accepted,
            } => {
                if protocol_version != PROTOCOL_VERSION || self.is_local_device(&device_id) {
                    return;
                }

                self.upsert_discovered_device(
                    device_id.clone(),
                    device_name.clone(),
                    address.clone(),
                    tcp_port,
                );

                if accepted {
                    {
                        let mut state = self.state.lock().unwrap();
                        state.trusted_devices.insert(
                            device_id.clone(),
                            StoredTrustedDevice {
                                device_id,
                                device_name,
                                address,
                                tcp_port,
                                last_seen: Utc::now(),
                            },
                        );
                    }

                    if let Err(error) = self.persist_trusted_devices() {
                        self.set_last_error(format!("保存信任设备失败: {}", error));
                    }
                } else {
                    self.set_last_error("对方拒绝了配对请求".to_string());
                }

                self.emit_status();
            }
            LanWireMessage::SyncText {
                protocol_version,
                device_id,
                device_name,
                tcp_port,
                message_id,
                text,
                ..
            } => {
                if protocol_version != PROTOCOL_VERSION || self.is_local_device(&device_id) {
                    return;
                }

                self.upsert_discovered_device(
                    device_id.clone(),
                    device_name,
                    address.clone(),
                    tcp_port,
                );

                let is_trusted = {
                    let mut state = self.state.lock().unwrap();

                    if state.recent_message_ids.iter().any(|existing| existing == &message_id) {
                        return;
                    }

                    remember_message_id(&mut state, message_id);
                    state.trusted_devices.contains_key(&device_id)
                };

                if !is_trusted {
                    return;
                }

                let clipboard_manager = self.clipboard_manager.clone();
                let app = self.app.clone();

                tauri::async_runtime::spawn(async move {
                    match clipboard_manager.handle_synced_text(text.clone()).await {
                        Ok(Some(_)) => {
                            let _ = app.emit("lan-sync-copy-text", text.clone());
                            let _ = app.emit("lan-sync-updated", text);
                        }
                        Ok(None) => {}
                        Err(error) => {
                            eprintln!("处理局域网同步文本失败: {}", error);
                        }
                    }
                });
            }
        }
    }

    fn upsert_discovered_device(
        &self,
        device_id: String,
        device_name: String,
        address: String,
        tcp_port: u16,
    ) {
        if self.is_local_device(&device_id) {
            return;
        }

        let mut emit_status = false;

        {
            let mut state = self.state.lock().unwrap();
            let trusted = state.trusted_devices.contains_key(&device_id);
            let now = Utc::now();

            match state.discovered_devices.get_mut(&device_id) {
                Some(device) => {
                    if device.address != address
                        || device.device_name != device_name
                        || device.tcp_port != tcp_port
                        || device.trusted != trusted
                    {
                        emit_status = true;
                    }

                    device.device_name = device_name.clone();
                    device.address = address.clone();
                    device.tcp_port = tcp_port;
                    device.trusted = trusted;
                    device.last_seen = now;
                }
                None => {
                    state.discovered_devices.insert(
                        device_id.clone(),
                        LanDiscoveredDevice {
                            device_id: device_id.clone(),
                            device_name: device_name.clone(),
                            address: address.clone(),
                            tcp_port,
                            trusted,
                            last_seen: now,
                        },
                    );
                    emit_status = true;
                }
            }

            if let Some(trusted_device) = state.trusted_devices.get_mut(&device_id) {
                let changed = trusted_device.address != address
                    || trusted_device.device_name != device_name
                    || trusted_device.tcp_port != tcp_port;

                trusted_device.address = address;
                trusted_device.device_name = device_name;
                trusted_device.tcp_port = tcp_port;
                trusted_device.last_seen = now;

                if changed {
                    emit_status = true;
                }
            }
        }

        if let Err(error) = self.persist_trusted_devices() {
            self.set_last_error(format!("保存发现状态失败: {}", error));
        }

        if emit_status {
            self.emit_status();
        }
    }

    fn cleanup_stale_devices(&self) {
        let mut changed = false;

        {
            let mut state = self.state.lock().unwrap();
            let now = Utc::now();
            state.discovered_devices.retain(|_, device| {
                let keep = (now - device.last_seen).num_seconds() <= DISCOVERY_STALE_SECS;
                if !keep {
                    changed = true;
                }
                keep
            });
        }

        if changed {
            self.emit_status();
        }
    }

    fn emit_status(&self) {
        self.cleanup_stale_devices();
        let _ = self
            .app
            .emit("lan-sync-status-changed", self.get_status());
    }

    fn set_last_error(&self, error: String) {
        {
            let mut state = self.state.lock().unwrap();
            state.last_error = Some(error);
        }
        self.emit_status();
    }

    fn persist_trusted_devices(&self) -> Result<(), String> {
        let trusted_devices = {
            let state = self.state.lock().unwrap();
            state
                .trusted_devices
                .values()
                .cloned()
                .collect::<Vec<_>>()
        };

        let path = self.app_dir.join("lan-sync-trusted-devices.json");
        let json = serde_json::to_string_pretty(&trusted_devices)
            .map_err(|error| format!("序列化信任设备失败: {}", error))?;
        fs::create_dir_all(&self.app_dir).map_err(|error| error.to_string())?;
        fs::write(path, json).map_err(|error| error.to_string())
    }

    fn send_message(
        &self,
        address: &str,
        port: u16,
        message: &LanWireMessage,
    ) -> Result<(), String> {
        let json = serde_json::to_string(message)
            .map_err(|error| format!("序列化消息失败: {}", error))?;
        let socket_address: SocketAddr = format!("{}:{}", address, port)
            .parse()
            .map_err(|error| format!("地址格式无效: {}", error))?;

        let mut stream = TcpStream::connect_timeout(&socket_address, Duration::from_millis(SOCKET_TIMEOUT_MS))
            .map_err(|error| error.to_string())?;
        stream
            .set_read_timeout(Some(Duration::from_millis(SOCKET_TIMEOUT_MS)))
            .map_err(|error| error.to_string())?;
        stream
            .set_write_timeout(Some(Duration::from_millis(SOCKET_TIMEOUT_MS)))
            .map_err(|error| error.to_string())?;
        stream
            .write_all(json.as_bytes())
            .map_err(|error| error.to_string())?;
        stream
            .shutdown(Shutdown::Write)
            .map_err(|error| error.to_string())
    }

    fn is_generation_active(&self, generation: u64) -> bool {
        let running = {
            let state = self.state.lock().unwrap();
            state.running
        };

        running && self.generation.load(Ordering::SeqCst) == generation
    }

    fn is_local_device(&self, device_id: &str) -> bool {
        let state = self.state.lock().unwrap();
        state.device_id == device_id
    }
}

fn sanitize_port(value: i64, fallback: u16) -> u16 {
    if (1..=u16::MAX as i64).contains(&value) {
        value as u16
    } else {
        fallback
    }
}

fn normalize_device_name(value: &str) -> String {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        "我的设备".to_string()
    } else {
        trimmed.to_string()
    }
}

fn load_or_create_device_id(app_dir: &PathBuf) -> String {
    let path = app_dir.join("lan-sync-device-id.txt");
    if let Ok(value) = fs::read_to_string(&path) {
        let trimmed = value.trim();
        if !trimmed.is_empty() {
            return trimmed.to_string();
        }
    }

    let generated = generate_message_id("device");
    let _ = fs::create_dir_all(app_dir);
    let _ = fs::write(path, &generated);
    generated
}

fn load_trusted_devices(app_dir: &PathBuf) -> HashMap<String, StoredTrustedDevice> {
    let path = app_dir.join("lan-sync-trusted-devices.json");
    let Ok(content) = fs::read_to_string(path) else {
        return HashMap::new();
    };

    let Ok(devices) = serde_json::from_str::<Vec<StoredTrustedDevice>>(&content) else {
        return HashMap::new();
    };

    devices
        .into_iter()
        .map(|device| (device.device_id.clone(), device))
        .collect()
}

fn remember_message_id(state: &mut LanServiceState, message_id: String) {
    state.recent_message_ids.push_back(message_id);
    while state.recent_message_ids.len() > RECENT_MESSAGE_CACHE_SIZE {
        state.recent_message_ids.pop_front();
    }
}

fn generate_message_id(prefix: &str) -> String {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos();
    format!("{}-{:x}", prefix, timestamp)
}
