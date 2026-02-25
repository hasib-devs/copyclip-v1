mod commands;
mod db;
mod gamepad;
mod gamepad_manager;
mod models;
mod scroll;

use db::DatabaseService;
use gamepad_manager::GamepadManager;
use std::sync::Arc;
use tauri::Manager;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

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
            eprintln!("[AppSetup] Creating database at: {:?}", db_path);
            let db = match DatabaseService::new(db_path) {
                Ok(db) => {
                    log::info!("Database initialized successfully");
                    eprintln!("[AppSetup] Database created and initialized");
                    Arc::new(db)
                }
                Err(e) => {
                    log::error!("Failed to initialize database: {}", e);
                    eprintln!("[AppSetup] Failed to initialize database: {}", e);
                    return Err(format!("Failed to initialize database: {}", e).into());
                }
            };

            // Store database service in app state
            eprintln!("[AppSetup] Managing Arc<DatabaseService> in Tauri state");
            app_handle.manage(db.clone());

            // Initialize gamepad manager
            eprintln!("[AppSetup] Initializing GamepadManager...");
            match GamepadManager::new() {
                Ok(gamepad_manager) => {
                    // Set database for profile persistence
                    eprintln!(
                        "[AppSetup] Setting database on GamepadManager (will load profiles)..."
                    );
                    gamepad_manager.set_database(db);

                    // Start listening for gamepads immediately
                    eprintln!("[AppSetup] Starting gamepad listener on app startup...");
                    match gamepad_manager.start() {
                        Ok(_) => {
                            eprintln!("[AppSetup] Gamepad listener started successfully");
                        }
                        Err(e) => {
                            eprintln!("[AppSetup] Failed to start gamepad listener: {}", e);
                        }
                    }

                    eprintln!("[AppSetup] Managing GamepadManager in Tauri state");
                    app_handle.manage(gamepad_manager);
                    log::info!("Gamepad manager initialized successfully");
                    eprintln!("[AppSetup] Gamepad manager fully initialized");
                }
                Err(e) => {
                    log::error!("Failed to initialize gamepad manager: {}", e);
                    eprintln!("[AppSetup] Failed to initialize gamepad manager: {}", e);
                }
            }

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::init_database,
            commands::save_clipboard_item,
            commands::get_clipboard_items,
            commands::get_clipboard_item,
            commands::update_clipboard_item,
            commands::delete_clipboard_item,
            commands::clear_clipboard_history,
            commands::get_clipboard_count,
            commands::load_initial_history,
            commands::start_gamepad,
            commands::stop_gamepad,
            commands::get_gamepads,
            commands::get_gamepad,
            commands::get_gamepad_profiles,
            commands::save_gamepad_profile,
            commands::delete_gamepad_profile,
            commands::set_active_gamepad_profile,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
