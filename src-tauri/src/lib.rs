mod clipboard;
mod models;
mod storage;
mod window_manager;
mod shortcut_manager;
mod tray_manager;
mod fuzzy_search;

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
            window_manager: WindowManager::new(settings, database),
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
    app: tauri::AppHandle,
    settings: AppSettings,
) -> Result<(), String> {
    let state = state.lock().await;
    
    // 更新设置
    state.clipboard_manager.save_settings(&settings).await?;
    
    // 更新开机自启状态
    if let Err(e) = update_autostart(&app, settings.auto_start).await {
        eprintln!("更新开机自启状态失败: {}", e);
    }
    
    // 如果剪贴板窗口可见，重新定位窗口（窗口位置设置实时生效）
    if let Some(window) = app.get_webview_window("clipboard") {
        if let Ok(true) = window.is_visible() {
            if let Err(e) = state.window_manager.reposition_window(&window).await {
                eprintln!("重新定位窗口失败: {}", e);
            }
        }
    }
    
    Ok(())
}

/// 更新开机自启状态
async fn update_autostart(app: &tauri::AppHandle, enable: bool) -> Result<(), String> {
    use tauri_plugin_autostart::ManagerExt;

    let autostart_manager = app.autolaunch();

    // 先检查当前状态，避免不必要的操作
    let current_state = match autostart_manager.is_enabled() {
        Ok(state) => state,
        Err(_) => {
            // 如果无法获取状态（开发模式或未安装），静默忽略
            // 在实际安装后的应用中，这不应该发生
            return Ok(());
        }
    };

    // 如果状态已经符合要求，不需要操作
    if current_state == enable {
        return Ok(());
    }

    if enable {
        if let Err(e) = autostart_manager.enable() {
            // Windows上开发模式可能出现注册表错误，静默处理
            let err_str = e.to_string();
            if err_str.contains("os error 2") || err_str.contains("系统找不到指定的文件") {
                println!("开机自启启用失败（开发模式）: {}", e);
                return Ok(());
            }
            return Err(err_str);
        }
        println!("开机自启已启用");
    } else {
        if let Err(e) = autostart_manager.disable() {
            // Windows上注册表项不存在时会出现错误，静默处理
            let err_str = e.to_string();
            if err_str.contains("os error 2") || err_str.contains("系统找不到指定的文件") {
                println!("开机自启禁用失败（注册表项不存在）: {}", e);
                return Ok(());
            }
            return Err(err_str);
        }
        println!("开机自启已禁用");
    }

    Ok(())
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
fn get_file_size(path: String) -> Result<u64, String> {
    let metadata = std::fs::metadata(&path)
        .map_err(|e| format!("无法获取文件信息: {}", e))?;
    Ok(metadata.len())
}

#[tauri::command]
fn update_tags(
    state: tauri::State<'_, Arc<Mutex<AppState>>>,
    id: i64,
    tags: Option<Vec<String>>,
) -> Result<(), String> {
    let state = state.blocking_lock();
    state.clipboard_manager.update_tags(id, tags)
}

#[tauri::command]
fn get_all_tags(
    state: tauri::State<'_, Arc<Mutex<AppState>>>,
) -> Result<Vec<(String, i64)>, String> {
    let state = state.blocking_lock();
    state.clipboard_manager.get_all_tags()
}

#[tauri::command]
fn validate_shortcut(hotkey: String) -> Result<(), String> {
    validate_hotkey(&hotkey)
}

#[tauri::command]
fn export_clipboard_data(
    state: tauri::State<'_, Arc<Mutex<AppState>>>,
) -> Result<String, String> {
    let state = state.blocking_lock();
    state.clipboard_manager.export_data()
}

#[tauri::command]
fn import_clipboard_data(
    state: tauri::State<'_, Arc<Mutex<AppState>>>,
    json_data: String,
) -> Result<i64, String> {
    let state = state.blocking_lock();
    state.clipboard_manager.import_data(&json_data)
}

#[tauri::command]
fn get_storage_paths(app: tauri::AppHandle) -> Result<std::collections::HashMap<String, String>, String> {
    let mut paths = std::collections::HashMap::new();
    
    // 获取数据存储路径
    let app_dir = app.path()
        .app_local_data_dir()
        .map_err(|e| e.to_string())?;
    paths.insert("data_dir".to_string(), app_dir.to_string_lossy().to_string());
    
    // 获取日志存储路径
    let log_dir = app.path()
        .app_log_dir()
        .map_err(|e| e.to_string())?;
    paths.insert("log_dir".to_string(), log_dir.to_string_lossy().to_string());
    
    Ok(paths)
}

#[tauri::command]
fn simulate_paste(paste_shortcut: String) -> Result<(), String> {
    // 模拟粘贴操作（根据设置使用 Ctrl+V 或 Shift+Insert）
    use std::thread;
    use std::time::Duration;

    // 等待窗口隐藏并焦点回到原窗口
    thread::sleep(Duration::from_millis(200));

    let use_shift_insert = paste_shortcut == "shift_insert";

    #[cfg(target_os = "windows")]
    {
        use winapi::um::winuser::{keybd_event, VK_SHIFT, VK_INSERT, KEYEVENTF_KEYUP, KEYEVENTF_SCANCODE, KEYEVENTF_EXTENDEDKEY};

        unsafe {
            if use_shift_insert {
                // 使用 Shift+Insert
                // Insert 是扩展键，需要 KEYEVENTF_EXTENDEDKEY 标志
                const SCANCODE_SHIFT: u8 = 0x2A;

                // 按下 Shift（使用虚拟键码 + 扫描码）
                keybd_event(VK_SHIFT as u8, SCANCODE_SHIFT, KEYEVENTF_SCANCODE, 0);
                thread::sleep(Duration::from_millis(20));

                // 按下 Insert（使用扩展键标志）
                keybd_event(VK_INSERT as u8, 0, KEYEVENTF_EXTENDEDKEY, 0);
                thread::sleep(Duration::from_millis(50));

                // 释放 Insert
                keybd_event(VK_INSERT as u8, 0, KEYEVENTF_EXTENDEDKEY | KEYEVENTF_KEYUP, 0);
                thread::sleep(Duration::from_millis(20));

                // 释放 Shift
                keybd_event(VK_SHIFT as u8, SCANCODE_SHIFT, KEYEVENTF_SCANCODE | KEYEVENTF_KEYUP, 0);
            } else {
                // 使用 Ctrl+V
                // V 键的扫描码是 0x2F, Ctrl 扫描码: 0x1D
                const SCANCODE_V: u8 = 0x2F;
                const SCANCODE_CTRL: u8 = 0x1D;
                const VK_CONTROL: u8 = 0xA3; // 右 Ctrl

                // 按下 Ctrl
                keybd_event(VK_CONTROL as u8, SCANCODE_CTRL, KEYEVENTF_SCANCODE, 0);
                thread::sleep(Duration::from_millis(20));

                // 按下 V
                keybd_event(0x41 + 21, SCANCODE_V, KEYEVENTF_SCANCODE, 0); // 'V' = 0x56
                thread::sleep(Duration::from_millis(50));

                // 释放 V
                keybd_event(0x41 + 21, SCANCODE_V, KEYEVENTF_SCANCODE | KEYEVENTF_KEYUP, 0);
                thread::sleep(Duration::from_millis(20));

                // 释放 Ctrl
                keybd_event(VK_CONTROL as u8, SCANCODE_CTRL, KEYEVENTF_SCANCODE | KEYEVENTF_KEYUP, 0);
            }
        }
    }

    #[cfg(target_os = "macos")]
    {
        // macOS 始终使用 Command+V
        use enigo::{Direction, Enigo, Key, Keyboard, Settings};
        let mut enigo = Enigo::new(&Settings::default()).map_err(|e| e.to_string())?;
        enigo.key(Key::Meta, Direction::Press).map_err(|e| e.to_string())?;
        enigo.key(Key::Unicode('v'), Direction::Click).map_err(|e| e.to_string())?;
        enigo.key(Key::Meta, Direction::Release).map_err(|e| e.to_string())?;
    }

    #[cfg(target_os = "linux")]
    {
        use enigo::{Direction, Enigo, Key, Keyboard, Settings};
        let mut enigo = Enigo::new(&Settings::default()).map_err(|e| e.to_string())?;
        
        if use_shift_insert {
            // 使用 Shift+Insert
            enigo.key(Key::Shift, Direction::Press).map_err(|e| e.to_string())?;
            enigo.key(Key::Insert, Direction::Click).map_err(|e| e.to_string())?;
            enigo.key(Key::Shift, Direction::Release).map_err(|e| e.to_string())?;
        } else {
            // 使用 Ctrl+V
            enigo.key(Key::Control, Direction::Press).map_err(|e| e.to_string())?;
            enigo.key(Key::Unicode('v'), Direction::Click).map_err(|e| e.to_string())?;
            enigo.key(Key::Control, Direction::Release).map_err(|e| e.to_string())?;
        }
    }

    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    use tauri_plugin_global_shortcut::ShortcutState;
    use tauri_plugin_autostart::MacosLauncher;

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_clipboard_x::init())
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .plugin(tauri_plugin_autostart::init(
            MacosLauncher::LaunchAgent,
            Some(vec!["--hidden"]),
        ))
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

            // 在移动 database 之前先检查是否是首次运行
            let is_first_run = database
                .is_first_run()
                .unwrap_or(true);

            let app_state = Arc::new(Mutex::new(AppState::new(database.clone(), settings.clone())));
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

            // 获取主窗口并设置事件监听
            if let Some(main_window) = app.get_webview_window("main") {
                let app_handle = app.handle().clone();
                main_window.on_window_event(move |event| {
                    match event {
                        tauri::WindowEvent::Focused(false) => {
                            // 窗口失去焦点时隐藏
                            let app_handle = app_handle.clone();
                            let _ = app_handle.emit("window-blur", ());
                        }
                        tauri::WindowEvent::CloseRequested { api, .. } => {
                            // 阻止默认关闭行为，改为隐藏窗口
                            api.prevent_close();
                            let app_handle = app_handle.clone();
                            if let Some(win) = app_handle.get_webview_window("main") {
                                let _ = win.hide();
                            }
                        }
                        _ => {}
                    }
                });
            }

            // 初始化系统托盘
            if let Err(e) = tray_manager::create_tray(app.handle()) {
                eprintln!("系统托盘初始化失败: {}", e);
            }

            // 根据设置初始化开机自启状态
            let auto_start = settings.blocking_lock().auto_start;
            let app_handle = app.handle().clone();
            let database_for_init = database.clone();
            tauri::async_runtime::spawn(async move {
                if let Err(e) = update_autostart(&app_handle, auto_start).await {
                    eprintln!("初始化开机自启状态失败: {}", e);
                }

                // 如果是首次运行，显示设置窗口并标记已初始化
                if is_first_run {
                    println!("首次运行，显示设置窗口");
                    if let Some(window) = app_handle.get_webview_window("main") {
                        let _ = window.show();
                        let _ = window.set_focus();
                    }
                    // 标记应用已初始化
                    if let Err(e) = database_for_init.mark_app_initialized() {
                        eprintln!("标记应用初始化状态失败: {}", e);
                    }
                }
            });

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
            get_file_size,
            update_tags,
            get_all_tags,
            validate_shortcut,
            export_clipboard_data,
            import_clipboard_data,
            get_storage_paths,
            simulate_paste,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}