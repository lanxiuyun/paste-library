use std::sync::Arc;
use tokio::sync::Mutex;
use tauri::Manager;
use tauri::Emitter;

use crate::models::AppSettings;
use crate::storage::Database;

/// 钉住模式状态
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PinMode {
    Standard,
    Pinned,
}

impl PinMode {
    pub fn is_pinned(&self) -> bool {
        matches!(self, PinMode::Pinned)
    }
}

pub struct WindowManager {
    settings: Arc<Mutex<AppSettings>>,
    database: Arc<Database>,
    pin_mode: Arc<Mutex<PinMode>>,
    window_opacity: Arc<Mutex<f32>>,
}

impl WindowManager {
    pub fn new(settings: Arc<Mutex<AppSettings>>, database: Arc<Database>) -> Self {
        Self {
            settings,
            database,
            pin_mode: Arc::new(Mutex::new(PinMode::Standard)),
            window_opacity: Arc::new(Mutex::new(1.0)),
        }
    }

    /// 创建 WindowManager 时共享 pin_mode 和 window_opacity 状态
    pub fn new_with_shared_state(
        settings: Arc<Mutex<AppSettings>>,
        database: Arc<Database>,
        pin_mode: Arc<Mutex<PinMode>>,
        window_opacity: Arc<Mutex<f32>>,
    ) -> Self {
        Self {
            settings,
            database,
            pin_mode,
            window_opacity,
        }
    }

    /// 获取当前钉住模式状态
    pub async fn get_pin_mode(&self) -> PinMode {
        *self.pin_mode.lock().await
    }

    /// 设置钉住模式状态
    pub async fn set_pin_mode(&self, app: &tauri::AppHandle, pinned: bool) -> Result<(), String> {
        let new_mode = if pinned { PinMode::Pinned } else { PinMode::Standard };
        let mut pin_mode = self.pin_mode.lock().await;
        let old_mode = *pin_mode;
        
        if old_mode == new_mode {
            return Ok(());
        }
        
        *pin_mode = new_mode;
        drop(pin_mode);

        if let Some(window) = app.get_webview_window("clipboard") {
            if pinned {
                // 进入钉住模式
                // 1. 设置窗口置顶
                window.set_always_on_top(true).map_err(|e| e.to_string())?;
                // 2. 保存当前窗口位置（确保用户自定义位置被记住）
                self.save_window_position(&window).await?;
                // 3. 重置透明度
                self.set_window_opacity(app, 1.0).await?;
            } else {
                // 退出钉住模式
                // 1. 恢复默认位置
                self.position_window(&window).await?;
                // 2. 重置透明度
                self.set_window_opacity(app, 1.0).await?;
            }
        }

        // 发送钉住模式变化事件
        let _ = app.emit("pin-mode-changed", serde_json::json!({ "pinned": pinned }));

        Ok(())
    }

    /// 切换钉住模式
    pub async fn toggle_pin_mode(&self, app: &tauri::AppHandle) -> Result<bool, String> {
        let current_mode = self.get_pin_mode().await;
        let new_pinned = !current_mode.is_pinned();
        self.set_pin_mode(app, new_pinned).await?;
        Ok(new_pinned)
    }

    /// 设置窗口透明度
    pub async fn set_window_opacity(&self, app: &tauri::AppHandle, opacity: f32) -> Result<(), String> {
        let clamped_opacity = opacity.clamp(0.0, 1.0);
        
        if let Some(window) = app.get_webview_window("clipboard") {
            #[cfg(target_os = "windows")]
            {
                use winapi::um::winuser::{SetLayeredWindowAttributes, GetWindowLongW, SetWindowLongW, GWL_EXSTYLE};
                use winapi::um::winuser::{WS_EX_LAYERED, LWA_ALPHA};
                use winapi::shared::windef::HWND;
                use tauri::Window;

                // 获取窗口句柄
                let hwnd = window.hwnd().map_err(|e| e.to_string())?;
                let hwnd = hwnd.0 as HWND;

                unsafe {
                    // 设置 WS_EX_LAYERED 样式
                    let ex_style = GetWindowLongW(hwnd, GWL_EXSTYLE);
                    SetWindowLongW(hwnd, GWL_EXSTYLE, ex_style | WS_EX_LAYERED as i32);
                    
                    // 设置透明度 (0-255)
                    let alpha = (clamped_opacity * 255.0) as u8;
                    SetLayeredWindowAttributes(hwnd, 0, alpha, LWA_ALPHA);
                }
            }
            
            #[cfg(target_os = "macos")]
            {
                // macOS 使用 set_transparent 或其他方式
                // 这里简化处理
            }
            
            #[cfg(target_os = "linux")]
            {
                // Linux 处理方式
            }
        }

        // 更新状态
        let mut current_opacity = self.window_opacity.lock().await;
        *current_opacity = clamped_opacity;

        // 发送透明度变化事件
        let _ = app.emit("window-opacity-change", serde_json::json!({ "opacity": clamped_opacity }));

        Ok(())
    }

    /// 获取窗口透明度
    pub async fn get_window_opacity(&self) -> f32 {
        *self.window_opacity.lock().await
    }

    /// 钉住模式：保持窗口在用户上次移动后的位置
    /// 钉住模式不强制固定到右上角，而是保持用户自定义位置
    async fn position_window_pinned(&self, window: &tauri::WebviewWindow) -> Result<(), String> {
        // 钉住模式下保持用户自定义位置，使用 remember 逻辑
        // 如果用户没有移动过窗口，则使用当前位置
        let settings = self.settings.lock().await;
        
        if let (Some(x), Some(y)) = (settings.window_pos_x, settings.window_pos_y) {
            // 使用保存的位置，确保窗口在屏幕内
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
            }
        }
        // 如果没有保存的位置，保持当前位置不变

        Ok(())
    }

    pub async fn toggle_clipboard_window(&self, app: &tauri::AppHandle) -> Result<bool, String> {
        let label = "clipboard";

        if let Some(window) = app.get_webview_window(label) {
            let is_visible = window.is_visible().map_err(|e| e.to_string())?;

            if is_visible {
                // 检查是否在钉住模式且处于半透明状态
                let pin_mode = self.get_pin_mode().await;
                let opacity = self.get_window_opacity().await;
                
                if pin_mode.is_pinned() && opacity < 1.0 {
                    // 钉住模式下，半透明时点击快捷键恢复窗口
                    self.set_window_opacity(app, 1.0).await?;
                    window.set_focus().map_err(|e| e.to_string())?;
                    Ok(true)
                } else {
                    window.hide().map_err(|e| e.to_string())?;
                    Ok(false)
                }
            } else {
                // 根据钉住模式决定窗口位置
                let pin_mode = self.get_pin_mode().await;
                if pin_mode.is_pinned() {
                    self.position_window_pinned(&window).await?;
                    window.set_always_on_top(true).map_err(|e| e.to_string())?;
                } else {
                    self.position_window(&window).await?;
                }
                window.show().map_err(|e| e.to_string())?;
                window.set_focus().map_err(|e| e.to_string())?;
                Ok(true)
            }
        } else {
            self.create_clipboard_window(app).await
        }
    }

    pub async fn hide_clipboard_window(&self, app: &tauri::AppHandle) -> Result<(), String> {
        // 钉住模式下不隐藏窗口，而是半透明化
        let pin_mode = self.get_pin_mode().await;
        
        eprintln!("[DEBUG] hide_clipboard_window called, pin_mode: {:?}", pin_mode);
        
        if pin_mode.is_pinned() {
            // 钉住模式：保存位置并半透明化
            eprintln!("[DEBUG] Pinned mode: setting opacity to 0.6");
            if let Some(window) = app.get_webview_window("clipboard") {
                self.save_window_position(&window).await?;
                self.set_window_opacity(app, 0.6).await?;
            }
            // 发送事件通知前端（不重置状态）
            let _ = app.emit("clipboard-window-blur", serde_json::json!({ "reset": false }));
        } else {
            // 默认模式：隐藏前保存窗口位置（如果是 remember 模式）
            eprintln!("[DEBUG] Standard mode: hiding window");
            if let Some(window) = app.get_webview_window("clipboard") {
                self.save_window_position(&window).await?;
                window.hide().map_err(|e| e.to_string())?;
            }
            // 发送事件通知前端（粘贴后重置状态）
            let _ = app.emit("clipboard-window-blur", serde_json::json!({ "reset": true }));
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
    /// 钉住模式下总是保存位置，默认模式下只在 remember 模式下保存
    async fn save_window_position(&self, window: &tauri::WebviewWindow) -> Result<(), String> {
        let pin_mode = *self.pin_mode.lock().await;
        let settings = self.settings.lock().await;
        
        // 钉住模式下总是保存位置；默认模式下只在 remember 模式下保存
        let should_save = pin_mode.is_pinned() || settings.window_position == "remember";
        
        if !should_save {
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
        .resizable(true)
        .skip_taskbar(true)
        .always_on_top(true)
        .accept_first_mouse(true)
        .build()
        .map_err(|e: tauri::Error| e.to_string())?;

        // 根据设置定位窗口
        self.position_window(&window).await?;

        let app_handle = app.clone();
        let settings_clone = self.settings.clone();
        let database_clone = self.database.clone();
        let pin_mode_clone = self.pin_mode.clone();
        let window_opacity_clone = self.window_opacity.clone();

        let app_handle_for_event = app.clone();
        window.on_window_event(move |event| {
            if let tauri::WindowEvent::Focused(false) = event {
                let app = app_handle_for_event.clone();
                let settings = settings_clone.clone();
                let database = database_clone.clone();
                let pin_mode = pin_mode_clone.clone();
                let window_opacity = window_opacity_clone.clone();

                tauri::async_runtime::spawn(async move {
                    // 增加延迟时间，给用户足够的时间完成 hover 和点击操作
                    tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;

                    if let Some(win) = app.get_webview_window("clipboard") {
                        if let Ok(is_focused) = win.is_focused() {
                            if !is_focused {
                                // 使用共享的 pin_mode 状态
                                let manager = WindowManager::new_with_shared_state(settings, database, pin_mode, window_opacity);
                                let current_pin_mode = manager.get_pin_mode().await;
                                
                                eprintln!("[DEBUG] Window blur event, pin_mode: {:?}", current_pin_mode);
                                
                                if current_pin_mode.is_pinned() {
                                    // 钉住模式下：半透明化而不是隐藏
                                    eprintln!("[DEBUG] Pinned mode: setting opacity to 0.6");
                                    let _ = manager.save_window_position(&win).await;
                                    let _ = manager.set_window_opacity(&app, 0.6).await;
                                    let _ = app.emit("clipboard-window-blur", serde_json::json!({ "reset": false }));
                                } else {
                                    // 默认模式下：隐藏窗口，保留搜索词和滚动位置
                                    eprintln!("[DEBUG] Standard mode: hiding window");
                                    let _ = manager.save_window_position(&win).await;
                                    let _ = win.hide();
                                    let _ = app.emit("clipboard-window-blur", serde_json::json!({ "reset": false }));
                                }
                            }
                        }
                    }
                });
            }
        });

        // 单独注册获得焦点事件，用于恢复透明度
        let app_handle_for_focus = app.clone();
        let pin_mode_clone_focus = self.pin_mode.clone();
        let window_opacity_clone_focus = self.window_opacity.clone();
        window.on_window_event(move |event| {
            if let tauri::WindowEvent::Focused(true) = event {
                let app = app_handle_for_focus.clone();
                let pin_mode = pin_mode_clone_focus.clone();
                let window_opacity = window_opacity_clone_focus.clone();
                
                tauri::async_runtime::spawn(async move {
                    // 直接读取状态，不需要创建 manager
                    let current_opacity = *window_opacity.lock().await;
                    let current_pin_mode = *pin_mode.lock().await;
                    
                    eprintln!("[DEBUG] Window focus event, pin_mode: {:?}, opacity: {}", current_pin_mode, current_opacity);
                    
                    if current_pin_mode.is_pinned() && current_opacity < 1.0 {
                        eprintln!("[DEBUG] Pinned mode: restoring opacity to 1.0");
                        // 创建临时 manager 来设置透明度
                        let manager = WindowManager::new_with_shared_state(
                            Arc::new(Mutex::new(AppSettings::default())),
                            Arc::new(Database::new(std::path::PathBuf::from(".")).unwrap()),
                            pin_mode, 
                            window_opacity
                        );
                        let _ = manager.set_window_opacity(&app, 1.0).await;
                    }
                });
            }
        });

        Ok(true)
    }
}
