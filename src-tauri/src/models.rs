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
    /// 窗口宽度
    pub window_width: f64,
    /// 窗口高度
    pub window_height: f64,
    /// 激活时回到顶部
    pub scroll_to_top_on_activate: bool,
    /// 激活时切换至全部分组
    pub switch_to_all_on_activate: bool,

    // 音效设置
    /// 复制音效
    pub copy_sound: bool,

    // 搜索设置
    /// 搜索框位置 (top/bottom)
    pub search_position: String,
    /// 默认聚焦搜索框
    pub auto_focus_search: bool,
    /// 激活时清除搜索框
    pub clear_search_on_activate: bool,

    // 内容设置
    /// 自动粘贴模式 (off/single/double)
    pub auto_paste: String,
    /// 图片OCR
    pub image_ocr: bool,
    /// 复制为纯文本
    pub copy_as_plain_text: bool,
    /// 粘贴为纯文本
    pub paste_as_plain_text: bool,
    /// 自动收藏
    pub auto_favorite: bool,
    /// 删除确认
    pub confirm_delete: bool,
    /// 自动排序 (复制已存在内容时置顶)
    pub auto_sort: bool,

    // 通用设置
    /// 唤醒快捷键 (默认 "Alt+V")
    pub hotkey: String,
    /// 是否开机自启
    pub auto_start: bool,
    /// 黑名单应用列表 (来源应用名)
    pub blacklist_apps: Vec<String>,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            // 历史记录
            max_history_count: 5000,
            auto_cleanup_days: 30,

            // 窗口
            window_position: "remember".to_string(),
            window_width: 800.0,
            window_height: 600.0,
            scroll_to_top_on_activate: false,
            switch_to_all_on_activate: true,

            // 音效
            copy_sound: false,

            // 搜索
            search_position: "bottom".to_string(),
            auto_focus_search: true,
            clear_search_on_activate: false,

            // 内容
            auto_paste: "double".to_string(),
            image_ocr: false,
            copy_as_plain_text: false,
            paste_as_plain_text: true,
            auto_favorite: false,
            confirm_delete: true,
            auto_sort: false,

            // 通用
            hotkey: "Alt+V".to_string(),
            auto_start: false,
            blacklist_apps: vec![],
        }
    }
}
