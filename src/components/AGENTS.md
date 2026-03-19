# src/components

Vue components (UI + settings).

## STRUCTURE

```
components/
├── ClipboardItem.vue    # Card for single clipboard entry
├── ClipboardList.vue    # Main list with tabs/search
├── ContextMenu.vue      # Right-click menu
├── DragHandle.vue       # Window drag region (36px)
├── DrawerEditor.vue     # Text/image preview modal
├── PasteQueuePanel.vue  # Batch paste panel
├── SmartSearch.vue      # Search input component
├── TagManager.vue       # Tag CRUD popup
├── clipboard/           # Clipboard-specific components
│   └── ...
└── settings/            # Settings panel components
    ├── components/      # Reusable setting widgets
    └── sections/        # Setting categories
```

## CONVENTIONS

- PascalCase in script, kebab-case in templates
- Props typed with `interface Props`
- Emit types: `defineEmits<{ action: [id: string] }>()`
- Scoped styles: `<style scoped>`
- Max ~200 lines per file

## WHERE TO LOOK

| Task | File |
|------|------|
| Add tab | ClipboardList.vue (tabs array) |
| Add setting | settings/sections/*.vue |
| Modify context menu | ContextMenu.vue |
| Item card styling | ClipboardItem.vue |
| Modify search | SmartSearch.vue + useSmartSearch.ts |
| Modify filter logic | ClipboardList.vue (async search) |

## SmartSearch.vue - @Mention 搜索

支持类似 QQ/IDE 的内联标签补全：
- 输入 `@` 触发补全面板（跟随光标位置）
- 支持标签（#）和类型（文本/图片/文件等）过滤
- 内联标签渲染为彩色标签块
- 键盘导航：↑↓ 选择，Enter/Tab 确认，Esc 关闭

## ClipboardList.vue - 异步搜索

搜索逻辑已从 computed 改为异步 Rust 后端调用：
- `performSearch()` 调用 `search_clipboard_advanced` Tauri 命令
- 防抖搜索（250ms 延迟）
- 显示加载状态指示器
- 支持多关键词 AND 匹配

## 搜索功能特性

通过 Rust `fuzzy_search.rs` 实现：
- **精确子串匹配** - 不区分大小写的子串包含检查

## NOTES

- Two root components: App.vue (settings), ClipboardView.vue (clipboard)
- URL-based routing in main.ts: `/clipboard` → ClipboardView, else → App
