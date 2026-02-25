use crate::db::DatabaseService;
use crate::gamepad::{
    Gamepad, GamepadAxisIndex, GamepadButton, GamepadButtonIndex, GamepadProfile,
};
use enigo::{Enigo, MouseControllable};
use gilrs::{Axis, Event, EventType, Gilrs, GilrsBuilder};
use serde_json;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

/// Main gamepad manager - handles all connected gamepads
pub struct GamepadManager {
    gamepads: Arc<Mutex<HashMap<usize, Gamepad>>>,
    profiles: Arc<Mutex<HashMap<String, GamepadProfile>>>,
    active_profile: Arc<Mutex<String>>,
    running: Arc<Mutex<bool>>,
    gilrs: Arc<Mutex<Option<Gilrs>>>,
    db: Arc<Mutex<Option<Arc<DatabaseService>>>>,
}

impl GamepadManager {
    pub fn new() -> Result<Self, String> {
        // Initialize gilrs with custom settings
        let gilrs = GilrsBuilder::new()
            .with_default_filters(true)
            .build()
            .map_err(|e| format!("Failed to initialize gilrs: {}", e))?;

        // Create default profile in memory
        let mut default_profiles = HashMap::new();
        default_profiles.insert("Default".to_string(), GamepadProfile::default());

        Ok(Self {
            gamepads: Arc::new(Mutex::new(HashMap::new())),
            profiles: Arc::new(Mutex::new(default_profiles)),
            active_profile: Arc::new(Mutex::new("Default".to_string())),
            running: Arc::new(Mutex::new(false)),
            gilrs: Arc::new(Mutex::new(Some(gilrs))),
            db: Arc::new(Mutex::new(None)),
        })
    }

    /// Set the database service for profile persistence
    pub fn set_database(&self, db: Arc<DatabaseService>) {
        eprintln!("[GamepadManager::set_database] Setting up database for profile persistence...");
        *self.db.lock().unwrap() = Some(db);
        // Load profiles from database on initialization
        eprintln!("[GamepadManager::set_database] Calling load_profiles_from_db...");
        self.load_profiles_from_db();
    }

    /// Load profiles from database into memory cache
    fn load_profiles_from_db(&self) {
        eprintln!("[GamepadManager::load_profiles_from_db] Starting profile load...");
        if let Some(ref db) = *self.db.lock().unwrap() {
            eprintln!("[GamepadManager::load_profiles_from_db] Database is available, querying profiles...");
            match db.get_gamepad_profiles() {
                Ok(profiles) => {
                    eprintln!(
                        "[GamepadManager::load_profiles_from_db] Database returned {} profiles",
                        profiles.len()
                    );
                    if profiles.is_empty() {
                        eprintln!("[GamepadManager::load_profiles_from_db] No profiles in database (fresh install or no saved profiles)");
                    } else {
                        for profile_json in &profiles {
                            eprintln!(
                                "[GamepadManager::load_profiles_from_db] Profile JSON: {:?}",
                                profile_json
                            );
                        }
                    }
                    let mut profiles_map = self.profiles.lock().unwrap();

                    // Keep the default profile that was created on init
                    let default_profile = profiles_map.get("Default").cloned();
                    profiles_map.clear();

                    // Restore default profile
                    if let Some(default) = default_profile {
                        profiles_map.insert("Default".to_string(), default);
                    }

                    for profile_json in profiles {
                        match serde_json::from_value::<GamepadProfile>(profile_json.clone()) {
                            Ok(profile) => {
                                eprintln!("[GamepadManager::load_profiles_from_db] Successfully deserialized profile: {}", profile.name);
                                profiles_map.insert(profile.name.clone(), profile);
                            }
                            Err(e) => {
                                eprintln!("[GamepadManager::load_profiles_from_db] Failed to deserialize profile: {}", e);
                            }
                        }
                    }
                    eprintln!(
                        "[GamepadManager::load_profiles_from_db] Total profiles loaded into memory: {}",
                        profiles_map.len()
                    );
                }
                Err(e) => {
                    eprintln!(
                        "[GamepadManager::load_profiles_from_db] Failed to load profiles from database: {}",
                        e
                    );
                }
            }
        } else {
            eprintln!("[GamepadManager::load_profiles_from_db] Database not available yet!");
        }
    }

    /// Start listening to gamepad input
    pub fn start(&self) -> Result<(), String> {
        eprintln!("[GamepadManager::start] Starting gamepad listener...");
        let is_running = *self.running.lock().unwrap();
        if is_running {
            eprintln!("[GamepadManager::start] Gamepad listener already running");
            return Err("Gamepad listener already running".to_string());
        }

        eprintln!("[GamepadManager::start] Spawning gamepad polling thread...");
        *self.running.lock().unwrap() = true;

        let gamepads = self.gamepads.clone();
        let running = self.running.clone();
        let gilrs_ref = self.gilrs.clone();

        thread::spawn(move || {
            let mut gilrs_instance = match gilrs_ref.lock().unwrap().take() {
                Some(g) => g,
                None => return,
            };

            log::info!("Gamepad listener started");

            let mut last_button_state: HashMap<(usize, GamepadButtonIndex), bool> = HashMap::new();

            loop {
                if !*running.lock().unwrap() {
                    log::info!("Gamepad listener stopped");
                    *gilrs_ref.lock().unwrap() = Some(gilrs_instance);
                    break;
                }

                // Process all gamepad events
                while let Some(Event { id, event, .. }) = gilrs_instance.next_event() {
                    match event {
                        EventType::Connected => {
                            let name = gilrs_instance.gamepad(id).name().to_string();
                            let gamepad = Gamepad::new(name.clone(), id.into());
                            gamepads.lock().unwrap().insert(id.into(), gamepad);
                            log::info!("Gamepad connected: {:?} - {}", id, name);
                        }
                        EventType::Disconnected => {
                            gamepads.lock().unwrap().remove(&(id.into()));
                            log::info!("Gamepad disconnected: {:?}", id);
                        }
                        EventType::AxisChanged(axis, value, _) => {
                            if let Some(gamepad) = gamepads.lock().unwrap().get_mut(&(id.into())) {
                                // Map gilrs axis to standard gamepad axis
                                if let Some(gamepad_axis) = Self::map_axis_to_gamepad(axis) {
                                    gamepad.set_axis(gamepad_axis, value);
                                    gamepad.update_timestamp();
                                }
                            }
                        }
                        EventType::ButtonPressed(button, _) => {
                            if let Some(gamepad) = gamepads.lock().unwrap().get_mut(&(id.into())) {
                                if let Some(gamepad_button) = Self::map_button_to_gamepad(button) {
                                    let btn = GamepadButton {
                                        pressed: true,
                                        touched: true,
                                        value: 1.0,
                                    };
                                    gamepad.set_button(gamepad_button, btn);
                                    gamepad.update_timestamp();

                                    // Track state for edge detection
                                    let key = (id.into(), gamepad_button);
                                    last_button_state.insert(key, true);
                                }
                            }
                        }
                        EventType::ButtonReleased(button, _) => {
                            if let Some(gamepad) = gamepads.lock().unwrap().get_mut(&(id.into())) {
                                if let Some(gamepad_button) = Self::map_button_to_gamepad(button) {
                                    let btn = GamepadButton {
                                        pressed: false,
                                        touched: false,
                                        value: 0.0,
                                    };
                                    gamepad.set_button(gamepad_button, btn);
                                    gamepad.update_timestamp();

                                    let key = (id.into(), gamepad_button);
                                    last_button_state.insert(key, false);
                                }
                            }
                        }
                        _ => {}
                    }
                }

                // Process gamepad input for mouse/keyboard control
                Self::process_gamepad_input(&gamepads, &mut last_button_state);
                thread::sleep(Duration::from_millis(16)); // ~60 FPS
            }
        });

        eprintln!(
            "[GamepadManager::start] Gamepad listener thread spawned successfully, returning Ok"
        );
        Ok(())
    }

    /// Stop listening to gamepad input
    pub fn stop(&self) {
        eprintln!("[GamepadManager::stop] Stopping gamepad listener...");
        *self.running.lock().unwrap() = false;
    }

    /// Get all connected gamepads
    pub fn get_gamepads(&self) -> Result<Vec<Gamepad>, String> {
        let gamepads = self.gamepads.lock().unwrap();
        let mut list: Vec<_> = gamepads.values().cloned().collect();
        list.sort_by_key(|g| g.index);
        Ok(list)
    }

    /// Get specific gamepad by index
    pub fn get_gamepad(&self, index: usize) -> Result<Option<Gamepad>, String> {
        Ok(self.gamepads.lock().unwrap().get(&index).cloned())
    }

    /// Get currently active profile
    // pub fn get_active_profile(&self) -> Result<GamepadProfile, String> {
    //     let profile_name = self.active_profile.lock().unwrap().clone();
    //     Ok(self
    //         .profiles
    //         .lock()
    //         .unwrap()
    //         .get(&profile_name)
    //         .cloned()
    //         .unwrap_or_default())
    // }

    /// Set active profile
    pub fn set_active_profile(&self, profile_name: String) -> Result<(), String> {
        if !self.profiles.lock().unwrap().contains_key(&profile_name) {
            return Err(format!("Profile '{}' not found", profile_name));
        }
        *self.active_profile.lock().unwrap() = profile_name;
        Ok(())
    }

    /// Save a gamepad profile
    pub fn save_profile(&self, profile: GamepadProfile) -> Result<(), String> {
        // Save to memory
        self.profiles
            .lock()
            .unwrap()
            .insert(profile.name.clone(), profile.clone());

        // Also save to database if available
        if let Some(ref db) = *self.db.lock().unwrap() {
            // Serialize complex fields to JSON
            let button_map_json =
                serde_json::to_string(&profile.button_map).unwrap_or_else(|_| "{}".to_string());
            let axis_map_json =
                serde_json::to_string(&profile.axis_map).unwrap_or_else(|_| "{}".to_string());
            let enabled_features_json = serde_json::to_string(&profile.enabled_features)
                .unwrap_or_else(|_| "{}".to_string());

            let _ = db
                .save_gamepad_profile(
                    &profile.name,
                    &profile.description,
                    profile.sensitivity,
                    profile.dead_zone,
                    profile.acceleration,
                    &button_map_json,
                    &axis_map_json,
                    &enabled_features_json,
                )
                .map_err(|e| {
                    eprintln!("[GamepadManager] Failed to save profile to database: {}", e);
                    format!("Failed to save profile to database: {}", e)
                });
        }

        Ok(())
    }

    /// Delete a gamepad profile
    pub fn delete_profile(&self, profile_name: &str) -> Result<(), String> {
        if profile_name == "Default" {
            return Err("Cannot delete default profile".to_string());
        }

        // Delete from memory
        self.profiles.lock().unwrap().remove(profile_name);

        // Also delete from database if available
        if let Some(ref db) = *self.db.lock().unwrap() {
            let _ = db.delete_gamepad_profile(profile_name).map_err(|e| {
                eprintln!(
                    "[GamepadManager] Failed to delete profile from database: {}",
                    e
                );
                format!("Failed to delete profile from database: {}", e)
            });
        }

        Ok(())
    }

    /// Get all profiles
    pub fn get_profiles(&self) -> Result<Vec<GamepadProfile>, String> {
        eprintln!("[GamepadManager::get_profiles] Called");
        let profiles_locked = self.profiles.lock().unwrap();
        let current_count = profiles_locked.len();
        eprintln!(
            "[GamepadManager::get_profiles] Current profiles in memory: {}",
            current_count
        );

        if current_count == 0 {
            eprintln!(
                "[GamepadManager::get_profiles] Cache is empty, checking if DB is available..."
            );
            drop(profiles_locked); // Release lock before trying to reload
            if self.db.lock().unwrap().is_some() {
                eprintln!("[GamepadManager::get_profiles] Loading from database...");
                self.load_profiles_from_db();
            }
        }

        let profiles = self.profiles.lock().unwrap();
        eprintln!(
            "[GamepadManager::get_profiles] Returning {} profiles",
            profiles.len()
        );
        Ok(profiles.values().cloned().collect())
    }

    // Helper: Map gilrs button to standard gamepad button
    fn map_button_to_gamepad(button: gilrs::Button) -> Option<GamepadButtonIndex> {
        use gilrs::Button::*;
        Some(match button {
            South => GamepadButtonIndex::South,
            East => GamepadButtonIndex::East,
            West => GamepadButtonIndex::West,
            North => GamepadButtonIndex::North,
            _ => return None,
        })
    }

    // Helper: Map gilrs axis to standard gamepad axis
    fn map_axis_to_gamepad(axis: Axis) -> Option<GamepadAxisIndex> {
        use gilrs::Axis::*;
        Some(match axis {
            LeftStickX => GamepadAxisIndex::LeftStickX,
            LeftStickY => GamepadAxisIndex::LeftStickY,
            RightStickX => GamepadAxisIndex::RightStickX,
            RightStickY => GamepadAxisIndex::RightStickY,
            _ => return None,
        })
    }

    // Process gamepad input for mouse/keyboard control
    fn process_gamepad_input(
        gamepads: &Arc<Mutex<HashMap<usize, Gamepad>>>,
        button_state: &mut HashMap<(usize, GamepadButtonIndex), bool>,
    ) {
        let g = gamepads.lock().unwrap();
        if g.is_empty() {
            return;
        }

        // Use first connected gamepad for mouse control
        if let Some(gamepad) = g.values().next() {
            if !gamepad.connected {
                return;
            }

            // Mouse control with left stick
            let stick_x = gamepad
                .get_axis(GamepadAxisIndex::LeftStickX)
                .unwrap_or(0.0);
            let stick_y = gamepad
                .get_axis(GamepadAxisIndex::LeftStickY)
                .unwrap_or(0.0);

            if stick_x.abs() > 0.05 || stick_y.abs() > 0.05 {
                let dx = (stick_x * 10.0) as i32;
                let dy = -(stick_y * 10.0) as i32; // Invert Y axis for correct mouse movement

                let mut enigo = Enigo::new();
                let _ = enigo.mouse_move_relative(dx, dy);
            }

            // Scroll control with right stick
            let stick_x_right = gamepad
                .get_axis(GamepadAxisIndex::RightStickX)
                .unwrap_or(0.0);
            let stick_y_right = gamepad
                .get_axis(GamepadAxisIndex::RightStickY)
                .unwrap_or(0.0);

            if stick_x_right.abs() > 0.05 || stick_y_right.abs() > 0.05 {
                eprintln!(
                    "[Scroll] Right stick - X: {:.2}, Y: {:.2}",
                    stick_x_right, stick_y_right
                );

                // Calculate scroll amount
                // Positive Y = down scroll, Negative Y = up scroll
                let vertical_scroll = (stick_y_right * 10.0) as i32;
                let horizontal_scroll = (stick_x_right * 10.0) as i32;

                eprintln!(
                    "[Scroll] Vertical: {}, Horizontal: {}",
                    vertical_scroll, horizontal_scroll
                );

                // TODO: Implement scroll via platform-specific API
                // For now, just log to verify detection
            }

            // Middle click via LB button
            let lb_pressed = gamepad
                .get_button(GamepadButtonIndex::LB)
                .map(|b| b.pressed)
                .unwrap_or(false);

            let lb_key = (gamepad.index, GamepadButtonIndex::LB);
            let lb_was_pressed = button_state.get(&lb_key).copied().unwrap_or(false);

            if lb_pressed && !lb_was_pressed {
                // Rising edge: LB just pressed
                eprintln!("[Click] Middle Click (LB)");
                let mut enigo = Enigo::new();
                let _ = enigo.mouse_down(enigo::MouseButton::Middle);
                thread::sleep(Duration::from_millis(10));
                let _ = enigo.mouse_up(enigo::MouseButton::Middle);
            }
            button_state.insert(lb_key, lb_pressed);

            // Double click via RB button
            let rb_pressed = gamepad
                .get_button(GamepadButtonIndex::RB)
                .map(|b| b.pressed)
                .unwrap_or(false);

            let rb_key = (gamepad.index, GamepadButtonIndex::RB);
            let rb_was_pressed = button_state.get(&rb_key).copied().unwrap_or(false);

            if rb_pressed && !rb_was_pressed {
                // Rising edge: RB just pressed
                eprintln!("[Click] Double Click (RB)");
                let mut enigo = Enigo::new();

                // First click
                let _ = enigo.mouse_down(enigo::MouseButton::Left);
                thread::sleep(Duration::from_millis(10));
                let _ = enigo.mouse_up(enigo::MouseButton::Left);

                // Second click
                thread::sleep(Duration::from_millis(20));
                let _ = enigo.mouse_down(enigo::MouseButton::Left);
                thread::sleep(Duration::from_millis(10));
                let _ = enigo.mouse_up(enigo::MouseButton::Left);
            }
            button_state.insert(rb_key, rb_pressed);

            // Right trigger = left click
            let rt = gamepad
                .get_button(GamepadButtonIndex::RT)
                .map(|b| b.pressed)
                .unwrap_or(false);

            if rt {
                let mut enigo = Enigo::new();
                let _ = enigo.mouse_down(enigo::MouseButton::Left);
                thread::sleep(Duration::from_millis(10));
                let _ = enigo.mouse_up(enigo::MouseButton::Left);
            }

            // Left trigger = right click
            let lt = gamepad
                .get_button(GamepadButtonIndex::LT)
                .map(|b| b.pressed)
                .unwrap_or(false);

            if lt {
                let mut enigo = Enigo::new();
                let _ = enigo.mouse_down(enigo::MouseButton::Right);
                thread::sleep(Duration::from_millis(10));
                let _ = enigo.mouse_up(enigo::MouseButton::Right);
            }
        }
    }
}

impl Default for GamepadManager {
    fn default() -> Self {
        Self::new().expect("Failed to initialize GamepadManager")
    }
}

impl Drop for GamepadManager {
    fn drop(&mut self) {
        self.stop();
    }
}
