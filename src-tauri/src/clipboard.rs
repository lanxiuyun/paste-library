use sha2::{Digest, Sha256};
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::models::{
    AppSettings, ClipboardContentType, ClipboardItem, ClearHistoryRequest, GetHistoryRequest,
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

    pub async fn handle_clipboard_change(&self, text: String, html: Option<String>) -> Result<Option<ClipboardItem>, String> {
        let settings = self.settings.lock().await;

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
        };

        let id = self.database.add_clipboard_item(&item).map_err(|e| {
            if e.to_string().contains("UNIQUE constraint failed") {
                return "DUPLICATE".to_string();
            }
            e.to_string()
        })?;

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
}