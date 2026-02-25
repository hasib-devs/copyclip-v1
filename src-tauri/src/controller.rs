use enigo::{Enigo, MouseControllable};
use gilrs::{Axis, Event, EventType, Gilrs};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

/// Controller settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ControllerSettings {
    pub sensitivity: f32,  // 0.5 - 3.0
    pub dead_zone: f32,    // 0.0 - 0.3
    pub acceleration: f32, // 0.8 - 2.0
    pub enabled: bool,
}

impl Default for ControllerSettings {
    fn default() -> Self {
        Self {
            sensitivity: 1.0,
            dead_zone: 0.1,
            acceleration: 1.0,
            enabled: false,
        }
    }
}

/// Controller state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ControllerState {
    pub connected: bool,
    pub left_stick_x: f32,
    pub left_stick_y: f32,
    pub left_trigger: f32,
    pub right_trigger: f32,
}

impl Default for ControllerState {
    fn default() -> Self {
        Self {
            connected: false,
            left_stick_x: 0.0,
            left_stick_y: 0.0,
            left_trigger: 0.0,
            right_trigger: 0.0,
        }
    }
}

/// Controller manager
pub struct ControllerManager {
    pub settings: Arc<Mutex<ControllerSettings>>,
    pub state: Arc<Mutex<ControllerState>>,
    running: Arc<Mutex<bool>>,
}

impl ControllerManager {
    pub fn new() -> Self {
        Self {
            settings: Arc::new(Mutex::new(ControllerSettings::default())),
            state: Arc::new(Mutex::new(ControllerState::default())),
            running: Arc::new(Mutex::new(false)),
        }
    }

    /// Start listening to controller input
    pub fn start(&self) -> Result<(), String> {
        let running = self.running.clone();
        let settings = self.settings.clone();
        let state = self.state.clone();

        let is_running = *running.lock().unwrap();
        if is_running {
            return Err("Controller listener already running".to_string());
        }

        *running.lock().unwrap() = true;

        thread::spawn(move || {
            match Gilrs::new() {
                Ok(mut gilrs) => {
                    log::info!("Gamepad listener initialized");

                    loop {
                        // Check if we should continue running
                        if !*running.lock().unwrap() {
                            log::info!("Controller listener stopped");
                            break;
                        }

                        // Update gamepad state
                        while let Some(Event { id, event, .. }) = gilrs.next_event() {
                            match event {
                                EventType::Connected => {
                                    log::info!("Gamepad connected: {:?}", id);
                                    let mut state = state.lock().unwrap();
                                    state.connected = true;
                                }
                                EventType::Disconnected => {
                                    log::info!("Gamepad disconnected: {:?}", id);
                                    let mut state = state.lock().unwrap();
                                    state.connected = false;
                                }
                                EventType::AxisChanged(axis, value, _) => {
                                    let settings = settings.lock().unwrap();
                                    let mut state = state.lock().unwrap();

                                    // Apply dead zone
                                    let value = if value.abs() < settings.dead_zone {
                                        0.0
                                    } else {
                                        value
                                    };

                                    match axis {
                                        Axis::LeftStickX => {
                                            state.left_stick_x = value;
                                        }
                                        Axis::LeftStickY => {
                                            state.left_stick_y = -value; // Invert Y
                                        }
                                        Axis::LeftZ => {
                                            // Left trigger (LT) - Z axis typically represents LT
                                            state.left_trigger = (value + 1.0) / 2.0;
                                        }
                                        Axis::RightZ => {
                                            // Right trigger (RT) - RZ axis typically represents RT
                                            state.right_trigger = (value + 1.0) / 2.0;
                                        }
                                        _ => {}
                                    }
                                }
                                _ => {}
                            }
                        }

                        // Move mouse based on left stick
                        {
                            let state = state.lock().unwrap();
                            let settings = settings.lock().unwrap();

                            if settings.enabled && state.connected {
                                let stick_x = state.left_stick_x;
                                let stick_y = state.left_stick_y;

                                if stick_x.abs() > 0.01 || stick_y.abs() > 0.01 {
                                    let dx =
                                        (stick_x * settings.sensitivity * settings.acceleration)
                                            as i32;
                                    let dy =
                                        (stick_y * settings.sensitivity * settings.acceleration)
                                            as i32;

                                    let mut enigo = Enigo::new();
                                    let _ = enigo.mouse_move_relative(dx, dy);
                                }

                                // Handle clicks with debouncing
                                if state.right_trigger > 0.5 {
                                    let mut enigo = Enigo::new();
                                    let _ = enigo.mouse_down(enigo::MouseButton::Left);
                                    let _ = enigo.mouse_up(enigo::MouseButton::Left);
                                }

                                if state.left_trigger > 0.5 {
                                    let mut enigo = Enigo::new();
                                    let _ = enigo.mouse_down(enigo::MouseButton::Right);
                                    let _ = enigo.mouse_up(enigo::MouseButton::Right);
                                }
                            }
                        }

                        thread::sleep(Duration::from_millis(16)); // ~60 FPS
                    }
                }
                Err(e) => {
                    log::error!("Failed to initialize gamepad listener: {}", e);
                }
            }
        });

        Ok(())
    }

    /// Stop listening to controller input
    pub fn stop(&self) {
        *self.running.lock().unwrap() = false;
    }

    /// Update settings
    pub fn update_settings(&self, settings: ControllerSettings) -> Result<(), String> {
        *self.settings.lock().unwrap() = settings;
        Ok(())
    }

    /// Get current state
    pub fn get_state(&self) -> Result<ControllerState, String> {
        Ok(self.state.lock().unwrap().clone())
    }

    /// Get current settings
    pub fn get_settings(&self) -> Result<ControllerSettings, String> {
        Ok(self.settings.lock().unwrap().clone())
    }
}

impl Default for ControllerManager {
    fn default() -> Self {
        Self::new()
    }
}
