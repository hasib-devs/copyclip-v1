use std::path::PathBuf;
use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
use sqlx::SqlitePool;
use uuid::Uuid;

use crate::models::{ClipboardItemModel, ClipboardQueryFilter};

/**
 * Database service for clipboard history
 * Handles all database operations
 */
pub struct DatabaseService {
    pool: SqlitePool,
}

impl DatabaseService {
    /**
     * Initialize database with connection pool
     */
    pub async fn new(db_path: PathBuf) -> Result<Self, Box<dyn std::error::Error>> {
        // Create parent directory if it doesn't exist
        if let Some(parent) = db_path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        // Set up connection options
        let options = SqliteConnectOptions::new()
            .filename(&db_path)
            .create_if_missing(true);

        // Create connection pool
        let pool = SqlitePoolOptions::new()
            .max_connections(5)
            .connect_with(options)
            .await?;

        // Run migrations
        sqlx::query(
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
        )
        .execute(&pool)
        .await?;

        // Create indexes for common queries
        sqlx::query(
            "CREATE INDEX IF NOT EXISTS idx_timestamp ON clipboard_items(timestamp DESC);",
        )
        .execute(&pool)
        .await?;

        sqlx::query(
            "CREATE INDEX IF NOT EXISTS idx_is_pinned ON clipboard_items(is_pinned);",
        )
        .execute(&pool)
        .await?;

        sqlx::query("CREATE INDEX IF NOT EXISTS idx_item_type ON clipboard_items(item_type);")
            .execute(&pool)
            .await?;

        Ok(Self { pool })
    }

    /**
     * Create a new clipboard item
     */
    pub async fn create_item(&self, item: ClipboardItemModel) -> Result<(), Box<dyn std::error::Error>> {
        sqlx::query(
            r#"
            INSERT INTO clipboard_items 
            (id, content, item_type, is_pinned, timestamp, image_base64, file_paths, created_at, updated_at)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(&item.id)
        .bind(&item.content)
        .bind(&item.item_type)
        .bind(item.is_pinned)
        .bind(item.timestamp)
        .bind(&item.image_base64)
        .bind(&item.file_paths)
        .bind(item.created_at)
        .bind(item.updated_at)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    /**
     * Get item by id
     */
    pub async fn get_item(&self, id: &str) -> Result<Option<ClipboardItemModel>, Box<dyn std::error::Error>> {
        let row = sqlx::query_as::<_, (String, String, String, bool, i64, Option<String>, Option<String>, i64, i64)>(
            "SELECT id, content, item_type, is_pinned, timestamp, image_base64, file_paths, created_at, updated_at FROM clipboard_items WHERE id = ?"
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(|(id, content, item_type, is_pinned, timestamp, image_base64, file_paths, created_at, updated_at)| {
            ClipboardItemModel {
                id,
                content,
                item_type,
                is_pinned,
                timestamp,
                image_base64,
                file_paths,
                created_at,
                updated_at,
            }
        }))
    }

    /**
     * Get all items with filtering
     */
    pub async fn get_items(
        &self,
        filter: ClipboardQueryFilter,
    ) -> Result<Vec<ClipboardItemModel>, Box<dyn std::error::Error>> {
        let mut query = String::from(
            "SELECT id, content, item_type, is_pinned, timestamp, image_base64, file_paths, created_at, updated_at FROM clipboard_items WHERE 1=1"
        );

        // Add search filter
        if filter.search.is_some() {
            query.push_str(" AND content LIKE ?");
        }

        // Add type filter
        if filter.item_type.is_some() {
            query.push_str(" AND item_type = ?");
        }

        // Add pinned filter
        if let Some(is_pinned) = filter.is_pinned {
            query.push_str(&format!(" AND is_pinned = {}", if is_pinned { 1 } else { 0 }));
        }

        // Add ordering and pagination
        query.push_str(" ORDER BY is_pinned DESC, timestamp DESC LIMIT ? OFFSET ?");

        let mut sql_query = sqlx::query_as::<_, (String, String, String, bool, i64, Option<String>, Option<String>, i64, i64)>(&query);

        if let Some(search) = filter.search {
            sql_query = sql_query.bind(format!("%{}%", search));
        }

        if let Some(item_type) = filter.item_type {
            sql_query = sql_query.bind(item_type);
        }

        sql_query = sql_query.bind(filter.limit as i64).bind(filter.offset as i64);

        let rows = sql_query.fetch_all(&self.pool).await?;

        Ok(rows.into_iter().map(|(id, content, item_type, is_pinned, timestamp, image_base64, file_paths, created_at, updated_at)| {
            ClipboardItemModel {
                id,
                content,
                item_type,
                is_pinned,
                timestamp,
                image_base64,
                file_paths,
                created_at,
                updated_at,
            }
        }).collect())
    }

    /**
     * Update item (toggle pin for example)
     */
    pub async fn update_item(&self, id: &str, is_pinned: bool) -> Result<(), Box<dyn std::error::Error>> {
        let now = chrono::Utc::now().timestamp_millis();

        sqlx::query("UPDATE clipboard_items SET is_pinned = ?, updated_at = ? WHERE id = ?")
            .bind(is_pinned)
            .bind(now)
            .bind(id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    /**
     * Delete item by id
     */
    pub async fn delete_item(&self, id: &str) -> Result<(), Box<dyn std::error::Error>> {
        sqlx::query("DELETE FROM clipboard_items WHERE id = ?")
            .bind(id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    /**
     * Delete all items
     */
    pub async fn delete_all(&self) -> Result<(), Box<dyn std::error::Error>> {
        sqlx::query("DELETE FROM clipboard_items")
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    /**
     * Get item count
     */
    pub async fn count_items(&self) -> Result<i64, Box<dyn std::error::Error>> {
        let (count,): (i64,) = sqlx::query_as("SELECT COUNT(*) FROM clipboard_items")
            .fetch_one(&self.pool)
            .await?;

        Ok(count)
    }

    /**
     * Delete items older than specified timestamp
     */
    pub async fn delete_old_items(&self, before_timestamp: i64) -> Result<u64, Box<dyn std::error::Error>> {
        let result = sqlx::query("DELETE FROM clipboard_items WHERE timestamp < ?")
            .bind(before_timestamp)
            .execute(&self.pool)
            .await?;

        Ok(result.rows_affected())
    }

    /**
     * Enforce max items limit
     */
    pub async fn enforce_max_items(&self, max_items: i64) -> Result<(), Box<dyn std::error::Error>> {
        sqlx::query(
            r#"
            DELETE FROM clipboard_items WHERE id IN (
                SELECT id FROM clipboard_items 
                WHERE is_pinned = 0
                ORDER BY timestamp ASC 
                LIMIT (SELECT COUNT(*) - ? FROM clipboard_items WHERE is_pinned = 0)
            )
            "#,
        )
        .bind(max_items)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    /**
     * Check if item with same content exists (for deduplication)
     */
    pub async fn check_duplicate(&self, content: &str, item_type: &str) -> Result<bool, Box<dyn std::error::Error>> {
        let (count,): (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM clipboard_items WHERE content = ? AND item_type = ? ORDER BY timestamp DESC LIMIT 1"
        )
        .bind(content)
        .bind(item_type)
        .fetch_one(&self.pool)
        .await?;

        Ok(count > 0)
    }
}
