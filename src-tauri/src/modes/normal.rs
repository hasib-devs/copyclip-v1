/// NORMAL Mode Bindings
/// Default mode for navigation, app control, and system features
///
/// Key patterns:
/// - Face buttons: A (click), B (escape), X (right-click), Y (menu/mode switch)
/// - D-Pad: Navigation and volume/brightness control
/// - Shoulders: Modifiers (LB=Alt, RB=Ctrl) and app switching
/// - Triggers: Click actions (LT, RT) and mode switching
/// - Sticks: Cursor movement and special functions

use crate::types::*;
use crate::gamepad::GamepadButtonIndex;

/// Build NORMAL mode keybinding registry
pub fn build_normal_mode_bindings() -> KeyBindingRegistry {
    let mut registry = KeyBindingRegistry::new();

    // ============ FACE BUTTONS ============

    // A Button - Left Click
    registry.add_binding(
        KeyBinding::new(
            InputPattern::SingleButton {
                button: GamepadButton { index: GamepadButtonIndex::South },
                input_type: InputType::Tap,
            },
            Action::MouseClick,
            GamepadMode::Normal,
        )
        .with_description("Left click / Select".to_string())
        .with_priority(50),
    );

    // B Button - Escape / Back
    registry.add_binding(
        KeyBinding::new(
            InputPattern::SingleButton {
                button: GamepadButton { index: GamepadButtonIndex::East },
                input_type: InputType::Tap,
            },
            Action::KeyPress {
                key: "Escape".to_string(),
            },
            GamepadMode::Normal,
        )
        .with_description("Back / Escape".to_string())
        .with_priority(50),
    );

    // X Button - Right Click / Context Menu
    registry.add_binding(
        KeyBinding::new(
            InputPattern::SingleButton {
                button: GamepadButton { index: GamepadButtonIndex::West },
                input_type: InputType::Tap,
            },
            Action::MouseRightClick,
            GamepadMode::Normal,
        )
        .with_description("Right click / Context menu".to_string())
        .with_priority(50),
    );

    // Y Button Hold - Switch to HOTKEY mode
    registry.add_binding(
        KeyBinding::new(
            InputPattern::SingleButton {
                button: GamepadButton { index: GamepadButtonIndex::North },
                input_type: InputType::Hold,
            },
            Action::SwitchMode {
                mode: GamepadMode::Hotkey,
            },
            GamepadMode::Normal,
        )
        .with_description("Hold for HOTKEY mode".to_string())
        .with_priority(70),
    );

    // Y Button Tap - Open application menu/launcher hint
    registry.add_binding(
        KeyBinding::new(
            InputPattern::SingleButton {
                button: GamepadButton { index: GamepadButtonIndex::North },
                input_type: InputType::Tap,
            },
            Action::AppLauncher,
            GamepadMode::Normal,
        )
        .with_description("Tap quick menu, hold for HOTKEY mode".to_string())
        .with_priority(50),
    );

    // ============ D-PAD ============

    // D-Pad Up - Volume Up
    registry.add_binding(
        KeyBinding::new(
            InputPattern::SingleButton {
                button: GamepadButton { index: GamepadButtonIndex::DPadUp },
                input_type: InputType::Tap,
            },
            Action::VolumeUp { amount: 10 },
            GamepadMode::Normal,
        )
        .with_description("Volume up".to_string())
        .with_priority(50),
    );

    // D-Pad Down - Volume Down
    registry.add_binding(
        KeyBinding::new(
            InputPattern::SingleButton {
                button: GamepadButton { index: GamepadButtonIndex::DPadDown },
                input_type: InputType::Tap,
            },
            Action::VolumeDown { amount: 10 },
            GamepadMode::Normal,
        )
        .with_description("Volume down".to_string())
        .with_priority(50),
    );

    // D-Pad Left - Previous App (with modifier)
    registry.add_binding(
        KeyBinding::new(
            InputPattern::ModifiedButton {
                button: GamepadButton { index: GamepadButtonIndex::DPadLeft },
                modifier: InputModifier::Alt,
                input_type: InputType::Tap,
            },
            Action::AppPrevious,
            GamepadMode::Normal,
        )
        .with_description("Alt+Tab Backward / Previous app".to_string())
        .with_priority(50),
    );

    // D-Pad Right - Next App (with modifier)
    registry.add_binding(
        KeyBinding::new(
            InputPattern::ModifiedButton {
                button: GamepadButton { index: GamepadButtonIndex::DPadRight },
                modifier: InputModifier::Alt,
                input_type: InputType::Tap,
            },
            Action::AppNext,
            GamepadMode::Normal,
        )
        .with_description("Alt+Tab Forward / Next app".to_string())
        .with_priority(50),
    );

    // ============ SHOULDER BUTTONS ============

    // LB Hold - Show app switcher
    registry.add_binding(
        KeyBinding::new(
            InputPattern::SingleButton {
                button: GamepadButton { index: GamepadButtonIndex::LB },
                input_type: InputType::Hold,
            },
            Action::AppSwitcher,
            GamepadMode::Normal,
        )
        .with_description("Hold to show app switcher".to_string())
        .with_priority(60),
    );

    // RB + Y Hold - Switch to MOTION mode
    registry.add_binding(
        KeyBinding::new(
            InputPattern::Chord {
                buttons: vec![
                    GamepadButton { index: GamepadButtonIndex::RB },
                    GamepadButton { index: GamepadButtonIndex::North },
                ],
            },
            Action::SwitchMode {
                mode: GamepadMode::Motion,
            },
            GamepadMode::Normal,
        )
        .with_description("Hold RB+Y for MOTION mode".to_string())
        .with_priority(70),
    );

    // ============ TRIGGER BUTTONS ============

    // LT (full press) - Left Click
    registry.add_binding(
        KeyBinding::new(
            InputPattern::SingleButton {
                button: GamepadButton { index: GamepadButtonIndex::LT },
                input_type: InputType::Tap,
            },
            Action::MouseClick,
            GamepadMode::Normal,
        )
        .with_description("Left click".to_string())
        .with_priority(40),
    );

    // RT (full press) - Right Click
    registry.add_binding(
        KeyBinding::new(
            InputPattern::SingleButton {
                button: GamepadButton { index: GamepadButtonIndex::RT },
                input_type: InputType::Tap,
            },
            Action::MouseRightClick,
            GamepadMode::Normal,
        )
        .with_description("Right click".to_string())
        .with_priority(40),
    );

    // ============ SPECIAL BUTTONS ============

    // Guide Button Hold - App Launcher (long press)
    registry.add_binding(
        KeyBinding::new(
            InputPattern::SingleButton {
                button: GamepadButton { index: GamepadButtonIndex::Guide },
                input_type: InputType::LongHold,
            },
            Action::AppLauncher,
            GamepadMode::Normal,
        )
        .with_description("Long press Guide for app launcher".to_string())
        .with_priority(75),
    );

    // Select Button - Settings/Preferences
    registry.add_binding(
        KeyBinding::new(
            InputPattern::SingleButton {
                button: GamepadButton { index: GamepadButtonIndex::Select },
                input_type: InputType::Tap,
            },
            Action::KeyPress {
                key: "F1".to_string(), // Help key
            },
            GamepadMode::Normal,
        )
        .with_description("Help / Preferences".to_string())
        .with_priority(50),
    );

    // ============ STICK BUTTONS ============

    // Left Stick Click - Screenshot
    registry.add_binding(
        KeyBinding::new(
            InputPattern::SingleButton {
                button: GamepadButton { index: GamepadButtonIndex::LeftStick },
                input_type: InputType::Tap,
            },
            Action::Screenshot,
            GamepadMode::Normal,
        )
        .with_description("Take screenshot".to_string())
        .with_priority(50),
    );

    // Right Stick Click - (Reserved - scroll mode indicator)
    registry.add_binding(
        KeyBinding::new(
            InputPattern::SingleButton {
                button: GamepadButton { index: GamepadButtonIndex::RightStick },
                input_type: InputType::Tap,
            },
            Action::NoOp,
            GamepadMode::Normal,
        )
        .with_description("[Reserved for scroll control]".to_string())
        .with_priority(30),
    );

    registry
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normal_mode_registry() {
        let registry = build_normal_mode_bindings();
        assert!(!registry.is_empty());

        // Verify some key bindings exist
        let a_button = GamepadButton {
            index: GamepadButtonIndex::South,
        };
        let bindings = registry.bindings_for_button(a_button);
        assert!(!bindings.is_empty());
    }
}
