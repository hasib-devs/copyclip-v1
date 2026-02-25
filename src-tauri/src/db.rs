use chrono::Utc;
use rusqlite::{Connection, OptionalExtension, Result as SqliteResult};
use serde_json;
use std::path::PathBuf;
use std::sync::Mutex;

use crate::models::{ClipboardItemModel, ClipboardQueryFilter};

/**
 * Database service for clipboard history
 * Handles all database operations using rusqlite
 * Wrapped in Mutex for thread-safe access in Tauri
 */
pub struct DatabaseService {
    conn: Mutex<Connection>,
}

impl DatabaseService {
    /**
     * Initialize database with connection
     */
    pub fn new(db_path: PathBuf) -> Result<Self, Box<dyn std::error::Error>> {
        // Create parent directory if it doesn't exist
        if let Some(parent) = db_path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        // Open connection
        let conn = Connection::open(&db_path)?;

        // Enable foreign keys
        conn.execute_batch("PRAGMA foreign_keys = ON;")?;

        // Create table
        conn.execute(
            r#"
            CREATE TABLE IF NOT EXISTS clipboard_items (
                id TEXT PRIMARY KEY,
                content TEXT NOT NULL,
                item_type TEXT NOT NULL,
                is_pinned BOOLEAN DEFAULT 0,
                timestamp INTEGER NOT NULL,
                image_base64 TEXT,
                file_paths TEXT,
                created_at INTEGER NOT NULL,
                updated_at INTEGER NOT NULL
            )
            "#,
            [],
        )?;

        // Create indexes
        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_timestamp ON clipboard_items(timestamp DESC);",
            [],
        )?;

        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_is_pinned ON clipboard_items(is_pinned);",
            [],
        )?;

        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_item_type ON clipboard_items(item_type);",
            [],
        )?;

        // Create gamepad profiles table
        conn.execute(
            r#"
            CREATE TABLE IF NOT EXISTS gamepad_profiles (
                name TEXT PRIMARY KEY,
                description TEXT NOT NULL,
                sensitivity REAL NOT NULL,
                dead_zone REAL NOT NULL,
                acceleration REAL NOT NULL,
                button_map TEXT NOT NULL,
                axis_map TEXT NOT NULL,
                enabled_features TEXT NOT NULL,
                created_at INTEGER NOT NULL,
                updated_at INTEGER NOT NULL
            )
            "#,
            [],
        )?;

        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_profile_created ON gamepad_profiles(created_at DESC);",
            [],
        )?;

        Ok(Self {
            conn: Mutex::new(conn),
        })
    }

    /**
     * Create a new clipboard item
     */
    pub fn create_item(&self, item: ClipboardItemModel) -> SqliteResult<usize> {
        eprintln!(
            "[DB::CREATE] Creating item: id={}, type={}",
            item.id, item.item_type
        );
        let conn = self.conn.lock().unwrap();
        eprintln!("[DB::CREATE] Database lock acquired");

        let result = conn.execute(
            r#"
            INSERT INTO clipboard_items 
            (id, content, item_type, is_pinned, timestamp, image_base64, file_paths, created_at, updated_at)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#,
            rusqlite::params![
                &item.id,
                &item.content,
                &item.item_type,
                item.is_pinned,
                item.timestamp,
                &item.image_base64,
                &item.file_paths,
                item.created_at,
                item.updated_at,
            ],
        ).map_err(|e| {
            eprintln!("[DB::CREATE] ERROR inserting item: {}", e);
            e
        })?;

        eprintln!("[DB::CREATE] Item inserted successfully, rows: {}", result);
        Ok(result)
    }

    /**
     * Get item by id
     */
    pub fn get_item(&self, id: &str) -> SqliteResult<Option<ClipboardItemModel>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, content, item_type, is_pinned, timestamp, image_base64, file_paths, created_at, updated_at FROM clipboard_items WHERE id = ?",
        )?;

        let item = stmt
            .query_row(rusqlite::params![id], |row| {
                Ok(ClipboardItemModel {
                    id: row.get(0)?,
                    content: row.get(1)?,
                    item_type: row.get(2)?,
                    is_pinned: row.get(3)?,
                    timestamp: row.get(4)?,
                    image_base64: row.get(5)?,
                    file_paths: row.get(6)?,
                    created_at: row.get(7)?,
                    updated_at: row.get(8)?,
                })
            })
            .optional()?;

        Ok(item)
    }

    /**
     * Get all items with filtering
     */
    pub fn get_items(&self, filter: ClipboardQueryFilter) -> SqliteResult<Vec<ClipboardItemModel>> {
        let conn = self.conn.lock().unwrap();
        let mut query = String::from(
            "SELECT id, content, item_type, is_pinned, timestamp, image_base64, file_paths, created_at, updated_at FROM clipboard_items WHERE 1=1"
        );

        let mut values: Vec<String> = Vec::new();

        if let Some(search) = &filter.search {
            query.push_str(" AND content LIKE ?");
            values.push(format!("%{}%", search));
        }

        if let Some(item_type) = &filter.item_type {
            query.push_str(" AND item_type = ?");
            values.push(item_type.clone());
        }

        if let Some(is_pinned) = filter.is_pinned {
            query.push_str(&format!(
                " AND is_pinned = {}",
                if is_pinned { 1 } else { 0 }
            ));
        }

        query.push_str(&format!(
            " ORDER BY is_pinned DESC, timestamp DESC LIMIT {} OFFSET {}",
            filter.limit, filter.offset
        ));

        let mut stmt = conn.prepare(&query)?;

        let items = stmt
            .query_map(rusqlite::params_from_iter(values), |row| {
                Ok(ClipboardItemModel {
                    id: row.get(0)?,
                    content: row.get(1)?,
                    item_type: row.get(2)?,
                    is_pinned: row.get(3)?,
                    timestamp: row.get(4)?,
                    image_base64: row.get(5)?,
                    file_paths: row.get(6)?,
                    created_at: row.get(7)?,
                    updated_at: row.get(8)?,
                })
            })?
            .collect::<Result<Vec<_>, _>>()?;

        Ok(items)
    }

    /**
     * Update item (toggle pin for example)
     */
    pub fn update_item(&self, id: &str, is_pinned: bool) -> SqliteResult<usize> {
        let conn = self.conn.lock().unwrap();
        let now = Utc::now().timestamp_millis();

        conn.execute(
            "UPDATE clipboard_items SET is_pinned = ?, updated_at = ? WHERE id = ?",
            rusqlite::params![is_pinned, now, id],
        )
    }

    /**
     * Delete item by id
     */
    pub fn delete_item(&self, id: &str) -> SqliteResult<usize> {
        eprintln!("[DB::DELETE] Starting delete operation for id: {}", id);

        let conn = self.conn.lock().unwrap();
        eprintln!("[DB::DELETE] Acquired database lock");

        // Check if item exists first
        let mut check_stmt = conn
            .prepare("SELECT id FROM clipboard_items WHERE id = ? LIMIT 1")
            .map_err(|e| {
                eprintln!("[DB::DELETE] Error preparing check query: {}", e);
                e
            })?;

        let exists = check_stmt.exists(rusqlite::params![id]).map_err(|e| {
            eprintln!("[DB::DELETE] Error checking if item exists: {}", e);
            e
        })?;

        eprintln!("[DB::DELETE] Item exists in database: {}", exists);
        drop(check_stmt);

        // Perform the delete
        let result = conn
            .execute(
                "DELETE FROM clipboard_items WHERE id = ?",
                rusqlite::params![id],
            )
            .map_err(|e| {
                eprintln!("[DB::DELETE] Error executing delete: {}", e);
                e
            })?;

        eprintln!(
            "[DB::DELETE] Delete query completed. Rows affected: {}",
            result
        );
        Ok(result)
    }

    /**
     * Delete all items
     */
    pub fn delete_all(&self) -> SqliteResult<usize> {
        let conn = self.conn.lock().unwrap();
        conn.execute("DELETE FROM clipboard_items", [])
    }

    /**
     * Get item count
     */
    pub fn count_items(&self) -> SqliteResult<i64> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare("SELECT COUNT(*) FROM clipboard_items")?;
        let count = stmt.query_row([], |row| row.get(0))?;
        Ok(count)
    }

    /**
     * Delete items older than specified timestamp
     */
    // pub fn delete_old_items(&self, before_timestamp: i64) -> SqliteResult<usize> {
    //     let conn = self.conn.lock().unwrap();
    //     conn.execute(
    //         "DELETE FROM clipboard_items WHERE timestamp < ?",
    //         rusqlite::params![before_timestamp],
    //     )
    // }

    /**
     * Enforce max items limit
     */
    pub fn enforce_max_items(&self, max_items: i64) -> SqliteResult<usize> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            r#"
            DELETE FROM clipboard_items WHERE id IN (
                SELECT id FROM clipboard_items 
                WHERE is_pinned = 0
                ORDER BY timestamp ASC 
                LIMIT MAX(0, (SELECT COUNT(*) - ? FROM clipboard_items WHERE is_pinned = 0))
            )
            "#,
            rusqlite::params![max_items],
        )
    }

    /**
     * Check if item with same content exists (for deduplication)
     */
    pub fn check_duplicate(&self, content: &str, item_type: &str) -> SqliteResult<bool> {
        eprintln!(
            "[DB::CHECK_DUP] Checking duplicate: type={}, content_len={}",
            item_type,
            content.len()
        );
        let conn = self.conn.lock().unwrap();
        eprintln!("[DB::CHECK_DUP] Database lock acquired");

        let mut stmt = conn.prepare(
            "SELECT COUNT(*) FROM clipboard_items WHERE content = ? AND item_type = ? ORDER BY timestamp DESC LIMIT 1"
        ).map_err(|e| {
            eprintln!("[DB::CHECK_DUP] ERROR preparing query: {}", e);
            e
        })?;

        let count = stmt
            .query_row(rusqlite::params![content, item_type], |row| {
                row.get::<_, i64>(0)
            })
            .map_err(|e| {
                eprintln!("[DB::CHECK_DUP] ERROR querying: {}", e);
                e
            })?;

        eprintln!("[DB::CHECK_DUP] Query result: count={}", count);
        Ok(count > 0)
    }

    /**
     * Save or update a gamepad profile
     */
    pub fn save_gamepad_profile(
        &self,
        name: &str,
        description: &str,
        sensitivity: f32,
        dead_zone: f32,
        acceleration: f32,
        button_map_json: &str,
        axis_map_json: &str,
        enabled_features_json: &str,
    ) -> SqliteResult<usize> {
        eprintln!("[DB::SAVE_PROFILE] Saving profile: {}", name);
        let conn = self.conn.lock().unwrap();
        let now = Utc::now().timestamp_millis();

        let mut stmt = conn.prepare("SELECT COUNT(*) FROM gamepad_profiles WHERE name = ?")?;

        let exists: bool = stmt.exists(rusqlite::params![name])?;
        drop(stmt);

        if exists {
            conn.execute(
                "UPDATE gamepad_profiles SET description = ?, sensitivity = ?, dead_zone = ?, acceleration = ?, button_map = ?, axis_map = ?, enabled_features = ?, updated_at = ? WHERE name = ?",
                rusqlite::params![description, sensitivity as f64, dead_zone as f64, acceleration as f64, button_map_json, axis_map_json, enabled_features_json, now, name],
            )
        } else {
            conn.execute(
                "INSERT INTO gamepad_profiles (name, description, sensitivity, dead_zone, acceleration, button_map, axis_map, enabled_features, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
                rusqlite::params![name, description, sensitivity as f64, dead_zone as f64, acceleration as f64, button_map_json, axis_map_json, enabled_features_json, now, now],
            )
        }
    }

    /**
     * Get all gamepad profiles as JSON
     */
    pub fn get_gamepad_profiles(&self) -> SqliteResult<Vec<serde_json::Value>> {
        eprintln!("[DB::GET_PROFILES] Starting profile fetch from database...");
        let conn = self.conn.lock().unwrap();

        let mut stmt = conn.prepare(
            "SELECT name, description, sensitivity, dead_zone, acceleration, button_map, axis_map, enabled_features FROM gamepad_profiles ORDER BY created_at DESC"
        )?;

        eprintln!("[DB::GET_PROFILES] Query prepared, executing...");

        let profiles = stmt.query_map([], |row| {
            let name = row.get::<_, String>(0)?;
            eprintln!("[DB::GET_PROFILES] Processing profile row: {}", name);
            Ok(serde_json::json!({
                "name": name,
                "description": row.get::<_, String>(1)?,
                "sensitivity": row.get::<_, f64>(2)? as f32,
                "dead_zone": row.get::<_, f64>(3)? as f32,
                "acceleration": row.get::<_, f64>(4)? as f32,
                "button_map": serde_json::from_str::<serde_json::Value>(&row.get::<_, String>(5)?).unwrap_or_default(),
                "axis_map": serde_json::from_str::<serde_json::Value>(&row.get::<_, String>(6)?).unwrap_or_default(),
                "enabled_features": serde_json::from_str::<serde_json::Value>(&row.get::<_, String>(7)?).unwrap_or_default(),
            }))
        })?;

        let mut result = Vec::new();
        for profile in profiles {
            if let Ok(p) = profile {
                eprintln!("[DB::GET_PROFILES] Successfully converted profile to JSON");
                result.push(p);
            } else {
                eprintln!("[DB::GET_PROFILES] Failed to convert profile row");
            }
        }

        eprintln!(
            "[DB::GET_PROFILES] Total profiles fetched from database: {}",
            result.len()
        );
        Ok(result)
    }

    /**
     * Delete a gamepad profile
     */
    pub fn delete_gamepad_profile(&self, name: &str) -> SqliteResult<usize> {
        eprintln!("[DB::DELETE_PROFILE] Deleting profile: {}", name);
        let conn = self.conn.lock().unwrap();

        conn.execute(
            "DELETE FROM gamepad_profiles WHERE name = ?",
            rusqlite::params![name],
        )
    }
}
