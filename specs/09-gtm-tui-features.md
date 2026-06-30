# 09 — gtm-tui: Features (Theme, Keybindings, Kitty, Footer, Icons)

## Theme System

HSL-based theme generation (ported from Nim's catppuccin-inspired algorithm).

### Colors

```rust
#[derive(Debug, Clone)]
pub struct Theme {
    pub rosewater:  Color,
    pub flamingo:   Color,
    pub pink:       Color,
    pub mauve:      Color,
    pub red:        Color,
    pub maroon:     Color,
    pub peach:      Color,
    pub yellow:     Color,
    pub green:      Color,
    pub teal:       Color,
    pub sky:        Color,
    pub sapphire:   Color,
    pub blue:       Color,
    pub lavender:   Color,
    pub text:       Color,
    pub subtext1:   Color,
    pub subtext0:   Color,
    pub overlay2:   Color,
    pub overlay1:   Color,
    pub overlay0:   Color,
    pub surface2:   Color,
    pub surface1:   Color,
    pub surface0:   Color,
    pub base:       Color,
    pub mantle:     Color,
    pub crust:      Color,
}
```

### Presets

| Name | Mode | Notes |
|------|------|-------|
| `catppuccin_mocha` | Dark | Default, all 26 colors |
| `catppuccin_latte` | Light | High contrast light theme |
| `nord` | Dark | Arctic, bluish |
| `gruvbox_dark` | Dark | Warm, retro |
| `gruvbox_light` | Light | Warm light |
| `tokyo_night` | Dark | Deep blue/purple |
| `solarized_dark` | Dark | Low contrast |
| `solarized_light` | Light | Low contrast light |
| `custom(seed)` | Either | Generated deterministically from seed string |

### HSL→RGB

```rust
/// Convert HSL (0-360, 0-100, 0-100) to RGB (0-255)
pub fn hsl_to_rgb(h: f64, s: f64, l: f64) -> (u8, u8, u8);
```

Algorithm:
```
1. s = s / 100, l = l / 100
2. c = (1 - |2l - 1|) * s      (chroma)
3. x = c * (1 - |(h/60) % 2 - 1|)
4. m = l - c/2
5. (r', g', b') = f(h, c, x)   (6-sector mapping)
6. (r, g, b) = (r'+m, g'+m, b'+m) * 255
```

## Keybinding System

```rust
#[derive(Debug, Clone)]
pub struct Keybindings {
    pub bindings: Vec<(KeyEvent, BoundCommand)>,
}

#[derive(Debug, Clone)]
pub struct BoundCommand {
    pub action: KeyboardAction,
    pub contexts: Vec<KeyContext>,
    pub description: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum KeyContext {
    Global,
    Normal,     // browsing tabs
    Filter,     // / or : input mode
    Overlay,    // modal open
    List,       // in a scrollable list
    MoveMode,   // queue reorder mode
}

#[derive(Debug, Clone)]
pub enum KeyboardAction {
    // Tab switching
    NextTab,
    PrevTab,
    SwitchTab(Tab),

    // Cursor
    MoveUp,
    MoveDown,
    PageUp,
    PageDown,
    Top,
    Bottom,

    // Selection / action
    Select,         // Enter
    EnqueueNext,
    EnqueueEnd,
    Delete,
    Move,

    // Filter
    EnterFilter,
    EnterCommand,
    ClearFilter,
    Confirm,

    // Playback
    PlayPause,
    Next,
    Prev,
    VolumeUp,
    VolumeDown,
    SeekForward,
    SeekBackward,
    ToggleShuffle,
    CycleRepeat,
    ToggleMute,

    // Navigation
    Back,

    // Meta
    Quit,
    ReloadConfig,
    ToggleHelp,
    Custom(String),
}

pub fn parse_keycode(name: &str) -> KeyEvent;
```

## Kitty Graphics Protocol

```
┌──────────────────  KittyGraphics  ──────────────────────┐
│                                                           │
│  struct KittyGraphics {                                   │
│      supported: bool,          // probed once on launch  │
│      next_image_id: u32,       // monotonically increas. │
│  }                                                       │
│                                                           │
│  Methods:                                                │
│  ┌───────────────────────────────────────────────────┐  │
│  │ fn probe() -> Self                                │  │
│  │   • Write escape sequence to query KITTY           │  │
│  │     GRAPHICS capability                             │  │
│  │   • Read response from stdin                       │  │
│  │   • Returns KittyGraphics with supported = T/F     │  │
│  ├───────────────────────────────────────────────────┤  │
│  │ fn transmit_image(&mut self, data, mime, id)       │  │
│  │   • Compress PNG if JPEG (Kitty prefers PNG)       │  │
│  │   • Write base64-encoded chunked APC sequence:     │  │
│  │     ESC_Gi=<id>,f=<fmt>,s=<w>,v=<h>,m=1;payload    │  │
│  │     ESC_\                                         │  │
│  ├───────────────────────────────────────────────────┤  │
│  │ fn delete_image(&self, id)                         │  │
│  │   • Write: ESC_Gi=<id>,a=d;ESC_\                  │  │
│  ├───────────────────────────────────────────────────┤  │
│  │ fn place_image(&self, id, x, y, w, h)             │  │
│  │   • Place at pixel/character coordinates:          │  │
│  │     ESC_Gi=<id>,a=p,c=<x>,r=<y>[,C=1];ESC_\       │  │
│  └───────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────┘
```

### Cover Image Flow

```
┌──────────────┐    ┌──────────────┐    ┌──────────────┐
│ Daemon       │    │ DaemonClient │    │ NowPlaying   │
│              │    │              │    │              │
│ Play started─┼───▶│ poll_events  │    │              │
│              │    │ PlaybackStart│───▶│ fetch cover  │
│              │    │  → track     │    │ via reqwest  │
│              │    │              │    │ (or use cache│
│              │    │              │    │  from event) │
│              │    │              │    │              │
│              │    │              │    │ decode image │
│              │    │              │    │ (image crate)│
│              │    │              │    │              │
│              │    │              │    │ KittyGraphics│
│              │    │              │    │ .transmit()  │
│              │    │              │    │ .place()     │
│              │    │              │    │              │
│              │    │              │    │ store id for │
│              │    │              │    │ later delete │
│              │    │              │    │ on next track│
│              │    │              │    │ .deleteOld() │
└──────────────┘    └──────────────┘    └──────────────┘
```

## Footer Modules

```
┌──────────────────────────────────────────────────────────────┐
│  ⏸  1:23 / 4:20  ████████░░░░  28%  │  Vol: 75%  🔀  🔁 All │
│  └──────── left section ─────────┘    └──── right section ───┘│
└──────────────────────────────────────────────────────────────┘
```

```rust
pub enum FooterModule {
    // Left-aligned
    PlaybackStatus,       // ▶ ⏸ ⏹
    TimePosition,         // 1:23 / 4:20
    ProgressBar,          // ████████░░░░
    TimePercent,          // 28%

    // Right-aligned
    Volume,               // Vol: 75%
    ShuffleIndicator,     // 🔀
    RepeatIndicator,      // 🔁 All / 🔂 One
    SleepTimer,           // ⏰ 5:00
    Date,                 // 2024-03-15
    Time,                 // 14:30
    QueueLength,          // Q: 5
    Custom { label: String, value: fn(&AppState) -> String },
}

pub struct FooterBar {
    pub left_modules: Vec<FooterModule>,
    pub right_modules: Vec<FooterModule>,
}

impl FooterBar {
    pub fn render(&self, area: Rect, buf: &mut Buffer, state: &AppState) {
        // Render left modules left-to-right
        // Render right modules right-to-left
        // Padding between them
    }
}
```

## Icons

```rust
pub struct IconSet {
    pub play: &'static str,
    pub pause: &'static str,
    pub stop: &'static str,
    pub next: &'static str,
    pub prev: &'static str,
    pub shuffle: &'static str,
    pub repeat_all: &'static str,
    pub repeat_one: &'static str,
    pub volume_high: &'static str,
    pub volume_low: &'static str,
    pub volume_mute: &'static str,
    pub heart: &'static str,
    pub heart_empty: &'static str,
    pub search: &'static str,
    pub music: &'static str,
    pub album: &'static str,
    pub artist: &'static str,
    pub clock: &'static str,
    pub list: &'static str,
    pub settings: &'static str,
    pub help: &'static str,
    pub youtube: &'static str,
    pub cursor: &'static str,
    pub cursor_sel: &'static str,
}
```

### Nerd Font variant (default)

```rust
pub const NERD_FONT: IconSet = IconSet {
    play:       "\u{23F5}",     // ▶
    pause:      "\u{23F8}",     // ⏸
    stop:       "\u{23F9}",     // ⏹
    next:       "\u{23ED}",     // ⏭
    prev:       "\u{23EE}",     // ⏮
    shuffle:    "\u{1F500}",    // 🔀
    repeat_all: "\u{1F501}",    // 🔁
    repeat_one: "\u{1F502}",    // 🔂
    volume_high:"\u{1F50A}",    // 🔊
    volume_low: "\u{1F509}",    // 🔉
    volume_mute:"\u{1F507}",    // 🔇
    heart:      "\u{2665}",     // ♥
    heart_empty:"\u{2661}",     // ♡
    search:     "\u{1F50D}",    // 🔍
    music:      "\u{266B}",     // ♫
    album:      "\u{1F4BF}",    // 💿
    artist:     "\u{1F3A4}",    // 🎤
    clock:      "\u{1F552}",    // 🕒
    list:       "\u{2630}",     // ☰
    settings:   "\u{2699}",     // ⚙
    help:       "\u{2753}",     // ❓
    youtube:    "\u{25B6}",     // ▶
    cursor:     " \u{2190}",    // ←
    cursor_sel: " \u{25B6}",    // ▶
};
```

### Emoji variant (fallback)

```rust
pub const EMOJI: IconSet = IconSet {
    play:       "▶️",
    pause:      "⏸️",
    // ... same concepts, emoji renderings
};
```
