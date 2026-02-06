mod clipboard;
mod models;
mod storage;
mod window_manager;

use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::Mutex;

use clipboard::ClipboardManager;
use models::{
    AppSettings, ClipboardItem, ClearHistoryRequest, GetHistoryRequest, SearchRequest,
};
use storage::Database;
use tauri::Manager;
use tauri::Emitter;
use window_manager::WindowManager;

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

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_clipboard_x::init())
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

            // 设置全局快捷键
            #[cfg(desktop)]
            {
                use tauri_plugin_global_shortcut::{Code, Modifiers, ShortcutState};
                
                let _settings_for_shortcut = settings.clone();
                let _ = app.handle().plugin(
                    tauri_plugin_global_shortcut::Builder::new()
                        .with_shortcuts(["alt+v"])?
                        .with_handler(move |app, shortcut, event| {
                            if event.state == ShortcutState::Pressed {
                                if shortcut.matches(Modifiers::ALT, Code::KeyV) {
                                    let app_handle = app.clone();
                                    let state = app_state.clone();
                                    tauri::async_runtime::spawn(async move {
                                        let state = state.lock().await;
                                        let _ = state.window_manager.toggle_clipboard_window(&app_handle).await;
                                    });
                                }
                            }
                        })
                        .build(),
                );
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

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            add_clipboard_item,
            get_clipboard_history,
            search_clipboard_history,
            delete_clipboard_item,
            clear_clipboard_history,
            get_settings,
            save_settings,
            toggle_clipboard_window,
            hide_clipboard_window,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}