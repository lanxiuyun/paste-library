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
        conn.execute(
            "INSERT OR IGNORE INTO settings (key, value) VALUES 
                ('max_history_count', '5000'),
                ('hotkey', 'Alt+V'),
                ('auto_start', 'false'),
                ('auto_cleanup_days', '30'),
                ('window_width', '800'),
                ('window_height', '600'),
                ('blacklist_apps', '[]')",
            [],
        )?;

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
                "hotkey" => settings.hotkey = value,
                "auto_start" => {
                    if let Ok(v) = value.parse() {
                        settings.auto_start = v;
                    }
                }
                "auto_cleanup_days" => {
                    if let Ok(v) = value.parse() {
                        settings.auto_cleanup_days = v;
                    }
                }
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

        conn.execute(
            "INSERT OR REPLACE INTO settings (key, value) VALUES 
                ('max_history_count', ?1),
                ('hotkey', ?2),
                ('auto_start', ?3),
                ('auto_cleanup_days', ?4),
                ('window_width', ?5),
                ('window_height', ?6),
                ('blacklist_apps', ?7)",
            params![
                settings.max_history_count.to_string(),
                settings.hotkey,
                settings.auto_start.to_string(),
                settings.auto_cleanup_days.to_string(),
                settings.window_width.to_string(),
                settings.window_height.to_string(),
                serde_json::to_string(&settings.blacklist_apps).unwrap_or_default(),
            ],
        )?;

        Ok(())
    }
}
