use crate::db::DatabaseService;
use crate::gamepad::{Gamepad, GamepadProfile};
use crate::gamepad_manager::GamepadManager;
use crate::models::{ClipboardItemModel, ClipboardQueryFilter};
use tauri::State;

/**
 * Initialize database (called on app startup)
 */
#[tauri::command]
pub fn init_database(_db_path: String) -> Result<String, String> {
    // Database is initialized in main setup
    Ok("Database initialized".to_string())
}

/**
 * Save clipboard item to database
 */
#[tauri::command]
pub fn save_clipboard_item(
    id: String,
    content: String,
    item_type: String,
    image_base64: Option<String>,
    file_paths: Option<String>,
    db: State<'_, DatabaseService>,
) -> Result<bool, String> {
    eprintln!(
        "[SAVE] Attempting to save item with id: {}, type: {}, content length: {}",
        id,
        item_type,
        content.len()
    );

    // Check for duplicate
    eprintln!("[SAVE] Checking for duplicates...");
    let is_duplicate = match db.check_duplicate(&content, &item_type) {
        Ok(is_dup) => {
            eprintln!("[SAVE] Duplicate check result: {}", is_dup);
            is_dup
        }
        Err(e) => {
            eprintln!("[SAVE] ERROR in duplicate check: {}", e);
            return Err(format!("Failed to check duplicate: {}", e));
        }
    };

    eprintln!("[SAVE] Is duplicate: {}", is_duplicate);

    if is_duplicate {
        eprintln!("[SAVE] Item is duplicate, skipping save");
        return Ok(false); // Duplicate item, not saved
    }

    let item = ClipboardItemModel::new(
        id.clone(),
        content.clone(),
        item_type.clone(),
        image_base64.clone(),
        file_paths.clone(),
    );

    eprintln!("[SAVE] Creating item model: {:?}", id);

    match db.create_item(item) {
        Ok(rows) => {
            eprintln!("[SAVE] Item created successfully, rows affected: {}", rows);
        }
        Err(e) => {
            eprintln!("[SAVE] ERROR creating item: {}", e);
            return Err(format!("Failed to create item: {}", e));
        }
    }

    eprintln!("[SAVE] Enforcing max items limit...");
    // Enforce max items limit (100)
    match db.enforce_max_items(100) {
        Ok(deleted) => {
            eprintln!("[SAVE] Enforced max items, deleted {} old items", deleted);
        }
        Err(e) => {
            eprintln!("[SAVE] ERROR enforcing max items: {}", e);
        }
    }

    eprintln!("[SAVE] Item saved successfully");

    Ok(true) // Item saved successfully
}

/**
 * Get clipboard items with filters
 */
#[tauri::command]
pub fn get_clipboard_items(
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

    db.get_items(filter).map_err(|e| e.to_string())
}

/**
 * Get single item by id
 */
#[tauri::command]
pub fn get_clipboard_item(
    id: String,
    db: State<'_, DatabaseService>,
) -> Result<Option<ClipboardItemModel>, String> {
    db.get_item(&id).map_err(|e| e.to_string())
}

/**
 * Update item (toggle pin status)
 */
#[tauri::command]
pub fn update_clipboard_item(
    id: String,
    is_pinned: bool,
    db: State<'_, DatabaseService>,
) -> Result<bool, String> {
    db.update_item(&id, is_pinned).map_err(|e| e.to_string())?;
    Ok(true)
}

/**
 * Delete single item
 */
#[tauri::command]
pub fn delete_clipboard_item(id: String, db: State<'_, DatabaseService>) -> Result<bool, String> {
    eprintln!("[DELETE] ========================================");
    eprintln!("[DELETE] Attempting to delete item with id: {}", id);
    eprintln!("[DELETE] ========================================");

    match db.delete_item(&id) {
        Ok(rows_affected) => {
            eprintln!("[DELETE] Successfully executed delete query");
            eprintln!("[DELETE] Rows affected: {}", rows_affected);
            if rows_affected == 0 {
                eprintln!("[DELETE] WARNING: No rows were deleted. Item may not exist in DB");
            }
            eprintln!("[DELETE] ========================================");
            Ok(true)
        }
        Err(e) => {
            eprintln!("[DELETE] ERROR: Failed to delete item {}: {}", id, e);
            eprintln!("[DELETE] ========================================");
            Err(format!("Failed to delete item: {}", e))
        }
    }
}

/**
 * Clear all clipboard history
 */
#[tauri::command]
pub fn clear_clipboard_history(db: State<'_, DatabaseService>) -> Result<bool, String> {
    db.delete_all().map_err(|e| e.to_string())?;
    log::info!("Cleared all clipboard history");
    Ok(true)
}

/**
 * Get total item count
 */
#[tauri::command]
pub fn get_clipboard_count(db: State<'_, DatabaseService>) -> Result<i64, String> {
    db.count_items().map_err(|e| e.to_string())
}

/**
 * Load all items on app startup
 */
#[tauri::command]
pub fn load_initial_history(
    db: State<'_, DatabaseService>,
) -> Result<Vec<ClipboardItemModel>, String> {
    let filter = ClipboardQueryFilter {
        search: None,
        item_type: None,
        is_pinned: None,
        limit: 100,
        offset: 0,
    };

    db.get_items(filter).map_err(|e| e.to_string())
}

// ============= GAMEPAD COMMANDS =============

/**
 * Start gamepad listener
 */
#[tauri::command]
pub fn start_gamepad(gamepad: State<'_, GamepadManager>) -> Result<String, String> {
    gamepad.start()?;
    Ok("Gamepad listener started".to_string())
}

/**
 * Stop gamepad listener
 */
#[tauri::command]
pub fn stop_gamepad(gamepad: State<'_, GamepadManager>) -> Result<String, String> {
    gamepad.stop();
    Ok("Gamepad listener stopped".to_string())
}

/**
 * Get all connected gamepads
 */
#[tauri::command]
pub fn get_gamepads(gamepad: State<'_, GamepadManager>) -> Result<Vec<Gamepad>, String> {
    gamepad.get_gamepads()
}

/**
 * Get specific gamepad by index
 */
#[tauri::command]
pub fn get_gamepad(
    index: usize,
    gamepad: State<'_, GamepadManager>,
) -> Result<Option<Gamepad>, String> {
    gamepad.get_gamepad(index)
}

/**
 * Get all gamepad profiles
 */
#[tauri::command]
pub fn get_gamepad_profiles(
    gamepad: State<'_, GamepadManager>,
) -> Result<Vec<GamepadProfile>, String> {
    gamepad.get_profiles()
}

/**
 * Save a gamepad profile
 */
#[tauri::command]
pub fn save_gamepad_profile(
    profile: GamepadProfile,
    gamepad: State<'_, GamepadManager>,
) -> Result<String, String> {
    gamepad.save_profile(profile)?;
    Ok("Profile saved".to_string())
}

/**
 * Delete a gamepad profile
 */
#[tauri::command]
pub fn delete_gamepad_profile(
    profile_name: String,
    gamepad: State<'_, GamepadManager>,
) -> Result<String, String> {
    gamepad.delete_profile(&profile_name)?;
    Ok("Profile deleted".to_string())
}

/**
 * Set active gamepad profile
 */
#[tauri::command]
pub fn set_active_gamepad_profile(
    profile_name: String,
    gamepad: State<'_, GamepadManager>,
) -> Result<String, String> {
    gamepad.set_active_profile(profile_name)?;
    Ok("Profile activated".to_string())
}
