use std::sync::Arc;
use tokio::sync::Mutex;
use tauri::Manager;
use tauri::Emitter;

use crate::models::AppSettings;
use crate::storage::Database;

pub struct WindowManager {
    settings: Arc<Mutex<AppSettings>>,
    database: Arc<Database>,
}

impl WindowManager {
    pub fn new(settings: Arc<Mutex<AppSettings>>, database: Arc<Database>) -> Self {
        Self { settings, database }
    }

    pub async fn toggle_clipboard_window(&self, app: &tauri::AppHandle) -> Result<bool, String> {
        let label = "clipboard";

        if let Some(window) = app.get_webview_window(label) {
            let is_visible = window.is_visible().map_err(|e| e.to_string())?;

            if is_visible {
                window.hide().map_err(|e| e.to_string())?;
                Ok(false)
            } else {
                // 根据设置重新定位窗口
                self.position_window(&window).await?;
                window.show().map_err(|e| e.to_string())?;
                window.set_focus().map_err(|e| e.to_string())?;
                Ok(true)
            }
        } else {
            self.create_clipboard_window(app).await
        }
    }

    pub async fn hide_clipboard_window(&self, app: &tauri::AppHandle) -> Result<(), String> {
        // 隐藏前保存窗口位置（如果是 remember 模式）
        if let Some(window) = app.get_webview_window("clipboard") {
            self.save_window_position(&window).await?;
            window.hide().map_err(|e| e.to_string())?;
        }
        Ok(())
    }

    pub async fn show_clipboard_window(&self, app: &tauri::AppHandle) -> Result<(), String> {
        let label = "clipboard";

        if let Some(window) = app.get_webview_window(label) {
            // 根据设置重新定位窗口
            self.position_window(&window).await?;
            window.show().map_err(|e| e.to_string())?;
            window.set_focus().map_err(|e| e.to_string())?;
        } else {
            self.create_clipboard_window(app).await?;
        }

        Ok(())
    }

    /// 重新定位窗口（用于设置改变时）
    pub async fn reposition_window(&self, window: &tauri::WebviewWindow) -> Result<(), String> {
        self.position_window(window).await
    }

    /// 根据设置定位窗口
    async fn position_window(&self, window: &tauri::WebviewWindow) -> Result<(), String> {
        let settings = self.settings.lock().await;
        let position_mode = &settings.window_position;

        match position_mode.as_str() {
            "remember" => {
                // 使用保存的位置
                if let (Some(x), Some(y)) = (settings.window_pos_x, settings.window_pos_y) {
                    // 确保窗口不超出屏幕边界
                    let size = window.outer_size().map_err(|e| e.to_string())?;
                    let screen = window.primary_monitor().map_err(|e| e.to_string())?;
                    if let Some(screen) = screen {
                        let screen_width = screen.size().width as i32;
                        let screen_height = screen.size().height as i32;
                        let win_width = size.width as i32;
                        let win_height = size.height as i32;

                        let mut pos_x = x;
                        let mut pos_y = y;

                        // 边界检查：确保窗口至少有一部分在屏幕内
                        if pos_x + win_width < 100 {
                            pos_x = 100 - win_width;
                        }
                        if pos_x > screen_width - 100 {
                            pos_x = screen_width - 100;
                        }
                        if pos_y + win_height < 100 {
                            pos_y = 100 - win_height;
                        }
                        if pos_y > screen_height - 100 {
                            pos_y = screen_height - 100;
                        }

                        window.set_position(tauri::Position::Physical(tauri::PhysicalPosition::new(
                            pos_x, pos_y,
                        ))).map_err(|e| e.to_string())?;
                    } else {
                        window.center().map_err(|e| e.to_string())?;
                    }
                } else {
                    // 没有保存的位置，默认居中
                    window.center().map_err(|e| e.to_string())?;
                }
            }
            "center" => {
                // 屏幕居中
                window.center().map_err(|e| e.to_string())?;
            }
            "cursor" => {
                // 跟随鼠标位置
                if let Ok((x, y)) = Self::get_cursor_position() {
                    let size = window.inner_size().map_err(|e| e.to_string())?;
                    // 确保窗口不超出屏幕边界
                    let screen = window.primary_monitor().map_err(|e| e.to_string())?;
                    if let Some(screen) = screen {
                        let screen_width = screen.size().width as f64;
                        let screen_height = screen.size().height as f64;
                        let win_width = size.width as f64;
                        let win_height = size.height as f64;
                        
                        let mut pos_x = x;
                        let mut pos_y = y;
                        
                        // 边界检查
                        if pos_x + win_width > screen_width {
                            pos_x = screen_width - win_width;
                        }
                        if pos_y + win_height > screen_height {
                            pos_y = screen_height - win_height;
                        }
                        
                        window.set_position(tauri::Position::Physical(tauri::PhysicalPosition::new(
                            pos_x as i32,
                            pos_y as i32,
                        ))).map_err(|e| e.to_string())?;
                    }
                } else {
                    // 如果无法获取鼠标位置，则居中
                    window.center().map_err(|e| e.to_string())?;
                }
            }
            _ => {
                // 默认居中
                window.center().map_err(|e| e.to_string())?;
            }
        }

        Ok(())
    }

    /// 保存窗口位置到设置
    async fn save_window_position(&self, window: &tauri::WebviewWindow) -> Result<(), String> {
        // 只在 remember 模式下保存位置
        let settings = self.settings.lock().await;
        if settings.window_position != "remember" {
            return Ok(());
        }
        drop(settings);

        // 获取窗口当前位置
        let position = window.outer_position().map_err(|e| e.to_string())?;
        let x = position.x;
        let y = position.y;

        // 更新设置
        let mut settings = self.settings.lock().await;
        settings.window_pos_x = Some(x);
        settings.window_pos_y = Some(y);

        // 保存到数据库
        if let Err(e) = self.database.save_settings(&settings) {
            eprintln!("保存窗口位置失败: {}", e);
        }

        Ok(())
    }

    /// 获取鼠标位置
    fn get_cursor_position() -> Result<(f64, f64), String> {
        #[cfg(target_os = "windows")]
        {
            use winapi::um::winuser::GetCursorPos;
            use winapi::shared::windef::POINT;
            
            unsafe {
                let mut point = POINT { x: 0, y: 0 };
                if GetCursorPos(&mut point) != 0 {
                    return Ok((point.x as f64, point.y as f64));
                } else {
                    return Err("无法获取鼠标位置".to_string());
                }
            }
        }
        
        #[cfg(target_os = "macos")]
        {
            // macOS: 使用 NSEvent 获取鼠标位置
            // 注意：这里需要链接 Cocoa 框架，简化实现返回屏幕中心
            return Ok((400.0, 300.0));
        }
        
        #[cfg(target_os = "linux")]
        {
            // Linux: 简化实现，可以尝试读取 X11 或使用 xdotool
            // 实际实现需要链接 X11 库
            return Ok((400.0, 300.0));
        }
        
        #[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
        {
            return Ok((400.0, 300.0));
        }
    }

    async fn create_clipboard_window(&self, app: &tauri::AppHandle) -> Result<bool, String> {
        let label = "clipboard";
        
        // 使用固定尺寸（已从设置中移除窗口尺寸配置）
        let width = 800.0;
        let height = 600.0;

        let window = tauri::webview::WebviewWindowBuilder::new(
            app,
            label,
            tauri::WebviewUrl::App("/clipboard".into())
        )
        .title("剪贴板历史")
        .inner_size(width, height)
        .decorations(false)
        .transparent(false)
        .resizable(true)
        .skip_taskbar(true)
        .always_on_top(true)
        .accept_first_mouse(true)
        .build()
        .map_err(|e| e.to_string())?;

        // 根据设置定位窗口
        self.position_window(&window).await?;

        let app_handle = app.clone();
        let settings_clone = self.settings.clone();
        let database_clone = self.database.clone();

        window.on_window_event(move |event| {
            if let tauri::WindowEvent::Focused(false) = event {
                let app = app_handle.clone();
                let settings = settings_clone.clone();
                let database = database_clone.clone();

                tauri::async_runtime::spawn(async move {
                    // 增加延迟时间，给用户足够的时间完成 hover 和点击操作
                    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

                    if let Some(win) = app.get_webview_window("clipboard") {
                        if let Ok(is_focused) = win.is_focused() {
                            if !is_focused {
                                // 隐藏前保存位置
                                let manager = WindowManager::new(settings, database);
                                let _ = manager.save_window_position(&win).await;
                                let _ = win.hide();
                                let _ = app.emit("clipboard-window-blur", ());
                            }
                        }
                    }
                });
            }
        });

        Ok(true)
    }
}
