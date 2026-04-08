//! Windows 平台特定实现

use winapi::um::winuser::GetCursorPos;
use winapi::shared::windef::POINT;

/// 获取鼠标光标位置（Windows 实现）
pub fn get_cursor_position() -> Result<(f64, f64), String> {
    unsafe {
        let mut point = POINT { x: 0, y: 0 };
        if GetCursorPos(&mut point) != 0 {
            Ok((point.x as f64, point.y as f64))
        } else {
            Err("无法获取鼠标位置".to_string())
        }
    }
}
