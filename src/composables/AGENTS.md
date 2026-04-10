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
| usePinMode.ts | Pin mode management | Controls blur auto-hide behavior and pin shortcut state |
| useSettings.ts | Settings persistence | Tauri invoke, change events |
| useSmartSearch.ts | Smart search logic | @tag/@type syntax, history |

## DETAILED DESCRIPTIONS

### useClipboard.ts (328 lines)

核心剪贴板功能：
- `history` - 响应式剪贴板历史数组
- `init()` - 启动剪贴板监听
- `loadHistory()` - 从后端加载历史记录
- `restoreToClipboard()` - 恢复条目到剪贴板（支持纯文本模式）
- 处理多种类型：text, html, rtf, image, file, folder, files
- 区分内部复制 vs 外部复制（用于智能激活）
- 图片恢复到系统剪贴板时需要考虑 Windows 11 下的句柄竞争；`writeImage()` 失败时优先做短暂重试/退避，而不是立刻判定为路径问题

### useSmartSearch.ts (397 lines)

智能搜索功能：
- `parseSearchQuery()` - 解析 @tag/@type 语法
- `removeTypeTokensFromQuery()` - 从查询中移除所有类型 token，供固定 tab 切换使用
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
- Pin 模式只影响失焦自动隐藏，不会禁止快捷键、`Esc` 或显式隐藏命令
- 默认快捷键：Ctrl+Shift+P

### useImageLoader.ts

图片加载管理：
- 5 次重试机制
- 加载状态管理
- 错误处理
- 配合 `convertFileSrc()` 加载本地图片

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
| Pin mode toggle | usePinMode.ts |
| Image loading issues | useImageLoader.ts |
| Settings persistence | useSettings.ts |
| Batch paste logic | usePasteQueue.ts |
| File operations | useFileOperations.ts |

## NOTES

- No test framework yet
- All state is component-local or Tauri-backed
- Composables should be focused and single-responsibility
- Window visibility/hide operations are done via direct `invoke()` calls in components, not through a dedicated composable
- `usePinMode.ts` 需要和后端 `window_manager.rs` 的语义保持一致：Pin 仅影响 blur auto-hide
- `useClipboard.ts` 的 `restoreToClipboard()` 要保持现有粘贴顺序：先恢复系统剪贴板，再由上层隐藏窗口/切焦点，最后执行 `simulatePaste()`
- 图片粘贴失败如果出现 `OSError(1418): 线程没有打开的剪贴板`，优先按 Windows 剪贴板竞争处理
- 搜索相关 composable 约束：
  - `useSearch.ts` 只消费 `searchQuery`，不要恢复独立 tab 过滤分支
  - `parseSearchQuery()` 是 fixed tab、自定义 tab、高亮态和后端搜索请求的共同基础
  - 固定 tab 与自定义 tab 的交互应通过增删 query token 实现，而不是维护第二份筛选状态
