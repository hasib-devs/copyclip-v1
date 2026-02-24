use chrono::Utc;
use serde::{Deserialize, Serialize};

/**
 * ClipboardItem entity - represents a clipboard history item in the database
 */
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClipboardItemModel {
    pub id: String,
    pub content: String,
    pub item_type: String, // 'text', 'image', 'html', 'file'
    pub is_pinned: bool,
    pub timestamp: i64,
    pub image_base64: Option<String>,
    pub file_paths: Option<String>, // JSON array
    pub created_at: i64,
    pub updated_at: i64,
}

impl ClipboardItemModel {
    pub fn new(
        id: String,
        content: String,
        item_type: String,
        image_base64: Option<String>,
        file_paths: Option<String>,
    ) -> Self {
        let now = Utc::now().timestamp_millis();
        Self {
            id,
            content,
            item_type,
            is_pinned: false,
            timestamp: now,
            image_base64,
            file_paths,
            created_at: now,
            updated_at: now,
        }
    }
}

/**
 * Database-agnostic query filters
 */
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClipboardQueryFilter {
    pub search: Option<String>,
    pub item_type: Option<String>,
    pub is_pinned: Option<bool>,
    pub limit: u64,
    pub offset: u64,
}

impl Default for ClipboardQueryFilter {
    fn default() -> Self {
        Self {
            search: None,
            item_type: None,
            is_pinned: None,
            limit: 50,
            offset: 0,
        }
    }
}
