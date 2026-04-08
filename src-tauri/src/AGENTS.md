# src-tauri/src

Rust backend (Tauri commands).

## FILES

| File | Purpose |
|------|---------|
| lib.rs | Entry + all #[tauri::command] handlers |
| main.rs | Binary entry (6 lines, calls lib::run) |
| clipboard.rs | Clipboard monitoring (text/image/files) |
| models.rs | Data structures (ClipboardItem, Settings) |
| storage.rs | SQLite CRUD via rusqlite |
| window_manager.rs | Clipboard window lifecycle, blur hide, pin behavior |
| tray_manager.rs | System tray setup |
| shortcut_manager.rs | Global hotkey handling |
| prevent_default.rs | Event prevention utilities |

## CONVENTIONS

- Commands: `#[tauri::command]` + `async fn`
- Use `tokio::sync::Mutex` (not std::sync::Mutex)
- Error handling: return `Result<T, String>`
- SQLite: rusqlite with SHA256 deduplication

## WHERE TO LOOK

| Task | File |
|------|------|
| Add Tauri command | lib.rs (add fn + register) |
| Modify clipboard logic | clipboard.rs |
| Database schema | storage.rs |
| Search algorithm | storage.rs (SQL LIKE filtering) |
| Clipboard window visibility / blur / pin behavior | window_manager.rs |
| Global shortcut registration | lib.rs + shortcut_manager.rs |

## NOTES

- lib.rs is large (~630 lines) - consider splitting if grows
- All commands called via `invoke("name", { param })` from frontend
- tauri.conf.json: permissions in capabilities/
- `window_manager.rs` owns clipboard window lifecycle: create/show/hide, blur-delay hiding, focus cancellation, and position persistence
- Clipboard window drag safety:
  - Frameless window dragging via `data-tauri-drag-region` can emit `Focused(false)` during native drag
  - On Windows, native drag may enter a modal move loop, so delayed blur-hide tasks can outlive focus recovery
  - Keep `WindowEvent::Moved` handling in `window_manager.rs` to clear `pending_hide`, otherwise dragging the clipboard window can hide it unexpectedly
- Pin mode semantics:
  - `Pinned` only disables blur-triggered auto-hide
  - Manual hide paths still work in pinned mode: global shortcut toggle, `Esc`, and explicit `hide_clipboard_window`
  - Pin mode is not a separate positioning strategy; window position still follows normal settings (`remember` / `center` / `cursor`)
- When changing clipboard window behavior, review both backend events (`clipboard-window-blur`, `pin-mode-changed`) and the frontend listeners that mirror window visibility state
