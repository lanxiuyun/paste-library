use rusqlite::{params, Connection, OptionalExtension, Result};
use std::path::PathBuf;
use std::sync::Mutex;

use crate::models::{AppSettings, ClipboardContentType, ClipboardItem, ClipboardMetadata};

/// Database manager
pub struct Database {
    conn: Mutex<Connection>,
}

impl Database {
    /// Initialize database connection
    pub fn new(app_dir: PathBuf) -> Result<Self> {
        let db_path = app_dir.join("clipboard.db");
        let conn = Connection::open(&db_path)?;

        let db = Self {
            conn: Mutex::new(conn),
        };

        db.init_tables()?;
        Ok(db)
    }

    /// Create necessary tables
    fn init_tables(&self) -> Result<()> {
        let conn = self.conn.lock().unwrap();

        // Clipboard history table
        conn.execute(
            "CREATE TABLE IF NOT EXISTS clipboard_history (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                content_type TEXT NOT NULL,
                content TEXT NOT NULL,
                created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
                content_hash TEXT NOT NULL UNIQUE,
                metadata TEXT,
                file_paths TEXT,
                thumbnail_path TEXT,
                tags TEXT
            )",
            [],
        )?;

        // Settings table
        conn.execute(
            "CREATE TABLE IF NOT EXISTS settings (
                key TEXT PRIMARY KEY,
                value TEXT NOT NULL
            )",
            [],
        )?;

        // Initialize default settings
        let defaults = vec![
            ("max_history_count", "5000"),
            ("auto_cleanup_days", "30"),
            ("window_position", "remember"),
            ("window_pos_x", ""),
            ("window_pos_y", ""),
            ("smart_activate", "true"),
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
            ("app_initialized", "false"),
        ];

        for (key, value) in defaults {
            conn.execute(
                "INSERT OR IGNORE INTO settings (key, value) VALUES (?1, ?2)",
                params![key, value],
            )?;
        }

        // Create index for faster searches
        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_content_hash ON clipboard_history(content_hash)",
            [],
        )?;
        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_created_at ON clipboard_history(created_at DESC)",
            [],
        )?;

        Ok(())
    }

    /// Check if first run
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

    /// Mark app as initialized
    pub fn mark_app_initialized(&self) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT OR REPLACE INTO settings (key, value) VALUES ('app_initialized', 'true')",
            [],
        )?;
        Ok(())
    }

    /// Add a clipboard item
    pub fn add_item(
        &self,
        content_type: ClipboardContentType,
        content: String,
        content_hash: String,
        metadata: Option<ClipboardMetadata>,
        file_paths: Option<Vec<String>>,
        thumbnail_path: Option<String>,
    ) -> Result<Option<ClipboardItem>> {
        let conn = self.conn.lock().unwrap();

        // Check if item already exists
        let existing: Option<i64> = conn
            .query_row(
                "SELECT id FROM clipboard_history WHERE content_hash = ?1",
                params![content_hash],
                |row| row.get(0),
            )
            .optional()?;

        if let Some(_id) = existing {
            // Update timestamp to move to top
            conn.execute(
                "UPDATE clipboard_history SET created_at = CURRENT_TIMESTAMP WHERE content_hash = ?1",
                params![content_hash],
            )?;

            return self.get_item_by_hash(&conn, &content_hash);
        }

        // Insert new item
        let metadata_json = metadata
            .as_ref()
            .map(|m| serde_json::to_string(m).ok())
            .flatten();
        let file_paths_json = file_paths
            .as_ref()
            .map(|v| serde_json::to_string(v).ok())
            .flatten();

        conn.execute(
            "INSERT INTO clipboard_history (content_type, content, content_hash, metadata, file_paths, thumbnail_path) 
             VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![
                serde_json::to_string(&content_type).unwrap_or_default(),
                content,
                content_hash,
                metadata_json,
                file_paths_json,
                thumbnail_path
            ],
        )?;

        let id = conn.last_insert_rowid();
        self.get_item_by_id(&conn, id)
    }

    /// Get item by ID
    fn get_item_by_id(&self, conn: &Connection, id: i64) -> Result<Option<ClipboardItem>> {
        conn.query_row(
            "SELECT id, content_type, content, created_at, content_hash, metadata, file_paths, thumbnail_path, tags 
             FROM clipboard_history WHERE id = ?1",
            params![id],
            |row| self.row_to_item(row),
        )
        .optional()
    }

    /// Get item by hash
    fn get_item_by_hash(
        &self,
        conn: &Connection,
        content_hash: &str,
    ) -> Result<Option<ClipboardItem>> {
        conn.query_row(
            "SELECT id, content_type, content, created_at, content_hash, metadata, file_paths, thumbnail_path, tags 
             FROM clipboard_history WHERE content_hash = ?1",
            params![content_hash],
            |row| self.row_to_item(row),
        )
        .optional()
    }

    /// Convert row to ClipboardItem
    fn row_to_item(&self, row: &rusqlite::Row) -> rusqlite::Result<ClipboardItem> {
        let content_type_str: String = row.get(1)?;
        let content_type: ClipboardContentType =
            serde_json::from_str(&content_type_str).unwrap_or(ClipboardContentType::Text);

        let metadata_str: Option<String> = row.get(5)?;
        let metadata: Option<ClipboardMetadata> =
            metadata_str.and_then(|s| serde_json::from_str(&s).ok());

        let file_paths_str: Option<String> = row.get(6)?;
        let file_paths: Option<Vec<String>> =
            file_paths_str.and_then(|s| serde_json::from_str(&s).ok());

        let tags_str: Option<String> = row.get(8)?;
        let tags: Option<Vec<String>> = tags_str.and_then(|s| serde_json::from_str(&s).ok());

        Ok(ClipboardItem {
            id: row.get(0)?,
            content_type,
            content: row.get(2)?,
            created_at: row.get(3)?,
            content_hash: row.get(4)?,
            metadata,
            file_paths,
            thumbnail_path: row.get(7)?,
            tags,
        })
    }

    /// Get clipboard history
    pub fn get_history(&self, limit: i64, offset: i64) -> Result<Vec<ClipboardItem>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, content_type, content, created_at, content_hash, metadata, file_paths, thumbnail_path, tags 
             FROM clipboard_history ORDER BY created_at DESC LIMIT ?1 OFFSET ?2"
        )?;

        let items = stmt
            .query_map(params![limit, offset], |row| self.row_to_item(row))?
            .filter_map(|r| r.ok())
            .collect();

        Ok(items)
    }

    /// Search clipboard history
    pub fn search(&self, query: &str, limit: i64) -> Result<Vec<ClipboardItem>> {
        let conn = self.conn.lock().unwrap();
        let search_pattern = format!("%{}%", query);

        let mut stmt = conn.prepare(
            "SELECT id, content_type, content, created_at, content_hash, metadata, file_paths, thumbnail_path, tags 
             FROM clipboard_history WHERE content LIKE ?1 ORDER BY created_at DESC LIMIT ?2"
        )?;

        let items = stmt
            .query_map(params![search_pattern, limit], |row| self.row_to_item(row))?
            .filter_map(|r| r.ok())
            .collect();

        Ok(items)
    }

    /// Delete item by ID
    pub fn delete_item(&self, id: i64) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute("DELETE FROM clipboard_history WHERE id = ?1", params![id])?;
        Ok(())
    }

    /// Clear history
    pub fn clear_history(&self, keep_count: Option<i64>, keep_days: Option<i64>) -> Result<i64> {
        let conn = self.conn.lock().unwrap();

        // Count total items
        let total: i64 = conn.query_row("SELECT COUNT(*) FROM clipboard_history", [], |row| {
            row.get(0)
        })?;

        // Delete by count
        if let Some(count) = keep_count {
            conn.execute(
                "DELETE FROM clipboard_history WHERE id NOT IN (
                    SELECT id FROM clipboard_history ORDER BY created_at DESC LIMIT ?1
                )",
                params![count],
            )?;
        }

        // Delete by days
        if let Some(days) = keep_days {
            conn.execute(
                "DELETE FROM clipboard_history WHERE created_at < datetime('now', ?1)",
                params![format!("-{} days", days)],
            )?;
        }

        let remaining: i64 =
            conn.query_row("SELECT COUNT(*) FROM clipboard_history", [], |row| {
                row.get(0)
            })?;

        Ok(total - remaining)
    }

    /// Get settings
    pub fn get_settings(&self) -> Result<AppSettings> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare("SELECT key, value FROM settings")?;

        let mut settings = AppSettings::default();
        let rows = stmt.query_map([], |row| {
            Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?))
        })?;

        for row in rows.flatten() {
            let (key, value) = row;
            match key.as_str() {
                "max_history_count" => settings.max_history_count = value.parse().unwrap_or(5000),
                "auto_cleanup_days" => settings.auto_cleanup_days = value.parse().unwrap_or(30),
                "window_position" => settings.window_position = value,
                "window_pos_x" => settings.window_pos_x = value.parse().ok(),
                "window_pos_y" => settings.window_pos_y = value.parse().ok(),
                "smart_activate" => settings.smart_activate = value == "true",
                "copy_sound" => settings.copy_sound = value == "true",
                "search_position" => settings.search_position = value,
                "focus_search_on_activate" => settings.focus_search_on_activate = value == "true",
                "click_action" => settings.click_action = value,
                "double_click_action" => settings.double_click_action = value,
                "paste_shortcut" => settings.paste_shortcut = value,
                "hide_window_after_copy" => settings.hide_window_after_copy = value == "true",
                "image_ocr" => settings.image_ocr = value == "true",
                "copy_as_plain_text" => settings.copy_as_plain_text = value == "true",
                "paste_as_plain_text" => settings.paste_as_plain_text = value == "true",
                "confirm_delete" => settings.confirm_delete = value == "true",
                "auto_sort" => settings.auto_sort = value == "true",
                "hotkey" => settings.hotkey = value,
                "auto_start" => settings.auto_start = value == "true",
                "number_key_shortcut" => settings.number_key_shortcut = value,
                _ => {}
            }
        }

        Ok(settings)
    }

    /// Save settings
    pub fn save_settings(&self, settings: &AppSettings) -> Result<()> {
        let conn = self.conn.lock().unwrap();

        let settings_map = vec![
            ("max_history_count", settings.max_history_count.to_string()),
            ("auto_cleanup_days", settings.auto_cleanup_days.to_string()),
            ("window_position", settings.window_position.clone()),
            (
                "window_pos_x",
                settings
                    .window_pos_x
                    .map(|v| v.to_string())
                    .unwrap_or_default(),
            ),
            (
                "window_pos_y",
                settings
                    .window_pos_y
                    .map(|v| v.to_string())
                    .unwrap_or_default(),
            ),
            ("smart_activate", settings.smart_activate.to_string()),
            ("copy_sound", settings.copy_sound.to_string()),
            ("search_position", settings.search_position.clone()),
            (
                "focus_search_on_activate",
                settings.focus_search_on_activate.to_string(),
            ),
            ("click_action", settings.click_action.clone()),
            ("double_click_action", settings.double_click_action.clone()),
            ("paste_shortcut", settings.paste_shortcut.clone()),
            (
                "hide_window_after_copy",
                settings.hide_window_after_copy.to_string(),
            ),
            ("image_ocr", settings.image_ocr.to_string()),
            (
                "copy_as_plain_text",
                settings.copy_as_plain_text.to_string(),
            ),
            (
                "paste_as_plain_text",
                settings.paste_as_plain_text.to_string(),
            ),
            ("confirm_delete", settings.confirm_delete.to_string()),
            ("auto_sort", settings.auto_sort.to_string()),
            ("hotkey", settings.hotkey.clone()),
            ("auto_start", settings.auto_start.to_string()),
            ("number_key_shortcut", settings.number_key_shortcut.clone()),
        ];

        for (key, value) in settings_map {
            conn.execute(
                "INSERT OR REPLACE INTO settings (key, value) VALUES (?1, ?2)",
                params![key, value],
            )?;
        }

        Ok(())
    }
}
