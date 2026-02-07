use std::sync::Arc;
use tokio::sync::Mutex;
use tauri::Manager;
use tauri::Emitter;

use crate::models::AppSettings;

pub struct WindowManager {
    settings: Arc<Mutex<AppSettings>>,
}

impl WindowManager {
    pub fn new(settings: Arc<Mutex<AppSettings>>) -> Self {
        Self { settings }
    }

    pub async fn toggle_clipboard_window(&self, app: &tauri::AppHandle) -> Result<bool, String> {
        let label = "clipboard";

        if let Some(window) = app.get_webview_window(label) {
            let is_visible = window.is_visible().map_err(|e| e.to_string())?;

            if is_visible {
                window.hide().map_err(|e| e.to_string())?;
                Ok(false)
            } else {
                window.show().map_err(|e| e.to_string())?;
                window.set_focus().map_err(|e| e.to_string())?;
                Ok(true)
            }
        } else {
            self.create_clipboard_window(app).await
        }
    }

    pub async fn hide_clipboard_window(&self, app: &tauri::AppHandle) -> Result<(), String> {
        if let Some(window) = app.get_webview_window("clipboard") {
            window.hide().map_err(|e| e.to_string())?;
        }
        Ok(())
    }

    pub async fn show_clipboard_window(&self, app: &tauri::AppHandle) -> Result<(), String> {
        let label = "clipboard";

        if let Some(window) = app.get_webview_window(label) {
            window.show().map_err(|e| e.to_string())?;
            window.set_focus().map_err(|e| e.to_string())?;
        } else {
            self.create_clipboard_window(app).await?;
        }

        Ok(())
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
        .center()
        .resizable(true)
        .skip_taskbar(true)
        .always_on_top(true)
        .build()
        .map_err(|e| e.to_string())?;

        let app_handle = app.clone();

        window.on_window_event(move |event| {
            if let tauri::WindowEvent::Focused(false) = event {
                let app = app_handle.clone();

                tauri::async_runtime::spawn(async move {
                    tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;

                    if let Some(win) = app.get_webview_window("clipboard") {
                        if let Ok(is_focused) = win.is_focused() {
                            if !is_focused {
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
