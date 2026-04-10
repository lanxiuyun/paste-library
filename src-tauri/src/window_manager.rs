use std::sync::Arc;
use tokio::sync::Mutex;
use tauri::Emitter;
use tauri::Manager;

use crate::models::AppSettings;
use crate::platform;
use crate::storage::Database;

/// 窗口配置常量
const WINDOW_WIDTH: f64 = 800.0;
const WINDOW_HEIGHT: f64 = 600.0;
// 保留一个很短的缓冲，避免瞬时焦点抖动导致误隐藏，同时让失焦隐藏更跟手。
const BLUR_DELAY_MS: u64 = 20;
const FOCUS_DELAY_MS: u64 = 50;
const WINDOW_MARGIN: i32 = 100;

/// 将 Tauri 错误转换为 String 的宏
macro_rules! map_err {
    ($e:expr) => {
        $e.map_err(|e| e.to_string())
    };
}

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

/// 计算窗口在屏幕内的有效位置
/// 确保窗口至少有一部分在屏幕内（考虑边距）
fn clamp_window_position(
    x: i32,
    y: i32,
    win_width: i32,
    win_height: i32,
    screen_width: i32,
    screen_height: i32,
    margin: i32,
) -> (i32, i32) {
    let mut pos_x = x;
    let mut pos_y = y;

    // 边界检查：确保窗口至少有一部分在屏幕内
    if pos_x + win_width < margin {
        pos_x = margin - win_width;
    }
    if pos_x > screen_width - margin {
        pos_x = screen_width - margin;
    }
    if pos_y + win_height < margin {
        pos_y = margin - win_height;
    }
    if pos_y > screen_height - margin {
        pos_y = screen_height - margin;
    }

    (pos_x, pos_y)
}

pub struct WindowManager {
    settings: Arc<Mutex<AppSettings>>,
    database: Arc<Database>,
    pin_mode: Arc<Mutex<PinMode>>,
    /// 标志位：是否有待执行的隐藏操作（用于取消点击窗口内部时的意外隐藏）
    pending_hide: Arc<Mutex<bool>>,
}

/// 状态引用集合（用于事件处理回调）
#[derive(Clone)]
struct StateRefs {
    settings: Arc<Mutex<AppSettings>>,
    database: Arc<Database>,
    pin_mode: Arc<Mutex<PinMode>>,
    pending_hide: Arc<Mutex<bool>>,
}

impl WindowManager {
    pub fn new(settings: Arc<Mutex<AppSettings>>, database: Arc<Database>) -> Self {
        Self {
            settings,
            database,
            pin_mode: Arc::new(Mutex::new(PinMode::Standard)),
            pending_hide: Arc::new(Mutex::new(false)),
        }
    }

    /// 获取状态引用（用于事件处理）
    fn get_state_refs(&self) -> StateRefs {
        StateRefs {
            settings: self.settings.clone(),
            database: self.database.clone(),
            pin_mode: self.pin_mode.clone(),
            pending_hide: self.pending_hide.clone(),
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

        if *pin_mode == new_mode {
            return Ok(());
        }

        *pin_mode = new_mode;
        drop(pin_mode);

        if pinned {
            *self.pending_hide.lock().await = false;
        }

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

    pub async fn toggle_clipboard_window(&self, app: &tauri::AppHandle) -> Result<bool, String> {
        let label = "clipboard";

        if let Some(window) = app.get_webview_window(label) {
            let is_visible = map_err!(window.is_visible())?;

            if is_visible {
                *self.pending_hide.lock().await = false;
                let _ = self.save_window_position(&window).await;
                map_err!(window.hide())?;
                Ok(false)
            } else {
                self.position_window(&window).await?;
                map_err!(window.show())?;
                map_err!(window.set_focus())?;
                Ok(true)
            }
        } else {
            self.create_clipboard_window(app).await
        }
    }

    pub async fn hide_clipboard_window(&self, app: &tauri::AppHandle) -> Result<(), String> {
        *self.pending_hide.lock().await = false;

        if let Some(window) = app.get_webview_window("clipboard") {
            self.save_window_position(&window).await?;
            map_err!(window.hide())?;
        }

        Self::emit_window_blur(app);
        Ok(())
    }

    pub async fn show_clipboard_window(&self, app: &tauri::AppHandle) -> Result<(), String> {
        if let Some(window) = app.get_webview_window("clipboard") {
            self.position_window(&window).await?;
            map_err!(window.show())?;
            map_err!(window.set_focus())?;
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

        match settings.window_position.as_str() {
            "remember" => {
                if let (Some(x), Some(y)) = (settings.window_pos_x, settings.window_pos_y) {
                    let size = map_err!(window.outer_size())?;
                    let screen = map_err!(window.primary_monitor())?;

                    if let Some(screen) = screen {
                        let (pos_x, pos_y) = clamp_window_position(
                            x, y,
                            size.width as i32, size.height as i32,
                            screen.size().width as i32, screen.size().height as i32,
                            WINDOW_MARGIN,
                        );

                        map_err!(window.set_position(tauri::Position::Physical(
                            tauri::PhysicalPosition::new(pos_x, pos_y)
                        )))?;
                    } else {
                        map_err!(window.center())?;
                    }
                } else {
                    map_err!(window.center())?;
                }
            }
            "center" => {
                map_err!(window.center())?;
            }
            "cursor" => {
                if let Ok((x, y)) = Self::get_cursor_position() {
                    let size = map_err!(window.inner_size())?;
                    let screen = map_err!(window.primary_monitor())?;

                    if let Some(screen) = screen {
                        let screen_width = screen.size().width as f64;
                        let screen_height = screen.size().height as f64;
                        let win_width = size.width as f64;
                        let win_height = size.height as f64;

                        let mut pos_x = x;
                        let mut pos_y = y;

                        if pos_x + win_width > screen_width {
                            pos_x = screen_width - win_width;
                        }
                        if pos_y + win_height > screen_height {
                            pos_y = screen_height - win_height;
                        }

                        map_err!(window.set_position(tauri::Position::Physical(
                            tauri::PhysicalPosition::new(pos_x as i32, pos_y as i32)
                        )))?;
                    }
                } else {
                    map_err!(window.center())?;
                }
            }
            _ => {
                map_err!(window.center())?;
            }
        }

        Ok(())
    }

    /// 保存窗口位置到设置
    async fn save_window_position(&self, window: &tauri::WebviewWindow) -> Result<(), String> {
        Self::save_window_position_internal(&self.get_state_refs(), window).await
    }

    /// 获取鼠标位置
    fn get_cursor_position() -> Result<(f64, f64), String> {
        platform::get_cursor_position()
    }

    async fn create_clipboard_window(&self, app: &tauri::AppHandle) -> Result<bool, String> {
        let window = self.build_clipboard_window(app).await?;
        self.setup_window_events(&window, app).await;

        // 显示窗口并设置焦点
        map_err!(window.show())?;
        map_err!(window.set_focus())?;

        Ok(true)
    }

    /// 构建剪贴板窗口
    async fn build_clipboard_window(&self, app: &tauri::AppHandle) -> Result<tauri::WebviewWindow, String> {
        let window = tauri::webview::WebviewWindowBuilder::new(
            app,
            "clipboard",
            tauri::WebviewUrl::App("/clipboard".into())
        )
        .title("剪贴板历史")
        .inner_size(WINDOW_WIDTH, WINDOW_HEIGHT)
        .decorations(false)
        .resizable(true)
        .skip_taskbar(true)
        .always_on_top(true)
        .accept_first_mouse(true)
        .build()
        .map_err(|e| e.to_string())?;

        // 根据设置定位窗口
        self.position_window(&window).await?;

        Ok(window)
    }

    /// 设置窗口事件处理
    async fn setup_window_events(&self, window: &tauri::WebviewWindow, app: &tauri::AppHandle) {
        let state_refs = self.get_state_refs();
        let app_handle = app.clone();
        window.on_window_event(move |event| {
            match event {
                tauri::WindowEvent::Focused(false) => {
                    Self::handle_blur_event(&app_handle, state_refs.clone());
                }
                tauri::WindowEvent::Focused(true) => {
                    Self::handle_focus_event(&app_handle, state_refs.clone());
                }
                tauri::WindowEvent::Moved(_) => {
                    Self::handle_moved_event(state_refs.clone());
                }
                _ => {}
            }
        });
    }

    /// 处理窗口失焦事件
    fn handle_blur_event(app: &tauri::AppHandle, state: StateRefs) {
        let app = app.clone();
        tauri::async_runtime::spawn(async move {
            *state.pending_hide.lock().await = true;
            tokio::time::sleep(tokio::time::Duration::from_millis(BLUR_DELAY_MS)).await;

            let Some(window) = Self::resolve_blur_hide_window(&app, &state).await else {
                return;
            };

            let _ = Self::hide_window_after_blur(&state, &window).await;
            Self::emit_window_blur(&app);
        });
    }

    /// 处理窗口获得焦点事件
    fn handle_focus_event(app: &tauri::AppHandle, state: StateRefs) {
        let app = app.clone();
        tauri::async_runtime::spawn(async move {
            tokio::time::sleep(tokio::time::Duration::from_millis(FOCUS_DELAY_MS)).await;

            let Some(win) = app.get_webview_window("clipboard") else { return };
            let Ok(true) = win.is_focused() else {
                return;
            };

            *state.pending_hide.lock().await = false;
        });
    }

    /// 处理窗口移动事件
    fn handle_moved_event(state: StateRefs) {
        tauri::async_runtime::spawn(async move {
            let mut pending_hide = state.pending_hide.lock().await;
            if *pending_hide {
                *pending_hide = false;
            }
        });
    }

    async fn resolve_blur_hide_window(
        app: &tauri::AppHandle,
        state: &StateRefs,
    ) -> Option<tauri::WebviewWindow> {
        if !*state.pending_hide.lock().await {
            return None;
        }

        let window = app.get_webview_window("clipboard")?;
        let Ok(false) = window.is_focused() else {
            return None;
        };

        if state.pin_mode.lock().await.is_pinned() {
            return None;
        }

        Some(window)
    }

    async fn hide_window_after_blur(
        state: &StateRefs,
        window: &tauri::WebviewWindow,
    ) -> Result<(), String> {
        *state.pending_hide.lock().await = false;
        Self::save_window_position_internal(state, window).await?;
        map_err!(window.hide())?;
        Ok(())
    }

    fn emit_window_blur(app: &tauri::AppHandle) {
        let _ = app.emit("clipboard-window-blur", ());
    }

    /// 保存窗口位置（内部实现，使用 StateRefs）
    async fn save_window_position_internal(state: &StateRefs, window: &tauri::WebviewWindow) -> Result<(), String> {
        let settings = state.settings.lock().await;

        if settings.window_position != "remember" {
            return Ok(());
        }
        drop(settings);

        let position = map_err!(window.outer_position())?;
        let mut settings = state.settings.lock().await;
        settings.window_pos_x = Some(position.x);
        settings.window_pos_y = Some(position.y);

        if let Err(e) = state.database.save_settings(&settings) {
            eprintln!("保存窗口位置失败: {}", e);
        }

        Ok(())
    }
}
