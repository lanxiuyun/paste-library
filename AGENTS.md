# AGENTS.md — Coding Guidelines for Tauri + Vue + TypeScript

## Project Overview

**paste-library**: A modern clipboard manager built with Tauri v2 and Vue 3. Features real-time clipboard monitoring, history persistence, search, and a clean card-based UI.

- **Frontend**: Vue 3 (Composition API with `<script setup>`), TypeScript, Vite
- **Backend**: Tauri v2 with Rust
- **Database**: SQLite (via rusqlite)
- **Plugins**: 
  - tauri-plugin-clipboard-x (clipboard monitoring)
  - tauri-plugin-global-shortcut (global hotkey Alt+V)
- **Status**: In development — core features implemented, settings panel complete

---

## Build, Lint, Test Commands

### Frontend (Vue/TypeScript)

```bash
# Development server (port 1422)
pnpm dev

# Type check (no emit)
pnpm run build  # Includes vue-tsc type check before bundling

# Production build
pnpm run build

# Preview production build
pnpm preview
```

### Tauri Desktop App

```bash
# Development (runs beforeDevCommand + Tauri dev)
pnpm tauri dev

# Production build (compiles Rust + frontend)
pnpm tauri build
```

### Running a Single Test

**Currently**: No test framework configured. If adding tests:
- Add `vitest` as devDep: `pnpm add -D vitest @vitest/ui`
- Create tests in `src/**/*.test.ts` or `src/**/*.test.vue`
- Run: `pnpm exec vitest run` or `pnpm exec vitest` (watch mode)

---

## Code Style Guidelines

### Imports & Modules

- **ES modules only** (`import`/`export`, not `require`)
- Order imports: Vue API → Tauri API → Local modules → Styles
  ```typescript
  import { ref, computed } from "vue";
  import { invoke } from "@tauri-apps/api/core";
  import { MyComponent } from "@/components/MyComponent";
  import "@/styles/global.css";
  ```
- Use explicit file extensions (`.ts`, `.vue`) in imports
- Organize local imports alphabetically within groups

### TypeScript & Types

- **Strict mode enabled** (`"strict": true` in tsconfig.json)
  - No `any` types without explicit justification
  - No `@ts-ignore`, `@ts-expect-error`, or `as any`
  - Unused variables/parameters forbidden
- Vue component types: Always use explicit generics for refs/computed
  ```typescript
  const count = ref<number>(0);
  const doubled = computed<number>(() => count.value * 2);
  ```
- Interface/type naming: PascalCase (e.g., `UserData`, `ApiResponse`)
- Avoid `interface` vs `type` debates — use `type` by default for consistency

### Naming Conventions

- **Variables/functions**: camelCase (e.g., `getUserData`, `isLoading`)
- **Components**: PascalCase in script, kebab-case in templates
  ```typescript
  // ✓ Correct
  import MyButton from "@/components/MyButton.vue";
  <MyButton />
  
  // ✗ Wrong
  import myButton from "@/components/myButton.vue";
  <my-button />
  ```
- **Constants**: UPPER_SNAKE_CASE for truly immutable values
  ```typescript
  const API_TIMEOUT = 5000;
  const MAX_RETRIES = 3;
  ```

### Vue Component Structure

- Use `<script setup>` (modern, preferred over `setup()` function)
- Keep components focused — max ~200 lines per file
- Props always typed and validated
  ```typescript
  interface Props {
    title: string;
    count?: number;
    disabled?: boolean;
  }
  withDefaults(defineProps<Props>(), { count: 0, disabled: false });
  ```
- Emit types explicitly
  ```typescript
  const emit = defineEmits<{ submit: [value: string] }>();
  ```

### Window UI Guidelines

**Settings Window** (Main window):
- Normal window with system title bar (`decorations: true`)
- Size: 600x700, resizable
- Left sidebar navigation + right content area

**Clipboard Window** (Popup window):
- Frameless window (`decorations: false`)
- Size: 800x600, resizable
- Uses `skip_taskbar(true)` to hide from taskbar
- Uses `always_on_top(true)` for floating behavior
- Auto-hide on blur

**Key points**:
- Use `data-tauri-drag-region` attribute for draggable areas
- Use `-webkit-app-region: drag` / `app-region: drag` in CSS
- Interactive elements (buttons, inputs) must have `app-region: no-drag`
- Top drag bar height: 36px with a simple line indicator

### Formatting & Spacing

- Use Vue/TypeScript defaults (no explicit prettier config needed — follow existing patterns)
- 2-space indentation (default in templates)
- Single quotes for strings (following existing code)
- Semicolons: Optional but consistent (currently omitted in existing files)

### Error Handling

- **Always use try-catch** for async operations, especially Tauri invokes
  ```typescript
  async function greet() {
    try {
      greetMsg.value = await invoke("greet", { name: name.value });
    } catch (err) {
      console.error("Greet failed:", err);
      // Handle error gracefully
    }
  }
  ```
- Log errors meaningfully (include context)
- No silent failures — notify user or log clearly
- Tauri errors: Expect `string` payloads, validate before using

### State Management

- Use Vue reactivity (`ref`, `computed`, `watch`) for component-level state
- For shared state across components → Consider composables or Tauri backend
- Use `reactive()` for form data in settings panel
- Avoid global state until project needs it
- Reactive refs for form data, UI state; computed for derived values

### CSS & Styling

- Scoped styles preferred (`<style scoped>`)
- Global styles in `src/` root CSS file (currently in App.vue as fallback)
- CSS variables for theming (already used: light/dark mode)
- Avoid inline styles unless truly dynamic
- Media queries for responsive design (already present for dark mode)
- **Card-based design**: Rounded corners (8px), subtle shadows, clean typography
- **Settings panel**: Left sidebar (220px) + right content area with grouped settings

### Comments & Documentation

- Explain *why*, not *what* (code should be self-documenting)
- Use JSDoc for public functions:
  ```typescript
  /**
   * Invokes Tauri command to greet user
   * @param name - User's name
   * @returns Greeting message
   */
  async function greet(name: string): Promise<string> {
    return await invoke("greet", { name });
  }
  ```
- No trailing TODOs; open GitHub issues instead

---

## File Structure

```
src/
  ├── main.ts                    # App entry point
  ├── App.vue                    # Root component (Settings window)
  ├── ClipboardView.vue          # Clipboard window entry
  ├── assets/                    # Static images/SVGs
  ├── components/                # Reusable Vue components
  │   ├── ClipboardItem.vue      # Card component for single clipboard item
  │   ├── ClipboardList.vue      # Main list with tabs and search
  │   ├── ContextMenu.vue        # Right-click context menu
  │   ├── DragHandle.vue         # Window drag capsule (for clipboard window)
  │   └── SettingsPanel.vue      # Settings panel with left navigation
  ├── composables/               # Reusable logic (hooks)
  │   ├── useClipboard.ts        # Clipboard monitoring logic (text/image/files)
  │   ├── useSettings.ts         # Settings management
  │   └── useWindow.ts           # Window management (toggle/show/hide)
  ├── types/                     # TypeScript type definitions
  │   └── index.ts               # Shared types (ClipboardItem, AppSettings, etc.)
  └── styles/                    # Global CSS (if needed)

src-tauri/
  ├── src/                       # Rust backend
  │   ├── lib.rs                 # Main entry + Tauri commands + global shortcut
  │   ├── clipboard.rs           # Clipboard manager logic
  │   ├── models.rs              # Data structures (ClipboardItem, AppSettings, etc.)
  │   ├── storage.rs             # SQLite database operations
  │   └── window_manager.rs      # Window management (create/hide/show clipboard window)
  ├── tauri.conf.json            # Tauri config
  ├── capabilities/              # Permission definitions
  └── Cargo.toml                 # Rust dependencies
```

---

## Tauri-Specific Guidelines

### Window Configuration

**Settings Window (Main)**:
```json
{
  "label": "main",
  "title": "Paste Library - 设置",
  "width": 600,
  "height": 700,
  "decorations": true,
  "center": true,
  "resizable": true
}
```

**Clipboard Window (Popup)**:
```rust
WebviewWindowBuilder::new(app, "clipboard", WebviewUrl::App("/clipboard".into()))
    .title("剪贴板历史")
    .inner_size(width, height)
    .decorations(false)
    .skip_taskbar(true)
    .always_on_top(true)
    .build()
```

### Required Permissions

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
    "global-shortcut:allow-unregister-all"
  ]
}
```

### Commands

- Commands in `src-tauri/src/lib.rs` prefixed with `#[tauri::command]`
- Frontend calls via `invoke("command_name", { param: value })`
- Always handle Tauri errors in try-catch (they're `string` payloads)
- Use `tokio::sync::Mutex` instead of `std::sync::Mutex` for async commands

### Global Shortcut

- Registered in `lib.rs` setup with `tauri-plugin-global-shortcut`
- Default hotkey: `Alt+V`
- Toggles clipboard window visibility
- Window auto-hides on blur via `on_window_event`

---

## Project Features

### Implemented ✅
- Real-time clipboard monitoring (text + HTML + image + files)
- SQLite persistence with automatic deduplication (SHA256 hash)
- Card-based UI with tabs (All/Text/Image/File/Favorite)
- Search functionality (fuzzy search)
- **Global hotkey** (configurable via key recording) to show/hide clipboard window
- **Settings panel** with left sidebar navigation
- **Window management** (frameless clipboard, normal settings)
- **Image/File clipboard support**:
  - Image thumbnails with dimensions display
  - File/folder icons with names
  - Multi-file count display
- **Context menu** (right-click):
  - Copy/Paste/Copy as plain text
  - Open file / Show in folder
  - Favorite/Unfavorite
  - Delete
- **Interaction enhancements**:
  - Single click: copy to clipboard
  - Double click: copy and paste
  - Right click: context menu
- **Favorite system** with database persistence
- Copy/delete clipboard items
- Data persistence with comprehensive settings

### In Progress ⏳
- Data export/backup functionality (backend)
- Settings panel enhancements

### Planned 📋
- System tray integration
- Cross-device sync architecture
- Dark theme (currently light only)
- Advanced search filters (by date range)
- Multi-language support

---

## Settings Panel Structure

### Navigation Items
1. **剪贴板** - Window settings, sound effects, search settings, content settings
2. **历史记录** - Max history count, auto cleanup
3. **通用设置** - Auto start, blacklist apps
4. **快捷键** - Hotkey display, window size
5. **数据备份** - Export/import data
6. **关于** - App info, open clipboard button

### Settings Categories

**窗口设置**:
- 窗口位置 (remember/center/cursor)
- 激活时回到顶部
- 激活时切换至全部分组

**音效设置**:
- 复制音效 (+ preview button)

**搜索设置**:
- 搜索框位置 (top/bottom)
- 默认聚焦
- 自动清除

**内容设置**:
- 自动粘贴 (off/single/double)
- 图片OCR
- 复制为纯文本
- 粘贴为纯文本
- 操作按钮 (customize)
- 自动收藏
- 删除确认
- 自动排序

**历史记录设置**:
- 最大历史记录数 (100-10000)
- 自动清理 (0/7/30/90 days)

**通用设置**:
- 开机自启
- 应用黑名单 (textarea, one per line)

**快捷键设置**:
- 唤醒快捷键 (按键录制, 如: Alt+V, Win+Shift+C)
- 窗口尺寸 (width × height)

---

## Notes for Agents

- **No linter/prettier config**: Follow existing code patterns (Vue template formatting, semicolons-optional style)
- **Never run tauri dev**: I will run by my self
- **No test framework yet**: Run type checks with `pnpm run build` (includes `vue-tsc`)
- **Type strictness is critical**: The project has `strict: true` and `noUnusedLocals`; zero tolerance for `any` types
- **Settings panel**: Normal window with title bar (decorations: true)
- **Clipboard window**: Frameless, skip taskbar, always on top (decorations: false)
- **Greenfield project**: Modern best practices take priority over legacy patterns
- **Desktop-first UX**: Consider Windows/macOS/Linux platform differences in UI
- **Global shortcut**: Configurable via key recording in settings (restart required to apply changes)

## Project Documentation

### 📁 文档目录结构

```
docs/
├── FEATURE_SPEC.md          # 功能规格说明 - 详细功能定义
├── TECH_DESIGN.md           # 技术设计方案 - 类型定义和架构
├── UI_DESIGN.md            # UI设计规范 - 颜色和组件规范
└── IMPLEMENTATION_PLAN.md  # 实施计划 - 开发任务清单

DEVELOPMENT_PLAN.md         # 主开发计划 - 项目概览和进度
```

### 📖 如何开始新任务

**步骤1**: 阅读主开发计划
```
请阅读 @DEVELOPMENT_PLAN.md 了解当前进度和待开发功能清单
```

**步骤2**: 根据任务类型查阅详细文档
- **实现具体功能** → 阅读 `docs/FEATURE_SPEC.md` + `docs/TECH_DESIGN.md`
- **UI开发/样式调整** → 阅读 `docs/UI_DESIGN.md`
- **了解开发顺序** → 阅读 `docs/IMPLEMENTATION_PLAN.md`

**步骤3**: 查看现有代码结构
```bash
# 了解当前实现
src/components/       # Vue组件
src/composables/      # 逻辑hooks
src/types/index.ts    # TypeScript类型
src-tauri/src/        # Rust后端
```

### 🎯 当前开发优先级

**✅ 已完成 - 核心功能（P0）**
1. ~~图片类型监听与显示（ClipboardItem显示缩略图）~~
2. ~~文件/文件夹类型监听与显示~~
3. ~~左键/双击/右键交互重构~~
4. ~~右键上下文菜单（ContextMenu组件）~~

**🟡 P1 - 增强体验（当前优先级）**
5. 设置面板完善（历史记录删除按钮、数据备份功能）
6. 存储路径显示

**🟢 P2 - 优化完善**
7. 多语言/主题切换
8. 系统托盘集成
9. 性能优化

### 💡 快速开发提示

**实现图片支持时参考:**
- 技术方案: `docs/TECH_DESIGN.md` → 图片处理方案
- UI规范: `docs/UI_DESIGN.md` → 类型标签颜色
- 类型定义: `src/types/index.ts` → ClipboardItem

**实现交互增强时参考:**
- 技术方案: `docs/TECH_DESIGN.md` → 交互实现方案
- 功能规格: `docs/FEATURE_SPEC.md` → 交互规格

**实现设置面板时参考:**
- UI规范: `docs/UI_DESIGN.md` → 设置面板布局
- 功能规格: `docs/FEATURE_SPEC.md` → 设置面板功能规格

### ⚠️ 重要约束

1. **从不使用 `any` 类型** - 项目启用 strict mode
2. **不提交代码** - 用户会自己运行和测试
3. **遵循现有代码风格** - 特别是 Vue Composition API 模式
4. **Rust 后端命令** - 所有新功能都需要对应的 Tauri 命令
5. **类型安全** - 更新类型定义后再实现功能

### 🔄 开发工作流程

```
1. 阅读相关文档了解需求
2. 更新 TypeScript 类型定义 (src/types/index.ts)
3. 更新 Rust 模型 (src-tauri/src/models.rs)
4. 实现 Rust 后端命令
5. 实现 Vue 前端组件
6. 运行类型检查: pnpm run build
7. 确认无 TypeScript 错误
```