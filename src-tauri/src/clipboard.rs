use crate::models::{
    AppSettings, ClipboardContentType, ClipboardItem, ClipboardMetadata, SearchRequest, GetHistoryRequest,
};
use crate::storage::Database;
use sha2::{Sha256, Digest};
use std::sync::Arc;
use tokio::sync::Mutex;

/// Clipboard manager
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

    /// Handle clipboard change (text/HTML)
    pub async fn handle_clipboard_change(
        &self,
        text: String,
        html: Option<String>,
        _is_internal: bool,
    ) -> Result<Option<ClipboardItem>, String> {
        // Determine content type
        let content_type = if html.is_some() {
            ClipboardContentType::Html
        } else {
            ClipboardContentType::Text
        };

        // Calculate hash for deduplication
        let content_hash = Self::calculate_hash(&text);

        // For now, store as plain text (HTML stored in content field)
        let content = text.clone();

        let item = self.database
            .add_item(
                content_type,
                content,
                content_hash,
                None,
                None,
                None,
            )
            .map_err(|e| e.to_string())?;

        Ok(item)
    }

    /// Handle clipboard change (extended - for images, files)
    pub async fn handle_clipboard_change_extended(
        &self,
        content_type: ClipboardContentType,
        content: String,
        file_paths: Option<Vec<String>>,
        thumbnail_path: Option<String>,
        metadata: Option<ClipboardMetadata>,
        _is_internal: bool,
    ) -> Result<Option<ClipboardItem>, String> {
        // Calculate hash for deduplication
        let content_hash = Self::calculate_hash(&content);

        let item = self.database
            .add_item(
                content_type,
                content,
                content_hash,
                metadata,
                file_paths,
                thumbnail_path,
            )
            .map_err(|e| e.to_string())?;

        Ok(item)
    }

    /// Get clipboard history
    pub fn get_history(&self, request: GetHistoryRequest) -> Result<Vec<ClipboardItem>, String> {
        let limit = request.limit.unwrap_or(100);
        let offset = request.offset.unwrap_or(0);

        self.database
            .get_history(limit, offset)
            .map_err(|e| e.to_string())
    }

    /// Search clipboard history
    pub fn search_history(&self, request: SearchRequest) -> Result<Vec<ClipboardItem>, String> {
        let limit = request.limit.unwrap_or(100);

        self.database
            .search(&request.query, limit)
            .map_err(|e| e.to_string())
    }

    /// Delete item
    pub fn delete_item(&self, id: i64) -> Result<(), String> {
        self.database
            .delete_item(id)
            .map_err(|e| e.to_string())
    }

    /// Clear history
    pub fn clear_history(&self, keep_count: Option<i64>, keep_days: Option<i64>) -> Result<i64, String> {
        self.database
            .clear_history(keep_count, keep_days)
            .map_err(|e| e.to_string())
    }

    /// Get settings
    pub fn get_settings(&self) -> Result<AppSettings, String> {
        self.database
            .get_settings()
            .map_err(|e| e.to_string())
    }

    /// Save settings
    pub fn save_settings(&self, settings: &AppSettings) -> Result<(), String> {
        self.database
            .save_settings(settings)
            .map_err(|e| e.to_string())
    }

    /// Calculate SHA256 hash
    fn calculate_hash(content: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(content.as_bytes());
        format!("{:x}", hasher.finalize())
    }
}
