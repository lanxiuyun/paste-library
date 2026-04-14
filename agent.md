# Local Net Sync Agent Guide

## 目标

该功能用于在同一局域网内同步 `文本` 剪贴板内容。第一版采用：

- UDP 广播自动发现设备
- TCP 点对点发送文本消息
- 首次手动确认配对
- Rust 后台常驻服务，不依赖剪贴板窗口是否打开

## 核心文件

- `src-tauri/src/sync_service.rs`: 局域网发现、配对、广播、入站处理、状态事件
- `src-tauri/src/lib.rs`: Tauri 生命周期挂载、命令注册、设置变更后的服务重载
- `src-tauri/src/clipboard.rs`: 复用现有入库逻辑，处理本地文本与远端文本
- `src-tauri/src/storage.rs`: 设置持久化、历史记录去重
- `src-tauri/src/models.rs`: `AppSettings`、`LanSyncStatus`、`ClipboardSource`
- `src/composables/useClipboard.ts`: 本地剪贴板监听与来源标记
- `src/composables/useLanSync.ts`: 前端同步状态读取与设备管理命令封装
- `src/composables/useLanSyncClipboardBridge.ts`: 远端文本写入本机剪贴板时的事件桥接去重
- `src/components/settings/sections/SyncSection.vue`: 同步设置 UI
- `src/components/clipboard/ClipboardList.vue`: 监听 `lan-sync-updated` 后刷新列表
- `src/main.ts`: 监听 `lan-sync-copy-text`，将远端文本写入系统剪贴板

## 数据流

### 本地复制

1. 前端监听系统剪贴板变化
2. `useClipboard.ts` 调用 `add_clipboard_item`
3. Rust `ClipboardManager` 入库并复用去重/裁剪逻辑
4. `LanSyncService` 向所有已信任设备广播文本

### 远端入站

1. `LanSyncService` 通过 TCP 收到 `SyncText`
2. 校验协议版本、设备身份、信任状态、`message_id`
3. 调用 `ClipboardManager::handle_synced_text()` 入库
4. 成功后发出：
   - `lan-sync-copy-text`
   - `lan-sync-updated`
5. 前端写入系统剪贴板并刷新历史列表

## 关键语义

- `ClipboardSource::LocalSystem`: 来自本地系统剪贴板，允许广播
- `ClipboardSource::InternalCopy`: 来自应用内 `restoreToClipboard`，不广播
- `ClipboardSource::LanSync`: 来自局域网入站，不再次广播

不要把这三类来源重新压回单个布尔值，否则回环和时间戳语义会再次变复杂。

## 协议约定

- 发现消息：`Heartbeat`
- 配对消息：`PairRequest` / `PairResponse`
- 同步消息：`SyncText`
- 每条同步消息必须包含：
  - `device_id`
  - `device_name`
  - `tcp_port`
  - `message_id`
  - `created_at`
  - `text`

## 前端事件

- `lan-sync-status-changed`: 在线设备、待确认请求、信任设备、错误状态变更
- `lan-sync-updated`: 远端文本成功入库后通知历史列表刷新
- `lan-sync-copy-text`: 远端文本成功入库后通知前端写入系统剪贴板

## Tauri 命令

- `get_lan_sync_status`
- `list_discovered_devices`
- `request_lan_pairing`
- `approve_lan_device`
- `reject_lan_device`
- `remove_trusted_lan_device`

## 当前能力

- 支持一对多广播
- 仅同步文本
- 首次配对需人工确认
- 信任设备列表单独持久化为 JSON
- 关闭剪贴板窗口后后台同步仍继续运行

## 修改注意事项

- 不要直接从同步服务写 SQLite，必须走 `ClipboardManager`
- 不要让 `lan-sync-copy-text` 在多个窗口重复写系统剪贴板
- 设置自动保存会频繁触发 `save_settings`，服务重载必须避免无意义重复重绑端口
- 任何新增同步内容类型前，先明确去重键、回环控制和前端写回剪贴板策略

## 已知限制

- 无端到端加密
- 无离线消息与重试队列
- 无设备分组或按设备过滤广播
- 发现机制依赖局域网广播，复杂网络环境下可能不稳定
