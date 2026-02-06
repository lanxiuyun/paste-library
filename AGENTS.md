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
  │   ├── DragHandle.vue         # Window drag capsule (for clipboard window)
  │   └── SettingsPanel.vue      # Settings panel with left navigation
  ├── composables/               # Reusable logic (hooks)
  │   ├── useClipboard.ts        # Clipboard monitoring logic
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
  ├── tauri.conf.json            # Tauri config (settings: decorations=true, clipboard: decorations=false)
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
- Real-time clipboard monitoring (text + HTML)
- SQLite persistence with automatic deduplication (SHA256 hash)
- Card-based UI with tabs (All/Text/Image/File/Favorite)
- Search functionality (fuzzy search)
- **Global hotkey (Alt+V)** to show/hide clipboard window
- **Settings panel** with left sidebar navigation:
  - 剪贴板: 窗口设置、音效设置、搜索设置、内容设置
  - 历史记录: 最大记录数、自动清理
  - 通用设置: 开机自启、应用黑名单
  - 快捷键: 唤醒快捷键、窗口尺寸
  - 数据备份: 导出/导入（UI ready）
  - 关于: 应用信息、打开剪贴板按钮
- **Window management**:
  - Settings: Normal window with title bar
  - Clipboard: Frameless, skip taskbar, always on top, auto-hide on blur
- Copy/delete clipboard items
- Data persistence with comprehensive settings

### In Progress ⏳
- System tray integration
- Data export/backup functionality (backend)

### Planned 📋
- Image clipboard support (with OCR)
- Cross-device sync architecture
- Dark theme (currently light only)
- Advanced search filters (by date range)

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
- 唤醒快捷键 (display only: Alt+V)
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
- **Global shortcut**: Alt+V is hardcoded in Rust, display-only in settings UI
