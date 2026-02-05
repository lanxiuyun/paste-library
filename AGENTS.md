# AGENTS.md — Coding Guidelines for Tauri + Vue + TypeScript

## Project Overview

**paste-library**: A modern clipboard manager built with Tauri v2 and Vue 3. Features real-time clipboard monitoring, history persistence, search, and a clean card-based UI.

- **Frontend**: Vue 3 (Composition API with `<script setup>`), TypeScript, Vite
- **Backend**: Tauri v2 with Rust
- **Database**: SQLite (via rusqlite)
- **Plugins**: tauri-plugin-clipboard-x (clipboard monitoring)
- **Status**: In development — core features implemented

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

The app uses a **frameless window** with a minimal drag handle:

```vue
<!-- DragHandle.vue - Simple drag line at top -->
<div class="drag-bar" data-tauri-drag-region>
  <div class="drag-capsule" data-tauri-drag-region>
    <div class="drag-line"></div>
  </div>
</div>
```

**Key points**:
- Use `data-tauri-drag-region` attribute for draggable areas
- Use `-webkit-app-region: drag` / `app-region: drag` in CSS
- Interactive elements (buttons, inputs) must have `app-region: no-drag`
- Top drag bar height: 36px with a simple line indicator
- No window control buttons (minimize/close) in the drag area

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
- Avoid global state until project needs it
- Reactive refs for form data, UI state; computed for derived values

### CSS & Styling

- Scoped styles preferred (`<style scoped>`)
- Global styles in `src/` root CSS file (currently in App.vue as fallback)
- CSS variables for theming (already used: light/dark mode)
- Avoid inline styles unless truly dynamic
- Media queries for responsive design (already present for dark mode)
- **Card-based design**: Rounded corners (8px), subtle shadows, clean typography

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
  ├── main.ts                # App entry point
  ├── App.vue                # Root component (contains DragHandle)
  ├── assets/                # Static images/SVGs
  ├── components/            # Reusable Vue components
  │   ├── ClipboardItem.vue  # Card component for single clipboard item
  │   ├── ClipboardList.vue  # Main list with tabs and search
  │   └── DragHandle.vue     # Window drag capsule + controls
  ├── composables/           # Reusable logic (hooks)
  │   ├── useClipboard.ts    # Clipboard monitoring logic
  │   └── useSettings.ts     # Settings management
  ├── types/                 # TypeScript type definitions
  │   └── index.ts           # Shared types (ClipboardItem, etc.)
  └── styles/                # Global CSS (if needed)

src-tauri/
  ├── src/                   # Rust backend
  │   ├── lib.rs             # Main entry + Tauri commands
  │   ├── clipboard.rs       # Clipboard manager logic
  │   ├── models.rs          # Data structures (ClipboardItem, etc.)
  │   └── storage.rs         # SQLite database operations
  ├── tauri.conf.json        # Tauri config (frameless window)
  ├── capabilities/          # Permission definitions
  └── Cargo.toml             # Rust dependencies
```

---

## Tauri-Specific Guidelines

### Frameless Window Configuration

```json
{
  "app": {
    "windows": [{
      "decorations": false,
      "transparent": false,
      "center": true,
      "resizable": true
    }]
  }
}
```

### Required Permissions

```json
{
  "permissions": [
    "core:default",
    "clipboard-x:default",
    "core:window:allow-minimize",
    "core:window:allow-hide"
  ]
}
```

### Commands

- Commands in `src-tauri/src/lib.rs` prefixed with `#[tauri::command]`
- Frontend calls via `invoke("command_name", { param: value })`
- Always handle Tauri errors in try-catch (they're `string` payloads)
- Use `tokio::sync::Mutex` instead of `std::sync::Mutex` for async commands

---

## Project Features

### Implemented ✅
- Real-time clipboard monitoring (text + HTML)
- SQLite persistence with automatic deduplication
- Card-based UI with tabs (All/Text/Image/File/Favorite)
- Search functionality
- Frameless window with drag handle
- Window controls (minimize/close)
- Copy/delete clipboard items

### In Progress ⏳
- Global hotkey (Alt+V) to show/hide window
- System tray integration
- Settings panel
- Data export/backup

### Planned 📋
- Image clipboard support
- Cross-device sync architecture
- Dark theme (currently light only)
- Advanced search filters

---

## Notes for Agents

- **No linter/prettier config**: Follow existing code patterns (Vue template formatting, semicolons-optional style)
- **Never run tauri dev**: I will run by my self
- **No test framework yet**: Run type checks with `pnpm run build` (includes `vue-tsc`)
- **Type strictness is critical**: The project has `strict: true` and `noUnusedLocals`; zero tolerance for `any` types
- **Frameless window**: Always test dragging behavior after UI changes
- **Greenfield project**: Modern best practices take priority over legacy patterns
- **Desktop-first UX**: Consider Windows/macOS/Linux platform differences in UI
