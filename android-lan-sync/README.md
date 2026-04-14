# Android LAN Sync Peer (最小可用版)

这是一个独立 Android 示例 App，用来和本仓库 `sync_service.rs` 的协议做联调。

## 已实现能力

- 发送 `heartbeat`（UDP 广播）
- 发送 `pair_request`（TCP）
- 发送 `sync_text`（TCP）
- 监听 `pair_response`（TCP）
- 一键顺序执行 Heartbeat -> PairRequest -> SyncText

## 协议对齐

和后端保持一致：

- `kind`: `heartbeat` / `pair_request` / `pair_response` / `sync_text`
- `protocol_version`: `1`
- `sync_text` 包含：`device_id`、`device_name`、`tcp_port`、`message_id`、`created_at`、`text`

## 构建 APK

1. 用 Android Studio 打开目录 `android-lan-sync`
2. 等待 Gradle Sync 完成
3. 菜单 `Build -> Build Bundle(s) / APK(s) -> Build APK(s)`
4. 生成路径通常是：`app/build/outputs/apk/debug/app-debug.apk`

## 使用方式（和桌面端联调）

1. PC 端运行 Tauri 应用并开启局域网同步
2. 手机和 PC 处于同一局域网
3. 在手机 App 里：
   - `目标主机` 填 PC 的局域网 IP
   - `目标 TCP 端口` 默认 `48571`
   - `发现 UDP 端口` 默认 `48572`
   - `本机监听端口` 默认 `48581`
4. 点击 `开始监听`，然后 `发送 PairRequest`
5. 在 PC 设置页里批准配对
6. 点击 `发送 SyncText`，PC 端应收到同步文本
