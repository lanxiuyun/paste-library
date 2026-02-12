use tauri::menu::{Menu, MenuItem, PredefinedMenuItem};
use tauri::tray::{MouseButton, MouseButtonState, TrayIcon, TrayIconBuilder, TrayIconEvent};
use tauri::{Emitter, Manager};

/// 创建系统托盘
pub fn create_tray(app: &tauri::AppHandle) -> Result<TrayIcon, Box<dyn std::error::Error>> {
    // 创建托盘菜单
    let menu = create_tray_menu(app)?;

    // 创建托盘图标
    let tray = TrayIconBuilder::new()
        .icon(app.default_window_icon().unwrap().clone())
        .icon_as_template(false)
        .menu(&menu)
        .show_menu_on_left_click(false)
        .on_tray_icon_event(|tray, event| {
            let app = tray.app_handle();

            match event {
                // 双击托盘图标 - 打开设置面板
                TrayIconEvent::DoubleClick {
                    button: MouseButton::Left,
                    ..
                } => {
                    show_settings_window(app);
                }
                // 左键单击 - 也打开设置面板（可选行为）
                TrayIconEvent::Click {
                    button: MouseButton::Left,
                    button_state: MouseButtonState::Up,
                    ..
                } => {
                    // 可以选择单击打开，或者保持单击显示菜单
                    // 这里让左键单击打开设置面板，右键显示菜单
                    show_settings_window(app);
                }
                _ => {}
            }
        })
        .on_menu_event(|app, event| match event.id.as_ref() {
            "open_settings" => {
                show_settings_window(app);
            }
            "show_clipboard" => {
                show_clipboard_window(app);
            }
            "quit" => {
                app.exit(0);
            }
            _ => {}
        })
        .build(app)?;

    Ok(tray)
}

/// 创建托盘右键菜单
fn create_tray_menu(
    app: &tauri::AppHandle,
) -> Result<Menu<tauri::Wry>, Box<dyn std::error::Error>> {
    let menu = Menu::new(app)?;

    // 打开设置
    let open_settings = MenuItem::with_id(app, "open_settings", "打开设置", true, None::<&str>)?;
    menu.append(&open_settings)?;

    // 显示剪贴板历史
    let show_clipboard =
        MenuItem::with_id(app, "show_clipboard", "显示剪贴板历史", true, None::<&str>)?;
    menu.append(&show_clipboard)?;

    // 分隔线
    menu.append(&PredefinedMenuItem::separator(app)?)?;

    // 退出
    let quit = MenuItem::with_id(app, "quit", "退出", true, None::<&str>)?;
    menu.append(&quit)?;

    Ok(menu)
}

/// 显示设置窗口
fn show_settings_window(app: &tauri::AppHandle) {
    if let Some(window) = app.get_webview_window("main") {
        // 如果窗口已存在，显示并聚焦
        let _ = window.show();
        let _ = window.set_focus();
        let _ = window.unminimize();
    } else {
        // 重新创建设置窗口
        let window = tauri::webview::WebviewWindowBuilder::new(
            app,
            "main",
            tauri::WebviewUrl::App("/".into()),
        )
        .title("Paste Library - 设置")
        .inner_size(600.0, 700.0)
        .decorations(true)
        .transparent(false)
        .center()
        .resizable(true)
        .minimizable(true)
        .maximizable(false)
        .closable(true)
        .always_on_top(false)
        .build();

        // 添加关闭事件处理，关闭时隐藏窗口而非退出应用
        if let Ok(window) = window {
            let app_handle = app.clone();
            window.on_window_event(move |event| {
                if let tauri::WindowEvent::CloseRequested { api, .. } = event {
                    // 阻止默认关闭行为
                    api.prevent_close();
                    // 隐藏窗口
                    if let Some(win) = app_handle.get_webview_window("main") {
                        let _ = win.hide();
                    }
                }
            });
        }
    }
}

/// 显示剪贴板窗口
fn show_clipboard_window(app: &tauri::AppHandle) {
    if let Some(window) = app.get_webview_window("clipboard") {
        let _ = window.show();
        let _ = window.set_focus();
    } else {
        // 通过 emit 事件通知前端显示剪贴板窗口
        // 或者直接调用 window_manager 的方法
        let _ = app.emit("show-clipboard-window", ());
    }
}
