# 08 — gtm-tui: Overlays

## Purpose

Overlays are modal widgets rendered on top of the active tab. They capture keyboard input
until dismissed. Examples: command palette, fuzzy finder, track detail, confirm dialog.

## Overlay Enum

```rust
pub enum Overlay {
    CommandPalette(CommandPaletteState),
    FuzzyFinder(FuzzyFinderState),
    QueuePicker(QueuePickerState),
    ThemePicker(ThemePickerState),
    Confirm(ConfirmState),
    TrackDetail(TrackDetailState),
}
```

---

## Command Palette (`:`)

```
┌──────────────────────────────────────────────────────────────────┐
│  :play some song                                       (28/100)  │
├──────────────────────────────────────────────────────────────────┤
│  > play some song                                          🎵   │
│  play track "some" from library                           ▶     │
│  play album "Some Album"                                         │
│  search YouTube for "some song"                                  │
│  add "some song" to queue                                        │
│  play "some other song" from favourites                          │
│  scan music directory                                            │
│  shuffle all tracks                                              │
│  set volume 75                                                   │
│  create playlist "new mix"                                       │
│  ...                                                             │
│                                                                  │
│  Available commands:                                             │
│    play <query>        play search library                       │
│    pause               pause playback                            │
│    stop                stop playback                             │
│    next                next track                                │
│    prev                previous track                            │
│    volume <0-100>      set volume                                │
│    shuffle             toggle shuffle                            │
│    repeat <off|one|all> set repeat mode                          │
│    crossfade <secs>    set crossfade duration                    │
│    sleep <min>         set sleep timer                           │
│    scan <path>         scan directory into library               │
│    search <query>      search tracks                             │
│    quit                quit gtm                                  │
└──────────────────────────────────────────────────────────────────┘
```

### State

```rust
pub struct CommandPaletteState {
    pub query: String,
    pub results: Vec<CommandItem>,
    pub cursor: usize,
}

pub struct CommandItem {
    pub command: String,
    pub description: String,
    pub action: Box<dyn Fn(&mut AppState)>,
    pub icon: &'static str,
}
```

---

## Fuzzy Finder

```
┌──────────────────────────────────────────────────────────────────┐
│  Filter tracks: lofij                                           │
├──────────────────────────────────────────────────────────────────┤
│  ♪  Lofi Jazz Beats — Chill Music Lab                  3:45    │
│  ♪  Lo-fi Study Mix — Study Girl                       2:15    │
│  ♪  Jazz & Lofi — Relax Daily                          4:30    │
│  ♪  lofi hip hop radio — beats to relax/study to    ∞  │
│  ♪  Midnight Lo-Fi — Sleepy Beats                    3:00  ← │
│  ♪  ...                                                         │
│                                                                  │
│  Matches: 48 of 1,234 tracks                                    │
└──────────────────────────────────────────────────────────────────┘
```

### State

```rust
pub struct FuzzyFinderState {
    pub query: String,
    pub items: Vec<FuzzyItem>,
    pub cursor: usize,
    pub mode: FuzzyMode, // Tracks | Playlists | Queue | YouTube
}

pub struct FuzzyItem {
    pub primary: String,     // title
    pub secondary: String,   // artist/album
    pub detail: String,      // duration, metadata
    pub icon: &'static str,
}
```

---

## Queue Picker (insert position selector)

```
┌──────────────────────────────────────────────────────────────────┐
│  Insert at position in queue:                                    │
├──────────────────────────────────────────────────────────────────┤
│  ▶ Now Playing by Current Artist                   1:23        │
│  ─────────────────────────────────────────────────────────────  │
│  1.  First track in queue                         3:45         │
│  2.  Second track                                2:15  ← pos  │
│  3.  Third track                                 4:30         │
│  4.  Fourth track                                3:00         │
│  ─────────────────────────────────────────────────────────────  │
│  (end of queue) — append here                                  │
│                                                                  │
│  j/k to position, Enter to confirm, Esc to cancel                │
└──────────────────────────────────────────────────────────────────┘
```

### State

```rust
pub struct QueuePickerState {
    pub cursor: usize,
}
```

---

## Theme Picker

```
┌──────────────────────────────────────────────────────────────────┐
│  Select Theme: cat                                              │
├──────────────────────────────────────────────────────────────────┤
│  ╭──────────────────────────────────────────────────────────╮   │
│  │  Preview: Catppuccin Mocha                                │   │
│  │  ┌──────────────────────────────────────────────────────┐ │   │
│  │  │  ♪  Sample Text (base text)                         │ │   │
│  │  │  ▶  Active item (mauve)    ← cursor (lavender)     │ │   │
│  │  │  ████████░░░░ progress (green, sky, surface2)       │ │   │
│  │  │  🔀 🔁 (peach, yellow, teal icons)                 │ │   │
│  │  └──────────────────────────────────────────────────────┘ │   │
│  ╰──────────────────────────────────────────────────────────╯   │
│                                                                  │
│  > Catppuccin Mocha                                        Dark │
│  > Catppuccin Latte                                      Light  │
│  > Nord                                                    Dark │
│  > Gruvbox Dark                                           Dark  │
│  > Gruvbox Light                                         Light  │
│  > Tokyo Night                                            Dark  │
│  > Solarized Dark                                        Dark  │
│  > Random (seed: 0x7f3a)                                       │
│                                                                  │
│  Enter to apply, Esc to cancel                                   │
└──────────────────────────────────────────────────────────────────┘
```

### State

```rust
pub struct ThemePickerState {
    pub query: String,
    pub cursor: usize,
    pub themes: Vec<ThemeEntry>,
    pub preview_theme: Option<Theme>,
}

pub struct ThemeEntry {
    pub name: &'static str,
    pub mode: ThemeMode,
    pub apply: fn() -> Theme,
}
```

---

## Confirm Dialog

```
┌──────────────────────────────────────────────────────────────────┐
│                                                                  │
│  ╭──────────────────────────────────────────────────────────╮   │
│  │  ⚠  Clear entire queue?                                 │   │
│  │                                                          │   │
│  │        [Y] Yes              [N] No                       │   │
│  ╰──────────────────────────────────────────────────────────╯   │
│                                                                  │
└──────────────────────────────────────────────────────────────────┘
```

Other examples:

```
┌──────────────────────────────────────────────────────────────────┐
│                                                                  │
│  ╭──────────────────────────────────────────────────────────╮   │
│  │  ⚠  Delete playlist "Road Trip"?                        │   │
│  │      12 tracks will be affected. This cannot be undone. │   │
│  │                                                          │   │
│  │        [Y] Yes              [N] No                       │   │
│  ╰──────────────────────────────────────────────────────────╯   │
│                                                                  │
└──────────────────────────────────────────────────────────────────┘
```

### State

```rust
pub struct ConfirmState {
    pub message: String,
    pub details: Option<String>,
    pub on_confirm: Box<dyn FnOnce(&mut AppState)>,
}
```

---

## Track Detail

```
┌──────────────────────────────────────────────────────────────────┐
│  ╭──────────────────────────────────────────────────────────╮   │
│  │  ♪  Song Title                                        ♫  │   │
│  │  ──────────────────────────────────────────────────────  │   │
│  │                                                          │   │
│  │    Artist:    Artist Name                                │   │
│  │    Album:     Album Name (2014)                          │   │
│  │    Duration:  4:20                                       │   │
│  │    Genre:     Jazz / Ambient                             │   │
│  │    Bitrate:   320 kbps                                   │   │
│  │    Sample    44,100 Hz                                   │   │
│  │                                                          │   │
│  │    File:      /home/user/Music/Artist/Album/song.flac    │   │
│  │    Added:     2024-03-15                                 │   │
│  │    Favourite: Yes ☆                                      │   │
│  │                                                          │   │
│  │  ──────────────────────────────────────────────────────  │   │
│  │                                                          │   │
│  │  [P] Play    [Q] Queue    [F] Favourite    [Esc] Close   │   │
│  ╰──────────────────────────────────────────────────────────╯   │
│                                                                  │
└──────────────────────────────────────────────────────────────────┘
```

### With embedded album art:

```
┌──────────────────────────────────────────────────────────────────┐
│  ╭──────────────────────────────────────────────────────────╮   │
│  │  ┌──────┐  ♪  Song Title                              ♫  │   │
│  │  │      │     Artist:    Artist Name                     │   │
│  │  │ Art  │     Album:     Album Name (2014)               │   │
│  │  │      │     Duration:  4:20                             │   │
│  │  └──────┘     Genre:     Jazz / Ambient                  │   │
│  │                Bitrate:   320 kbps                        │   │
│  │                Sample:    44,100 Hz                       │   │
│  │                                                           │   │
│  │                File: /home/user/Music/song.flac          │   │
│  │                                                           │   │
│  │  [P] Play  [Q] Queue  [F] Favourite  [Esc] Close         │   │
│  ╰──────────────────────────────────────────────────────────╯   │
│                                                                  │
└──────────────────────────────────────────────────────────────────┘
```

### State

```rust
pub struct TrackDetailState {
    pub track: TrackInfo,
    pub cover: Option<ImageData>,
    pub lyrics: Option<LrcData>,
}
```

## Overlay Render Logic

```rust
impl Overlay {
    pub fn render(&mut self, area: Rect, buf: &mut Buffer, state: &AppState) {
        // 1. dim the background (optional — clear with a semi-transparent block)
        let overlay_area = centered_rect(area, 80, 70);  // 80% width, 70% height
        clear_area(overlay_area, state.theme.base, buf);

        // 2. render specific overlay content
        match self {
            Overlay::CommandPalette(p) => p.render(overlay_area, buf, state),
            Overlay::FuzzyFinder(f)    => f.render(overlay_area, buf, state),
            // ...
        }
    }

    pub fn handle_key(&mut self, key: KeyEvent, state: &mut AppState) -> bool {
        match self {
            Overlay::CommandPalette(p) => p.handle_key(key, state),
            // ...
        }
    }
}

/// Center an overlay rect within the given area
fn centered_rect(area: Rect, percent_x: u16, percent_y: u16) -> Rect {
    let popup_x = area.width * percent_x / 100;
    let popup_y = area.height * percent_y / 100;
    Rect {
        x: area.x + (area.width - popup_x) / 2,
        y: area.y + (area.height - popup_y) / 2,
        width: popup_x,
        height: popup_y,
    }
}
```

## File Structure

```
gtm-tui/src/overlays/
├── mod.rs              # Overlay enum + dispatch
├── command_palette.rs
├── fuzzy_finder.rs
├── queue_picker.rs
├── theme_picker.rs
├── confirm_dialog.rs
└── track_detail.rs
```
