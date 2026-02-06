use rusqlite::{params, Connection, Result};
use std::path::PathBuf;
use std::sync::Mutex;

use crate::models::{AppSettings, ClearHistoryRequest, ClipboardContentType, ClipboardItem};

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
                content_hash TEXT NOT NULL UNIQUE
            )",
            [],
        )?;

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
            ("window_width", "800"),
            ("window_height", "600"),
            ("scroll_to_top_on_activate", "false"),
            ("switch_to_all_on_activate", "true"),
            ("copy_sound", "false"),
            ("search_position", "bottom"),
            ("auto_focus_search", "true"),
            ("clear_search_on_activate", "false"),
            ("auto_paste", "double"),
            ("image_ocr", "false"),
            ("copy_as_plain_text", "false"),
            ("paste_as_plain_text", "true"),
            ("auto_favorite", "false"),
            ("confirm_delete", "true"),
            ("auto_sort", "false"),
            ("hotkey", "Alt+V"),
            ("auto_start", "false"),
            ("blacklist_apps", "[]"),
        ];

        for (key, value) in defaults {
            conn.execute(
                "INSERT OR IGNORE INTO settings (key, value) VALUES (?1, ?2)",
                params![key, value],
            )?;
        }

        Ok(())
    }

    /// 添加剪贴板记录
    pub fn add_clipboard_item(&self, item: &ClipboardItem) -> Result<i64> {
        let conn = self.conn.lock().unwrap();

        conn.execute(
            "INSERT INTO clipboard_history (content_type, content, created_at, content_hash)
             VALUES (?1, ?2, ?3, ?4)
             ON CONFLICT(content_hash) DO UPDATE SET
                created_at = excluded.created_at",
            params![
                match item.content_type {
                    ClipboardContentType::Text => "text",
                    ClipboardContentType::Html => "html",
                    ClipboardContentType::Rtf => "rtf",
                },
                item.content,
                item.created_at.to_rfc3339(),
                item.content_hash
            ],
        )?;

        Ok(conn.last_insert_rowid())
    }

    /// 获取历史记录
    pub fn get_history(&self, limit: i64, offset: i64) -> Result<Vec<ClipboardItem>> {
        let conn = self.conn.lock().unwrap();

        let mut stmt = conn.prepare(
            "SELECT id, content_type, content, created_at, content_hash
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
                    _ => ClipboardContentType::Text,
                };

                let created_at_str: String = row.get(3)?;
                let created_at = created_at_str
                    .parse::<chrono::DateTime<chrono::Utc>>()
                    .unwrap_or_else(|_| chrono::Utc::now());

                Ok(ClipboardItem {
                    id: row.get(0)?,
                    content_type,
                    content: row.get(2)?,
                    created_at,
                    content_hash: row.get(4)?,
                })
            })?
            .collect::<Result<Vec<_>>>()?;

        Ok(items)
    }

    /// 搜索历史记录
    pub fn search_history(&self, query: &str, limit: i64) -> Result<Vec<ClipboardItem>> {
        let conn = self.conn.lock().unwrap();

        let search_pattern = format!("%{}%", query);

        let mut stmt = conn.prepare(
            "SELECT id, content_type, content, created_at, content_hash
             FROM clipboard_history
             WHERE content LIKE ?1
             ORDER BY created_at DESC
             LIMIT ?2",
        )?;

        let items = stmt
            .query_map(params![search_pattern, limit], |row| {
                let content_type_str: String = row.get(1)?;
                let content_type = match content_type_str.as_str() {
                    "text" => ClipboardContentType::Text,
                    "html" => ClipboardContentType::Html,
                    "rtf" => ClipboardContentType::Rtf,
                    _ => ClipboardContentType::Text,
                };

                let created_at_str: String = row.get(3)?;
                let created_at = created_at_str
                    .parse::<chrono::DateTime<chrono::Utc>>()
                    .unwrap_or_else(|_| chrono::Utc::now());

                Ok(ClipboardItem {
                    id: row.get(0)?,
                    content_type,
                    content: row.get(2)?,
                    created_at,
                    content_hash: row.get(4)?,
                })
            })?
            .collect::<Result<Vec<_>>>()?;

        Ok(items)
    }

    /// 删除单条记录
    pub fn delete_item(&self, id: i64) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute("DELETE FROM clipboard_history WHERE id = ?1", params![id])?;
        Ok(())
    }

    /// 清空历史 (支持按条数或日期)
    pub fn clear_history(&self, request: &ClearHistoryRequest) -> Result<i64> {
        let conn = self.conn.lock().unwrap();

        let rows_affected = if let Some(keep_count) = request.keep_count {
            // 按条数保留
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
            // 按日期保留
            let cutoff_date = chrono::Utc::now() - chrono::Duration::days(keep_days);
            conn.execute(
                "DELETE FROM clipboard_history WHERE created_at < ?1",
                params![cutoff_date.to_rfc3339()],
            )?
        } else {
            // 全部清空
            conn.execute("DELETE FROM clipboard_history", [])?
        };

        Ok(rows_affected as i64)
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
                "window_width" => {
                    if let Ok(v) = value.parse() {
                        settings.window_width = v;
                    }
                }
                "window_height" => {
                    if let Ok(v) = value.parse() {
                        settings.window_height = v;
                    }
                }
                "scroll_to_top_on_activate" => {
                    if let Ok(v) = value.parse() {
                        settings.scroll_to_top_on_activate = v;
                    }
                }
                "switch_to_all_on_activate" => {
                    if let Ok(v) = value.parse() {
                        settings.switch_to_all_on_activate = v;
                    }
                }
                "copy_sound" => {
                    if let Ok(v) = value.parse() {
                        settings.copy_sound = v;
                    }
                }
                "search_position" => settings.search_position = value,
                "auto_focus_search" => {
                    if let Ok(v) = value.parse() {
                        settings.auto_focus_search = v;
                    }
                }
                "clear_search_on_activate" => {
                    if let Ok(v) = value.parse() {
                        settings.clear_search_on_activate = v;
                    }
                }
                "auto_paste" => settings.auto_paste = value,
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
                "auto_favorite" => {
                    if let Ok(v) = value.parse() {
                        settings.auto_favorite = v;
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
                "blacklist_apps" => {
                    if let Ok(v) = serde_json::from_str(&value) {
                        settings.blacklist_apps = v;
                    }
                }
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
            ("window_width", settings.window_width.to_string()),
            ("window_height", settings.window_height.to_string()),
            (
                "scroll_to_top_on_activate",
                settings.scroll_to_top_on_activate.to_string(),
            ),
            (
                "switch_to_all_on_activate",
                settings.switch_to_all_on_activate.to_string(),
            ),
            ("copy_sound", settings.copy_sound.to_string()),
            ("search_position", settings.search_position.clone()),
            ("auto_focus_search", settings.auto_focus_search.to_string()),
            (
                "clear_search_on_activate",
                settings.clear_search_on_activate.to_string(),
            ),
            ("auto_paste", settings.auto_paste.clone()),
            ("image_ocr", settings.image_ocr.to_string()),
            (
                "copy_as_plain_text",
                settings.copy_as_plain_text.to_string(),
            ),
            (
                "paste_as_plain_text",
                settings.paste_as_plain_text.to_string(),
            ),
            ("auto_favorite", settings.auto_favorite.to_string()),
            ("confirm_delete", settings.confirm_delete.to_string()),
            ("auto_sort", settings.auto_sort.to_string()),
            ("hotkey", settings.hotkey.clone()),
            ("auto_start", settings.auto_start.to_string()),
            (
                "blacklist_apps",
                serde_json::to_string(&settings.blacklist_apps).unwrap_or_default(),
            ),
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
