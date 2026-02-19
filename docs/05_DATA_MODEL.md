# 05 - 数据模型与 API

> **范围**: TypeScript 类型、数据库 Schema、局域网同步  
> **目标**: 数据结构定义  
> **最后更新**: 2026-02-19

---

## 一、TypeScript 类型定义

### 1.1 基础类型

```typescript
/**
 * 剪贴板内容类型
 */
export type ClipboardContentType =
  | 'text'
  | 'html'
  | 'rtf'
  | 'image'
  | 'file'
  | 'folder'
  | 'files';

/**
 * 剪贴板元数据
 */
export interface ClipboardMetadata {
  // 图片相关
  width?: number;
  height?: number;
  format?: string;

  // 文件相关
  file_name?: string;
  file_size?: number;
  mime_type?: string;

  // 文件夹相关
  folder_name?: string;
  item_count?: number;
}

/**
 * 标签定义
 */
export interface Tag {
  id: string;        // UUID
  name: string;      // 标签名称
  color: string;     // hex 颜色
  created_at: string;
}

/**
 * 剪贴板历史记录项
 */
export interface ClipboardItem {
  id: number;                           // 本地自增 ID
  uuid: string;                         // UUIDv7（全局唯一，用于同步）
  content_type: ClipboardContentType;
  content: string;                      // 文本内容或路径
  content_hash: string;                 // SHA256 哈希
  created_at: string;                   // ISO 8601
  updated_at: string;                   // ISO 8601
  device_id: string;                    // 创建设备
  is_favorite: boolean;                 // 收藏状态
  tags?: string[];                      // 标签 ID 列表
  metadata?: ClipboardMetadata;
  file_paths?: string[];                // 文件路径列表
  thumbnail_path?: string;              // 缩略图路径
  
  // 同步相关（预留）
  vector_clock?: Record<string, number>; // 向量时钟
  sync_status?: 'local' | 'pending' | 'synced' | 'conflict';
}

/**
 * 应用设置
 */
export interface AppSettings {
  // 历史记录设置
  max_history_count: number;
  auto_cleanup_days: number;

  // 窗口设置
  window_position: 'remember' | 'center' | 'cursor';
  window_pos_x?: number;
  window_pos_y?: number;

  // 智能激活
  smart_activate: boolean;
  focus_search_on_activate: boolean;

  // 音效设置
  copy_sound: boolean;

  // 搜索设置
  search_position: 'top' | 'bottom';

  // 内容设置
  click_action: 'copy' | 'paste' | 'none';
  double_click_action: 'copy' | 'paste' | 'none';
  paste_shortcut: 'ctrl_v' | 'shift_insert';
  hide_window_after_copy: boolean;
  image_ocr: boolean;
  copy_as_plain_text: boolean;
  paste_as_plain_text: boolean;
  confirm_delete: boolean;
  auto_sort: boolean;

  // 通用设置
  hotkey: string;
  auto_start: boolean;
  
  // 局域网同步
  lan_sync_enabled: boolean;
  device_name: string;
  device_id: string;

  // 快捷键设置
  number_key_shortcut: string;
}

/**
 * 获取历史记录响应
 */
export interface GetHistoryResponse {
  items: ClipboardItem[];
  total: number;
}

/**
 * 搜索请求
 */
export interface SearchRequest {
  query: string;
  limit?: number;
  offset?: number;
  content_type?: ClipboardContentType;
}

/**
 * 存储路径
 */
export interface StoragePaths {
  data_path: string;
  thumbnail_path: string;
  log_path: string;
}

/**
 * 同步设备
 */
export interface SyncDevice {
  device_id: string;
  device_name: string;
  platform: 'windows' | 'macos' | 'linux' | 'ios' | 'android';
  address: string;
  port: number;
  is_trusted: boolean;
  last_seen: string;
  status: 'online' | 'offline';
}

/**
 * 同步消息
 */
export interface SyncMessage {
  type: 'ping' | 'pong' | 'auth' | 'auth_success' | 'auth_failed' 
      | 'clipboard_item' | 'sync_request' | 'sync_response' | 'new_item' | 'item_deleted';
  payload?: unknown;
  timestamp: string;
  device_id: string;
}
```

### 1.2 局域网同步类型

```typescript
/**
 * 同步协议消息
 */
export enum SyncMessageType {
  Ping = 'ping',
  Pong = 'pong',
  Auth = 'auth',
  AuthSuccess = 'auth_success',
  AuthFailed = 'auth_failed',
  ClipboardItem = 'clipboard_item',
  SyncRequest = 'sync_request',
  SyncResponse = 'sync_response',
  NewItem = 'new_item',
  ItemDeleted = 'item_deleted',
}

/**
 * 同步剪贴板项（网络传输格式）
 */
export interface SyncClipboardItem {
  uuid: string;
  content_type: ClipboardContentType;
  content_hash: string;
  content: string;           // 加密后的内容
  created_at: string;
  device_id: string;
  vector_clock: Record<string, number>;
  metadata?: ClipboardMetadata;
}

/**
 * 设备认证信息
 */
export interface DeviceAuth {
  device_id: string;
  device_name: string;
  platform: string;
  pairing_code_hash: string;  // 配对码哈希
  public_key?: string;        // 可选：公钥
}
```

---

## 二、数据库 Schema

### 2.1 核心表结构

```sql
-- 剪贴板历史表
CREATE TABLE clipboard_history (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    uuid TEXT UNIQUE NOT NULL,           -- 全局唯一 ID (UUIDv7)
    content_type TEXT NOT NULL,          -- text/html/rtf/image/file/folder/files
    content TEXT NOT NULL,               -- 文本内容或文件路径
    content_hash TEXT NOT NULL UNIQUE,   -- SHA256 哈希
    created_at TEXT NOT NULL,            -- ISO 8601
    updated_at TEXT NOT NULL,            -- ISO 8601
    device_id TEXT NOT NULL,             -- 创建设备 ID
    is_favorite INTEGER DEFAULT 0,       -- 0=false, 1=true
    tags TEXT,                           -- JSON 数组 [tag_id1, tag_id2]
    metadata TEXT,                       -- JSON: {width, height, file_name, ...}
    file_paths TEXT,                     -- JSON 数组（多文件）
    thumbnail_path TEXT,                 -- 缩略图路径
    
    -- 同步相关（预留）
    vector_clock TEXT,                   -- JSON: {device_id: count}
    sync_status TEXT DEFAULT 'local'     -- local/pending/synced/conflict
);

-- 标签表
CREATE TABLE tags (
    id TEXT PRIMARY KEY,                 -- UUID
    name TEXT NOT NULL UNIQUE,           -- 标签名称
    color TEXT NOT NULL,                 -- hex 颜色
    created_at TEXT NOT NULL             -- ISO 8601
);

-- 设备表（局域网同步）
CREATE TABLE sync_devices (
    device_id TEXT PRIMARY KEY,          -- 设备唯一 ID
    device_name TEXT NOT NULL,           -- 设备名称
    platform TEXT NOT NULL,              -- windows/macos/linux/ios/android
    address TEXT,                        -- IP 地址
    port INTEGER DEFAULT 8765,           -- 端口号
    is_trusted INTEGER DEFAULT 0,        -- 0=false, 1=true
    pairing_code_hash TEXT,              -- 配对码哈希
    public_key TEXT,                     -- 公钥（可选）
    last_seen TEXT,                      -- ISO 8601
    created_at TEXT NOT NULL             -- ISO 8601
);

-- 同步队列（离线支持）
CREATE TABLE sync_queue (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    item_uuid TEXT NOT NULL,             -- 剪贴板项 UUID
    operation TEXT NOT NULL,             -- add/delete/update
    target_device_id TEXT,               -- 目标设备（null=广播）
    created_at TEXT NOT NULL,            -- ISO 8601
    retry_count INTEGER DEFAULT 0        -- 重试次数
);
```

### 2.2 索引

```sql
-- 核心索引
CREATE INDEX idx_content_hash ON clipboard_history(content_hash);
CREATE INDEX idx_created_at ON clipboard_history(created_at DESC);
CREATE INDEX idx_updated_at ON clipboard_history(updated_at DESC);
CREATE INDEX idx_content_type ON clipboard_history(content_type);
CREATE INDEX idx_is_favorite ON clipboard_history(is_favorite);

-- 搜索索引
CREATE INDEX idx_content_search ON clipboard_history(content);

-- 同步索引
CREATE INDEX idx_sync_status ON clipboard_history(sync_status, updated_at);
CREATE INDEX idx_device_id ON clipboard_history(device_id);
CREATE INDEX idx_uuid ON clipboard_history(uuid);

-- 标签索引
CREATE INDEX idx_tag_name ON tags(name);
```

### 2.3 初始化数据

```sql
-- 预置标签颜色（可选）
INSERT INTO tags (id, name, color, created_at) VALUES
('tag-work', '工作', '#1890ff', datetime('now')),
('tag-personal', '个人', '#52c41a', datetime('now')),
('tag-code', '代码', '#722ed1', datetime('now'));
```

---

## 三、局域网同步设计

### 3.1 架构设计

```
┌─ Windows ─┐      mDNS 自动发现      ┌─ macOS ─┐
│ Desktop   │◄─────────────────────►│ Desktop │
│ (全功能)   │                         │ (全功能) │
└─────┬─────┘                         └────┬────┘
      │                                    │
      │         TCP 长连接通信              │
      │         端口: 8765 (默认)           │
      └──────────────┬─────────────────────┘
                     │
              ┌──────▼──────┐
              │   iOS App   │  ← 仅接收 + 手动发送
              │ (受限功能)   │
              └─────────────┘
                     │
              ┌──────▼──────┐
              │ Android App │  ← 仅接收 + 手动发送
              │ (受限功能)   │
              └─────────────┘
```

### 3.2 核心技术选型

| 组件 | 技术 | 说明 |
|------|------|------|
| 设备发现 | mDNS (DNS-SD) | Bonjour/Zeroconf，跨平台 |
| 通信协议 | TCP + 自定义协议 | 长连接，JSON 消息 |
| 加密 | AES-256-GCM | 对称加密 |
| 密钥派生 | Argon2 | 从配对码派生密钥 |
| 防循环 | UUID + LRU Cache | 防止广播风暴 |

### 3.3 mDNS 服务注册

```rust
use mdns_sd::{ServiceDaemon, ServiceInfo};

const SERVICE_TYPE: &str = "_paste-lib._tcp.local.";
const PORT: u16 = 8765;

pub fn register_service(device_id: &str, device_name: &str) {
    let mdns = ServiceDaemon::new().expect("Failed to create mDNS daemon");
    
    let service_info = ServiceInfo::new(
        SERVICE_TYPE,
        device_name,
        &format!("{}.local.", device_name),
        "",                               // 自动获取 IP
        PORT,
        &[
            ("device_id", device_id),
            ("platform", std::env::consts::OS),
            ("version", env!("CARGO_PKG_VERSION")),
        ][..],
    ).expect("valid service info");
    
    mdns.register(service_info).expect("Failed to register");
}
```

### 3.4 TCP 通信协议

**消息格式**:
```
[4字节长度][JSON 内容]
```

**消息类型**:

```rust
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum SyncMessage {
    // 心跳
    Ping,
    Pong,
    
    // 身份验证
    Auth {
        device_id: String,
        device_name: String,
        platform: String,
        pairing_code_hash: String,
    },
    AuthSuccess,
    AuthFailed {
        reason: String,
    },
    
    // 剪贴板数据
    ClipboardItem {
        item: SyncClipboardItem,
    },
    
    // 批量同步
    SyncRequest {
        since: String,  // ISO 8601
    },
    SyncResponse {
        items: Vec<SyncClipboardItem>,
    },
    
    // 实时通知
    NewItem {
        item: SyncClipboardItem,
    },
    ItemDeleted {
        uuid: String,
    },
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SyncClipboardItem {
    pub uuid: String,
    pub content_type: String,
    pub content_hash: String,
    pub content: Vec<u8>,  // 加密后的内容
    pub created_at: String,
    pub device_id: String,
    pub vector_clock: HashMap<String, u64>,
    pub metadata: Option<String>,  // JSON
}
```

### 3.5 加密方案

**密钥派生**:
```rust
use argon2::{Argon2, PasswordHasher};
use aes_gcm::{Aes256Gcm, Key, Nonce};
use aes_gcm::aead::{Aead, KeyInit, OsRng};

/// 从配对码派生 32 字节密钥
pub fn derive_key(pairing_code: &str) -> [u8; 32] {
    let salt = b"paste-lib-salt-v1";
    let argon2 = Argon2::default();
    let mut key = [0u8; 32];
    argon2.hash_password_into(
        pairing_code.as_bytes(),
        salt,
        &mut key,
    ).expect("Key derivation failed");
    key
}

/// 加密数据
pub fn encrypt(plaintext: &[u8], key: &[u8; 32]) -> Vec<u8> {
    let cipher = Aes256Gcm::new(Key::<Aes256Gcm>::from_slice(key));
    let mut nonce = [0u8; 12];
    OsRng.fill_bytes(&mut nonce);
    
    let ciphertext = cipher
        .encrypt(Nonce::from_slice(&nonce), plaintext)
        .expect("Encryption failed");
    
    // [nonce (12 bytes)][ciphertext]
    [nonce.as_slice(), &ciphertext].concat()
}

/// 解密数据
pub fn decrypt(ciphertext: &[u8], key: &[u8; 32]) -> Option<Vec<u8>> {
    if ciphertext.len() < 12 {
        return None;
    }
    
    let (nonce, encrypted) = ciphertext.split_at(12);
    let cipher = Aes256Gcm::new(Key::<Aes256Gcm>::from_slice(key));
    
    cipher.decrypt(Nonce::from_slice(nonce), encrypted).ok()
}
```

### 3.6 防循环机制

```rust
use lru::LruCache;
use std::num::NonZeroUsize;
use std::sync::Mutex;
use once_cell::sync::Lazy;

// 记录最近处理的 UUID，防止 A→B→C→A 循环
static SEEN_UUIDS: Lazy<Mutex<LruCache<String, ()>>> = 
    Lazy::new(|| Mutex::new(LruCache::new(NonZeroUsize::new(1000).unwrap())));

/// 处理接收到的剪贴板项
pub async fn handle_incoming_item(item: ClipboardItem, from_device: &str) {
    let mut seen = SEEN_UUIDS.lock().unwrap();
    
    // 检查是否已处理过
    if seen.get(&item.uuid).is_some() {
        return; // 忽略重复
    }
    
    // 记录已见
    seen.put(item.uuid.clone(), ());
    drop(seen);
    
    // 保存到本地
    save_to_local_db(&item);
    
    // 转发给其他设备（除了来源）
    broadcast_to_other_peers(&item, from_device).await;
}
```

### 3.7 向量时钟（冲突检测）

```rust
use std::collections::HashMap;

/// 向量时钟
#[derive(Debug, Clone, Default)]
pub struct VectorClock(pub HashMap<String, u64>);

impl VectorClock {
    /// 递增本地设备计数
    pub fn increment(&mut self, device_id: &str) {
        *self.0.entry(device_id.to_string()).or_insert(0) += 1;
    }
    
    /// 合并两个向量时钟
    pub fn merge(&mut self, other: &VectorClock) {
        for (device, count) in &other.0 {
            let entry = self.0.entry(device.clone()).or_insert(0);
            *entry = (*entry).max(*count);
        }
    }
}

/// 比较向量时钟
#[derive(Debug, PartialEq)]
pub enum VectorClockOrdering {
    Before,      // self 发生在 other 之前
    After,       // self 发生在 other 之后
    Concurrent,  // 并发，需要冲突解决
}

pub fn compare_vector_clocks(a: &VectorClock, b: &VectorClock) -> VectorClockOrdering {
    let mut a_before_b = true;
    let mut b_before_a = true;
    
    // 检查 a 是否在 b 之前
    for (device, count_a) in &a.0 {
        let count_b = b.0.get(device).copied().unwrap_or(0);
        if count_a > &count_b {
            a_before_b = false;
        }
        if count_a < &count_b {
            b_before_a = false;
        }
    }
    
    // 检查 b 中独有的设备
    for (device, count_b) in &b.0 {
        let count_a = a.0.get(device).copied().unwrap_or(0);
        if count_b > &count_a {
            b_before_a = false;
        }
        if count_b < &count_a {
            a_before_b = false;
        }
    }
    
    match (a_before_b, b_before_a) {
        (true, false) => VectorClockOrdering::Before,
        (false, true) => VectorClockOrdering::After,
        (true, true) => VectorClockOrdering::Before, // 相等
        (false, false) => VectorClockOrdering::Concurrent,
    }
}
```

### 3.8 移动端限制

**iOS 限制**:
- 后台无法访问剪贴板
- 前台访问会弹出系统提示
- 后台任务最多 30 秒

**Android 限制**:
- Android 10+ 仅前台可读取剪贴板
- 后台服务可能被系统杀死

**移动端策略**:
- 手动同步模式（打开 App → 点击同步 → 拉取数据）
- 系统分享扩展（分享菜单发送到桌面端）
- 推送通知（可选增强）

---

## 四、API 汇总

### 4.1 剪贴板 API

| 方法 | 参数 | 返回值 | 说明 |
|------|------|--------|------|
| `add_clipboard_item` | `{text, html?, is_internal_copy?}` | `ClipboardItem` | 添加文本 |
| `add_clipboard_item_extended` | `{content_type, content, ...}` | `ClipboardItem` | 添加图片/文件 |
| `get_clipboard_history` | `{limit?, offset?}` | `GetHistoryResponse` | 获取历史 |
| `search_clipboard_history` | `{query, limit?}` | `ClipboardItem[]` | 搜索 |
| `delete_clipboard_item` | `{id}` | `boolean` | 删除 |
| `clear_clipboard_history` | - | `boolean` | 清空 |
| `update_item_tags` | `{id, tags}` | `ClipboardItem` | 更新标签 |
| `create_tag` | `{name}` | `Tag` | 创建标签 |
| `delete_tag` | `{id}` | `boolean` | 删除标签 |

### 4.2 队列 API

| 方法 | 参数 | 返回值 | 说明 |
|------|------|--------|------|
| `get_paste_queue` | - | `ClipboardItem[]` | 获取队列 |
| `add_to_queue` | `{id}` | `boolean` | 添加 |
| `remove_from_queue` | `{id}` | `boolean` | 移除 |
| `reorder_queue` | `{ids}` | `boolean` | 重排序 |
| `clear_queue` | - | `boolean` | 清空 |
| `batch_paste` | `{separator}` | `boolean` | 批量粘贴 |

### 4.3 设置 API

| 方法 | 参数 | 返回值 | 说明 |
|------|------|--------|------|
| `get_settings` | - | `AppSettings` | 获取设置 |
| `save_settings` | `{settings}` | `boolean` | 保存设置 |
| `get_storage_paths` | - | `StoragePaths` | 获取路径 |
| `export_data` | `{path}` | `boolean` | 导出 |
| `import_data` | `{path}` | `boolean` | 导入 |

### 4.4 同步 API

| 方法 | 参数 | 返回值 | 说明 |
|------|------|--------|------|
| `lan_sync_start` | `{device_name, pairing_code}` | `boolean` | 启动服务 |
| `lan_sync_stop` | - | `boolean` | 停止服务 |
| `lan_sync_get_devices` | - | `SyncDevice[]` | 获取设备 |
| `lan_sync_pair_device` | `{device_id, pairing_code}` | `boolean` | 配对 |
| `lan_sync_unpair_device` | `{device_id}` | `boolean` | 取消配对 |
| `lan_sync_pull_from_device` | `{device_id}` | `ClipboardItem[]` | 拉取 |

---

## 相关文档

- [01_OVERVIEW.md](./01_OVERVIEW.md) - 项目概述
- [02_PRD.md](./02_PRD.md) - 产品需求
- [03_ARCHITECTURE.md](./03_ARCHITECTURE.md) - 技术架构
- [04_UI_SPEC.md](./04_UI_SPEC.md) - UI 设计规范
- [06_IMPLEMENTATION.md](./06_IMPLEMENTATION.md) - 实施计划
