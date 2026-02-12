use sha2::{Digest, Sha256};
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::models::{
    AppSettings, ClipboardContentType, ClipboardItem, ClipboardMetadata, ClearHistoryRequest, GetHistoryRequest,
    SearchRequest,
};
use crate::storage::Database;

pub struct ClipboardManager {
    database: Arc<Database>,
    settings: Arc<Mutex<AppSettings>>,
}

impl ClipboardManager {
    pub fn new(database: Arc<Database>, settings: Arc<Mutex<AppSettings>>) -> Self {
        Self {
            database,
            settings,
        }
    }

    /// 处理剪贴板变化（文本/HTML类型）
    ///
    /// # 参数
    /// - `text`: 纯文本内容
    /// - `html`: HTML内容（可选）
    /// - `is_internal_copy`: 是否是软件内部复制。true表示用户点击项目复制，false表示来自系统剪贴板
    pub async fn handle_clipboard_change(&self, text: String, html: Option<String>, is_internal_copy: bool) -> Result<Option<ClipboardItem>, String> {
        let settings = self.settings.lock().await;
        let auto_sort = settings.auto_sort;

        let count = self.database.get_count().map_err(|e| e.to_string())?;
        if count >= settings.max_history_count {
            let cleanup_request = ClearHistoryRequest {
                keep_count: Some(settings.max_history_count - 1),
                keep_days: None,
            };
            self.database
                .clear_history(&cleanup_request)
                .map_err(|e| e.to_string())?;
        }
        drop(settings);

        let (content_type, content) = if let Some(html_content) = html {
            (ClipboardContentType::Html, html_content)
        } else {
            (ClipboardContentType::Text, text)
        };

        let mut hasher = Sha256::new();
        hasher.update(&content);
        let content_hash = format!("{:x}", hasher.finalize());

        let item = ClipboardItem {
            id: 0,
            content_type,
            content,
            created_at: chrono::Utc::now(),
            content_hash,
            metadata: None,
            file_paths: None,
            thumbnail_path: None,
            tags: None,
        };

        let id = self.database.add_clipboard_item(&item, auto_sort, is_internal_copy).map_err(|e| e.to_string())?;

        let mut item_with_id = item;
        item_with_id.id = id;

        Ok(Some(item_with_id))
    }

    /// 处理剪贴板变化（扩展类型：图片、文件等）
    ///
    /// # 参数
    /// - `content_type`: 内容类型
    /// - `content`: 内容
    /// - `file_paths`: 文件路径列表（可选）
    /// - `thumbnail_path`: 缩略图路径（可选）
    /// - `metadata`: 元数据（可选）
    /// - `is_internal_copy`: 是否是软件内部复制。true表示用户点击项目复制，false表示来自系统剪贴板
    pub async fn handle_clipboard_change_extended(
        &self,
        content_type: ClipboardContentType,
        content: String,
        file_paths: Option<Vec<String>>,
        thumbnail_path: Option<String>,
        metadata: Option<ClipboardMetadata>,
        is_internal_copy: bool,
    ) -> Result<Option<ClipboardItem>, String> {
        let settings = self.settings.lock().await;
        let auto_sort = settings.auto_sort;

        let count = self.database.get_count().map_err(|e| e.to_string())?;
        if count >= settings.max_history_count {
            let cleanup_request = ClearHistoryRequest {
                keep_count: Some(settings.max_history_count - 1),
                keep_days: None,
            };
            self.database
                .clear_history(&cleanup_request)
                .map_err(|e| e.to_string())?;
        }
        drop(settings);

        // 生成内容哈希
        let mut hasher = Sha256::new();
        hasher.update(&content);
        if let Some(ref paths) = file_paths {
            for path in paths {
                hasher.update(path.as_bytes());
            }
        }
        let content_hash = format!("{:x}", hasher.finalize());

        let item = ClipboardItem {
            id: 0,
            content_type,
            content,
            created_at: chrono::Utc::now(),
            content_hash,
            metadata,
            file_paths,
            thumbnail_path,
            tags: None,
        };

        let id = self.database.add_clipboard_item(&item, auto_sort, is_internal_copy).map_err(|e| e.to_string())?;

        let mut item_with_id = item;
        item_with_id.id = id;

        Ok(Some(item_with_id))
    }

    pub fn get_history(&self, request: GetHistoryRequest) -> Result<Vec<ClipboardItem>, String> {
        let limit = request.limit.unwrap_or(100);
        let offset = request.offset.unwrap_or(0);

        self.database
            .get_history(limit, offset)
            .map_err(|e| e.to_string())
    }

    pub fn search_history(&self, request: SearchRequest) -> Result<Vec<ClipboardItem>, String> {
        let limit = request.limit.unwrap_or(100);

        self.database
            .search_history(&request.query, limit)
            .map_err(|e| e.to_string())
    }

    pub fn delete_item(&self, id: i64) -> Result<(), String> {
        self.database.delete_item(id).map_err(|e| e.to_string())
    }

    pub fn clear_history(&self, request: ClearHistoryRequest) -> Result<i64, String> {
        self.database
            .clear_history(&request)
            .map_err(|e| e.to_string())
    }

    pub fn get_settings(&self) -> Result<AppSettings, String> {
        self.database.get_settings().map_err(|e| e.to_string())
    }

    pub async fn save_settings(&self, new_settings: &AppSettings) -> Result<(), String> {
        let mut settings = self.settings.lock().await;
        *settings = new_settings.clone();

        self.database
            .save_settings(new_settings)
            .map_err(|e| e.to_string())
    }

    pub fn update_tags(&self, id: i64, tags: Option<Vec<String>>) -> Result<(), String> {
        self.database
            .update_tags(id, &tags)
            .map_err(|e| e.to_string())
    }

    pub fn get_all_tags(&self) -> Result<Vec<(String, i64)>, String> {
        self.database
            .get_all_tags()
            .map_err(|e| e.to_string())
    }

    pub fn export_data(&self) -> Result<String, String> {
        let items = self.database
            .get_history(10000, 0)
            .map_err(|e| e.to_string())?;
        
        serde_json::to_string(&items)
            .map_err(|e| format!("导出失败: {}", e))
    }

    pub fn import_data(&self, json_data: &str) -> Result<i64, String> {
        let items: Vec<ClipboardItem> = serde_json::from_str(json_data)
            .map_err(|e| format!("导入失败: JSON 格式错误 - {}", e))?;

        let mut count = 0i64;
        for item in items {
            // 导入时：
            // - 启用 auto_sort（true）避免重复项被忽略
            // - 标记为外部复制（false）以便更新时间戳
            if self.database.add_clipboard_item(&item, true, false).is_ok() {
                count += 1;
            }
        }

        Ok(count)
    }
}