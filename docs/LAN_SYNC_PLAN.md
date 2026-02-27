# 局域网剪贴板同步 - 开发计划

## 概述

为 paste-library 添加局域网设备发现与剪贴板同步功能，支持在同一局域网内的多设备间自动同步剪贴板内容。

### 核心特性

- **设备发现**: 自动发现局域网内的 paste-library 设备
- **设备配对**: PIN 码确认式配对
- **自动同步**: 配对后自动同步剪贴板内容
- **多平台支持**: 当前 Windows/Mac，后续支持 Android
- **互联网预留**: 架构支持未来互联网同步

---

## 技术方案

### 1. 设备发现

| 技术 | 选型 | 理由 |
|------|------|------|
| 发现协议 | mDNS (multicast DNS) | 零配置，跨平台，主流操作系统原生支持 |
| 备选 | UDP 广播 | 网络环境不支持 mDNS 时的降级方案 |

**服务类型**: `_pastelib._tcp`

### 2. 传输协议

| 场景 | 协议 | 加密 |
|------|------|------|
| 设备发现 | mDNS DNS-SD | 无需 |
| 配对认证 | TCP (WebSocket) | 首次 PIN 验证 |
| 数据同步 | TCP (自定义二进制协议) | ChaCha20Poly1305 |

### 3. 数据同步模式

- **引用同步**: 仅同步剪贴板元数据（内容Hash、类型、时间戳、设备ID）
- **按需传输**: 对方点击查看时再传输实际内容
- **文本直传**: 小文本直接通过 TCP 发送

### 4. 冲突策略

- **时间戳胜出**: 最新修改覆盖旧内容
- **删除同步**: 删除操作同步到所有设备
- **标签独立**: 每个设备的标签独立维护（不同步）

---

## 数据模型

### 新增类型定义

```typescript
// src/types/index.ts

// 设备信息
interface SyncDevice {
  id: string;              // 设备唯一ID (UUID)
  name: string;            // 设备名称 (用户自定义)
  platform: 'windows' | 'macos' | 'linux' | 'android';
  lastSeen: number;        // 最后在线时间戳
  isOnline: boolean;       // 当前是否在线
  isPaired: boolean;       // 是否已配对
}

// 配对请求
interface PairingRequest {
  deviceId: string;
  deviceName: string;
  pin: string;             // 6位数字PIN
  timestamp: number;
}

// 同步消息
interface SyncMessage {
  type: 'text' | 'image' | 'file' | 'delete' | 'ack';
  contentHash: string;     // SHA256
  sourceDeviceId: string;
  timestamp: number;
  payload?: string;       // 文本内容或Base64缩略图
}

// 同步状态
interface SyncState {
  enabled: boolean;
  autoSync: boolean;
  pairedDevices: SyncDevice[];
  pendingRequests: PairingRequest[];
}
```

### 数据库 Schema 变更

```sql
-- 设备表
CREATE TABLE sync_devices (
  id TEXT PRIMARY KEY,
  name TEXT NOT NULL,
  platform TEXT NOT NULL,
  last_seen INTEGER NOT NULL,
  is_paired INTEGER DEFAULT 0,
  created_at INTEGER NOT NULL
);

-- 同步历史（可选，用于冲突解决）
CREATE TABLE sync_history (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  content_hash TEXT NOT NULL,
  source_device_id TEXT NOT NULL,
  action TEXT NOT NULL, -- 'sync', 'delete'
  timestamp INTEGER NOT NULL
);

-- 设置表新增字段
ALTER TABLE settings ADD COLUMN sync_enabled INTEGER DEFAULT 0;
ALTER TABLE settings ADD COLUMN auto_sync INTEGER DEFAULT 1;
```

---

## Rust 后端设计

### 新增模块

```
src-tauri/src/
├── sync/
│   ├── mod.rs            # 模块入口
│   ├── discovery.rs      # mDNS 设备发现
│   ├── pairing.rs        # 配对逻辑
│   ├── connection.rs     # P2P 连接管理
│   ├── protocol.rs       # 二进制协议
│   └── crypto.rs         # ChaCha20 加密
```

### Tauri 命令

```rust
// 设备发现
#[tauri::command]
async fn start_device_discovery() -> Result<(), String>;

#[tauri::command]
async fn stop_device_discovery() -> Result<(), String>;

#[tauri::command]
async fn get_discovered_devices() -> Result<Vec<SyncDevice>, String>;

// 配对
#[tauri::command]
async fn request_pair(device_id: String) -> Result<String, String>; // 返回PIN

#[tauri::command]
async fn confirm_pair(device_id: String, pin: String) -> Result<(), String>;

#[tauri::command]
async fn unpair_device(device_id: String) -> Result<(), String>;

// 同步控制
#[tauri::command]
async fn get_paired_devices() -> Result<Vec<SyncDevice>, String>;

#[tauri::command]
async fn set_sync_enabled(enabled: bool) -> Result<(), String>;

#[tauri::command]
async fn set_auto_sync(enabled: bool) -> Result<(), String>;

// 内容获取（按需传输）
#[tauri::command]
async fn request_content(content_hash: String, device_id: String) -> Result<Vec<u8>, String>;
```

### 消息协议（二进制）

```
[1 byte: type][4 bytes: length][N bytes: payload][16 bytes: nonce/tag]

类型:
  0x01 - 文本
  0x02 - 图片元数据
  0x03 - 文件元数据
  0x04 - 删除
  0x05 - ACK
  0x10 - 内容请求
  0x11 - 内容响应
```

---

## 前端设计

### 设置面板新增区域

```
设置面板/
├── 剪贴板
├── 历史记录
├── 同步设置（新增）
├── 通用设置
├── 快捷键
├── 数据备份
└── 关于
```

### 组件结构

```
src/components/settings/
├── sections/
│   └── SyncSection.vue     # 同步设置主面板
```

---

## 实现阶段

### 阶段 1: 设备发现 + 前端设备列表 ✅ 已完成

**任务**:

1. **Rust - mDNS 发现服务**
   - ✅ 添加 `mdns-sd` crate 依赖
   - ✅ 实现 `sync/discovery.rs`
   - ✅ 实现 `sync/pairing.rs` (配对逻辑)
   - ✅ 实现 `sync/connection.rs` (连接管理)
   - ✅ 实现 `sync/protocol.rs` (二进制协议)
   - ✅ 实现 `sync/crypto.rs` (ChaCha20加密)
   - ✅ 注册 `_pastelib._tcp` 服务

2. **Tauri 命令**
   - ✅ `start_device_discovery()`
   - ✅ `stop_device_discovery()`
   - ✅ `get_discovered_devices()`
   - ✅ `get_paired_devices()`
   - ✅ `request_pair()`
   - ✅ `confirm_pair()`
   - ✅ `unpair_device()`
   - ✅ `set_sync_enabled()`
   - ✅ `set_auto_sync()`

3. **前端 - 同步设置 UI**
   - ✅ 创建 `SyncSection.vue`
   - ✅ 集成到设置面板导航
   - ✅ 添加同步设置菜单项

4. **类型定义**
   - ✅ 添加 `SyncDevice`, `Platform`, `PairingRequest`, `SyncState`

**进度**: ✅ 100%

---

### 阶段 2: 配对机制 ⏳ 待开始

**任务**:

1. **Rust - 配对逻辑** (部分已完成)
   - ✅ PIN 生成（6位随机数）- 已在 pairing.rs 实现
   - ⏳ 实现 TCP 监听（配对端口）
   - ⏳ 实现设备认证
   - ⏳ 保存配对信息到数据库

2. **前端 - 配对交互**
   - ⏳ 配对请求弹窗
   - ⏳ PIN 显示与确认
   - ⏳ 配对结果反馈

3. **数据库**
   - ⏳ 创建 `sync_devices` 表
   - ⏳ 迁移设置表

**进度**: ⏳ 0%

---

### 阶段 3: P2P 连接与文本同步 ⏳ 待开始

**任务**:

1. **Rust - P2P 连接**
   - ⏳ 实现 TCP 连接池
   - ⏳ 心跳保活（30秒间隔）
   - ⏳ 断线重连逻辑

2. **同步协议**
   - ✅ 二进制协议 - 已在 protocol.rs 实现
   - ✅ ChaCha20 加密 - 已在 crypto.rs 实现
   - ⏳ 实现消息队列

3. **文本同步**
   - ⏳ 剪贴板变化检测
   - ⏳ 文本内容广播
   - ⏳ ACK 确认机制

**进度**: ⏳ 0%

---

### 阶段 4: 图片/文件引用同步 ⏳ 待开始

**任务**:

1. **引用同步**
   - ⏳ 仅同步内容Hash + 元数据
   - ⏳ 显示来源设备标签

2. **按需传输**
   - ⏳ 实现内容请求命令
   - ⏳ 实现大块数据传输
   - ⏳ 传输进度显示

3. **图片处理**
   - ⏳ 同步缩略图
   - ⏳ 原始图片按需获取

**进度**: ⏳ 0%

---

### 阶段 5: 测试与优化 ⏳ 待开始

**任务**:

1. ⏳ 多设备配对测试
2. ⏳ 离线重连测试
3. ⏳ 大文件传输测试
4. ⏳ 性能优化
5. ⏳ 文档更新

**进度**: ⏳ 0%

---

## 文件变更清单

### 新增文件

```
src/
├── components/settings/sections/
│   └── SyncSection.vue        # 同步设置主面板

src-tauri/src/
└── sync/
    ├── mod.rs                 # 模块入口
    ├── discovery.rs            # mDNS 设备发现
    ├── pairing.rs             # 配对逻辑
    ├── connection.rs          # P2P 连接管理
    ├── protocol.rs            # 二进制协议
    └── crypto.rs              # ChaCha20 加密
```

### 修改文件

```
src/types/index.ts                              # 新增 SyncDevice, Platform 等类型
src/components/settings/SettingsPanel.vue        # 添加同步导航项
src-tauri/src/lib.rs                            # 注册 sync 模块和命令
src-tauri/Cargo.toml                           # 添加依赖
```

### 依赖添加

```toml
# Cargo.toml
mdns-sd = "0.12"
uuid = { version = "1", features = ["v4"] }
whoami = "1"
chacha20poly1305 = "0.10"
lazy_static = "1.4"
```

---

## 后续扩展

### Android 支持

- 使用 Kotlin 实现 Android 端
- 复用相同的 mDNS + TCP 协议
- 统一设备ID格式（跨平台兼容）

### 互联网同步（预留）

- 中继服务器架构
- WebRTC 打洞
- TURN 服务器（无法 P2P 时）

---

## 开发顺序建议

1. 先实现后端设备发现
2. 前端设备列表 UI
3. 配对流程
4. P2P 连接
5. 文本同步（最小可行性）
6. 引用同步（完整功能）

---

*Generated: 2026-02-27*
*Feature: 局域网剪贴板同步*
