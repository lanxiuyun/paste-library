use rusqlite::{params, Connection, OptionalExtension, Result};
use std::path::PathBuf;
use std::sync::Mutex;

use crate::models::{
    AdvancedSearchRequest, AppSettings, ClearHistoryRequest, ClipboardContentType, ClipboardItem,
    ClipboardMetadata,
};

/// 数据库管理器
pub struct Database {
    conn: Mutex<Connection>,
}

impl Database {
    /// 初始化数据库连接
    pub fn new(app_dir: PathBuf) -> Result<Self> {
        let db_path = app_dir.join("clipboard.db");
        let conn = Connection::open(&db_path)?;

        let db = Self {
            conn: Mutex::new(conn),
        };

        db.init_tables()?;
        Ok(db)
    }

    /// 创建必要的表
    fn init_tables(&self) -> Result<()> {
        let conn = self.conn.lock().unwrap();

        // 剪贴板历史表
        conn.execute(
            "CREATE TABLE IF NOT EXISTS clipboard_history (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                content_type TEXT NOT NULL,
                content TEXT NOT NULL,
                created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
                content_hash TEXT NOT NULL UNIQUE,
                text_content TEXT,
                metadata TEXT,
                file_paths TEXT,
                thumbnail_path TEXT,
                tags TEXT
            )",
            [],
        )?;

        // 数据库迁移：为旧表添加新字段
        let columns: Vec<String> = conn
            .prepare("PRAGMA table_info(clipboard_history)")?
            .query_map([], |row| row.get::<_, String>(1))?
            .filter_map(|r| r.ok())
            .collect();

        if !columns.contains(&"metadata".to_string()) {
            conn.execute("ALTER TABLE clipboard_history ADD COLUMN metadata TEXT", [])?;
        }
        if !columns.contains(&"file_paths".to_string()) {
            conn.execute(
                "ALTER TABLE clipboard_history ADD COLUMN file_paths TEXT",
                [],
            )?;
        }
        if !columns.contains(&"thumbnail_path".to_string()) {
            conn.execute(
                "ALTER TABLE clipboard_history ADD COLUMN thumbnail_path TEXT",
                [],
            )?;
        }
        if !columns.contains(&"tags".to_string()) {
            conn.execute("ALTER TABLE clipboard_history ADD COLUMN tags TEXT", [])?;
        }
        if !columns.contains(&"text_content".to_string()) {
            conn.execute(
                "ALTER TABLE clipboard_history ADD COLUMN text_content TEXT",
                [],
            )?;
        }

        // 数据迁移：将 is_favorite 转换为标签（如果存在旧字段）
        if columns.contains(&"is_favorite".to_string()) {
            conn.execute(
                "UPDATE clipboard_history SET tags = '[\"收藏\"]' WHERE is_favorite = 1 AND tags IS NULL",
                [],
            )?;
        }

        // 设置表
        conn.execute(
            "CREATE TABLE IF NOT EXISTS settings (
                key TEXT PRIMARY KEY,
                value TEXT NOT NULL
            )",
            [],
        )?;

        // 初始化默认设置
        let defaults = vec![
            ("max_history_count", "5000"),
            ("auto_cleanup_days", "30"),
            ("window_position", "remember"),
            ("window_pos_x", ""),
            ("window_pos_y", ""),
            ("copy_sound", "false"),
            ("search_position", "bottom"),
            ("focus_search_on_activate", "false"),
            ("click_action", "copy"),
            ("double_click_action", "paste"),
            ("paste_shortcut", "ctrl_v"),
            ("hide_window_after_copy", "false"),
            ("image_ocr", "false"),
            ("copy_as_plain_text", "false"),
            ("paste_as_plain_text", "true"),
            ("confirm_delete", "true"),
            ("auto_sort", "false"),
            ("hotkey", "Alt+V"),
            ("auto_start", "false"),
            ("number_key_shortcut", "ctrl"),
            ("pin_shortcut", "Ctrl+Shift+P"),
            ("app_initialized", "false"),
        ];

        for (key, value) in defaults {
            conn.execute(
                "INSERT OR IGNORE INTO settings (key, value) VALUES (?1, ?2)",
                params![key, value],
            )?;
        }

        Ok(())
    }

    /// 检查是否是首次运行
    pub fn is_first_run(&self) -> Result<bool> {
        let conn = self.conn.lock().unwrap();
        let result: Option<String> = conn
            .query_row(
                "SELECT value FROM settings WHERE key = 'app_initialized'",
                [],
                |row| row.get(0),
            )
            .optional()?;

        match result {
            Some(value) => Ok(value != "true"),
            None => Ok(true),
        }
    }

    /// 标记应用已初始化（非首次运行）
    pub fn mark_app_initialized(&self) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT OR REPLACE INTO settings (key, value) VALUES ('app_initialized', 'true')",
            [],
        )?;
        Ok(())
    }

    /// 添加剪贴板记录
    pub fn add_clipboard_item(
        &self,
        item: &ClipboardItem,
        auto_sort: bool,
        is_internal_copy: bool,
    ) -> Result<i64> {
        let conn = self.conn.lock().unwrap();

        let metadata_json = item
            .metadata
            .as_ref()
            .map(|m| serde_json::to_string(m).ok())
            .flatten();
        let file_paths_json = item
            .file_paths
            .as_ref()
            .map(|p| serde_json::to_string(p).ok())
            .flatten();
        let tags_json = item
            .tags
            .as_ref()
            .map(|t| serde_json::to_string(t).ok())
            .flatten();

        let should_update_timestamp = auto_sort || !is_internal_copy;

        let conflict_sql = if should_update_timestamp {
            "ON CONFLICT(content_hash) DO UPDATE SET
                created_at = excluded.created_at"
        } else {
            "ON CONFLICT(content_hash) DO NOTHING"
        };

        conn.execute(
            &format!("INSERT INTO clipboard_history (content_type, content, created_at, content_hash, text_content, metadata, file_paths, thumbnail_path, tags)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)
             {}", conflict_sql),
            params![
                match item.content_type {
                    ClipboardContentType::Text => "text",
                    ClipboardContentType::Html => "html",
                    ClipboardContentType::Rtf => "rtf",
                    ClipboardContentType::Image => "image",
                    ClipboardContentType::File => "file",
                    ClipboardContentType::Folder => "folder",
                    ClipboardContentType::Files => "files",
                },
                item.content,
                item.created_at.to_rfc3339(),
                item.content_hash,
                item.text_content,
                metadata_json,
                file_paths_json,
                item.thumbnail_path,
                tags_json,
            ],
        )?;

        let id: i64 = conn.query_row(
            "SELECT id FROM clipboard_history WHERE content_hash = ?1",
            params![item.content_hash],
            |row| row.get(0),
        )?;

        Ok(id)
    }

    /// 获取历史记录
    pub fn get_history(&self, limit: i64, offset: i64) -> Result<Vec<ClipboardItem>> {
        let conn = self.conn.lock().unwrap();

        let mut stmt = conn.prepare(
            "SELECT id, content_type, content, created_at, content_hash, text_content, metadata, file_paths, thumbnail_path, tags
             FROM clipboard_history
             ORDER BY created_at DESC
             LIMIT ?1 OFFSET ?2",
        )?;

        let items = stmt
            .query_map(params![limit, offset], |row| {
                let content_type_str: String = row.get(1)?;
                let content_type = match content_type_str.as_str() {
                    "text" => ClipboardContentType::Text,
                    "html" => ClipboardContentType::Html,
                    "rtf" => ClipboardContentType::Rtf,
                    "image" => ClipboardContentType::Image,
                    "file" => ClipboardContentType::File,
                    "folder" => ClipboardContentType::Folder,
                    "files" => ClipboardContentType::Files,
                    _ => ClipboardContentType::Text,
                };

                let created_at_str: String = row.get(3)?;
                let created_at = created_at_str
                    .parse::<chrono::DateTime<chrono::Utc>>()
                    .unwrap_or_else(|_| chrono::Utc::now());

                let text_content: Option<String> = row.get(5)?;
                let metadata: Option<ClipboardMetadata> = row
                    .get::<_, Option<String>>(6)?
                    .and_then(|s| serde_json::from_str(&s).ok());
                let file_paths: Option<Vec<String>> = row
                    .get::<_, Option<String>>(7)?
                    .and_then(|s| serde_json::from_str(&s).ok());
                let tags: Option<Vec<String>> = row
                    .get::<_, Option<String>>(9)?
                    .and_then(|s| serde_json::from_str(&s).ok());

                Ok(ClipboardItem {
                    id: row.get(0)?,
                    content_type,
                    content: row.get(2)?,
                    created_at,
                    content_hash: row.get(4)?,
                    text_content,
                    metadata,
                    file_paths,
                    thumbnail_path: row.get(8)?,
                    tags,
                })
            })?
            .collect::<Result<Vec<_>>>()?;

        Ok(items)
    }

    /// 搜索历史记录
    pub fn search_history(&self, query: &str, limit: i64) -> Result<Vec<ClipboardItem>> {
        let conn = self.conn.lock().unwrap();
        let query_lower = query.to_lowercase();

        let mut stmt = conn.prepare(
            "SELECT id, content_type, content, created_at, content_hash, text_content, metadata, file_paths, thumbnail_path, tags
             FROM clipboard_history
             ORDER BY created_at DESC
             LIMIT 1000",
        )?;

        let items = stmt
            .query_map([], |row| {
                let content_type_str: String = row.get(1)?;
                let content_type = match content_type_str.as_str() {
                    "text" => ClipboardContentType::Text,
                    "html" => ClipboardContentType::Html,
                    "rtf" => ClipboardContentType::Rtf,
                    "image" => ClipboardContentType::Image,
                    "file" => ClipboardContentType::File,
                    "folder" => ClipboardContentType::Folder,
                    "files" => ClipboardContentType::Files,
                    _ => ClipboardContentType::Text,
                };

                let created_at_str: String = row.get(3)?;
                let created_at = created_at_str
                    .parse::<chrono::DateTime<chrono::Utc>>()
                    .unwrap_or_else(|_| chrono::Utc::now());

                let text_content: Option<String> = row.get(5)?;
                let metadata_str: Option<String> = row.get(6)?;
                let metadata = metadata_str.and_then(|s| serde_json::from_str(&s).ok());

                let file_paths_str: Option<String> = row.get(7)?;
                let file_paths = file_paths_str.and_then(|s| serde_json::from_str(&s).ok());

                let tags_str: Option<String> = row.get(9)?;
                let tags = tags_str.and_then(|s| serde_json::from_str(&s).ok());

                Ok(ClipboardItem {
                    id: row.get(0)?,
                    content_type,
                    content: row.get(2)?,
                    created_at,
                    content_hash: row.get(4)?,
                    text_content,
                    metadata,
                    file_paths,
                    thumbnail_path: row.get(8)?,
                    tags,
                })
            })?
            .filter_map(|item| item.ok())
            .filter(|item| {
                let content_lower = item.content.to_lowercase();
                content_lower.contains(&query_lower)
                    || (item
                        .tags
                        .as_ref()
                        .map_or(false, |tags| tags.iter().any(|tag| tag.to_lowercase().contains(&query_lower))))
            })
            .take(limit as usize)
            .collect();

        Ok(items)
    }

    /// 高级搜索历史记录（支持标签和类型过滤）
    /// 
    /// 优化策略：
    /// 1. 标签过滤：使用 SQL LIKE 在数据库层面过滤（JSON 数组格式: ["tag1","tag2"]）
    /// 2. 内容过滤：使用 SQL LIKE 在数据库层面过滤 content 和 text_content
    /// 3. 只返回匹配的记录，避免大量数据传输
    pub fn search_history_advanced(&self, request: &AdvancedSearchRequest) -> Result<Vec<ClipboardItem>> {
        let conn = self.conn.lock().unwrap();

        let target_limit = request.limit.unwrap_or(50);

        // 构建 SQL 条件
        let mut conditions: Vec<String> = vec!["1=1".to_string()];
        let mut sql_params: Vec<Box<dyn rusqlite::ToSql>> = vec![];

        // 类型过滤
        if !request.types.is_empty() {
            let placeholders: Vec<String> = (0..request.types.len())
                .map(|i| format!("?{}", i + 1))
                .collect();
            conditions.push(format!("content_type IN ({})", placeholders.join(", ")));
            for t in &request.types {
                let type_str = format!("{:?}", t).to_lowercase();
                sql_params.push(Box::new(type_str));
            }
        }

        // 标签过滤：使用 LIKE 匹配 JSON 数组格式（不区分大小写）
        // 例如: LOWER(tags) LIKE '%"nekotick-api"%' 匹配 {"nekotick-api"}
        for tag in &request.tags {
            let tag_pattern = format!("%\"{}\"%", tag.to_lowercase());
            conditions.push("LOWER(tags) LIKE ?".to_string());
            sql_params.push(Box::new(tag_pattern));
        }

        // 关键词过滤（不区分大小写）：
        // - HTML类型：只用 text_content（纯文本提取内容）过滤
        // - 其他类型：用 content 过滤
        // - 多个关键词之间是 AND 关系（必须同时匹配所有关键词）
        if !request.keywords.is_empty() {
            let mut keyword_groups: Vec<String> = vec![];

            for keyword in &request.keywords {
                let keyword_lower = keyword.to_lowercase();
                let keyword_pattern = format!("%{}%", keyword_lower);

                // 每个关键词作为一个组：(HTML条件 OR 非HTML条件)
                // 组内是 OR（匹配 HTML 内容或非 HTML 内容）
                // 组间是 AND（所有关键词都必须匹配）
                let group = "(content_type = 'html' AND LOWER(text_content) LIKE ?) OR (content_type != 'html' AND LOWER(content) LIKE ?)".to_string();
                keyword_groups.push(format!("({})", group));
                sql_params.push(Box::new(keyword_pattern.clone()));
                sql_params.push(Box::new(keyword_pattern));
            }

            conditions.push(format!("({})", keyword_groups.join(" AND ")));
        }

        let sql = format!(
            "SELECT id, content_type, content, created_at, content_hash, text_content, metadata, file_paths, thumbnail_path, tags
             FROM clipboard_history
             WHERE {}
             ORDER BY created_at DESC
             LIMIT {}",
            conditions.join(" AND "),
            target_limit
        );

        let mut stmt = conn.prepare(&sql)?;

        let items: Vec<ClipboardItem> = stmt
            .query_map(rusqlite::params_from_iter(sql_params.iter()), |row| {
                let content_type_str: String = row.get(1)?;
                let content_type = match content_type_str.as_str() {
                    "text" => ClipboardContentType::Text,
                    "html" => ClipboardContentType::Html,
                    "rtf" => ClipboardContentType::Rtf,
                    "image" => ClipboardContentType::Image,
                    "file" => ClipboardContentType::File,
                    "folder" => ClipboardContentType::Folder,
                    "files" => ClipboardContentType::Files,
                    _ => ClipboardContentType::Text,
                };

                let created_at_str: String = row.get(3)?;
                let created_at = created_at_str
                    .parse::<chrono::DateTime<chrono::Utc>>()
                    .unwrap_or_else(|_| chrono::Utc::now());

                let text_content: Option<String> = row.get(5)?;
                let metadata_str: Option<String> = row.get(6)?;
                let metadata = metadata_str.and_then(|s| serde_json::from_str(&s).ok());

                let file_paths_str: Option<String> = row.get(7)?;
                let file_paths = file_paths_str.and_then(|s| serde_json::from_str(&s).ok());

                let tags_str: Option<String> = row.get(9)?;
                let tags = tags_str.and_then(|s| serde_json::from_str(&s).ok());

                Ok(ClipboardItem {
                    id: row.get(0)?,
                    content_type,
                    content: row.get(2)?,
                    created_at,
                    content_hash: row.get(4)?,
                    text_content,
                    metadata,
                    file_paths,
                    thumbnail_path: row.get(8)?,
                    tags,
                })
            })?
            .filter_map(|item| item.ok())
            .collect();

        Ok(items)
    }

    /// 删除单条记录
    pub fn delete_item(&self, id: i64) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute("DELETE FROM clipboard_history WHERE id = ?1", params![id])?;
        Ok(())
    }

    /// 更新标签
    pub fn update_tags(&self, id: i64, tags: &Option<Vec<String>>) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        let tags_json = tags
            .as_ref()
            .map(|t| serde_json::to_string(t).ok())
            .flatten();
        conn.execute(
            "UPDATE clipboard_history SET tags = ?1 WHERE id = ?2",
            params![tags_json, id],
        )?;
        Ok(())
    }

    /// 获取所有标签
    pub fn get_all_tags(&self) -> Result<Vec<(String, i64)>> {
        let conn = self.conn.lock().unwrap();

        let mut stmt = conn.prepare("SELECT tags FROM clipboard_history WHERE tags IS NOT NULL")?;

        let rows: Vec<String> = stmt
            .query_map([], |row| row.get::<_, String>(0))?
            .filter_map(|r| r.ok())
            .collect();

        let mut tag_counts: std::collections::HashMap<String, i64> =
            std::collections::HashMap::new();
        for tags_json in rows {
            if let Ok(tags) = serde_json::from_str::<Vec<String>>(&tags_json) {
                for tag in tags {
                    *tag_counts.entry(tag).or_insert(0) += 1;
                }
            }
        }

        let mut result: Vec<(String, i64)> = tag_counts.into_iter().collect();
        result.sort_by(|a, b| b.1.cmp(&a.1));

        Ok(result)
    }

    /// 清空历史
    pub fn clear_history(&self, request: &ClearHistoryRequest) -> Result<i64> {
        let conn = self.conn.lock().unwrap();

        let rows_affected = if let Some(keep_count) = request.keep_count {
            conn.execute(
                "DELETE FROM clipboard_history
                 WHERE id NOT IN (
                     SELECT id FROM clipboard_history
                     ORDER BY created_at DESC
                     LIMIT ?1
                 )",
                params![keep_count],
            )?
        } else if let Some(keep_days) = request.keep_days {
            let cutoff_date = chrono::Utc::now() - chrono::Duration::days(keep_days);
            conn.execute(
                "DELETE FROM clipboard_history WHERE created_at < ?1",
                params![cutoff_date.to_rfc3339()],
            )?
        } else {
            conn.execute("DELETE FROM clipboard_history", [])?
        };

        Ok(rows_affected as i64)
    }

    /// 启动时自动清理
    pub fn startup_cleanup(&self, max_history_count: i64, auto_cleanup_days: i64) -> Result<i64> {
        let conn = self.conn.lock().unwrap();
        let mut total_deleted = 0i64;

        let count_without_tags: i64 = conn.query_row(
            "SELECT COUNT(*) FROM clipboard_history WHERE tags IS NULL OR tags = '' OR tags = 'null'",
            [],
            |row| row.get(0),
        )?;

        if count_without_tags > max_history_count {
            let to_delete_count = count_without_tags - max_history_count;

            let rows_affected = conn.execute(
                "DELETE FROM clipboard_history
                 WHERE id IN (
                     SELECT id FROM clipboard_history
                     WHERE tags IS NULL OR tags = '' OR tags = 'null'
                     ORDER BY created_at ASC
                     LIMIT ?1
                 )",
                params![to_delete_count],
            )?;
            total_deleted += rows_affected as i64;
        }

        if auto_cleanup_days > 0 {
            let cutoff_date = chrono::Utc::now() - chrono::Duration::days(auto_cleanup_days);

            let rows_affected = conn.execute(
                "DELETE FROM clipboard_history
                 WHERE (tags IS NULL OR tags = '' OR tags = 'null')
                   AND created_at < ?1",
                params![cutoff_date.to_rfc3339()],
            )?;
            total_deleted += rows_affected as i64;
        }

        Ok(total_deleted)
    }

    /// 获取记录总数
    pub fn get_count(&self) -> Result<i64> {
        let conn = self.conn.lock().unwrap();
        let count: i64 = conn.query_row("SELECT COUNT(*) FROM clipboard_history", [], |row| {
            row.get(0)
        })?;
        Ok(count)
    }

    /// 获取设置
    pub fn get_settings(&self) -> Result<AppSettings> {
        let conn = self.conn.lock().unwrap();

        let mut settings = AppSettings::default();

        let mut stmt = conn.prepare("SELECT key, value FROM settings")?;
        let rows = stmt.query_map([], |row| {
            let key: String = row.get(0)?;
            let value: String = row.get(1)?;
            Ok((key, value))
        })?;

        for row in rows {
            let (key, value) = row?;
            match key.as_str() {
                "max_history_count" => {
                    if let Ok(v) = value.parse() {
                        settings.max_history_count = v;
                    }
                }
                "auto_cleanup_days" => {
                    if let Ok(v) = value.parse() {
                        settings.auto_cleanup_days = v;
                    }
                }
                "window_position" => settings.window_position = value,
                "window_pos_x" => {
                    if !value.is_empty() {
                        if let Ok(v) = value.parse() {
                            settings.window_pos_x = Some(v);
                        }
                    }
                }
                "window_pos_y" => {
                    if !value.is_empty() {
                        if let Ok(v) = value.parse() {
                            settings.window_pos_y = Some(v);
                        }
                    }
                }
                "copy_sound" => {
                    if let Ok(v) = value.parse() {
                        settings.copy_sound = v;
                    }
                }
                "search_position" => settings.search_position = value,
                "focus_search_on_activate" => {
                    if let Ok(v) = value.parse() {
                        settings.focus_search_on_activate = v;
                    }
                }
                "auto_focus_search" => {
                    if let Ok(v) = value.parse() {
                        settings.focus_search_on_activate = v;
                    }
                }
                "click_action" => settings.click_action = value,
                "double_click_action" => settings.double_click_action = value,
                "paste_shortcut" => settings.paste_shortcut = value,
                "hide_window_after_copy" => {
                    if let Ok(v) = value.parse() {
                        settings.hide_window_after_copy = v;
                    }
                }
                "image_ocr" => {
                    if let Ok(v) = value.parse() {
                        settings.image_ocr = v;
                    }
                }
                "copy_as_plain_text" => {
                    if let Ok(v) = value.parse() {
                        settings.copy_as_plain_text = v;
                    }
                }
                "paste_as_plain_text" => {
                    if let Ok(v) = value.parse() {
                        settings.paste_as_plain_text = v;
                    }
                }
                "confirm_delete" => {
                    if let Ok(v) = value.parse() {
                        settings.confirm_delete = v;
                    }
                }
                "auto_sort" => {
                    if let Ok(v) = value.parse() {
                        settings.auto_sort = v;
                    }
                }
                "hotkey" => settings.hotkey = value,
                "auto_start" => {
                    if let Ok(v) = value.parse() {
                        settings.auto_start = v;
                    }
                }
                "number_key_shortcut" => settings.number_key_shortcut = value,
                "pin_shortcut" => settings.pin_shortcut = value,
                _ => {}
            }
        }

        Ok(settings)
    }

    /// 保存设置
    pub fn save_settings(&self, settings: &AppSettings) -> Result<()> {
        let conn = self.conn.lock().unwrap();

        let settings_to_save = vec![
            ("max_history_count", settings.max_history_count.to_string()),
            ("auto_cleanup_days", settings.auto_cleanup_days.to_string()),
            ("window_position", settings.window_position.clone()),
            ("window_pos_x", settings.window_pos_x.map(|v| v.to_string()).unwrap_or_default()),
            ("window_pos_y", settings.window_pos_y.map(|v| v.to_string()).unwrap_or_default()),
            ("copy_sound", settings.copy_sound.to_string()),
            ("search_position", settings.search_position.clone()),
            ("focus_search_on_activate", settings.focus_search_on_activate.to_string()),
            ("click_action", settings.click_action.clone()),
            ("double_click_action", settings.double_click_action.clone()),
            ("paste_shortcut", settings.paste_shortcut.clone()),
            ("hide_window_after_copy", settings.hide_window_after_copy.to_string()),
            ("image_ocr", settings.image_ocr.to_string()),
            ("copy_as_plain_text", settings.copy_as_plain_text.to_string()),
            ("paste_as_plain_text", settings.paste_as_plain_text.to_string()),
            ("confirm_delete", settings.confirm_delete.to_string()),
            ("auto_sort", settings.auto_sort.to_string()),
            ("hotkey", settings.hotkey.clone()),
            ("auto_start", settings.auto_start.to_string()),
            ("number_key_shortcut", settings.number_key_shortcut.clone()),
            ("pin_shortcut", settings.pin_shortcut.clone()),
        ];

        for (key, value) in settings_to_save {
            conn.execute(
                "INSERT OR REPLACE INTO settings (key, value) VALUES (?1, ?2)",
                params![key, value],
            )?;
        }

        Ok(())
    }
}