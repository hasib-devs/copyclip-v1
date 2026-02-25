/**
 * Gamepad Type Definitions
 * Browser-compatible Gamepad API types following HTML5 standard
 */

export interface GamepadButton {
  pressed: boolean;
  touched: boolean;
  value: number; // 0.0 - 1.0
}

export interface Gamepad {
  id: string;
  index: number;
  connected: boolean;
  timestamp: number;
  buttons: GamepadButton[];
  axes: number[];
  mapping: string;
  vibration_actuators: number;
}

export enum GamepadButtonIndex {
  South = 0, // X or A
  East = 1, // Circle or B
  West = 2, // Square or X
  North = 3, // Triangle or Y
  LB = 4, // L1 or LB
  RB = 5, // R1 or RB
  LT = 6, // L2 or LT
  RT = 7, // R2 or RT
  Select = 8,
  Start = 9,
  LeftStick = 10,
  RightStick = 11,
  Guide = 12,
}

export enum GamepadAxisIndex {
  LeftStickX = 0,
  LeftStickY = 1,
  RightStickX = 2,
  RightStickY = 3,
}

/**
 * Scroll configuration
 */
export interface ScrollSettings {
  enabled: boolean;
  vertical_speed: number; // Multiplier: 0.5x - 5.0x
  horizontal_speed: number; // Multiplier: 0.5x - 5.0x
  reverse_vertical: boolean;
  reverse_horizontal: boolean;
}

/**
 * Click type enumeration
 */
export enum ClickType {
  Left = "left",
  Right = "right",
  Middle = "middle",
  Double = "double",
}

/**
 * Keyboard key mapping
 */
export interface KeyMapping {
  single?: string; // Single key like "Return"
  combination?: string[]; // Like ["Cmd", "Right"] for Cmd+Right
}

/**
 * D-Pad button mapping
 */
export interface DPadMapping {
  up: KeyMapping;
  down: KeyMapping;
  left: KeyMapping;
  right: KeyMapping;
}

export interface GamepadProfile {
  name: string;
  description: string;
  sensitivity: number; // 0.5 - 3.0
  dead_zone: number; // 0.0 - 0.3
  acceleration: number; // 0.8 - 2.0
  button_map: Record<string, number>;
  axis_map: Record<string, number>;
  enabled_features: GamepadFeatures;
  scroll_settings: ScrollSettings;
  dpad_mapping: DPadMapping;
}

export interface GamepadFeatures {
  mouse_control: boolean;
  keyboard_emulation: boolean;
  vibration: boolean;
  adaptive_triggers: boolean;
  scroll_control: boolean;
}

/**
 * Gamepad Context State
 */
export interface GamepadContextState {
  gamepads: Gamepad[];
  activeGamepadIndex: number;
  profiles: GamepadProfile[];
  activeProfile: GamepadProfile | null;
  isListening: boolean;
  isLoading: boolean;
  error: string | null;
}

/**
 * Gamepad Context Actions
 */
export type GamepadAction =
  | {
      type: "SET_GAMEPADS";
      payload: Gamepad[];
    }
  | {
      type: "SET_ACTIVE_GAMEPAD";
      payload: number;
    }
  | {
      type: "SET_PROFILES";
      payload: GamepadProfile[];
    }
  | {
      type: "SET_ACTIVE_PROFILE";
      payload: GamepadProfile | null;
    }
  | {
      type: "SET_LISTENING";
      payload: boolean;
    }
  | {
      type: "SET_LOADING";
      payload: boolean;
    }
  | {
      type: "SET_ERROR";
      payload: string | null;
    }
  | {
      type: "RESET_ERROR";
    }
  | {
      type: "PROFILE_ADDED";
      payload: GamepadProfile;
    }
  | {
      type: "PROFILE_DELETED";
      payload: string;
    }
  | {
      type: "PROFILE_UPDATED";
      payload: GamepadProfile;
    };
