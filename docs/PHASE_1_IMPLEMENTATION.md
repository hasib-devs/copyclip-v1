# Phase 1 Implementation Guide: Enhanced Navigation

## Quick Start - Phase 1 Priority Features

### 1. Scroll Control (Week 1-2)
### 2. Multi-Click Support (Week 3-4)  
### 3. Basic Keyboard Emulation (Week 5-6)

---

## Feature 1: Scroll Control Via Right Joystick

### Architecture

```
┌─ Right Stick Input ─┐
│  X-axis (horiz)     │
│  Y-axis (vertical)  │ ──→ Detect movement
└─────────────────────┘
         │
         ▼
   Apply deadzone
   (only move if > 0.1)
         │
         ▼
    Determine scroll direction
    - Positive Y = Down
    - Negative Y = Up
    - Positive X = Right scroll
    - Negative X = Left scroll
         │
         ▼
    Calculate pixels/amount
    amount = stick_value * sensitivity * 10
         │
         ▼
    Emit scroll command
    (use enigo or macOS API)
```

### Implementation Steps

#### Step 1: Add Scroll Configuration to Types

**File**: `src/types/gamepad.types.ts`

```typescript
export interface GamepadProfile {
  name: string;
  description: string;
  sensitivity: number;
  dead_zone: number;
  acceleration: number;
  button_map: Record<string, number>;
  axis_map: Record<string, number>;
  enabled_features: GamepadFeatures;
  
  // ADD THESE:
  scroll_settings: ScrollSettings;
}

export interface ScrollSettings {
  enabled: boolean;
  vertical_speed: number;      // Multiplier: 0.5x - 5.0x
  horizontal_speed: number;
  reverse_vertical: boolean;
  reverse_horizontal: boolean;
  use_trackpad_scroll: boolean; // vs direct scroll
}

export interface GamepadFeatures {
  mouse_control: boolean;
  keyboard_emulation: boolean;
  vibration: boolean;
  adaptive_triggers: boolean;
  scroll_control: boolean;      // ADD THIS
}
```

#### Step 2: Backend Scroll Implementation

**File**: `src-tauri/src/gamepad_manager.rs`

Add to the main polling loop (around line 190):

```rust
// Scroll control with right stick
let stick_x_right = gamepad
    .get_axis(GamepadAxisIndex::RightStickX)
    .unwrap_or(0.0);
let stick_y_right = gamepad
    .get_axis(GamepadAxisIndex::RightStickY)
    .unwrap_or(0.0);

if stick_x_right.abs() > 0.05 || stick_y_right.abs() > 0.05 {
    // Get scroll settings from active profile
    let scroll_config = profiles.get("Default")
        .map(|p| p.scroll_settings.clone())
        .unwrap_or_default();
    
    if scroll_config.enabled {
        // Calculate scroll amount
        let vertical_scroll = if scroll_config.reverse_vertical {
            -(stick_y_right * scroll_config.vertical_speed * 10.0) as i32
        } else {
            (stick_y_right * scroll_config.vertical_speed * 10.0) as i32
        };
        
        let horizontal_scroll = if scroll_config.reverse_horizontal {
            -(stick_x_right * scroll_config.horizontal_speed * 10.0) as i32
        } else {
            (stick_x_right * scroll_config.horizontal_speed * 10.0) as i32
        };
        
        // TODO: Implement scroll via enigo
        let _ = Self::emit_scroll(vertical_scroll, horizontal_scroll);
    }
}

// Helper function to add
impl GamepadManager {
    fn emit_scroll(vertical: i32, horizontal: i32) -> Result<(), String> {
        // macOS: Use CGEventCreateScrollWheelEvent
        // Windows: Use mouse_event with WHEEL_DELTA
        // Linux: Use X11 scroll simulation
        eprintln!("[Scroll] Vertical: {}, Horizontal: {}", vertical, horizontal);
        // Implementation will vary by OS
        Ok(())
    }
}
```

#### Step 3: Platform-Specific Scroll Implementation

Since `enigo` doesn't support scroll natively, we need platform-specific code:

**File**: `src-tauri/src/platform/scroll.rs` (NEW FILE)

```rust
#[cfg(target_os = "macos")]
mod macos_scroll {
    use cocoa::appkit::NSEvent;
    use cocoa::foundation::NSPoint;
    
    pub fn scroll(vertical: i32, horizontal: i32) -> Result<(), String> {
        // Use CGEventCreateScrollWheelEvent
        // vertical: lines to scroll (positive = down)
        // horizontal: lines to scroll (positive = right)
        
        unsafe {
            let point = NSPoint { x: 0.0, y: 0.0 };
            let _event = NSEvent::scrollWheelEvent_withLocation_modifierFlags_timestamp_windowNumber_context_deltaX_deltaY_deltaZ(
                point,
                0,
                0.0,
                1,
                0,
                horizontal as f64 / 10.0,
                vertical as f64 / 10.0,
                0.0,
            );
            // Post event to system
        }
        Ok(())
    }
}

#[cfg(target_os = "windows")]
mod windows_scroll {
    use winapi::um::winuser::{mouse_event, MOUSEEVENTF_WHEEL};
    
    pub fn scroll(vertical: i32, _horizontal: i32) -> Result<(), String> {
        let wheel_delta = vertical * 120; // Windows uses 120 per notch
        unsafe {
            mouse_event(MOUSEEVENTF_WHEEL, 0, 0, wheel_delta as u32, 0);
        }
        Ok(())
    }
}

#[cfg(target_os = "linux")]
mod linux_scroll {
    pub fn scroll(vertical: i32, horizontal: i32) -> Result<(), String> {
        // Use X11 button events (buttons 4/5 for vertical, 6/7 for horizontal)
        // Or use xinput for scroll events
        eprintln!("[Linux Scroll] Vertical: {}, Horizontal: {}", vertical, horizontal);
        Ok(())
    }
}

pub fn scroll(vertical: i32, horizontal: i32) -> Result<(), String> {
    #[cfg(target_os = "macos")]
    return macos_scroll::scroll(vertical, horizontal);
    
    #[cfg(target_os = "windows")]
    return windows_scroll::scroll(vertical, horizontal);
    
    #[cfg(target_os = "linux")]
    return linux_scroll::scroll(vertical, horizontal);
}
```

#### Step 4: Frontend UI for Scroll Settings

**File**: `src/components/GamepadConfig.tsx`

Add to the configuration section:

```tsx
{/* Scroll Settings */}
<div className="space-y-3 border-t pt-4">
  <Label className="text-base font-semibold">Scroll Control</Label>
  
  {/* Enable/Disable */}
  <div className="flex items-center justify-between p-3 border rounded-lg">
    <Label>Enable Right Stick Scroll</Label>
    <Switch
      checked={scrollSettings.enabled}
      onCheckedChange={(enabled) => 
        setScrollSettings({...scrollSettings, enabled})
      }
    />
  </div>
  
  {/* Vertical Speed */}
  <div className="space-y-2">
    <div className="flex justify-between">
      <Label>Vertical Speed</Label>
      <span className="text-sm text-muted-foreground">
        {scrollSettings.vertical_speed.toFixed(1)}x
      </span>
    </div>
    <Slider
      min={0.5}
      max={5}
      step={0.1}
      value={[scrollSettings.vertical_speed]}
      onValueChange={([val]) => 
        setScrollSettings({...scrollSettings, vertical_speed: val})
      }
    />
  </div>
  
  {/* Horizontal Speed */}
  <div className="space-y-2">
    <div className="flex justify-between">
      <Label>Horizontal Speed</Label>
      <span className="text-sm text-muted-foreground">
        {scrollSettings.horizontal_speed.toFixed(1)}x
      </span>
    </div>
    <Slider
      min={0.5}
      max={5}
      step={0.1}
      value={[scrollSettings.horizontal_speed]}
      onValueChange={([val]) => 
        setScrollSettings({...scrollSettings, horizontal_speed: val})
      }
    />
  </div>
  
  {/* Reverse Options */}
  <div className="grid grid-cols-2 gap-3">
    <div className="flex items-center justify-between p-3 border rounded">
      <Label className="text-sm">Reverse Vertical</Label>
      <Switch
        checked={scrollSettings.reverse_vertical}
        onCheckedChange={(val) => 
          setScrollSettings({...scrollSettings, reverse_vertical: val})
        }
      />
    </div>
    <div className="flex items-center justify-between p-3 border rounded">
      <Label className="text-sm">Reverse Horizontal</Label>
      <Switch
        checked={scrollSettings.reverse_horizontal}
        onCheckedChange={(val) => 
          setScrollSettings({...scrollSettings, reverse_horizontal: val})
        }
      />
    </div>
  </div>
</div>
```

#### Step 5: Testing Strategy

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scroll_calculation() {
        // stick_y = 1.0, speed multiplier = 2.0
        // expected: (1.0 * 2.0 * 10.0) = 20 pixels
        assert_eq!(compute_scroll_amount(1.0, 2.0), 20);
    }

    #[test]
    fn test_scroll_deadzone() {
        // stick_y = 0.04 (below 0.05 deadzone)
        // expected: no scroll
        assert!(should_scroll(0.04) == false);
    }

    #[test]
    fn test_reverse_scroll() {
        // stick_y = 1.0, reversed
        // expected: -20 (inverted)
        let amount = compute_scroll_amount(1.0, 2.0);
        assert_eq!(-amount, apply_reverse(amount));
    }
}
```

---

## Feature 2: Multi-Click Support

### Implementation

#### Step 1: Add Click Modes to Types

**File**: `src/types/gamepad.types.ts`

```typescript
export enum ClickType {
  Left = "left",
  Right = "right",
  Middle = "middle",
  Double = "double",
}

export interface ButtonClickMapping {
  button: GamepadButtonIndex;
  click_type: ClickType;
  double_click_timeout: number; // ms
}

export interface GamepadProfile {
  // ... existing fields ...
  button_click_mappings: ButtonClickMapping[];
}
```

#### Step 2: Backend Click Implementation

**File**: `src-tauri/src/gamepad_manager.rs`

Add click handling to polling loop:

```rust
// Middle click
let lb = gamepad
    .get_button(GamepadButtonIndex::LB)
    .map(|b| b.pressed)
    .unwrap_or(false);

if lb && !last_button_state.get(&(id.into(), GamepadButtonIndex::LB)).unwrap_or(&false) {
    eprintln!("[Click] Middle Click");
    let mut enigo = Enigo::new();
    // Middle click via enigo or direct mouse_event
    let _ = Self::emit_click(ClickType::Middle);
}

// Double click
let rb = gamepad
    .get_button(GamepadButtonIndex::RB)
    .map(|b| b.pressed)
    .unwrap_or(false);

if rb && !last_button_state.get(&(id.into(), GamepadButtonIndex::RB)).unwrap_or(&false) {
    eprintln!("[Click] Double Click");
    let _ = Self::emit_double_click();
}

// Update button state tracking
last_button_state.insert((id.into(), GamepadButtonIndex::LB), lb);
last_button_state.insert((id.into(), GamepadButtonIndex::RB), rb);
```

#### Step 3: Click Emission Function

```rust
enum ClickType {
    Left,
    Right,
    Middle,
}

impl GamepadManager {
    fn emit_click(click_type: ClickType) -> Result<(), String> {
        #[cfg(target_os = "macos")]
        {
            use cocoa::appkit::NSEvent;
            use core_graphics::event::{CGEventType, CGMouseButton};
            
            let button = match click_type {
                ClickType::Left => CGMouseButton::Left,
                ClickType::Right => CGMouseButton::Right,
                ClickType::Middle => CGMouseButton::Center,
            };
            
            let event_type = match click_type {
                ClickType::Left => CGEventType::LeftMouseDown,
                ClickType::Right => CGEventType::RightMouseDown,
                ClickType::Middle => CGEventType::OtherMouseDown,
            };
            
            // Create and post events
            // This requires unsafe code and macOS APIs
        }
        
        Ok(())
    }
    
    fn emit_double_click() -> Result<(), String> {
        // Double click = click down, click up, click down, click up
        // With timing: 10ms between clicks
        Self::emit_click(ClickType::Left)?;
        std::thread::sleep(Duration::from_millis(10));
        Self::emit_click(ClickType::Left)?;
        Ok(())
    }
}
```

---

## Feature 3: Keyboard Emulation Basics

### D-Pad Mapping Implementation

#### Step 1: Define Key Mappings

**File**: `src-tauri/src/keyboard/mappings.rs` (NEW)

```rust
use std::collections::HashMap;

pub enum KeyMapping {
    Single(String), // Single key like "Return"
    Combination(Vec<String>), // Like ["Cmd", "Right"] for Cmd+Right
}

pub fn get_dpad_mappings() -> HashMap<String, KeyMapping> {
    let mut map = HashMap::new();
    
    map.insert("up".to_string(), 
        KeyMapping::Combination(vec!["Fn".to_string(), "Up".to_string()])); // Page Up
    
    map.insert("down".to_string(), 
        KeyMapping::Combination(vec!["Fn".to_string(), "Down".to_string()])); // Page Down
    
    map.insert("left".to_string(), 
        KeyMapping::Combination(vec!["Cmd".to_string(), "[".to_string()])); // Back
    
    map.insert("right".to_string(), 
        KeyMapping::Combination(vec!["Cmd".to_string(), "]".to_string()])); // Forward
    
    map
}

pub fn get_button_key_mappings() -> HashMap<String, KeyMapping> {
    let mut map = HashMap::new();
    
    map.insert("cross".to_string(), KeyMapping::Single("Return".to_string()));
    map.insert("square".to_string(), KeyMapping::Single("Escape".to_string()));
    map.insert("circle".to_string(), KeyMapping::Single("Delete".to_string()));
    
    map
}
```

#### Step 2: Keyboard Emulation Backend

**File**: `src-tauri/src/keyboard/mod.rs` (NEW)

```rust
pub mod mappings;

use enigo::{Enigo, Key, Keyboard};

pub fn press_key(key_name: &str) -> Result<(), String> {
    let mut enigo = Enigo::new();
    let key = match key_name {
        "Return" => Key::Return,
        "Escape" => Key::Escape,
        "Delete" => Key::Delete,
        "Up" => Key::UpArrow,
        "Down" => Key::DownArrow,
        "Left" => Key::LeftArrow,
        "Right" => Key::RightArrow,
        _ => return Err(format!("Unknown key: {}", key_name)),
    };
    
    enigo.key(key, true)?; // Press
    enigo.key(key, false)?; // Release
    Ok(())
}

pub fn press_key_combination(keys: &[&str]) -> Result<(), String> {
    let mut enigo = Enigo::new();
    
    // Press all modifier keys first
    for key_name in &keys[0..keys.len()-1] {
        let key = parse_key(key_name)?;
        enigo.key(key, true)?; // Hold down
    }
    
    // Press the main key
    if let Some(last_key) = keys.last() {
        let key = parse_key(last_key)?;
        enigo.key(key, true)?;
        enigo.key(key, false)?;
    }
    
    // Release all modifiers
    for key_name in &keys[0..keys.len()-1] {
        let key = parse_key(key_name)?;
        enigo.key(key, false)?; // Release
    }
    
    Ok(())
}

fn parse_key(key_name: &str) -> Result<Key, String> {
    match key_name {
        "Cmd" => Ok(Key::Meta),
        "Shift" => Ok(Key::Shift),
        "Alt" => Ok(Key::Alt),
        "Fn" => Ok(Key::Function), // May not exist on all platforms
        "Return" => Ok(Key::Return),
        "Escape" => Ok(Key::Escape),
        "Delete" => Ok(Key::Delete),
        "Up" => Ok(Key::UpArrow),
        "Down" => Ok(Key::DownArrow),
        "Left" => Ok(Key::LeftArrow),
        "Right" => Ok(Key::RightArrow),
        "[" => Ok(Key::LeftBracket),
        "]" => Ok(Key::RightBracket),
        _ => Err(format!("Unknown key: {}", key_name)),
    }
}
```

#### Step 3: Integrate into Gamepad Manager Polling

Add to polling loop in gamepad_manager.rs:

```rust
// D-Pad Button Event Handling
let mut dpad_buttons = vec![
    (self.prev_dpad_up, gamepad.get_button_pressed(GamepadButtonIndex::DPadUp), "up"),
    (self.prev_dpad_down, gamepad.get_button_pressed(GamepadButtonIndex::DPadDown), "down"),
    (self.prev_dpad_left, gamepad.get_button_pressed(GamepadButtonIndex::DPadLeft), "left"),
    (self.prev_dpad_right, gamepad.get_button_pressed(GamepadButtonIndex::DPadRight), "right"),
];

for (prev_state, current_state, direction) in dpad_buttons {
    if !prev_state && current_state {
        // Button just pressed
        if let Some(mapping) = get_dpad_mappings().get(direction) {
            match mapping {
                KeyMapping::Single(key) => {
                    let _ = keyboard::press_key(key);
                }
                KeyMapping::Combination(keys) => {
                    let key_refs: Vec<&str> = keys.iter().map(|s| s.as_str()).collect();
                    let _ = keyboard::press_key_combination(&key_refs);
                }
            }
        }
    }
}
```

---

## Testing Plan for Phase 1

### Manual Testing Checklist

```
Scroll Control:
- [ ] Right stick up scrolls up in browser
- [ ] Right stick down scrolls down
- [ ] Left stick still controls cursor
- [ ] Scroll speed slider changes behavior
- [ ] Reverse scroll option works
- [ ] Horizontal scroll works (for long content)

Multi-Click:
- [ ] LB triggers middle click (paste in browser)
- [ ] RB triggers double click (select word)
- [ ] Left/Right triggers still work normally
- [ ] Can click + scroll simultaneously

Keyboard Emulation:
- [ ] D-Pad up = Page Up
- [ ] D-Pad down = Page Down
- [ ] D-Pad left = Go Back (Cmd+[)
- [ ] D-Pad right = Go Forward (Cmd+])
- [ ] Cross button = Enter
- [ ] Square = Escape
- [ ] Circle = Delete

Profile Persistence:
- [ ] Create new profile with custom scroll settings
- [ ] Close and reopen app
- [ ] Settings persist
```

---

## Deployment Strategy

### Phase 1 Release

1. **Beta Release** (Internal Testing - 1 week)
   - Share with 5-10 developer friends
   - Collect feedback on scroll sensitivity
   - Identify crash scenarios

2. **Early Access Release** (Wider Testing - 1 week)
   - GitHub releases with CHANGELOG
   - Request feedback on features
   - Bug bounty for critical issues

3. **Stable Release** (1.1.0)
   - Merge to main branch
   - Tag release on GitHub
   - Update documentation

---

## Success Criteria for Phase 1

- ✅ Scroll via right stick: Smooth, responsive
- ✅ Click accuracy: No missed clicks
- ✅ Keyboard input: Reliable  
- ✅ No crashes over 30 min continuous use
- ✅ Settings save/restore properly
- ✅ User feedback score: >4.0/5.0

---

## Resources & Dependencies

### New Dependencies to Add

```toml
# Cargo.toml
[dependencies]
# For macOS scroll events
cocoa = "0.25"
core-graphics = "0.23"

# For keyboard simulation improvements
device_state = "0.1"

# For better platform abstraction
cfg_aliases = "0.1"
```

### Useful Links

- macOS Scroll Events: https://developer.apple.com/documentation/coregraphics/cgeventcreatescrollwheelevent
- Enigo Documentation: https://docs.rs/enigo/
- Gamepad API Reference: https://html.spec.whatwg.org/multipage/input.html#gamepad-interface

---

This guide provides everything needed to implement Phase 1 features. Start with scroll control as it's the most straightforward, then move to multi-click support, and finally keyboard emulation.
