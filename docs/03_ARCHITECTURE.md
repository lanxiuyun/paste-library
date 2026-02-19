# 03 - 技术架构设计

> **范围**: 技术栈、文件结构、API 设计  
> **目标**: 开发架构指导  
> **最后更新**: 2026-02-19

---

## 一、技术栈

### 1.1 完整技术栈

| 层级 | 技术 | 版本 | 说明 |
|------|------|------|------|
| **前端框架** | Vue | 3.4+ | Composition API + `<script setup>` |
| **语言** | TypeScript | 5.3+ | 严格模式 |
| **构建工具** | Vite | 5.0+ | 快速 HMR |
| **桌面框架** | Tauri | 2.0+ | 跨平台桌面 |
| **后端语言** | Rust | 1.75+ | 系统编程 |
| **数据库** | SQLite | 3.40+ | rusqlite bundled |
| **状态管理** | Vue Reactivity | - | ref/reactive/computed |
| **样式** | Scoped CSS | - | CSS Variables |

### 1.2 核心依赖

**Frontend (package.json)**:
```json
{
  "dependencies": {
    "vue": "^3.4.0",
    "@tauri-apps/api": "^2.0.0"
  },
  "devDependencies": {
    "@vitejs/plugin-vue": "^5.0.0",
    "typescript": "^5.3.0",
    "vite": "^5.0.0",
    "vue-tsc": "^1.8.0"
  }
}
```

**Backend (Cargo.toml)**:
```toml
[dependencies]
tauri = { version = "2.0", features = [] }
tauri-plugin-clipboard-x = "2.0"
tauri-plugin-global-shortcut = "2.0"
tauri-plugin-tray = "2.0"
tauri-plugin-autostart = "2.0"
rusqlite = { version = "0.32", features = ["bundled"] }
tokio = { version = "1", features = ["full"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
uuid = { version = "1.7", features = ["v7", "serde"] }
chrono = { version = "0.4", features = ["serde"] }

# LAN Sync
mdns-sd = "0.10"
aes-gcm = "0.10"
argon2 = "0.5"
lru = "0.12"
```

### 1.3 Tauri 插件

| 插件 | 用途 | 权限 |
|------|------|------|
| `clipboard-x` | 剪贴板监听 | `clipboard-x:default` |
| `global-shortcut` | 全局快捷键 | `global-shortcut:allow-*` |
| `tray` | 系统托盘 | `tray:default` |
| `autostart` | 开机自启 | `autostart:default` |

---

## 二、文件结构

### 2.1 项目目录

```
paste-library/
├── src/                          # 前端源码
│   ├── components/               # Vue 组件
│   │   ├── ClipboardItem.vue     # 剪贴板项卡片
│   │   ├── ClipboardList.vue     # 剪贴板列表
│   │   ├── ContextMenu.vue       # 右键菜单
│   │   ├── DrawerEditor.vue      # 抽屉编辑器
│   │   ├── PasteQueuePanel.vue   # 粘贴队列
│   │   ├── SettingsPanel.vue     # 设置面板
│   │   ├── SmartSearch.vue       # 智能搜索
│   │   ├── TagManager.vue        # 标签管理
│   │   ├── LanSyncPanel.vue      # 局域网同步
│   │   └── DragHandle.vue        # 窗口拖拽
│   ├── composables/              # 可复用逻辑
│   │   ├── useClipboard.ts       # 剪贴板监听
│   │   ├── usePasteQueue.ts      # 粘贴队列
│   │   ├── useSettings.ts        # 设置管理
│   │   ├── useSmartSearch.ts     # 智能搜索
│   │   ├── useWindow.ts          # 窗口管理
│   │   └── useLanSync.ts         # 局域网同步
│   ├── types/
│   │   └── index.ts              # TypeScript 类型
│   ├── utils/
│   │   └── tagColors.ts          # 标签颜色
│   ├── App.vue                   # 设置窗口
│   ├── ClipboardView.vue         # 剪贴板窗口
│   └── main.ts                   # 入口
├── src-tauri/
│   ├── src/                      # Rust 源码
│   │   ├── lib.rs                # 主入口
│   │   ├── main.rs               # 程序入口
│   │   ├── clipboard.rs          # 剪贴板管理
│   │   ├── models.rs             # 数据模型
│   │   ├── storage.rs            # 数据库操作
│   │   ├── fuzzy_search.rs       # 模糊搜索
│   │   ├── window_manager.rs     # 窗口管理
│   │   ├── tray_manager.rs       # 系统托盘
│   │   ├── shortcut_manager.rs   # 快捷键
│   │   └── lan_sync/             # 局域网同步
│   │       ├── mod.rs
│   │       ├── discovery.rs      # mDNS 发现
│   │       ├── protocol.rs       # 通信协议
│   │       ├── crypto.rs         # 加密
│   │       └── sync_engine.rs    # 同步引擎
│   ├── capabilities/             # 权限配置
│   └── Cargo.toml
├── docs/                         # 文档
│   ├── 01_OVERVIEW.md
│   ├── 02_PRD.md
│   ├── 03_ARCHITECTURE.md
│   ├── 04_UI_SPEC.md
│   ├── 05_DATA_MODEL.md
│   └── 06_IMPLEMENTATION.md
└── package.json
```

### 2.2 模块职责

| 模块 | 职责 | 文件 |
|------|------|------|
| **clipboard** | 剪贴板监听、数据处理 | `clipboard.rs` |
| **storage** | 数据库 CRUD、查询 | `storage.rs` |
| **fuzzy_search** | 模糊搜索、拼音匹配 | `fuzzy_search.rs` |
| **window_manager** | 窗口创建、显示、隐藏 | `window_manager.rs` |
| **tray_manager** | 托盘图标、菜单 | `tray_manager.rs` |
| **shortcut_manager** | 全局快捷键注册 | `shortcut_manager.rs` |
| **lan_sync** | 设备发现、通信、同步 | `lan_sync/` |

---

## 三、Tauri 命令列表

### 3.1 剪贴板操作

| 命令 | 参数 | 返回值 | 说明 |
|------|------|--------|------|
| `add_clipboard_item` | `{ text, html, is_internal_copy }` | `ClipboardItem` | 添加文本/HTML |
| `add_clipboard_item_extended` | `{ content_type, content, file_paths, thumbnail_path, metadata, is_internal_copy }` | `ClipboardItem` | 添加图片/文件 |
| `get_clipboard_history` | `{ limit?, offset? }` | `GetHistoryResponse` | 获取历史 |
| `search_clipboard_history` | `{ query, limit? }` | `ClipboardItem[]` | 搜索 |
| `delete_clipboard_item` | `{ id }` | `boolean` | 删除单条 |
| `clear_clipboard_history` | - | `boolean` | 清空历史 |
| `update_item_tags` | `{ id, tags }` | `ClipboardItem` | 更新标签 |
| `create_tag` | `{ name }` | `Tag` | 创建标签 |
| `delete_tag` | `{ id }` | `boolean` | 删除标签 |

### 3.2 粘贴队列操作

| 命令 | 参数 | 返回值 | 说明 |
|------|------|--------|------|
| `get_paste_queue` | - | `ClipboardItem[]` | 获取队列 |
| `add_to_queue` | `{ id }` | `boolean` | 添加到队列 |
| `remove_from_queue` | `{ id }` | `boolean` | 从队列移除 |
| `reorder_queue` | `{ ids }` | `boolean` | 重排序 |
| `clear_queue` | - | `boolean` | 清空队列 |
| `batch_paste` | `{ separator }` | `boolean` | 批量粘贴 |

### 3.3 设置操作

| 命令 | 参数 | 返回值 | 说明 |
|------|------|--------|------|
| `get_settings` | - | `AppSettings` | 获取设置 |
| `save_settings` | `{ settings }` | `boolean` | 保存设置 |
| `get_storage_paths` | - | `StoragePaths` | 获取路径 |
| `export_data` | `{ path }` | `boolean` | 导出数据 |
| `import_data` | `{ path }` | `boolean` | 导入数据 |

### 3.4 文件操作

| 命令 | 参数 | 返回值 | 说明 |
|------|------|--------|------|
| `open_file` | `{ path }` | `boolean` | 打开文件 |
| `show_in_folder` | `{ path }` | `boolean` | 在文件夹中显示 |
| `copy_file_to_clipboard` | `{ path }` | `boolean` | 复制文件到剪贴板 |

### 3.5 窗口操作

| 命令 | 参数 | 返回值 | 说明 |
|------|------|--------|------|
| `toggle_clipboard_window` | - | `boolean` | 切换窗口 |
| `hide_clipboard_window` | - | `boolean` | 隐藏窗口 |
| `show_clipboard_window` | - | `boolean` | 显示窗口 |
| `show_settings_window` | - | `boolean` | 显示设置窗口 |

### 3.6 局域网同步

| 命令 | 参数 | 返回值 | 说明 |
|------|------|--------|------|
| `lan_sync_start` | `{ device_name, pairing_code }` | `boolean` | 启动同步服务 |
| `lan_sync_stop` | - | `boolean` | 停止同步服务 |
| `lan_sync_get_devices` | - | `SyncDevice[]` | 获取设备列表 |
| `lan_sync_pair_device` | `{ device_id, pairing_code }` | `boolean` | 配对设备 |
| `lan_sync_unpair_device` | `{ device_id }` | `boolean` | 取消配对 |
| `lan_sync_pull_from_device` | `{ device_id }` | `ClipboardItem[]` | 从设备拉取 |

---

## 四、前端调用示例

### 4.1 基础调用

```typescript
import { invoke } from '@tauri-apps/api/core';

// 获取历史记录
const history = await invoke<GetHistoryResponse>('get_clipboard_history', {
  limit: 100,
  offset: 0
});

// 添加剪贴板项
const item = await invoke<ClipboardItem>('add_clipboard_item', {
  text: 'Hello World',
  is_internal_copy: false
});

// 删除项
await invoke('delete_clipboard_item', { id: 123 });
```

### 4.2 事件监听

```typescript
import { listen } from '@tauri-apps/api/event';

// 监听新剪贴板项
listen<ClipboardItem>('clipboard:new-item', (event) => {
  console.log('New item:', event.payload);
});

// 监听同步事件
listen<SyncMessage>('lan-sync:item-received', (event) => {
  console.log('Sync item:', event.payload);
});
```

---

## 五、配置说明

### 5.1 Tauri 配置 (tauri.conf.json)

```json
{
  "productName": "Paste Library",
  "identifier": "com.yourcompany.paste-library",
  "version": "1.0.0",
  "app": {
    "windows": [
      {
        "label": "main",
        "title": "Paste Library - 设置",
        "width": 600,
        "height": 700,
        "center": true,
        "resizable": true
      },
      {
        "label": "clipboard",
        "title": "剪贴板历史",
        "width": 800,
        "height": 600,
        "visible": false,
        "decorations": false,
        "skipTaskbar": true,
        "alwaysOnTop": true
      }
    ],
    "security": {
      "csp": "default-src 'self'; img-src 'self' asset: http://asset.localhost"
    }
  },
  "plugins": {
    "clipboard-x": {},
    "global-shortcut": {},
    "tray": {}
  }
}
```

### 5.2 权限配置 (capabilities)

```json
{
  "permissions": [
    "core:default",
    "clipboard-x:default",
    "core:window:allow-minimize",
    "core:window:allow-hide",
    "core:window:allow-show",
    "core:window:allow-is-visible",
    "core:window:allow-set-focus",
    "core:window:allow-create",
    "core:window:allow-is-focused",
    "global-shortcut:allow-is-registered",
    "global-shortcut:allow-register",
    "global-shortcut:allow-unregister",
    "global-shortcut:allow-unregister-all",
    "core:tray:default",
    "core:menu:default",
    "autostart:default"
  ]
}
```

---

## 六、开发环境

### 6.1 依赖安装

**系统依赖**:
- Rust 1.75+
- Node.js 18+
- Windows: Microsoft Visual Studio C++ Build Tools
- macOS: Xcode Command Line Tools
- Linux: `libgtk-3-dev`, `libwebkit2gtk-4.0-dev`

**项目安装**:
```bash
# 安装前端依赖
pnpm install

# 安装 Tauri CLI
pnpm add -D @tauri-apps/cli
```

### 6.2 开发命令

```bash
# 开发模式
pnpm tauri dev

# 构建
pnpm tauri build

# 仅前端开发
pnpm dev

# 类型检查
pnpm vue-tsc --noEmit
```

---

## 七、性能优化

### 7.1 前端优化

- **虚拟滚动**: 大数据量时使用 `vue-virtual-scroller`
- **图片懒加载**: 缩略图进入视口再加载
- **防抖搜索**: 搜索输入防抖 200ms
- **组件懒加载**: 抽屉编辑器、设置面板懒加载

### 7.2 后端优化

- **连接池**: SQLite 使用连接池
- **批量操作**: 批量插入、批量查询
- **索引优化**: 常用查询字段建立索引
- **缩略图缓存**: 限制最大缓存数量

### 7.3 目标指标

- 启动时间 < 3s
- 剪贴板响应 < 100ms
- 搜索响应 < 200ms
- 内存占用 < 200MB
- 支持 5000+ 条记录不卡顿

---

## 相关文档

- [01_OVERVIEW.md](./01_OVERVIEW.md) - 项目概述
- [02_PRD.md](./02_PRD.md) - 产品需求
- [04_UI_SPEC.md](./04_UI_SPEC.md) - UI 设计规范
- [05_DATA_MODEL.md](./05_DATA_MODEL.md) - 数据模型
- [06_IMPLEMENTATION.md](./06_IMPLEMENTATION.md) - 实施计划
