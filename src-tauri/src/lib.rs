mod clipboard;
mod models;
mod storage;

use std::sync::Arc;
use tokio::sync::Mutex;

use clipboard::ClipboardManager;
use models::{
    AppSettings, ClipboardContentType, ClipboardItem, ClipboardMetadata, ClearHistoryRequest,
    GetHistoryRequest, SearchRequest,
};
use storage::Database;
use tauri::{Manager, Emitter};
use tauri_plugin_global_shortcut::{GlobalShortcutExt, Shortcut, ShortcutState};

pub struct AppState {
    pub clipboard_manager: ClipboardManager,
    pub database: Arc<Database>,
}

impl AppState {
    pub fn new(database: Arc<Database>, settings: Arc<Mutex<AppSettings>>) -> Self {
        Self {
            clipboard_manager: ClipboardManager::new(database.clone(), settings),
            database,
        }
    }
}

/// Add clipboard item (text/HTML)
#[tauri::command]
async fn add_clipboard_item(
    state: tauri::State<'_, Arc<Mutex<AppState>>>,
    text: String,
    html: Option<String>,
    is_internal_copy: Option<bool>,
) -> Result<Option<ClipboardItem>, String> {
    let state = state.lock().await;
    let is_internal = is_internal_copy.unwrap_or(false);
    state
        .clipboard_manager
        .handle_clipboard_change(text, html, is_internal)
        .await
}

/// Add clipboard item (extended - for images, files)
#[tauri::command]
async fn add_clipboard_item_extended(
    state: tauri::State<'_, Arc<Mutex<AppState>>>,
    content_type: ClipboardContentType,
    content: String,
    file_paths: Option<Vec<String>>,
    thumbnail_path: Option<String>,
    metadata: Option<ClipboardMetadata>,
    is_internal_copy: Option<bool>,
) -> Result<Option<ClipboardItem>, String> {
    let state = state.lock().await;
    let is_internal = is_internal_copy.unwrap_or(false);
    state
        .clipboard_manager
        .handle_clipboard_change_extended(
            content_type,
            content,
            file_paths,
            thumbnail_path,
            metadata,
            is_internal,
        )
        .await
}

/// Get clipboard history
#[tauri::command]
fn get_clipboard_history(
    state: tauri::State<'_, Arc<Mutex<AppState>>>,
    request: GetHistoryRequest,
) -> Result<Vec<ClipboardItem>, String> {
    let state = state.blocking_lock();
    state.clipboard_manager.get_history(request)
}

/// Search clipboard history
#[tauri::command]
fn search_clipboard_history(
    state: tauri::State<'_, Arc<Mutex<AppState>>>,
    request: SearchRequest,
) -> Result<Vec<ClipboardItem>, String> {
    let state = state.blocking_lock();
    state.clipboard_manager.search_history(request)
}

/// Delete clipboard item
#[tauri::command]
fn delete_clipboard_item(
    state: tauri::State<'_, Arc<Mutex<AppState>>>,
    id: i64,
) -> Result<(), String> {
    let state = state.blocking_lock();
    state.clipboard_manager.delete_item(id)
}

/// Clear clipboard history
#[tauri::command]
fn clear_clipboard_history(
    state: tauri::State<'_, Arc<Mutex<AppState>>>,
    request: ClearHistoryRequest,
) -> Result<i64, String> {
    let state = state.blocking_lock();
    state
        .clipboard_manager
        .clear_history(request.keep_count, request.keep_days)
}

/// Get settings
#[tauri::command]
fn get_settings(state: tauri::State<'_, Arc<Mutex<AppState>>>) -> Result<AppSettings, String> {
    let state = state.blocking_lock();
    state.clipboard_manager.get_settings()
}

/// Save settings
#[tauri::command]
fn save_settings(
    state: tauri::State<'_, Arc<Mutex<AppState>>>,
    settings: AppSettings,
) -> Result<(), String> {
    let state = state.blocking_lock();
    state.clipboard_manager.save_settings(&settings)
}

/// Check if first run
#[tauri::command]
fn is_first_run(state: tauri::State<'_, Arc<Mutex<AppState>>>) -> Result<bool, String> {
    let state = state.blocking_lock();
    state.database.is_first_run().map_err(|e| e.to_string())
}

/// Mark app as initialized
#[tauri::command]
fn mark_app_initialized(state: tauri::State<'_, Arc<Mutex<AppState>>>) -> Result<(), String> {
    let state = state.blocking_lock();
    state
        .database
        .mark_app_initialized()
        .map_err(|e| e.to_string())
}

/// Show clipboard window
#[tauri::command]
fn show_clipboard_window(app: tauri::AppHandle) -> Result<(), String> {
    if let Some(window) = app.get_webview_window("clipboard") {
        window.show().map_err(|e| e.to_string())?;
        window.set_focus().map_err(|e| e.to_string())?;
    }
    Ok(())
}

/// Hide clipboard window
#[tauri::command]
fn hide_clipboard_window(app: tauri::AppHandle) -> Result<(), String> {
    if let Some(window) = app.get_webview_window("clipboard") {
        window.hide().map_err(|e| e.to_string())?;
    }
    Ok(())
}

/// Toggle clipboard window visibility
#[tauri::command]
fn toggle_clipboard_window(app: tauri::AppHandle) -> Result<(), String> {
    if let Some(window) = app.get_webview_window("clipboard") {
        if window.is_visible().unwrap_or(false) {
            window.hide().map_err(|e| e.to_string())?;
        } else {
            window.show().map_err(|e| e.to_string())?;
            window.set_focus().map_err(|e| e.to_string())?;
        }
    }
    Ok(())
}

/// Show settings window
#[tauri::command]
fn show_settings_window(app: tauri::AppHandle) -> Result<(), String> {
    if let Some(window) = app.get_webview_window("main") {
        window.show().map_err(|e| e.to_string())?;
        window.set_focus().map_err(|e| e.to_string())?;
    }
    Ok(())
}

/// Hide settings window
#[tauri::command]
fn hide_settings_window(app: tauri::AppHandle) -> Result<(), String> {
    if let Some(window) = app.get_webview_window("main") {
        window.hide().map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_clipboard_x::init())
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .plugin(tauri_plugin_autostart::init(
            tauri_plugin_autostart::MacosLauncher::LaunchAgent,
            None,
        ))
        .setup(|app| {
            // Get app data directory
            let app_dir = app
                .path()
                .app_data_dir()
                .expect("Failed to get app data directory");

            // Create directory if not exists
            std::fs::create_dir_all(&app_dir).expect("Failed to create app data directory");

            // Initialize database
            let database = Arc::new(
                Database::new(app_dir).expect("Failed to initialize database"),
            );

            // Get initial settings
            let settings = Arc::new(Mutex::new(
                database.get_settings().expect("Failed to get settings"),
            ));

            // Create app state
            let state = Arc::new(Mutex::new(AppState::new(
                database.clone(),
                settings,
            )));

            // Manage state
            app.manage(state);

            // Register global shortcut (Alt+C)
            let app_handle = app.handle().clone();
            let shortcut: Shortcut = "Alt+C".parse().unwrap();
            
            // Unregister if already registered (to handle restarts)
            let _ = app.global_shortcut().unregister(shortcut.clone());
            
            let _ = app.global_shortcut().on_shortcut(shortcut, move |_app, _shortcut, event| {
                if event.state == ShortcutState::Pressed {
                    // Toggle clipboard window
                    if let Some(window) = app_handle.get_webview_window("clipboard") {
                        if window.is_visible().unwrap_or(false) {
                            let _ = window.hide();
                        } else {
                            let _ = window.show();
                            let _ = window.set_focus();
                        }
                    }
                }
            });

            // Handle clipboard window blur event - auto hide
            if let Some(clipboard_window) = app.get_webview_window("clipboard") {
                let cw = clipboard_window.clone();
                clipboard_window.on_window_event(move |event| {
                    if let tauri::WindowEvent::Focused(focused) = event {
                        if !focused {
                            let _ = cw.hide();
                        }
                    }
                });
            }

            // Check if first run, show settings window
            let is_first = database.is_first_run().unwrap_or(true);
            if is_first {
                if let Some(main_window) = app.get_webview_window("main") {
                    let _ = main_window.show();
                    let _ = main_window.set_focus();
                }
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
            is_first_run,
            mark_app_initialized,
            show_clipboard_window,
            hide_clipboard_window,
            toggle_clipboard_window,
            show_settings_window,
            hide_settings_window,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
