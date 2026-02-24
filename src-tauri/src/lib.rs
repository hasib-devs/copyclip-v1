mod commands;
mod db;
mod models;

use db::DatabaseService;
use tauri::Manager;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // Initialize logger
    env_logger::Builder::from_default_env()
        .format_timestamp_millis()
        .init();

    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_clipboard::init())
        .setup(|app| {
            // Initialize database on app startup
            let app_handle = app.handle();

            // Get app data directory
            let app_data_dir = if let Some(project_dirs) =
                directories::ProjectDirs::from("dev", "hasib", "copyclip")
            {
                project_dirs.data_dir().to_path_buf()
            } else {
                // Fallback to current directory if ProjectDirs fails
                std::env::current_dir().expect("failed to get current directory")
            };

            // Create database path
            let db_path = app_data_dir.join("copyclip.db");

            // Initialize database synchronously (rusqlite is sync)
            match DatabaseService::new(db_path) {
                Ok(db) => {
                    // Store database service in app state
                    app_handle.manage(db);
                    log::info!("Database initialized successfully");
                }
                Err(e) => {
                    log::error!("Failed to initialize database: {}", e);
                }
            }

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            greet,
            commands::init_database,
            commands::save_clipboard_item,
            commands::get_clipboard_items,
            commands::get_clipboard_item,
            commands::update_clipboard_item,
            commands::delete_clipboard_item,
            commands::clear_clipboard_history,
            commands::get_clipboard_count,
            commands::load_initial_history,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
