mod clipboard;
mod models;
mod storage;
mod window_manager;
mod shortcut_manager;
mod tray_manager;

use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::Mutex;

use clipboard::ClipboardManager;
use models::{
    AppSettings, ClipboardItem, ClipboardContentType, ClipboardMetadata, ClearHistoryRequest, GetHistoryRequest, SearchRequest,
};
use storage::Database;
use tauri::Manager;
use tauri::Emitter;
use window_manager::WindowManager;
use shortcut_manager::validate_hotkey;
use tauri_plugin_global_shortcut::GlobalShortcutExt;

pub struct AppState {
    clipboard_manager: ClipboardManager,
    window_manager: WindowManager,
}

impl AppState {
    pub fn new(database: Arc<Database>, settings: Arc<Mutex<AppSettings>>) -> Self {
        Self {
            clipboard_manager: ClipboardManager::new(database.clone(), settings.clone()),
            window_manager: WindowManager::new(settings),
        }
    }
}

#[tauri::command]
async fn add_clipboard_item(
    state: tauri::State<'_, Arc<Mutex<AppState>>>,
    text: String,
    html: Option<String>,
) -> Result<Option<ClipboardItem>, String> {
    let state = state.lock().await;
    state.clipboard_manager.handle_clipboard_change(text, html).await
}

#[tauri::command]
async fn add_clipboard_item_extended(
    state: tauri::State<'_, Arc<Mutex<AppState>>>,
    content_type: ClipboardContentType,
    content: String,
    file_paths: Option<Vec<String>>,
    thumbnail_path: Option<String>,
    metadata: Option<ClipboardMetadata>,
) -> Result<Option<ClipboardItem>, String> {
    let state = state.lock().await;
    state.clipboard_manager.handle_clipboard_change_extended(
        content_type,
        content,
        file_paths,
        thumbnail_path,
        metadata,
    ).await
}

#[tauri::command]
fn get_clipboard_history(
    state: tauri::State<'_, Arc<Mutex<AppState>>>,
    request: GetHistoryRequest,
) -> Result<Vec<ClipboardItem>, String> {
    let state = state.blocking_lock();
    state.clipboard_manager.get_history(request)
}

#[tauri::command]
fn search_clipboard_history(
    state: tauri::State<'_, Arc<Mutex<AppState>>>,
    request: SearchRequest,
) -> Result<Vec<ClipboardItem>, String> {
    let state = state.blocking_lock();
    state.clipboard_manager.search_history(request)
}

#[tauri::command]
fn delete_clipboard_item(
    state: tauri::State<'_, Arc<Mutex<AppState>>>,
    id: i64,
) -> Result<(), String> {
    let state = state.blocking_lock();
    state.clipboard_manager.delete_item(id)
}

#[tauri::command]
fn clear_clipboard_history(
    state: tauri::State<'_, Arc<Mutex<AppState>>>,
    request: ClearHistoryRequest,
) -> Result<i64, String> {
    let state = state.blocking_lock();
    state.clipboard_manager.clear_history(request)
}

#[tauri::command]
fn get_settings(state: tauri::State<'_, Arc<Mutex<AppState>>>) -> Result<AppSettings, String> {
    let state = state.blocking_lock();
    state.clipboard_manager.get_settings()
}

#[tauri::command]
async fn save_settings(
    state: tauri::State<'_, Arc<Mutex<AppState>>>,
    settings: AppSettings,
) -> Result<(), String> {
    let state = state.lock().await;
    state.clipboard_manager.save_settings(&settings).await
}

#[tauri::command]
async fn toggle_clipboard_window(
    state: tauri::State<'_, Arc<Mutex<AppState>>>,
    app: tauri::AppHandle,
) -> Result<bool, String> {
    let state = state.lock().await;
    state.window_manager.toggle_clipboard_window(&app).await
}

#[tauri::command]
async fn hide_clipboard_window(
    state: tauri::State<'_, Arc<Mutex<AppState>>>,
    app: tauri::AppHandle,
) -> Result<(), String> {
    let state = state.lock().await;
    state.window_manager.hide_clipboard_window(&app).await
}

#[tauri::command]
fn open_file(path: String) -> Result<(), String> {
    #[cfg(target_os = "windows")]
    {
        std::process::Command::new("cmd")
            .args(["/C", "start", "", &path])
            .spawn()
            .map_err(|e| e.to_string())?;
    }
    #[cfg(target_os = "macos")]
    {
        std::process::Command::new("open")
            .arg(&path)
            .spawn()
            .map_err(|e| e.to_string())?;
    }
    #[cfg(target_os = "linux")]
    {
        std::process::Command::new("xdg-open")
            .arg(&path)
            .spawn()
            .map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[tauri::command]
fn show_in_folder(path: String) -> Result<(), String> {
    #[cfg(target_os = "windows")]
    {
        std::process::Command::new("explorer")
            .args(["/select,", &path])
            .spawn()
            .map_err(|e| e.to_string())?;
    }
    #[cfg(target_os = "macos")]
    {
        std::process::Command::new("open")
            .args(["-R", &path])
            .spawn()
            .map_err(|e| e.to_string())?;
    }
    #[cfg(target_os = "linux")]
    {
        // Linux: open parent directory
        let parent = std::path::Path::new(&path)
            .parent()
            .map(|p| p.to_string_lossy().to_string())
            .unwrap_or(path);
        std::process::Command::new("xdg-open")
            .arg(&parent)
            .spawn()
            .map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[tauri::command]
fn toggle_favorite(
    state: tauri::State<'_, Arc<Mutex<AppState>>>,
    id: i64,
    is_favorite: bool,
) -> Result<(), String> {
    let state = state.blocking_lock();
    state.clipboard_manager.toggle_favorite(id, is_favorite)
}

#[tauri::command]
fn validate_shortcut(hotkey: String) -> Result<(), String> {
    validate_hotkey(&hotkey)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    use tauri_plugin_global_shortcut::ShortcutState;

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_clipboard_x::init())
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .setup(|app| {
            let app_dir = app
                .path()
                .app_local_data_dir()
                .unwrap_or_else(|_| PathBuf::from("."));

            let database = Arc::new(
                Database::new(app_dir).expect("Failed to initialize database"),
            );

            let settings = database
                .get_settings()
                .unwrap_or_else(|_| AppSettings::default());
            let settings = Arc::new(Mutex::new(settings));

            let app_state = Arc::new(Mutex::new(AppState::new(database, settings.clone())));
            app.manage(app_state.clone());

            // 尝试注册主快捷键
            let hotkey = settings.blocking_lock().hotkey.clone();
            let app_handle_for_shortcut = app.handle().clone();
            let app_state_for_shortcut = app_state.clone();

            if let Ok((modifiers, code)) = shortcut_manager::parse_shortcut(&hotkey) {
                let shortcut = tauri_plugin_global_shortcut::Shortcut::new(Some(modifiers), code);

                // 注册快捷键处理器
                let result = app.global_shortcut().on_shortcut(shortcut, move |app, _shortcut, event| {
                    if event.state == ShortcutState::Pressed {
                        let app_handle = app.clone();
                        let state = app_state_for_shortcut.clone();
                        tauri::async_runtime::spawn(async move {
                            let state = state.lock().await;
                            let _ = state.window_manager.toggle_clipboard_window(&app_handle).await;
                        });
                    }
                });

                if let Err(e) = result {
                    eprintln!("警告: 快捷键 '{}' 注册失败: {}", hotkey, e);
                    eprintln!("提示: 快捷键可能已被其他程序占用");
                    eprintln!("请修改设置中的快捷键后重启应用");

                    // 发送事件到前端通知用户
                    let hotkey_clone = hotkey.clone();
                    tauri::async_runtime::spawn(async move {
                        let _ = app_handle_for_shortcut.emit("shortcut-registration-failed", hotkey_clone);
                    });
                } else {
                    println!("快捷键 '{}' 注册成功", hotkey);
                }
            }

            // 获取主窗口并设置失焦监听
            if let Some(main_window) = app.get_webview_window("main") {
                let app_handle = app.handle().clone();
                main_window.on_window_event(move |event| {
                    if let tauri::WindowEvent::Focused(false) = event {
                        // 窗口失去焦点时隐藏
                        let app_handle = app_handle.clone();
                        let _ = app_handle.emit("window-blur", ());
                    }
                });
            }

            // 初始化系统托盘
            if let Err(e) = tray_manager::create_tray(app.handle()) {
                eprintln!("系统托盘初始化失败: {}", e);
            }

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            add_clipboard_item,
            add_clipboard_item_extended,
            get_clipboard_history,
            search_clipboard_history,
            delete_clipboard_item,
            clear_clipboard_history,
            get_settings,
            save_settings,
            toggle_clipboard_window,
            hide_clipboard_window,
            open_file,
            show_in_folder,
            toggle_favorite,
            validate_shortcut,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}