use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Gamepad button state (browser Gamepad API compatible)
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub struct GamepadButton {
    pub pressed: bool,
    pub touched: bool,
    pub value: f32, // 0.0 - 1.0 for analog buttons
}

impl Default for GamepadButton {
    fn default() -> Self {
        Self {
            pressed: false,
            touched: false,
            value: 0.0,
        }
    }
}

/// Standard gamepad button indices (following HTML Gamepad API)
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub enum GamepadButtonIndex {
    // Face buttons
    South = 0, // X or A
    East = 1,  // Circle or B
    West = 2,  // Square or X
    North = 3, // Triangle or Y
    // Shoulder buttons
    LB = 4, // LB or L1
    RB = 5, // RB or R1
    LT = 6, // LT or L2
    RT = 7, // RT or R2
    // Menu buttons
    Select = 8, // Back/Select
    Start = 9,  // Start
    // Stick clicks
    LeftStick = 10,
    RightStick = 11,
    // Guide/Home
    Guide = 12,
    // D-Pad buttons
    DPadUp = 13,
    DPadDown = 14,
    DPadLeft = 15,
    DPadRight = 16,
    A,
    B,
}

/// Standard gamepad axes indices (following HTML Gamepad API)
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub enum GamepadAxisIndex {
    LeftStickX = 0,
    LeftStickY = 1,
    RightStickX = 2,
    RightStickY = 3,
}

/// Complete gamepad state (browser Gamepad API compatible)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Gamepad {
    pub id: String,   // Unique identifier (e.g., "PS5 Controller (0)")
    pub index: usize, // Gamepad index in array
    pub connected: bool,
    pub timestamp: f64, // Milliseconds since epoch
    pub buttons: Vec<GamepadButton>,
    pub axes: Vec<f32>,             // Analog stick values (-1.0 to 1.0)
    pub mapping: String,            // "standard" or custom mapping name
    pub vibration_actuators: usize, // Number of vibration motors
}

impl Default for Gamepad {
    fn default() -> Self {
        Self {
            id: String::new(),
            index: 0,
            connected: false,
            timestamp: 0.0,
            buttons: vec![GamepadButton::default(); 32], // Increased from 16 to accommodate all button types
            axes: vec![0.0; 4],                          // Standard has 4 axes
            mapping: "standard".to_string(),
            vibration_actuators: 0,
        }
    }
}

impl Gamepad {
    pub fn new(id: String, index: usize) -> Self {
        Self {
            id,
            index,
            connected: true,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs_f64()
                * 1000.0,
            buttons: vec![GamepadButton::default(); 32], // Increased to accommodate all button types
            axes: vec![0.0; 4],
            mapping: "standard".to_string(),
            vibration_actuators: 2, // Most modern controllers have 2
        }
    }

    pub fn update_timestamp(&mut self) {
        self.timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs_f64()
            * 1000.0;
    }

    pub fn get_button(&self, button: GamepadButtonIndex) -> Option<&GamepadButton> {
        self.buttons.get(button as usize)
    }

    pub fn set_button(&mut self, button: GamepadButtonIndex, state: GamepadButton) {
        if (button as usize) < self.buttons.len() {
            self.buttons[button as usize] = state;
        }
    }

    pub fn get_axis(&self, axis: GamepadAxisIndex) -> Option<f32> {
        self.axes.get(axis as usize).copied()
    }

    pub fn set_axis(&mut self, axis: GamepadAxisIndex, value: f32) {
        if (axis as usize) < self.axes.len() {
            self.axes[axis as usize] = value.clamp(-1.0, 1.0);
        }
    }
}

/// Gamepad event types
#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(dead_code)]
pub enum GamepadEventType {
    Connected,
    Disconnected,
    ButtonPressed(GamepadButtonIndex),
    ButtonReleased(GamepadButtonIndex),
    AxisChanged(GamepadAxisIndex, f32),
}

/// Gamepad event
#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct GamepadEvent {
    pub gamepad_index: usize,
    pub gamepad_id: String,
    pub event_type: GamepadEventType,
    pub timestamp: f64,
}

/// Scroll configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScrollSettings {
    pub enabled: bool,
    pub vertical_speed: f32,   // Multiplier: 0.5 - 5.0
    pub horizontal_speed: f32, // Multiplier: 0.5 - 5.0
    pub reverse_vertical: bool,
    pub reverse_horizontal: bool,
}

impl Default for ScrollSettings {
    fn default() -> Self {
        Self {
            enabled: true,
            vertical_speed: 1.5,
            horizontal_speed: 1.5,
            reverse_vertical: false,
            reverse_horizontal: false,
        }
    }
}

/// Click type enumeration
#[derive(Debug, Clone, Copy, Eq, PartialEq, Serialize, Deserialize)]
#[allow(dead_code)]
pub enum ClickType {
    Left,
    Right,
    Middle,
    Double,
}

/// Keyboard key mapping
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyMapping {
    pub single: Option<String>,
    pub combination: Option<Vec<String>>,
}

/// D-Pad button mapping
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DPadMapping {
    pub up: KeyMapping,
    pub down: KeyMapping,
    pub left: KeyMapping,
    pub right: KeyMapping,
}

impl Default for DPadMapping {
    fn default() -> Self {
        Self {
            up: KeyMapping {
                single: None,
                combination: Some(vec!["Page".to_string(), "Up".to_string()]),
            },
            down: KeyMapping {
                single: None,
                combination: Some(vec!["Page".to_string(), "Down".to_string()]),
            },
            left: KeyMapping {
                single: None,
                combination: Some(vec!["Cmd".to_string(), "[".to_string()]),
            },
            right: KeyMapping {
                single: None,
                combination: Some(vec!["Cmd".to_string(), "]".to_string()]),
            },
        }
    }
}

/// Gamepad profile for custom button/axis mapping
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GamepadProfile {
    pub name: String,
    pub description: String,
    pub sensitivity: f32,
    pub dead_zone: f32,
    pub acceleration: f32,
    pub button_map: HashMap<String, GamepadButtonIndex>,
    pub axis_map: HashMap<String, GamepadAxisIndex>,
    pub enabled_features: GamepadFeatures,
    pub scroll_settings: ScrollSettings,
    pub dpad_mapping: DPadMapping,
}

/// Feature flags for gamepad functionality
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GamepadFeatures {
    pub mouse_control: bool,
    pub keyboard_emulation: bool,
    pub vibration: bool,
    pub adaptive_triggers: bool, // PS5 specific
    pub scroll_control: bool,
}

impl Default for GamepadFeatures {
    fn default() -> Self {
        Self {
            mouse_control: true,
            keyboard_emulation: false,
            vibration: true,
            adaptive_triggers: false,
            scroll_control: true,
        }
    }
}

impl Default for GamepadProfile {
    fn default() -> Self {
        Self {
            name: "Default".to_string(),
            description: "Default gamepad profile".to_string(),
            sensitivity: 1.0,
            dead_zone: 0.1,
            acceleration: 1.0,
            button_map: HashMap::new(),
            axis_map: HashMap::new(),
            enabled_features: GamepadFeatures::default(),
            scroll_settings: ScrollSettings::default(),
            dpad_mapping: DPadMapping::default(),
        }
    }
}
