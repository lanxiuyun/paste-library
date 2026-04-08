//! 平台抽象模块
//! 提供跨平台的窗口功能抽象，包括光标位置获取

/// 获取当前鼠标光标位置
pub fn get_cursor_position() -> Result<(f64, f64), String> {
    #[cfg(target_os = "windows")]
    {
        windows::get_cursor_position()
    }

    #[cfg(target_os = "macos")]
    {
        macos::get_cursor_position()
    }

    #[cfg(target_os = "linux")]
    {
        linux::get_cursor_position()
    }

    #[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
    {
        Ok((400.0, 300.0))
    }
}

#[cfg(target_os = "windows")]
mod windows;

#[cfg(target_os = "macos")]
mod macos;

#[cfg(target_os = "linux")]
mod linux;
