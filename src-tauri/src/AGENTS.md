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
| fuzzy_search.rs | Search with pinyin + initials |
| window_manager.rs | Window create/hide/show |
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
| Search algorithm | fuzzy_search.rs |

## NOTES

- lib.rs is large (~630 lines) - consider splitting if grows
- All commands called via `invoke("name", { param })` from frontend
- tauri.conf.json: permissions in capabilities/
