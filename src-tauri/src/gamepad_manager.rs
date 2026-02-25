use crate::actions::execute_action;
use crate::db::DatabaseService;
use crate::gamepad::{
    Gamepad, GamepadAxisIndex, GamepadButton, GamepadButtonIndex, GamepadProfile,
};
use crate::modes::{get_mode_bindings, GamepadModeManager};
use crate::scroll;
use crate::types::{Action, GamepadMode, InputPattern, InputType};
use enigo::{Enigo, KeyboardControllable, MouseControllable};
use gilrs::{Axis, Event, EventType, Gilrs, GilrsBuilder};
use serde_json;
use std::collections::HashMap;
use std::panic::{catch_unwind, AssertUnwindSafe};
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
    mode_manager: Arc<Mutex<GamepadModeManager>>,
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
            mode_manager: Arc::new(Mutex::new(GamepadModeManager::new())),
        })
    }

    /// Set the database service for profile persistence
    pub fn set_database(&self, db: Arc<DatabaseService>) {
        eprintln!("[GamepadManager::set_database] Setting up database for profile persistence...");
        *self.db.lock().unwrap_or_else(|e| e.into_inner()) = Some(db);
        // Load profiles from database on initialization
        eprintln!("[GamepadManager::set_database] Calling load_profiles_from_db...");
        self.load_profiles_from_db();
    }

    /// Load profiles from database into memory cache
    fn load_profiles_from_db(&self) {
        eprintln!("[GamepadManager::load_profiles_from_db] Starting profile load...");
        if let Some(ref db) = *self.db.lock().unwrap_or_else(|e| e.into_inner()) {
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
                    let mut profiles_map = self.profiles.lock().unwrap_or_else(|e| e.into_inner());

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
        let is_running = *self.running.lock().unwrap_or_else(|e| e.into_inner());
        if is_running {
            eprintln!("[GamepadManager::start] Gamepad listener already running");
            return Err("Gamepad listener already running".to_string());
        }

        eprintln!("[GamepadManager::start] Spawning gamepad polling thread...");
        *self.running.lock().unwrap_or_else(|e| e.into_inner()) = true;

        let gamepads = self.gamepads.clone();
        let running = self.running.clone();
        let gilrs_ref = self.gilrs.clone();
        let mode_manager = self.mode_manager.clone();

        thread::spawn(move || {
            let mut gilrs_instance =
                match gilrs_ref.lock().unwrap_or_else(|e| e.into_inner()).take() {
                    Some(g) => g,
                    None => return,
                };

            log::info!("Gamepad listener started");

            let mut last_button_state: HashMap<(usize, GamepadButtonIndex), bool> = HashMap::new();

            loop {
                if !*running.lock().unwrap_or_else(|e| e.into_inner()) {
                    log::info!("Gamepad listener stopped");
                    *gilrs_ref.lock().unwrap_or_else(|e| e.into_inner()) = Some(gilrs_instance);
                    break;
                }

                // Process all gamepad events
                while let Some(Event { id, event, .. }) = gilrs_instance.next_event() {
                    match event {
                        EventType::Connected => {
                            let name = gilrs_instance.gamepad(id).name().to_string();
                            let gamepad = Gamepad::new(name.clone(), id.into());
                            let mut gamepads_lock =
                                gamepads.lock().unwrap_or_else(|e| e.into_inner());
                            gamepads_lock.insert(id.into(), gamepad);
                            log::info!("Gamepad connected: {:?} - {}", id, name);
                        }
                        EventType::Disconnected => {
                            let mut gamepads_lock =
                                gamepads.lock().unwrap_or_else(|e| e.into_inner());
                            gamepads_lock.remove(&(id.into()));
                            log::info!("Gamepad disconnected: {:?}", id);
                        }
                        EventType::AxisChanged(axis, value, _) => {
                            if let Some(gamepad) = gamepads
                                .lock()
                                .unwrap_or_else(|e| e.into_inner())
                                .get_mut(&(id.into()))
                            {
                                // Map gilrs axis to standard gamepad axis
                                if let Some(gamepad_axis) = Self::map_axis_to_gamepad(axis) {
                                    gamepad.set_axis(gamepad_axis, value);
                                    gamepad.update_timestamp();
                                }
                            }
                        }
                        EventType::ButtonPressed(button, _) => {
                            if let Some(gamepad) = gamepads
                                .lock()
                                .unwrap_or_else(|e| e.into_inner())
                                .get_mut(&(id.into()))
                            {
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
                            if let Some(gamepad) = gamepads
                                .lock()
                                .unwrap_or_else(|e| e.into_inner())
                                .get_mut(&(id.into()))
                            {
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
                Self::process_gamepad_input(&gamepads, &mut last_button_state, &mode_manager);
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
        let mut running = self.running.lock().unwrap_or_else(|e| e.into_inner());
        *running = false;
    }

    /// Get all connected gamepads
    pub fn get_gamepads(&self) -> Result<Vec<Gamepad>, String> {
        let gamepads = self.gamepads.lock().unwrap_or_else(|e| e.into_inner());
        let mut list: Vec<_> = gamepads.values().cloned().collect();
        list.sort_by_key(|g| g.index);
        Ok(list)
    }

    /// Get specific gamepad by index
    pub fn get_gamepad(&self, index: usize) -> Result<Option<Gamepad>, String> {
        Ok(self
            .gamepads
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .get(&index)
            .cloned())
    }

    /// Get current gamepad mode
    pub fn get_current_mode(&self) -> Result<GamepadMode, String> {
        let mode_manager = self.mode_manager.lock().unwrap_or_else(|e| e.into_inner());
        Ok(mode_manager.current_mode())
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
        if !self
            .profiles
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .contains_key(&profile_name)
        {
            return Err(format!("Profile '{}' not found", profile_name));
        }
        *self
            .active_profile
            .lock()
            .unwrap_or_else(|e| e.into_inner()) = profile_name;
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
        if let Some(ref db) = *self.db.lock().unwrap_or_else(|e| e.into_inner()) {
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
        self.profiles
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .remove(profile_name);

        // Also delete from database if available
        if let Some(ref db) = *self.db.lock().unwrap_or_else(|e| e.into_inner()) {
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
        let profiles_locked = self.profiles.lock().unwrap_or_else(|e| e.into_inner());
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
            if self.db.lock().unwrap_or_else(|e| e.into_inner()).is_some() {
                eprintln!("[GamepadManager::get_profiles] Loading from database...");
                self.load_profiles_from_db();
            }
        }

        let profiles = self.profiles.lock().unwrap_or_else(|e| e.into_inner());
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
            // Face buttons
            South => GamepadButtonIndex::South,
            East => GamepadButtonIndex::East,
            West => GamepadButtonIndex::West,
            North => GamepadButtonIndex::North,
            // Shoulder buttons
            LeftTrigger => GamepadButtonIndex::LT,
            RightTrigger => GamepadButtonIndex::RT,
            LeftTrigger2 => GamepadButtonIndex::LB,
            RightTrigger2 => GamepadButtonIndex::RB,
            // Menu buttons
            Select => GamepadButtonIndex::Select,
            Start => GamepadButtonIndex::Start,
            // Stick clicks
            LeftThumb => GamepadButtonIndex::LeftStick,
            RightThumb => GamepadButtonIndex::RightStick,
            // Guide/Home
            Mode => GamepadButtonIndex::Guide,
            // D-Pad buttons
            DPadUp => GamepadButtonIndex::DPadUp,
            DPadDown => GamepadButtonIndex::DPadDown,
            DPadLeft => GamepadButtonIndex::DPadLeft,
            DPadRight => GamepadButtonIndex::DPadRight,
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
        mode_manager: &Arc<Mutex<GamepadModeManager>>,
    ) {
        let g = gamepads.lock().unwrap_or_else(|e| e.into_inner());
        if g.is_empty() {
            return;
        }

        // Use first connected gamepad for mouse control
        if let Some(gamepad) = g.values().next() {
            if !gamepad.connected {
                return;
            }

            let mut mode_manager_locked = mode_manager.lock().unwrap_or_else(|e| e.into_inner());
            let current_mode = mode_manager_locked.current_mode();

            // ============ PHASE 1: CONTINUOUS STICK CONTROL ============
            // Mouse control with left stick (always available across all modes)
            let stick_x = gamepad
                .get_axis(GamepadAxisIndex::LeftStickX)
                .unwrap_or(0.0);
            let stick_y = gamepad
                .get_axis(GamepadAxisIndex::LeftStickY)
                .unwrap_or(0.0);

            if stick_x.abs() > 0.05 || stick_y.abs() > 0.05 {
                let dx = (stick_x * 10.0) as i32;
                let dy = -(stick_y * 10.0) as i32;

                let mut enigo = Enigo::new();
                let _ = enigo.mouse_move_relative(dx, dy);
            }

            // Scroll control with right stick (Phase 1)
            let stick_x_right = gamepad
                .get_axis(GamepadAxisIndex::RightStickX)
                .unwrap_or(0.0);
            let stick_y_right = gamepad
                .get_axis(GamepadAxisIndex::RightStickY)
                .unwrap_or(0.0);

            if stick_x_right.abs() > 0.05 || stick_y_right.abs() > 0.05 {
                let vertical_scroll = (stick_y_right * 10.0) as i32;
                let horizontal_scroll = (stick_x_right * 10.0) as i32;
                let _ = scroll::scroll(vertical_scroll, horizontal_scroll);
            }

            // ============ PHASE 2: MODE SWITCHING ============
            // Check for mode switch combinations (RB+Y for Motion, LB+Y for Hotkey)
            let rb_pressed = gamepad
                .get_button(GamepadButtonIndex::RB)
                .map(|b| b.pressed)
                .unwrap_or(false);

            let lb_pressed = gamepad
                .get_button(GamepadButtonIndex::LB)
                .map(|b| b.pressed)
                .unwrap_or(false);

            let north_pressed = gamepad
                .get_button(GamepadButtonIndex::North)
                .map(|b| b.pressed)
                .unwrap_or(false);

            // RB + Y (North) = Motion mode
            if rb_pressed && north_pressed {
                let north_key = (gamepad.index, GamepadButtonIndex::North);
                let north_was_pressed = button_state.get(&north_key).copied().unwrap_or(false);

                if north_pressed && !north_was_pressed {
                    eprintln!("[Phase2] Mode switch: RB+Y -> MOTION");
                    mode_manager_locked.switch_mode(GamepadMode::Motion);
                }
            }

            // LB + Y (North) = Hotkey mode
            if lb_pressed && north_pressed {
                let north_key = (gamepad.index, GamepadButtonIndex::North);
                let north_was_pressed = button_state.get(&north_key).copied().unwrap_or(false);

                if north_pressed && !north_was_pressed {
                    eprintln!("[Phase2] Mode switch: LB+Y -> HOTKEY");
                    mode_manager_locked.switch_mode(GamepadMode::Hotkey);
                }
            }

            // Exit from Motion/Hotkey back to Normal (RB+Y hold)
            if current_mode != GamepadMode::Normal {
                if rb_pressed && north_pressed {
                    let north_key = (gamepad.index, GamepadButtonIndex::North);
                    let north_was_pressed = button_state.get(&north_key).copied().unwrap_or(false);

                    if !north_pressed && north_was_pressed {
                        eprintln!("[Phase2] Mode switch: RB+Y release -> NORMAL");
                        mode_manager_locked.reset_to_normal();
                    }
                }
            }

            // ============ PHASE 2: KEYBINDING EXECUTION ============
            // Get current mode's keybinding registry
            let bindings = get_mode_bindings(current_mode);

            // Process all buttons and emit actions based on current mode
            let all_buttons = vec![
                GamepadButtonIndex::South,
                GamepadButtonIndex::East,
                GamepadButtonIndex::West,
                GamepadButtonIndex::North,
                GamepadButtonIndex::LB,
                GamepadButtonIndex::RB,
                GamepadButtonIndex::LT,
                GamepadButtonIndex::RT,
                GamepadButtonIndex::DPadUp,
                GamepadButtonIndex::DPadDown,
                GamepadButtonIndex::DPadLeft,
                GamepadButtonIndex::DPadRight,
                GamepadButtonIndex::Select,
                GamepadButtonIndex::Start,
                GamepadButtonIndex::LeftStick,
                GamepadButtonIndex::RightStick,
            ];

            for button_idx in all_buttons {
                // Skip mode-switch buttons to avoid double-triggering
                if current_mode != GamepadMode::Normal
                    && ((rb_pressed && button_idx == GamepadButtonIndex::North)
                        || (lb_pressed && button_idx == GamepadButtonIndex::North))
                {
                    continue;
                }

                let button_pressed = gamepad
                    .get_button(button_idx)
                    .map(|b| b.pressed)
                    .unwrap_or(false);

                let button_key = (gamepad.index, button_idx);
                let button_was_pressed = button_state.get(&button_key).copied().unwrap_or(false);

                // Rising edge: button just pressed
                if button_pressed && !button_was_pressed {
                    // Create input pattern for keybinding lookup
                    let pattern = InputPattern::SingleButton {
                        button: crate::types::GamepadButton { index: button_idx },
                        input_type: InputType::Tap,
                    };

                    // Try to find binding for this button in current mode
                    if let Some(binding) = bindings.get_binding(&pattern) {
                        eprintln!(
                            "[Phase2] Button {:?} -> Action: {}",
                            button_idx, binding.action
                        );

                        // Handle mode switches specially
                        if binding.action.is_mode_switch() {
                            if let Action::SwitchMode { mode } = binding.action {
                                mode_manager_locked.switch_mode(mode);
                            }
                        } else {
                            // Execute action via executor with block_on
                            let action = binding.action.clone();
                            drop(mode_manager_locked); // Release lock before blocking on async

                            // Wrap in catch_unwind to prevent panics from poisoning locks
                            let result = catch_unwind(AssertUnwindSafe(|| {
                                let rt = tokio::runtime::Runtime::new()
                                    .expect("Failed to create runtime");
                                rt.block_on(execute_action(&action))
                            }));

                            match result {
                                Ok(Ok(_)) => eprintln!("[Phase2] Action executed: {}", action),
                                Ok(Err(e)) => {
                                    eprintln!("[Phase2] Action failed: {} - {}", action, e)
                                }
                                Err(_) => {
                                    eprintln!("[Phase2] Action panicked, recovering gracefully")
                                }
                            }

                            mode_manager_locked =
                                mode_manager.lock().unwrap_or_else(|e| e.into_inner());
                        }
                    }
                }

                button_state.insert(button_key, button_pressed);
            }
        }
    }

    /// Emit a single key press
    fn emit_key_press(key: enigo::Key) {
        let mut enigo = Enigo::new();
        let _ = enigo.key_click(key);
    }

    /// Emit multiple key presses for key combinations (like Cmd+arrow)
    fn emit_key_combination(keys: &[enigo::Key]) {
        let mut enigo = Enigo::new();
        // Press all keys
        for key in keys {
            let _ = enigo.key_down(*key);
        }
        thread::sleep(Duration::from_millis(10));
        // Release all keys in reverse order
        for key in keys.iter().rev() {
            let _ = enigo.key_up(*key);
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
