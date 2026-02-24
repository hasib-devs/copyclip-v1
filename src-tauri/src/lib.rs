mod commands;
mod db;
mod models;

use db::DatabaseService;
use std::path::PathBuf;
use tauri::Manager;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_clipboard::init())
        .setup(|app| {
            // Initialize database on app startup
            let app_handle = app.handle();
            let app_dir = app
                .path_resolver()
                .app_data_dir()
                .expect("failed to get app data directory");

            // Create database path
            let db_path = app_dir.join("copyclip.db");

            // Spawn async task to initialize database
            let db_path_clone = db_path.clone();
            tauri::async_runtime::spawn(async move {
                match DatabaseService::new(db_path_clone).await {
                    Ok(db) => {
                        // Store database service in app state
                        let handle = app_handle;
                        handle.manage(db);
                        log::info!("Database initialized successfully");
                    }
                    Err(e) => {
                        log::error!("Failed to initialize database: {}", e);
                    }
                }
            });

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
