# src/components

Vue components (UI + settings).

## STRUCTURE

```
components/
├── clipboard/                 # Clipboard-specific sub-components
│   ├── ClipboardList.vue      # Main list with virtual scrolling
│   ├── TabBar.vue             # Tab navigation with pinned searches
│   ├── EmptyState.vue         # Empty state display
│   └── DeleteConfirmDialog.vue # Delete confirmation dialog
├── settings/                  # Settings panel components
│   ├── SettingsPanel.vue      # Main settings container with sidebar
│   ├── components/            # Reusable setting widgets
│   │   ├── SettingItem.vue    # Individual setting row wrapper
│   │   ├── PathDisplay.vue    # Path display with copy/open buttons
│   │   └── KeyRecorder.vue    # Hotkey recording input
│   └── sections/              # Setting categories
│       ├── ClipboardSection.vue   # 剪贴板设置
│       ├── HistorySection.vue     # 历史记录设置
│       ├── GeneralSection.vue     # 通用设置
│       ├── HotkeySection.vue      # 快捷键设置
│       ├── BackupSection.vue      # 数据备份设置
│       └── AboutSection.vue       # 关于页面
├── ClipboardItem.vue        # Individual clipboard item card
├── ContextMenu.vue          # Right-click context menu
├── DragHandle.vue           # Window drag region (36px)
├── DrawerEditor.vue         # Slide-out editor for text/image preview
├── PasteQueuePanel.vue      # Batch paste panel (shopping cart mode)
├── SmartSearch.vue          # Advanced search with @mention support
└── TagManager.vue           # Tag CRUD popup dialog
```

## CONVENTIONS

- PascalCase in script, kebab-case in templates
- Props typed with `interface Props`
- Emit types: `defineEmits<{ action: [id: string] }>()`
- Scoped styles: `<style scoped>`
- Max ~200 lines per file (may be exceeded for complex components like ClipboardList.vue)

## COMPONENT DETAILS

### ClipboardList.vue

核心列表组件，1169 行，包含：
- 虚拟滚动列表（使用 vue-virtual-scroller）
- 搜索集成（异步调用 Rust 后端）
- Tab 过滤（全部/文本/图片/文件）
- 键盘导航（↑/↓/Enter/1-9/Esc）
- 右键上下文菜单
- 抽屉编辑器集成
- 标签管理器弹窗
- 固定搜索标签（可拖拽）
- Pin 模式状态管理

### ClipboardItem.vue

剪贴板条目卡片，987 行，包含：
- 类型徽章显示（文本/图片/文件/文件夹等）
- 内容预览（文本截断 3 行，图片缩略图，文件路径）
- 悬停快捷操作按钮（详情/标签/删除）
- 标签显示（可点击过滤）
- 单击/双击处理（可配置复制/粘贴/无操作）
- 选中状态高亮

### SmartSearch.vue

高级搜索输入组件，1089 行，包含：
- contenteditable 编辑器实现
- `@` 触发补全面板（类似 IDE/QQ）
- `@tag` 语法用于标签过滤
- `@type` 语法用于类型过滤（如 `@图片`, `@html`）
- 内联标签渲染为彩色标签块
- 搜索历史管理（localStorage）
- 键盘导航：↑↓ 选择，Enter/Tab 确认，Esc 关闭

### TabBar.vue

Tab 导航组件，214 行，包含：
- 固定标签：全部/文本/图片/文件
- 可拖拽的固定搜索标签
- 标签激活状态管理
- 关闭固定搜索按钮

### DrawerEditor.vue

抽屉式编辑器，771 行，包含：
- 滑出面板动画
- 文本编辑器（带预览模式）
- 图片查看器（带元数据显示）
- 文件信息展示
- 全屏/关闭控制

### TagManager.vue

标签管理器，533 行，包含：
- 弹窗对话框
- 创建新标签
- 选择/取消选择已有标签
- 标签颜色自动生成
- 键盘快捷键支持

### ContextMenu.vue

右键菜单，314 行，包含：
- 上下文感知菜单项
- 复制/粘贴/纯文本复制
- 打开文件/在文件夹中显示
- 复制路径
- 添加/移除标签
- 删除（带确认）

## WHERE TO LOOK

| Task | File |
|------|------|
| Add tab | ClipboardList.vue (tabs array) |
| Add setting | settings/sections/*.vue |
| Modify context menu | ContextMenu.vue |
| Item card styling | ClipboardItem.vue |
| Modify search | SmartSearch.vue + useSmartSearch.ts |
| Modify filter logic | ClipboardList.vue (async search) |
| Add pinned search | SmartSearch.vue + TabBar.vue |
| Modify tags | TagManager.vue |
| Modify drawer | DrawerEditor.vue |
| Modify keyboard nav | ClipboardList.vue |

## NOTES

- Two root components: App.vue (settings), ClipboardView.vue (clipboard)
- URL-based routing in main.ts: `/clipboard` → ClipboardView, else → App
- ClipboardList.vue is the most complex component - handle with care
