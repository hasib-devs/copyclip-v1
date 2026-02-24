use tauri::State;
use crate::db::DatabaseService;
use crate::models::{ClipboardItemModel, ClipboardQueryFilter};

/**
 * Initialize database (called on app startup)
 */
#[tauri::command]
pub async fn init_database(db_path: String) -> Result<String, String> {
    // Database is initialized in main setup
    Ok("Database initialized".to_string())
}

/**
 * Save clipboard item to database
 */
#[tauri::command]
pub async fn save_clipboard_item(
    id: String,
    content: String,
    item_type: String,
    image_base64: Option<String>,
    file_paths: Option<String>,
    db: State<'_, DatabaseService>,
) -> Result<bool, String> {
    // Check for duplicate
    let is_duplicate = db
        .check_duplicate(&content, &item_type)
        .await
        .map_err(|e| e.to_string())?;

    if is_duplicate {
        return Ok(false); // Duplicate item, not saved
    }

    let item = ClipboardItemModel::new(id, content, item_type, image_base64, file_paths);

    db.create_item(item)
        .await
        .map_err(|e| e.to_string())?;

    // Enforce max items limit (100)
    db.enforce_max_items(100)
        .await
        .map_err(|e| e.to_string())?;

    Ok(true) // Item saved successfully
}

/**
 * Get clipboard items with filters
 */
#[tauri::command]
pub async fn get_clipboard_items(
    search: Option<String>,
    item_type: Option<String>,
    is_pinned: Option<bool>,
    limit: u64,
    offset: u64,
    db: State<'_, DatabaseService>,
) -> Result<Vec<ClipboardItemModel>, String> {
    let filter = ClipboardQueryFilter {
        search,
        item_type,
        is_pinned,
        limit,
        offset,
    };

    db.get_items(filter)
        .await
        .map_err(|e| e.to_string())
}

/**
 * Get single item by id
 */
#[tauri::command]
pub async fn get_clipboard_item(
    id: String,
    db: State<'_, DatabaseService>,
) -> Result<Option<ClipboardItemModel>, String> {
    db.get_item(&id)
        .await
        .map_err(|e| e.to_string())
}

/**
 * Update item (toggle pin status)
 */
#[tauri::command]
pub async fn update_clipboard_item(
    id: String,
    is_pinned: bool,
    db: State<'_, DatabaseService>,
) -> Result<bool, String> {
    db.update_item(&id, is_pinned)
        .await
        .map_err(|e| e.to_string())?;
    Ok(true)
}

/**
 * Delete single item
 */
#[tauri::command]
pub async fn delete_clipboard_item(
    id: String,
    db: State<'_, DatabaseService>,
) -> Result<bool, String> {
    db.delete_item(&id)
        .await
        .map_err(|e| e.to_string())?;
    Ok(true)
}

/**
 * Clear all clipboard history
 */
#[tauri::command]
pub async fn clear_clipboard_history(
    db: State<'_, DatabaseService>,
) -> Result<bool, String> {
    db.delete_all()
        .await
        .map_err(|e| e.to_string())?;
    Ok(true)
}

/**
 * Get total item count
 */
#[tauri::command]
pub async fn get_clipboard_count(
    db: State<'_, DatabaseService>,
) -> Result<i64, String> {
    db.count_items()
        .await
        .map_err(|e| e.to_string())
}

/**
 * Load all items on app startup
 */
#[tauri::command]
pub async fn load_initial_history(
    db: State<'_, DatabaseService>,
) -> Result<Vec<ClipboardItemModel>, String> {
    let filter = ClipboardQueryFilter {
        search: None,
        item_type: None,
        is_pinned: None,
        limit: 100,
        offset: 0,
    };

    db.get_items(filter)
        .await
        .map_err(|e| e.to_string())
}
