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
    /// 最大历史记录条数 (默认 5000)
    pub max_history_count: i64,
    /// 唤醒快捷键 (默认 "Alt+V")
    pub hotkey: String,
    /// 是否开机自启
    pub auto_start: bool,
    /// 自动清理周期 (天, 0 表示不自动清理)
    pub auto_cleanup_days: i64,
    /// 窗口宽度
    pub window_width: f64,
    /// 窗口高度
    pub window_height: f64,
    /// 黑名单应用列表 (来源应用名)
    pub blacklist_apps: Vec<String>,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            max_history_count: 5000,
            hotkey: "Alt+V".to_string(),
            auto_start: false,
            auto_cleanup_days: 30,
            window_width: 800.0,
            window_height: 600.0,
            blacklist_apps: vec![],
        }
    }
}
