# src/composables

Shared Vue composition functions (hooks).

## FILES

| File | Purpose | Key Features |
|------|---------|--------------|
| useClipboard.ts | Clipboard monitoring, CRUD operations | Internal/external copy detection, plain text support |
| useContentType.ts | Content type detection | Type inference from content/mime |
| useFileOperations.ts | File/folder operations | Open file, reveal in explorer |
| useImageLoader.ts | Image loading with retry | 5 retry attempts, loading states |
| usePasteQueue.ts | Paste queue state | Shopping cart mode for batch paste |
| usePinMode.ts | Pin mode management | Window stays open after paste |
| useSettings.ts | Settings persistence | Tauri invoke, change events |
| useSmartSearch.ts | Smart search logic | @tag/@type syntax, history |
| useWindow.ts | Window visibility | Toggle, hide clipboard window |

## DETAILED DESCRIPTIONS

### useClipboard.ts (328 lines)

核心剪贴板功能：
- `history` - 响应式剪贴板历史数组
- `init()` - 启动剪贴板监听
- `loadHistory()` - 从后端加载历史记录
- `restoreToClipboard()` - 恢复条目到剪贴板（支持纯文本模式）
- 处理多种类型：text, html, rtf, image, file, folder, files
- 区分内部复制 vs 外部复制（用于智能激活）

### useSmartSearch.ts (397 lines)

智能搜索功能：
- `parseSearchQuery()` - 解析 @tag/@type 语法
- `matchItemWithQuery()` - 匹配条目与查询
- `searchHistory` - 搜索历史管理（localStorage）
- `highlightMatches()` - 高亮匹配文本
- 支持标签过滤（@标签名）和类型过滤（@图片/@html等）

### useSettings.ts (99 lines)

设置管理：
- `settings` - 响应式设置对象
- `loadSettings()` / `saveSettings()` - 通过 Tauri invoke 读写
- 发射 `settings-changed` 事件供其他组件监听

### usePinMode.ts (80 lines)

Pin 模式管理：
- `isPinned` - 当前 Pin 状态
- `togglePinMode()` - 切换 Pin 模式
- `setPinMode()` - 设置 Pin 状态
- Pin 模式下窗口在粘贴后保持打开
- 默认快捷键：Ctrl+Shift+P

### useImageLoader.ts

图片加载管理：
- 5 次重试机制
- 加载状态管理
- 错误处理
- 配合 `convertFileSrc()` 加载本地图片

### useWindow.ts (56 lines)

窗口管理：
- `toggleClipboardWindow()` - 显示/隐藏剪贴板窗口
- `hideClipboardWindow()` - 隐藏窗口
- 监听 `clipboard-window-blur` 事件自动隐藏

### usePasteQueue.ts

粘贴队列管理：
- 购物车模式（选择多个条目批量粘贴）
- 队列添加/移除/清空
- 批量粘贴执行

### useContentType.ts

内容类型检测：
- 从内容推断类型
- MIME 类型处理

### useFileOperations.ts

文件操作：
- `openFile()` - 打开文件
- `revealInFolder()` - 在文件夹中显示

## CONVENTIONS

- Named `use*.ts` (Vue 3 composition API pattern)
- All async Tauri calls wrapped in try-catch
- Return typed refs: `const items = ref<ClipboardItem[]>([])`
- Use `@/` path alias (maps to `src/`)

## WHERE TO LOOK

| Task | File |
|------|------|
| Add new clipboard type | useContentType.ts |
| Modify search behavior | useSmartSearch.ts |
| Window management | useWindow.ts |
| Pin mode toggle | usePinMode.ts |
| Image loading issues | useImageLoader.ts |
| Settings persistence | useSettings.ts |
| Batch paste logic | usePasteQueue.ts |
| File operations | useFileOperations.ts |

## NOTES

- No test framework yet
- All state is component-local or Tauri-backed
- Composables should be focused and single-responsibility
