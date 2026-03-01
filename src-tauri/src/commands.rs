use crate::db::DatabaseService;
use crate::gamepad::{Gamepad, GamepadProfile};
use crate::gamepad_manager::GamepadManager;
use std::sync::Arc;
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
 * Load all items on app startup
 */
#[tauri::command]
pub fn load_initial_history(
    db: State<'_, Arc<DatabaseService>>,
) -> Result<Vec<serde_json::Value>, String> {
    // Return empty for now since clipboard is removed
    Ok(vec![])
}

// ============= GAMEPAD COMMANDS =============

/**
 * Start gamepad listener
 */
#[tauri::command]
pub fn start_gamepad(gamepad: State<'_, GamepadManager>) -> Result<String, String> {
    eprintln!("[Commands::start_gamepad] Calling gamepad.start()...");
    gamepad.start().map(|_| {
        eprintln!("[Commands::start_gamepad] Successfully started");
        "Gamepad listener started".to_string()
    })
}

/**
 * Stop gamepad listener
 */
#[tauri::command]
pub fn stop_gamepad(gamepad: State<'_, GamepadManager>) -> Result<String, String> {
    eprintln!("[Commands::stop_gamepad] Calling gamepad.stop()...");
    gamepad.stop();
    eprintln!("[Commands::stop_gamepad] Successfully stopped");
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
    eprintln!(
        "[Commands::save_gamepad_profile] Saving profile: {}",
        profile.name
    );
    gamepad.save_profile(profile).map(|_| {
        eprintln!("[Commands::save_gamepad_profile] Profile saved successfully");
        "Profile saved".to_string()
    })
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

/**
 * Get current gamepad mode (Normal/Motion/Hotkey)
 */
#[tauri::command]
pub fn get_gamepad_mode(gamepad: State<'_, GamepadManager>) -> Result<String, String> {
    let mode_str = gamepad
        .get_current_mode()
        .map(|mode| format!("{:?}", mode))
        .unwrap_or_else(|_| "Unknown".to_string());
    Ok(mode_str)
}

/**
 * Get all gamepad keybindings
 */
#[tauri::command]
pub fn get_gamepad_keybindings() -> Result<Vec<(String, String)>, String> {
    // TODO: Load from database or file
    // For now, return default keybindings
    let defaults = vec![
        ("South".to_string(), "LeftClick".to_string()),
        ("East".to_string(), "RightClick".to_string()),
        ("North".to_string(), "Key_Return".to_string()),
        ("West".to_string(), "Key_Escape".to_string()),
        ("LB".to_string(), "MiddleClick".to_string()),
        ("RB".to_string(), "Key_Space".to_string()),
        ("Select".to_string(), "SwitchModeNormal".to_string()),
        ("Start".to_string(), "SwitchModeHotkey".to_string()),
    ];
    Ok(defaults)
}

/**
 * Save gamepad keybindings
 */
#[tauri::command]
pub fn save_gamepad_keybindings(keybindings: Vec<(String, String)>) -> Result<String, String> {
    // TODO: Save to database or file
    eprintln!("[KEYBINDINGS] Saving {} keybindings", keybindings.len());
    for (button, action) in &keybindings {
        eprintln!("[KEYBINDINGS]   {} → {}", button, action);
    }
    Ok("Keybindings saved".to_string())
}

/**
 * Get gamepad settings (sensitivity, dead zone, etc.)
 */
#[tauri::command]
pub fn get_gamepad_settings() -> Result<serde_json::Value, String> {
    // TODO: Load from database or file
    // For now, return default settings
    let defaults = serde_json::json!({
        "sensitivity": 1.5,
        "deadZone": 0.1,
        "acceleration": 1.0,
        "scrollVerticalSpeed": 1.0,
        "scrollHorizontalSpeed": 1.0,
        "reverseScrollVertical": false,
        "reverseScrollHorizontal": false,
        "vibrationEnabled": true,
    });
    Ok(defaults)
}

/**
 * Save gamepad settings
 */
#[tauri::command]
pub fn save_gamepad_settings(settings: serde_json::Value) -> Result<String, String> {
    // TODO: Save to database or file
    eprintln!("[SETTINGS] Saving gamepad settings: {}", settings);
    Ok("Settings saved".to_string())
}
