use chrono::Utc;
use rusqlite::{Connection, OptionalExtension, Result as SqliteResult};
use serde_json;
use std::path::PathBuf;
use std::sync::Mutex;

/**
 * Database service
 * Handles database operations using rusqlite
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

        // Run migrations
        Self::migrate_gamepad_profiles(&conn)?;

        Ok(Self {
            conn: Mutex::new(conn),
        })
    }

    /**
     * Run database migrations for gamepad_profiles table
     */
    fn migrate_gamepad_profiles(conn: &Connection) -> SqliteResult<()> {
        eprintln!("[DB::MIGRATE] Checking for gamepad_profiles schema...");

        // Check if button_map column exists
        let mut stmt = conn.prepare("PRAGMA table_info(gamepad_profiles)")?;
        let columns: Vec<String> = stmt
            .query_map([], |row| row.get(1))?
            .collect::<Result<Vec<_>, _>>()?;

        eprintln!("[DB::MIGRATE] Existing columns: {:?}", columns);

        // Add missing columns if they don't exist
        if !columns.contains(&"button_map".to_string()) {
            eprintln!("[DB::MIGRATE] Adding button_map column...");
            conn.execute(
                "ALTER TABLE gamepad_profiles ADD COLUMN button_map TEXT NOT NULL DEFAULT '{}'",
                [],
            )?;
        }

        if !columns.contains(&"axis_map".to_string()) {
            eprintln!("[DB::MIGRATE] Adding axis_map column...");
            conn.execute(
                "ALTER TABLE gamepad_profiles ADD COLUMN axis_map TEXT NOT NULL DEFAULT '{}'",
                [],
            )?;
        }

        if !columns.contains(&"enabled_features".to_string()) {
            eprintln!("[DB::MIGRATE] Adding enabled_features column...");
            let default_features = r#"{"mouse_control":true,"keyboard_emulation":false,"vibration":true,"adaptive_triggers":false,"scroll_control":true}"#;
            conn.execute(
                &format!("ALTER TABLE gamepad_profiles ADD COLUMN enabled_features TEXT NOT NULL DEFAULT '{}'", default_features),
                []
            )?;
        }

        if !columns.contains(&"scroll_settings".to_string()) {
            eprintln!("[DB::MIGRATE] Adding scroll_settings column...");
            let default_scroll = r#"{"enabled":true,"vertical_speed":1.0,"horizontal_speed":1.0,"reverse_vertical":false,"reverse_horizontal":false}"#;
            conn.execute(
                &format!("ALTER TABLE gamepad_profiles ADD COLUMN scroll_settings TEXT NOT NULL DEFAULT '{}'", default_scroll),
                []
            )?;
        }

        if !columns.contains(&"dpad_mapping".to_string()) {
            eprintln!("[DB::MIGRATE] Adding dpad_mapping column...");
            let default_dpad = r#"{"up":{"single":"Up"},"down":{"single":"Down"},"left":{"single":"Left"},"right":{"single":"Right"}}"#;
            conn.execute(
                &format!("ALTER TABLE gamepad_profiles ADD COLUMN dpad_mapping TEXT NOT NULL DEFAULT '{}'", default_dpad),
                []
            )?;
        }

        eprintln!("[DB::MIGRATE] Migration complete");
        Ok(())
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
        scroll_settings_json: &str,
        dpad_mapping_json: &str,
    ) -> SqliteResult<usize> {
        eprintln!("[DB::SAVE_PROFILE] Saving profile: {}", name);
        let conn = self.conn.lock().unwrap();
        let now = Utc::now().timestamp_millis();

        let mut stmt = conn.prepare("SELECT COUNT(*) FROM gamepad_profiles WHERE name = ?")?;

        let exists: bool = stmt.exists(rusqlite::params![name])?;
        drop(stmt);

        if exists {
            conn.execute(
                "UPDATE gamepad_profiles SET description = ?, sensitivity = ?, dead_zone = ?, acceleration = ?, button_map = ?, axis_map = ?, enabled_features = ?, scroll_settings = ?, dpad_mapping = ?, updated_at = ? WHERE name = ?",
                rusqlite::params![description, sensitivity as f64, dead_zone as f64, acceleration as f64, button_map_json, axis_map_json, enabled_features_json, scroll_settings_json, dpad_mapping_json, now, name],
            )
        } else {
            conn.execute(
                "INSERT INTO gamepad_profiles (name, description, sensitivity, dead_zone, acceleration, button_map, axis_map, enabled_features, scroll_settings, dpad_mapping, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
                rusqlite::params![name, description, sensitivity as f64, dead_zone as f64, acceleration as f64, button_map_json, axis_map_json, enabled_features_json, scroll_settings_json, dpad_mapping_json, now, now],
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
            "SELECT name, description, sensitivity, dead_zone, acceleration, button_map, axis_map, enabled_features, scroll_settings, dpad_mapping FROM gamepad_profiles ORDER BY created_at DESC"
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
                "scroll_settings": serde_json::from_str::<serde_json::Value>(&row.get::<_, String>(8)?).unwrap_or_default(),
                "dpad_mapping": serde_json::from_str::<serde_json::Value>(&row.get::<_, String>(9)?).unwrap_or_default(),
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
