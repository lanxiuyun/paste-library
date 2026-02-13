use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// 剪贴板内容类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ClipboardContentType {
    /// 纯文本
    Text,
    /// HTML 格式
    Html,
    /// 富文本 (RTF)
    Rtf,
    /// 图片
    Image,
    /// 单个文件
    File,
    /// 单个文件夹
    Folder,
    /// 多个文件/文件夹
    Files,
}

/// 剪贴板元数据
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ClipboardMetadata {
    // 图片相关
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub format: Option<String>,

    // 文件相关
    pub file_name: Option<String>,
    pub file_size: Option<u64>,
    pub mime_type: Option<String>,

    // 文件夹相关
    pub folder_name: Option<String>,
    pub item_count: Option<u32>,
}

/// 标签定义
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tag {
    pub id: String,
    pub name: String,
    pub color: String,
}

/// 剪贴板历史记录项
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClipboardItem {
    /// 唯一 ID
    pub id: i64,
    /// 内容类型
    pub content_type: ClipboardContentType,
    /// 文本内容 (纯文本或 HTML)
    pub content: String,
    /// 创建时间
    pub created_at: DateTime<Utc>,
    /// 内容哈希 (用于去重)
    pub content_hash: String,
    /// 元数据 (图片尺寸、文件信息等)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<ClipboardMetadata>,
    /// 文件路径列表 (用于文件/文件夹类型)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_paths: Option<Vec<String>>,
    /// 缩略图路径 (用于图片类型)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail_path: Option<String>,
    /// 标签列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
}

/// 创建剪贴板项的请求
#[derive(Debug, Deserialize)]
pub struct CreateClipboardItemRequest {
    pub content_type: ClipboardContentType,
    pub content: String,
}

/// 搜索请求
#[derive(Debug, Deserialize)]
pub struct SearchRequest {
    pub query: String,
    pub limit: Option<i64>,
}

/// 获取历史记录的请求
#[derive(Debug, Deserialize)]
pub struct GetHistoryRequest {
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

/// 清空历史请求 (支持按日期/条数)
#[derive(Debug, Deserialize)]
pub struct ClearHistoryRequest {
    /// 保留的条数 (null 表示不按条数清理)
    pub keep_count: Option<i64>,
    /// 保留的天数 (null 表示不按日期清理)
    pub keep_days: Option<i64>,
}

/// 应用设置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppSettings {
    // 历史记录设置
    /// 最大历史记录条数 (默认 5000)
    pub max_history_count: i64,
    /// 自动清理周期 (天, 0 表示不自动清理)
    pub auto_cleanup_days: i64,

    // 窗口设置
    /// 窗口位置 (remember/center/cursor)
    pub window_position: String,
    /// 窗口位置 X 坐标 (remember 模式使用)
    pub window_pos_x: Option<i32>,
    /// 窗口位置 Y 坐标 (remember 模式使用)
    pub window_pos_y: Option<i32>,

    // 智能激活设置 (新增)
    /// 如果激活时间与上次复制间隔<5秒，自动回到顶部、切换全部、聚焦搜索
    pub smart_activate: bool,

    // 音效设置
    /// 复制音效
    pub copy_sound: bool,

    // 搜索设置
    /// 搜索框位置 (top/bottom)
    pub search_position: String,
    /// 激活窗口时自动聚焦搜索框（与smart_activate独立）
    pub focus_search_on_activate: bool,

    // 内容设置
    /// 单击动作 (copy/paste/none)
    pub click_action: String,
    /// 双击动作 (copy/paste/none)
    pub double_click_action: String,
    /// 粘贴快捷键 (ctrl_v/shift_insert)
    pub paste_shortcut: String,
    /// 复制后隐藏窗口
    pub hide_window_after_copy: bool,
    /// 图片OCR
    pub image_ocr: bool,
    /// 复制为纯文本
    pub copy_as_plain_text: bool,
    /// 粘贴为纯文本
    pub paste_as_plain_text: bool,
    /// 删除确认
    pub confirm_delete: bool,
    /// 自动排序 (复制已存在内容时置顶)
    pub auto_sort: bool,

    // 通用设置
    /// 唤醒快捷键 (默认 "Alt+V")
    pub hotkey: String,
    /// 是否开机自启
    pub auto_start: bool,

    // 快捷键设置
    /// 数字键 1-9 快速粘贴修饰键组合，如 "ctrl", "ctrl+shift", "alt", "none" 等，默认 "ctrl"
    pub number_key_shortcut: String,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            // 历史记录
            max_history_count: 5000,
            auto_cleanup_days: 30,

            // 窗口
            window_position: "remember".to_string(),
            window_pos_x: None,
            window_pos_y: None,

            // 智能激活 (新增)
            smart_activate: true,

            // 音效
            copy_sound: false,

            // 搜索
            search_position: "top".to_string(),
            focus_search_on_activate: false,

            // 内容
            click_action: "copy".to_string(),
            double_click_action: "paste".to_string(),
            paste_shortcut: "ctrl_v".to_string(),
            hide_window_after_copy: false,
            image_ocr: false,
            copy_as_plain_text: false,
            paste_as_plain_text: true,
            confirm_delete: true,
            auto_sort: false,

            // 通用
            hotkey: "Alt+V".to_string(),
            auto_start: false,

            // 快捷键设置
            number_key_shortcut: "ctrl".to_string(),
        }
    }
}
