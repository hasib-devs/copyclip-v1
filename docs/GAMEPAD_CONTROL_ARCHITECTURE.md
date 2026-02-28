# Gamepad Control Architecture: Universal PC Control System

**Document Status:** Strategic Architecture Plan for Phase 2+  
**Created:** 2026-02-25  
**Scope:** Comprehensive gamepad control system with modal editing, motion control, and context awareness

---

## Executive Summary

Transform a gamepad controller into a universal PC control interface by combining insights from:
- **Vim**: Modal editing, leader keys, mnemonics
- **Gaming**: Profile-based keybinding, sensitivity controls, gesture recognition
- **Accessibility**: Switch control patterns, time-based interactions, context-aware mappings

**Key Principle:** Efficiency through **modal layers**, **time-based interactions**, and **application context awareness** to maximize 17 controller buttons.

---

## Table of Contents

1. [Available Resources](#available-resources)
2. [Modal System Architecture](#modal-system-architecture)
3. [Key Combination Patterns](#key-combination-patterns)
4. [Motion Control System](#motion-control-system)
5. [Feature Mapping](#feature-mapping)
6. [Context & Application Profiles](#context--application-profiles)
7. [Implementation Roadmap](#implementation-roadmap)
8. [Type System Design](#type-system-design)

---

## Available Resources

### Controller Hardware Inventory

```
BUTTONS (17 total)
├── Face Buttons (4): Y/Triangle, X/Square, B/Circle, A/Cross
├── Shoulder Buttons (4): LB, RB, LT (as button), RT (as button)
├── D-Pad (4): Up, Down, Left, Right
├── Special (3): Select/Menu, Home/Guide, Left Stick Click, Right Stick Click
└── Stick Axes (4): Left X/Y, Right X/Y
    └── Plus: Triggers as analog axes (0-255 range)

AXES (6 total)
├── Left Stick: X axis (-100 to +100), Y axis (-100 to +100)
├── Right Stick: X axis (-100 to +100), Y axis (-100 to +100)
├── Left Trigger: 0-255 (partial press detection)
└── Right Trigger: 0-255 (partial press detection)

TIME-BASED INTERACTIONS
├── Tap (0-200ms): Quick press
├── Hold (200ms+): Long press
├── Double Tap (0-300ms between taps)
├── Multi-Tap (3x, 4x, etc.)
└── Hold Duration: Variable duration (200ms intervals)

MOTION SENSORS (where available)
├── Gyroscope: 3-axis rotation
├── Accelerometer: 3-axis acceleration
└── Combined: Gesture recognition (shake, tilt, rotation)
```

**Effective Key Count:** 17 buttons × 3 time states (tap/hold/release) = 51 primary actions
Plus: D-Pad contextual (8 directions + hold), Analog triggers (256 levels), Sticks (motion + click)

---

## Modal System Architecture

### Core Modes (Inspired by Vim)

```
┌─────────────────────────────────────────────────────────────────┐
│                         GAMEPAD MODES                           │
├─────────────────────────────────────────────────────────────────┤
│                                                                   │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐           │
│  │  NORMAL      │  │   MOTION     │  │   HOTKEY     │           │
│  │   MODE       │  │    MODE      │  │   MODE       │           │
│  │              │  │              │  │              │           │
│  │ Navigation   │  │ Mouse/Cursor │  │ Key combos   │           │
│  │ Selection    │  │ Precision    │  │ Leader key   │           │
│  │ App control  │  │ movement     │  │ chords       │           │
│  └──────────────┘  └──────────────┘  └──────────────┘           │
│        ↓                 ↓                  ↓                     │
│    [Toggle]         [Toggle]          [Toggle]                  │
│   Hold RB+Y         Hold LB+Y          Hold LT+RT               │
│                                                                   │
└─────────────────────────────────────────────────────────────────┘

Transition Matrix:
- Any mode → Toggle button: Switch to new mode
- Context sensitive: Some modes auto-activate based on active window
- Quick return: Release toggle = return to previous mode
```

### Mode Definitions

#### 1. **NORMAL Mode** (Default)
Primary interface for application control and navigation.

```
PURPOSE: Navigate apps, switch windows, access system features
ACTIVATION: Default mode on startup
KEY FEATURES:
  - App launcher (hold Guide button)
  - Alt+Tab / App Switcher
  - Spotlight/Search launch
  - Volume, brightness controls
  - Screenshot/recording
```

**Button Mapping (NORMAL):**
```
Face Buttons:
  A (Cross)     → Mouse click / Select action
  B (Circle)    → Back / Escape
  X (Square)    → Right-click context menu
  Y (Triangle)  → Open app menu / Launcher

D-Pad:
  Up            → Volume up / Scroll up
  Down          → Volume down / Scroll down
  Left          → Previous app / Window nav left
  Right         → Next app / Window nav right

Triggers:
  LT (full)     → Mouse left-click
  RT (full)     → Mouse right-click
  LT + RT       → Mouse middle-click

Bumpers:
  LB            → Alt key (for shortcuts)
  RB            → Ctrl key (for shortcuts)
  LB + RB       → Shift key

Sticks:
  Left          → Mouse movement
  Right         → Scroll (already implemented)
  LStick-Click  → Window snap/tile
  RStick-Click  → Screenshot

Special:
  Select        → Settings/Preferences
  Menu/Guide    → Hold: App Launcher
  
Mode Switching:
  RB + Y (hold) → Enter MOTION mode
  LB + Y (hold) → Enter HOTKEY mode
```

#### 2. **MOTION Mode** (Precision Control)
Mouse/cursor control with sensitivity options and precision targeting.

```
PURPOSE: Precise cursor control, drawing, GUI manipulation
ACTIVATION: Hold RB + Y
KEY FEATURES:
  - Both sticks for cursor movement
  - Acceleration profiles
  - Sensitivity adjustment
  - Grid snapping (for precise UI interaction)
  - Gesture support (swipe, circle, etc.)
```

**Button Mapping (MOTION):**
```
Left Stick:
  Movement      → Move cursor (variable speed based on hold)
  Click         → Mouse click
  Hold + Tap    → Drag operation

Right Stick:
  Movement      → Fine adjustment (slower, more precise)
  Click         → Right-click
  Hold + Tap    → Custom action (profile-dependent)

Face Buttons:
  A (Cross)     → Click (confirm)
  B (Circle)    → Escape / Cancel
  X (Square)    → Right-click
  Y (Triangle)  → Double-click

D-Pad:
  Up/Down       → Scroll vertical (precision)
  Left/Right    → Scroll horizontal (precision)

Triggers:
  LT            → Hold: Drag mode
  RT            → Hold: Slow mode (0.5x speed)
  LT + RT       → Auto-center cursor to screen middle

Bumpers:
  LB            → Alt (for system interactions)
  RB            → Ctrl (for selections)

Special:
  Select        → Adjust sensitivity (cycle through 3 levels)
  Menu          → Grid snap toggle
  RB + Y again  → Exit MOTION mode
```

**Sensitivity Profiles:**
```
Level 1 (Slow):   0.5x, 5 DPI equivalent
Level 2 (Normal): 1.0x, 10 DPI equivalent
Level 3 (Fast):   2.0x, 20 DPI equivalent
Level 4 (Turbo):  3.0x, 40 DPI equivalent

Toggle via: Select button cycles through levels
Display: On-screen indicator in MOTION mode
```

#### 3. **HOTKEY Mode** (Key Combinations & Commands)
Complex key sequences, leader key patterns, and modal chords.

```
PURPOSE: Execute keyboard shortcuts, complex sequences, app-specific commands
ACTIVATION: Hold LB + Y
KEY FEATURES:
  - Leader key pattern (press leader, then key)
  - Key chords (simultaneous button press)
  - Sequential combos
  - App-specific command palette
```

**Button Mapping (HOTKEY):**
```
LEADER KEY PATTERN:
  Primary Leader: Hold Face Button (Y/Triangle)
  Then Tap: Other buttons to execute commands

Example Sequences:
  Y + A       → Cmd+A (Select All)
  Y + C       → Cmd+C (Copy)
  Y + V       → Cmd+V (Paste)
  Y + Z       → Cmd+Z (Undo)
  Y + Shift+Z → Cmd+Shift+Z (Redo)

CHORD PATTERNS (simultaneous press):
  LB + X      → Cmd+X (Cut)
  LB + S      → Cmd+S (Save)
  LB + W      → Cmd+W (Close Tab)
  LB + Q      → Cmd+Q (Quit App)
  
  RB + N      → Cmd+N (New)
  RB + T      → Cmd+T (New Tab)
  RB + O      → Cmd+O (Open)

D-PAD HOTKEYS:
  Up + Hold LB    → Page Up
  Down + Hold LB  → Page Down
  Left + Hold LB  → Home key
  Right + Hold LB → End key
  
  Up + Hold RB    → Window snap top
  Down + Hold RB  → Window snap bottom
  Left + Hold RB  → Window snap left
  Right + Hold RB → Window snap right

MODIFIER COMBINATIONS:
  LB (Hold)  → Alt modifier
  RB (Hold)  → Ctrl modifier
  LB+RB      → Shift modifier
  LB+RB+X    → Ctrl+Shift+X (complex combo)

TRIGGER HOTKEYS:
  LT (full)  → F1 (Help)
  RT (full)  → F12 (Developer Tools)

Special:
  Select + D-Pad Up    → Brightness up
  Select + D-Pad Down  → Brightness down
  Select + D-Pad Left  → Media Previous
  Select + D-Pad Right → Media Next
  
  LB + Y again       → Exit HOTKEY mode
```

---

## Key Combination Patterns

### Pattern Hierarchy

```
LEVEL 1: SINGLE BUTTON
├── Tap (0-200ms)
├── Hold (200-500ms)
└── Long Hold (500ms+)

LEVEL 2: MODIFIER + BUTTON
├── LB (Alt) + Button
├── RB (Ctrl) + Button
├── LB + RB (Shift) + Button
└── Trigger + Button

LEVEL 3: LEADER PATTERN
├── Hold Leader (Y button) + Tap Button
├── Sequence: Leader then secondary
└── App-specific leaders

LEVEL 4: CHORD PATTERN
├── Simultaneous press (within 100ms window)
├── Example: LB + X = Cut
└── Limited to practical combinations

LEVEL 5: SEQUENCE PATTERN
├── First button: Context setter
├── Second button: Action
├── Timeout: 2 seconds between presses
└── Example: Y then X = custom action
```

### Implementation Strategy

```rust
// Pseudo-code structure
enum InputPattern {
    SingleTap(Button),
    Hold(Button, Duration),
    Chord(Vec<Button>),
    LeaderKey(Button, Button),
    Sequence(Vec<Button>, TimeWindow),
}

enum Modifier {
    Alt,     // LB
    Ctrl,    // RB
    Shift,   // LB + RB
    None,
}

struct KeyBinding {
    pattern: InputPattern,
    modifier: Modifier,
    action: Action,
    priority: u8,
    context: Option<ApplicationContext>,
}
```

---

## Motion Control System

### Stick Movement Logic

#### Movement Phases

```
ACCELERATION CURVE:
  
  Speed
    │     
    │        ╱╱╱╱  Turbo (3x)
    │      ╱╱╱╱    
    │    ╱╱╱╱      Normal (1x)
    │  ╱╱╱╱        
    │╱╱╱╱          Slow (0.5x)
    └──────────────────► Stick Deflection %
         Zones: 0%  25%  50%  75%  100%

Algorithm:
  if deflection < 25%:
    speed = 0.5x
  elif deflection < 50%:
    speed = 0.8x
  elif deflection < 75%:
    speed = 1.0x
  else:
    speed = 2.0x
```

#### Dead Zone & Ramp

```
DEAD ZONE: 0-15% stick deflection = no movement
RAMP ZONE: 15-30% = gradual acceleration
MOVEMENT ZONE: 30-100% = full speed range

This prevents jitter and provides predictable movement.
```

#### Motion Types

```
1. CONTINUOUS MOTION (Analog)
   └─ Smooth cursor following
   └─ For: Mouse movement, scrolling, volume control
   
2. DISCRETE MOTION (Grid-based)
   └─ Snap to UI elements
   └─ For: Menu navigation, file selection
   
3. GESTURE MOTION (Tilt-based)
   └─ Shake detection (for screenshots, undo)
   └─ Tilt detection (for rotation)
   └─ Swipe patterns (for navigation)
```

### Gesture Recognition

```
AVAILABLE GESTURES (via gyro/accelerometer):
├── Shake     → Screenshot / Quick save
├── Tilt Up   → Increase brightness
├── Tilt Down → Decrease brightness
├── Roll Left → Previous
├── Roll Right→ Next
└── Circular  → Volume control

DETECTION THRESHOLDS:
├── Shake: Acceleration > 2.5G for 200ms
├── Tilt: Consistent rotation > 45° for 500ms
└── Gesture timeout: 2 seconds

FILTERING:
├── Exponential moving average for smooth response
├── Duplicate prevention (ignore similar gestures within 1s)
└── Intensity levels (light shake vs hard shake)
```

---

## Feature Mapping

### System-Level Controls (Always Available)

```
LEVEL 1: ALWAYS ACTIVE (From NORMAL mode)
├── Volume Control
│   ├── D-Pad Up    → Volume +10%
│   ├── D-Pad Down  → Volume -10%
│   └── Hold Y+Up   → Volume +5% (fine control)
│
├── Brightness
│   ├── Select + Up → Brightness +10%
│   └── Select + Down → Brightness -10%
│
├── Media Control
│   ├── Select + Left  → Previous Track
│   ├── Select + Right → Next Track
│   ├── Select + A     → Play/Pause
│   └── Select + B     → Stop
│
├── Screenshot/Recording
│   ├── LStick Click → Screenshot
│   ├── LStick Click + Hold → Record
│   └── RStick Click → Screenshot with annotation
│
├── App Switcher (Alt+Tab)
│   ├── LB + Left  → Previous app
│   ├── LB + Right → Next app
│   └── Hold LB + Tap Right/Left → Show app list
│
├── Spotlight/Search
│   ├── LB + S    → Open Search
│   └── LB + /    → Open Help/Search
│
└── System Power
    ├── LB + LT + RT → Sleep
    └── LB + LT + RT + A → Shutdown (with confirmation)
```

### Application-Level Features (Context-Aware)

```
BROWSER PROFILE
├── D-Pad Left  → Back button
├── D-Pad Right → Forward button
├── X           → Reload page
├── LB + T      → New Tab
├── LB + W      → Close Tab
├── LB + ]      → Next Tab
├── LB + [      → Previous Tab
├── LB + Shift+T → Reopen Closed Tab
└── RB + F      → Find in page

TEXT EDITOR PROFILE
├── NORMAL mode:
│   ├── D-Pad Direction → Navigate
│   ├── A → Enter/Select
│   ├── B → Escape/Cancel
│   └── Y → Command Palette
│
├── HOTKEY mode:
│   ├── Leader + A → Select All (Cmd+A)
│   ├── Leader + C → Copy (Cmd+C)
│   ├── Leader + V → Paste (Cmd+V)
│   ├── Leader + Z → Undo (Cmd+Z)
│   ├── Leader + Shift+Z → Redo (Cmd+Shift+Z)
│   ├── Leader + S → Save (Cmd+S)
│   ├── Leader + / → Comment (Cmd+/)
│   └── LB + I → Open file browser
│
└── MOTION mode:
    └─ Cursor for clicking/selecting

FILE MANAGER PROFILE
├── D-Pad Up/Down    → Navigate files
├── D-Pad Left       → Go back / collapse folder
├── D-Pad Right      → Enter folder / expand
├── A                → Open file / action
├── B                → Cancel / Back
├── X                → Properties / Info
├── Y                → New file/folder
├── LB + X           → Delete
├── LB + C           → Copy
├── LB + X           → Cut
└── LB + V           → Paste

TERMINAL/CLI PROFILE
├── Right Stick → Mouse (for pasting)
├── LB + A     → Clear screen (Ctrl+L)
├── LB + C     → Cancel (Ctrl+C)
├── LB + D     → EOF (Ctrl+D)
├── LB + U     → Clear line (Ctrl+U)
└── LB + R     → Reverse search (Ctrl+R)
```

---

## Context & Application Profiles

### Profile System Architecture

```
PROFILE HIERARCHY:
┌─────────────────────────────────────────┐
│        Global Defaults                  │
│        (All apps)                       │
├─────────────────────────────────────────┤
│  OS-Specific Overrides                  │
│  (macOS defaults, Windows defaults)     │
├─────────────────────────────────────────┤
│  Category Profiles                      │
│  (Browser, Editor, Terminal, etc.)      │
├─────────────────────────────────────────┤
│  Application-Specific                   │
│  (Chrome, VS Code, iTerm2)              │
└─────────────────────────────────────────┘

MATCHING PRIORITY (top to bottom):
1. Exact app match (focused window bundle ID)
2. App category match (detected via heuristics)
3. OS-specific default
4. Global default
```

### Active Window Detection

```
MACOS:
  Accessibility API → Get front most app bundle ID
  Example: com.google.Chrome
  Fallback: Process name

WINDOWS:
  Windows API → GetForegroundWindow() → GetWindowTitle()
  Registry check for executable name
  
LINUX:
  X11: _NET_ACTIVE_WINDOW property
  Wayland: DBus session properties
```

### Profile Management

```yaml
# profiles.yaml structure

global:
  default_mode: NORMAL
  stick_sensitivity: 1.0x
  dead_zone: 15%
  mappings: [...]

os_specific:
  macos:
    cmd_button: RB  # Cmd = Ctrl button
    alt_button: LB
  windows:
    cmd_button: null  # N/A
    alt_button: LB
  linux:
    cmd_button: null
    alt_button: LB

categories:
  browser:
    inherits: global
    app_list:
      - com.google.Chrome
      - org.mozilla.firefox
      - com.apple.Safari
    mappings:
      D-Pad-Left:
        action: browser_back
        key_sequence: [Alt, Left]

  text_editor:
    inherits: global
    app_list:
      - com.microsoft.VSCode
      - com.sublimetext.3
      - JetBrains.*
    sensitive_features:
      - bracket_matching
      - multicursor_support

applications:
  com.google.Chrome:
    inherits: categories.browser
    mappings:
      RB+T:
        action: new_tab
        custom: true
  
  com.microsoft.VSCode:
    inherits: categories.text_editor
    mappings:
      Y: command_palette
      LB+Shift+P: command_palette
```

---

## Key Click & Selection Patterns

### Click Type System

```
CLICK TYPES AVAILABLE:
├── Single Click      (already implemented via RT)
├── Double Click      (already implemented via RB)
├── Triple Click      (select line in text)
├── Long Press        (context menu equivalent)
├── Drag Select       (click + hold stick movement)
├── Multi-Select      (Shift + click, Cmd + click)
└── Precision Click   (in MOTION mode)

IMPLEMENTATION:
┌─ LT (full) ────────────→ Left Click
├─ RT (full) ────────────→ Right Click
├─ LB + LT ─────────────→ Middle Click / Paste
├─ RB (hold) + A ──────→ Double Click
├─ LB + RB (hold) + A ─→ Triple Click
├─ LT (hold 500ms) ────→ Long Click (drag mode)
└─ Stick Movement (while holding LT) → Drag
  └─ Release LT → Drop
```

### Selection Logic

```
TEXT SELECTION:
├── Click position → Place cursor
├── LB + Click → Shift-click (extend selection)
├── RB + Click → Cmd/Ctrl-click (multiple selection)
└── Drag while LT held → Drag select

UI SELECTION:
├── A button → Select/Activate
├── B button → Deselect/Cancel
├── LB + A → Shift+A (extend selection)
└── RB + A → Cmd/Ctrl+A (alternative select)
```

---

## Implementation Roadmap

### Phase 1 (✅ Completed)
- [x] Basic cursor movement (left stick)
- [x] Click support (RT, LT, LB, RB for different click types)
- [x] Scroll support (right stick)
- [x] Keyboard emulation (D-Pad, face buttons)
- [x] Cross-platform foundation

### Phase 2: Modal System (Priority 1)

**Goal:** Implement mode switching and NORMAL mode system controls

```
TASKS:
1. Create mode state management
   └─ Add Mode enum to state
   └─ Implement mode transitions
   └─ Add visual indicator (on-screen)

2. Extend keybinding resolver
   └─ Map button → (mode, context) → action
   └─ Priority-based resolution

3. Implement NORMAL mode defaults
   └─ App launcher (Guide button)
   └─ Alt+Tab switcher
   └─ Volume/Brightness controls
   └─ Media controls
   └─ Screenshot/Recording

4. Create MOTION mode
   └─ Cursor precision controls
   └─ Sensitivity profiles
   └─ Fine-tuning UI

ESTIMATED TIME: 2-3 weeks
BLOCKERS: None (foundation exists)
```

### Phase 2.5: Active Window Detection

```
TASKS:
1. Implement platform-specific window detection
   └─ macOS: Accessibility API
   └─ Windows: Windows API
   └─ Linux: X11/Wayland

2. Create app categorization
   └─ Bundle ID → Category mapping
   └─ Heuristic detection (fallback)

3. Add profile loading system
   └─ YAML parser for profiles
   └─ Hot-reload capability

ESTIMATED TIME: 1-2 weeks
INTEGRATION: Between Phase 2 and Phase 3
```

### Phase 3: Application Profiles (Priority 2)

**Goal:** Context-aware keybinding for popular applications

```
TASKS:
1. Create browser profile
   └─ Back/forward buttons
   └─ Tab management
   └─ Reload, find

2. Create text editor profile
   └─ Command palette access
   └─ File operations
   └─ Git integration (if applicable)

3. Create file manager profile
   └─ Navigation
   └─ File operations
   └─ Folder browsing

4. Create terminal profile
   └─ Command history
   └─ Signal handling
   └─ Copy/paste

ESTIMATED TIME: 2-3 weeks
MILESTONE: Full workflow support for common apps
```

### Phase 4: HOTKEY Mode & Gesture Recognition (Priority 3)

**Goal:** Complex key sequences and gesture support

```
TASKS:
1. Implement leader key pattern
   └─ Hold Y + tap buttons
   └─ Configurable mappings
   └─ Visual feedback

2. Implement chord detection
   └─ Simultaneous button detection
   └─ Timeout handling
   └─ Conflict resolution

3. Add gesture recognition
   └─ Shake detection
   └─ Tilt detection
   └─ Gesture mappings

4. Create HOTKEY mode UI
   └─ Leader key hint display
   └─ Key combination reference

ESTIMATED TIME: 2 weeks
```

### Phase 5: Advanced Features (Optional)

```
- Macro recording and playback
- Motion-based drawing/annotation
- Voice command integration
- Custom profile editor GUI
- Multi-device profile sync
- Performance analytics
- Accessibility mode enhancements
```

---

## Type System Design

### Rust Types

```rust
// Profile and binding types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GamepadMode {
    Normal,
    Motion,
    Hotkey,
}

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
pub enum GamepadButton {
    // Face buttons
    A, B, X, Y,
    // Shoulders
    LB, RB,
    // Triggers (as buttons)
    LT, RT,
    // D-Pad
    DPadUp, DPadDown, DPadLeft, DPadRight,
    // Special
    Select, Guide,
    // Stick clicks
    LStickClick, RStickClick,
}

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
pub enum Modifier {
    None,
    Alt,
    Ctrl,
    Shift,
    AltCtrl,
    AltShift,
    CtrlShift,
    AltCtrlShift,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub enum InputPattern {
    SingleTap(GamepadButton),
    Hold(GamepadButton, Duration),
    DoubleTap(GamepadButton),
    Chord(Vec<GamepadButton>),
    LeaderKey(GamepadButton, GamepadButton),
    Sequence(Vec<GamepadButton>, Duration),
}

#[derive(Debug, Clone)]
pub struct KeyBinding {
    pub pattern: InputPattern,
    pub modifier: Modifier,
    pub action: Action,
    pub context: Option<ApplicationContext>,
    pub priority: u8,
}

#[derive(Debug, Clone)]
pub enum Action {
    // Mouse actions
    MouseMove(i32, i32),
    MouseClick(MouseButton),
    MouseScroll(i32, i32),
    
    // Keyboard actions
    KeyPress(Key),
    KeyCombo(Vec<Key>),
    TextInput(String),
    
    // System actions
    AppLauncher,
    AppSwitcher,
    VolumeControl(i32),
    BrightnessControl(i32),
    Screenshot,
    Recording(bool),
    
    // Mode actions
    SwitchMode(GamepadMode),
    ModeToggle,
    
    // Application actions
    BrowserBack,
    BrowserForward,
    ReloadPage,
    NewTab,
    CloseTab,
    FindInPage,
    
    // Custom
    Custom(String),
}

#[derive(Debug, Clone)]
pub struct ApplicationContext {
    pub bundle_id: String,
    pub category: AppCategory,
    pub sensitivity_override: Option<f32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AppCategory {
    Browser,
    TextEditor,
    Terminal,
    FileManager,
    IDE,
    Media,
    Chat,
    Other(String),
}

#[derive(Debug, Clone)]
pub struct GamepadProfile {
    pub name: String,
    pub mode: GamepadMode,
    pub bindings: HashMap<InputPattern, KeyBinding>,
    pub stick_sensitivity: f32,
    pub dead_zone: u8,
    pub acceleration_curve: AccelerationCurve,
}

#[derive(Debug, Clone)]
pub struct AccelerationCurve {
    pub name: String,
    pub low: f32,      // Speed at 25% deflection
    pub medium: f32,   // Speed at 50% deflection
    pub high: f32,     // Speed at 75% deflection
    pub max: f32,      // Speed at 100% deflection
}
```

### TypeScript Types

```typescript
// Frontend types matching Rust
export enum GamepadMode {
  Normal = "NORMAL",
  Motion = "MOTION",
  Hotkey = "HOTKEY",
}

export enum GamepadButton {
  A = "A", B = "B", X = "X", Y = "Y",
  LB = "LB", RB = "RB", LT = "LT", RT = "RT",
  DPadUp = "DPAD_UP", DPadDown = "DPAD_DOWN",
  DPadLeft = "DPAD_LEFT", DPadRight = "DPAD_RIGHT",
  Select = "SELECT", Guide = "GUIDE",
  LStickClick = "LSTICK_CLICK", RStickClick = "RSTICK_CLICK",
}

export interface InputPattern {
  type: "tap" | "hold" | "double_tap" | "chord" | "leader" | "sequence";
  buttons: GamepadButton[];
  duration?: number;
}

export interface KeyBinding {
  pattern: InputPattern;
  modifier: Modifier;
  action: Action;
  context?: ApplicationContext;
  priority: number;
}

export interface GamepadProfile {
  name: string;
  mode: GamepadMode;
  bindings: Record<string, KeyBinding>;
  stickSensitivity: number;
  deadZone: number;
  accelerationCurve: AccelerationCurve;
}

export interface ControlsState {
  currentMode: GamepadMode;
  activeProfile: GamepadProfile;
  lastButtonPress?: GamepadButton;
  pressTimestamp?: number;
  heldButtons: Set<GamepadButton>;
  leaderKeyActive?: boolean;
  motionSensitivityLevel: 0 | 1 | 2 | 3;
}
```

---

## Design Principles

### 1. **Discoverability**
- On-screen hints for mode-specific controls
- Persistent legend showing current key bindings
- Tutorial mode for new users
- Context-sensitive help

### 2. **Consistency**
- Same button = same action across modes when possible
- Predictable mode transitions
- Clear visual feedback for mode changes
- Consistent terminology (button names, action names)

### 3. **Efficiency**
- Most common actions require single press
- Secondary actions use modifiers (LB/RB as Alt/Ctrl)
- Frequently used patterns get prime real estate
- Minimal mode switching for typical workflows

### 4. **Ergonomics**
- Avoid awkward button combinations (no 4-button chords)
- Trigger buttons for sustained actions (drag, scroll)
- Sticks for continuous motion (cursor, volume)
- D-Pad for discrete navigation

### 5. **Extensibility**
- Profile system allows per-app customization
- Keybinding resolver supports plugins
- Action system easily extensible
- Mode system allows new modes
- Custom action support

### 6. **Accessibility**
- Adjustable timing (hold, double-tap durations)
- Alternative access methods (voice, gesture)
- Colorblind-friendly mode indicators
- High contrast UI option

---

## Vim Inspiration Analysis

### What Works from Vim

```
✅ Modal editing: Different modes for different tasks
   → Apply to: NORMAL, MOTION, HOTKEY modes

✅ Leader key patterns: Mnemonic key sequences
   → Apply to: Y(leader) + C(copy), Y + V(paste)

✅ Composability: Combine operators with motions
   → Apply to: Modifier + Button combinations

✅ Mnemonics: Keys tied to commands (c=copy, d=delete)
   → Apply to: X(cut), C(copy), V(paste), S(save)

❌ Home row bias: HJKL navigation
   ❌ Not applicable: Gamepad has different key distribution
   
```

### What Works from Gaming

```
✅ Sensitivity profiles: Adjust for different situations
   → Apply to: MOTION mode sensitivity levels

✅ Context profiles: Different settings per game
   → Apply to: App-specific profiles

✅ Gesture recognition: Shake, tilt, rotation
   → Apply to: Screenshot, undo, navigation shortcuts

✅ Real-time feedback: Haptics, visual cues
   → Apply to: Mode changes, action confirmation

```

### What Works from Accessibility

```
✅ Switch control: Time-based interactions (hold, tap, dwell)
   → Apply to: Hold duration for different actions

✅ Dwell clicking: Extended hold triggers action
   → Apply to: Long press = right-click

✅ Voice commands: Alternative input method
   → Apply to: Future voice integration

✅ Large touch targets: Easy to hit buttons
   → Apply to: Generous dead zones, large UI elements

```

---

## Conflict Resolution & Priority System

### Binding Conflict Hierarchy

```
Priority Scale (Higher = Takes Precedence):
  
100 - Exact application match (com.microsoft.VSCode)
80  - Application category match (TextEditor)
60  - OS-specific default
40  - Global default
20  - Fallback action

Resolution Algorithm:
  1. Check exact app match
  2. If not found: Check category
  3. If not found: Check OS default
  4. If not found: Check global default
  5. If not found: Use fallback or no-op
  
Example:
  Input: (A button in VS Code)
  
  1. Check: VSCode exact match
     ✓ Found: "Open command palette"
     → Use this binding (priority 100)
```

### Time-Based Conflict Resolution

```
PATTERN AMBIGUITY: Single tap vs start of hold

Resolution:
  Tap detection: < 100ms + release
  Hold detection: > 100ms held
  
Timeline:
  0ms ──┬────── 100ms ──────┬─→ 500ms
       │                    │
    Press              Threshold
    Start              for hold
                       decision
```

---

## Configuration Example

### User Profile File

```yaml
# ~/.yinvim/profiles/default.yaml

name: "Default Controller Setup"
os: macos

modes:
  normal:
    enabled: true
    description: "Navigate and control applications"
    
    key_bindings:
      - button: A
        action: mouse_left_click
        description: "Click / Select"
      
      - button: B
        description: "Back / Escape"
        action: key_press
        key: Escape
      
      - button: Y
        modifiers: [RB]
        type: hold
        duration: 200
        action: switch_mode
        mode: HOTKEY
        description: "Enter HOTKEY mode"
      
      - button: Guide
        type: hold
        duration: 500
        action: app_launcher
        description: "Hold to show app launcher"
      
      - buttons: [D-Pad, Left]
        action: app_previous
        description: "Previous app (Alt+Tab backward)"
      
      - buttons: [D-Pad, Right]
        action: app_next
        description: "Next app (Alt+Tab)"
      
      - buttons: [D-Pad, Up]
        action: volume_up
        value: 10
        description: "Volume up by 10%"

  motion:
    enabled: true
    stick_sensitivity: 1.0
    accelerations:
      - deflection: 0-25%
        speed: 0.5x
      - deflection: 25-50%
        speed: 0.8x
      - deflection: 50-75%
        speed: 1.0x
      - deflection: 75-100%
        speed: 2.0x

  hotkey:
    enabled: true
    leader_key: Y
    key_bindings:
      - leader_sequence: [Y, A]
        action: key_combo
        keys: [Cmd, A]
        description: "Select All"
      
      - leader_sequence: [Y, C]
        action: key_combo
        keys: [Cmd, C]
        description: "Copy"
      
      - buttons: [LB, X]
        action: key_combo
        keys: [Cmd, X]
        description: "Cut"

application_profiles:
  - name: "Chrome Browser"
    app_bundle_id: "com.google.Chrome"
    inherits: "Browser"
    overrides:
      - button: D-Pad-Left
        action: browser_back
      
      - button: D-Pad-Right
        action: browser_forward
      
      - buttons: [LB, T]
        action: browser_new_tab
  
  - name: "VS Code"
    app_bundle_id: "com.microsoft.VSCode"
    inherits: "TextEditor"
    overrides:
      - button: Y
        action: command_palette
      
      - buttons: [LB, Shift, P]
        action: command_palette
        description: "Alternative command palette"

gestures:
  enabled: true
  mappings:
    - gesture: shake
      action: screenshot
      description: "Shake controller to take screenshot"
    
    - gesture: tilt_forward
      action: brightness_up
      description: "Tilt forward to increase brightness"
    
    - gesture: tilt_backward
      action: brightness_down
      description: "Tilt backward to decrease brightness"
```

---

## Testing & Validation

### Test Matrix

```
COMBINATIONS TO TEST:

Control Types:
  ✓ Single button presses
  ✓ Modifier combinations (LB, RB, LB+RB)
  ✓ Chord detection
  ✓ Leader sequences
  ✓ Time-based (tap vs hold)
  ✓ Stick movements (all angles + speed)
  ✓ Trigger analog levels
  ✓ Gesture recognition

Modes:
  ✓ Mode transitions
  ✓ Mode-specific behaviors
  ✓ Mode fallbacks
  ✓ Mode persistence

Applications:
  ✓ Browser (Chrome, Firefox, Safari)
  ✓ Text Editor (VS Code, Sublime, JetBrains)
  ✓ Terminal (iTerm2, Terminal.app, zsh)
  ✓ File Manager (Finder, Explorer, Nautilus)
  ✓ IDE (Xcode, VS Code, IntelliJ)

Edge Cases:
  ✓ Rapid button mashing
  ✓ Simultaneous stick + button input
  ✓ Mode switching during chord
  ✓ Application switching mid-action
  ✓ Gamepad disconnect/reconnect
  ✓ Profile loading errors
  ✓ Conflicting bindings
```

### Validation Checklist

```
FUNCTIONALITY:
  □ All buttons detected and mapped correctly
  □ Modifier combinations work reliably
  □ Mode transitions smooth and responsive
  □ Profile loading works
  □ App detection working
  □ Actions execute successfully

PERFORMANCE:
  □ Input latency < 50ms
  □ CPU usage < 5% idle
  □ Memory stable (no leaks)
  □ 60 FPS gamepad polling maintained
  □ Profile switching < 200ms

UX:
  □ Mode changes visible on screen
  □ Keybinding hints appear correctly
  □ Error messages clear
  □ Tutorial helpful
  □ Learning curve reasonable

CROSS-PLATFORM:
  □ macOS: All features working
  □ Windows: All features working
  □ Linux: All features working
  □ Controller detection reliable
  □ Profile sync functional
```

---

## Performance Considerations

### Polling & Input Latency

```
CURRENT STATE:
  Polling Interval: 16ms (60 FPS)
  Acceptable Latency: < 50ms
  
BREAKDOWN:
  ├─ Gamepad poll: 1-2ms
  ├─ Input pattern matching: 2-3ms
  ├─ Action resolution: 1-2ms
  ├─ enigo execution: 2-5ms
  │  └─ macOS mouse_move: ~2-4ms
  │  └─ Windows SetCursorPos: ~1-2ms
  └─ Total: ~9-17ms (well under 50ms budget)

OPTIMIZATION OPPORTUNITIES:
  1. Multithread polling in separate worker
  2. Prebuild pattern matching trees
  3. Cache application context
  4. Use double buffering for state
```

### Memory Usage

```
PROFILE IN-MEMORY FOOTPRINT:

Baseline: ~500KB
  ├─ Core state: ~50KB
  ├─ Button state: ~10KB
  ├─ Loaded profiles: ~200KB
  ├─ Pattern cache: ~100KB
  └─ Gesture recognition: ~140KB

Per Additional Profile: ~50-100KB
Target: < 50MB for all loaded profiles (easily achievable)
```

---

## Future Enhancements

### Planned Features (Post-Phase 4)

```
MACRO SYSTEM:
  ├─ Record button sequences
  ├─ Replay with variable speed
  ├─ Save/load macros
  └─ Per-app macro libraries

VOICE INTEGRATION:
  ├─ Voice command recognition
  ├─ Spoken action confirmation
  └─ Custom voice commands

HAPTIC FEEDBACK (if supported):
  ├─ Confirmation vibration
  ├─ Mode change rumble
  ├─ Action completion feedback
  └─ Error warning pattern

ADVANCED GESTURES:
  ├─ Drawing/annotation
  ├─ Handwriting recognition
  ├─ Complex motion patterns
  └─ Custom gesture recording

CLOUD SYNC:
  ├─ Profile backup to cloud
  ├─ Multi-device sync
  ├─ Shared profile library
  └─ Community profiles
```

---

## Conclusion

This architecture transforms a gamepad with 17 buttons into a comprehensive PC control interface by:

1. **Modal System**: Similar to Vim, different modes for different tasks
2. **Time-Based Interactions**: Hold duration, double-tap, tap+hold combinations
3. **Modifiers as Operators**: LB/RB as Alt/Ctrl for standard shortcuts
4. **Context Awareness**: App-specific profiles with sensible defaults
5. **Gesture Recognition**: Motion-based shortcuts (shake, tilt, etc.)
6. **Extensibility**: Plugin architecture for custom actions

**Key Metrics:**
- ~51 primary actions from 17 buttons
- <20ms total input latency
- Modal system ensures discoverability
- Vim-inspired mnemonics for efficiency
- Accessible alternatives for all actions

**Next Steps:**
1. Implement Phase 2: Modal system & NORMAL mode
2. Add Phase 2.5: Active window detection  
3. Build Phase 3: Application profiles
4. Add Phase 4: HOTKEY mode & gestures
5. Expand Phase 5: Macros, voice, haptics

This document serves as the complete blueprint for universal PC control via gamepad.
