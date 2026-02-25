use crate::gamepad::{
    Gamepad, GamepadAxisIndex, GamepadButton, GamepadButtonIndex, GamepadEvent, GamepadEventType,
    GamepadProfile,
};
use enigo::{Enigo, MouseControllable};
use gilrs::{Axis, Event, EventType, Gilrs, GilrsBuilder};
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
}

impl GamepadManager {
    pub fn new() -> Result<Self, String> {
        // Initialize gilrs with custom settings
        let gilrs = GilrsBuilder::new()
            .with_default_filters(true)
            .build()
            .map_err(|e| format!("Failed to initialize gilrs: {}", e))?;

        Ok(Self {
            gamepads: Arc::new(Mutex::new(HashMap::new())),
            profiles: Arc::new(Mutex::new(HashMap::new())),
            active_profile: Arc::new(Mutex::new("Default".to_string())),
            running: Arc::new(Mutex::new(false)),
            gilrs: Arc::new(Mutex::new(Some(gilrs))),
        })
    }

    /// Start listening to gamepad input
    pub fn start(&self) -> Result<(), String> {
        let is_running = *self.running.lock().unwrap();
        if is_running {
            return Err("Gamepad listener already running".to_string());
        }

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
                                    let mut btn = GamepadButton {
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
                Self::process_gamepad_input(&gamepads);

                thread::sleep(Duration::from_millis(16)); // ~60 FPS
            }
        });

        Ok(())
    }

    /// Stop listening to gamepad input
    pub fn stop(&self) {
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
    pub fn get_active_profile(&self) -> Result<GamepadProfile, String> {
        let profile_name = self.active_profile.lock().unwrap().clone();
        Ok(self
            .profiles
            .lock()
            .unwrap()
            .get(&profile_name)
            .cloned()
            .unwrap_or_default())
    }

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
        self.profiles
            .lock()
            .unwrap()
            .insert(profile.name.clone(), profile);
        Ok(())
    }

    /// Delete a gamepad profile
    pub fn delete_profile(&self, profile_name: &str) -> Result<(), String> {
        if profile_name == "Default" {
            return Err("Cannot delete default profile".to_string());
        }
        self.profiles.lock().unwrap().remove(profile_name);
        Ok(())
    }

    /// Get all profiles
    pub fn get_profiles(&self) -> Result<Vec<GamepadProfile>, String> {
        let profiles = self.profiles.lock().unwrap();
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
    fn process_gamepad_input(gamepads: &Arc<Mutex<HashMap<usize, Gamepad>>>) {
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
                let dy = (stick_y * 10.0) as i32;

                let mut enigo = Enigo::new();
                let _ = enigo.mouse_move_relative(dx, dy);
            }

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
