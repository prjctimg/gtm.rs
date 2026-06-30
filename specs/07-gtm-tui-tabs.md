# 07 — gtm-tui: Tabs

## Purpose

Each of the 6 tabs implements the `TabWidget` trait and renders its content into a `Rect` area
of the terminal. Tabs share the `AppState` for data and `DaemonClient` for IPC.

## TabWidget Trait

```rust
pub trait TabWidget {
    /// Render this tab's content into the given area
    fn render(&mut self, area: Rect, buf: &mut Buffer, state: &mut AppState);

    /// Handle a keyboard event. Returns true if consumed.
    fn handle_key(&mut self, key: KeyEvent, state: &mut AppState) -> bool;
}
```

---

## LibraryTab

```
┌──────────────────────────────────────────────────────────────┐
│  [Tracks] │ Playlists │ Favourites │ Recent                  │
├──────────────────────────────────────────────────────────────┤
│  ♪  artist - title                              3:45        │
│  ♪  another artist - longer title title         5:12        │
│  ♪  third artist - song                            ← cursor │
│  ♪  fourth - short                               2:01        │
│  ♪  fifth artist - really long title here       4:33        │
│  ♪  ...                                                        │
│                                                               │
│  Scan progress: ████████░░░░░░░░░░ 12/48 tracks scanned     │
│                                                               │
│  Filter: /search term...                                      │
├──────────────────────────────────────────────────────────────┤
│  Total: 248 tracks  •  15 playlists  •  23 favourites        │
└──────────────────────────────────────────────────────────────┘
```

### Details

```
State machine:
  ┌──────────┐     Enter playlist     ┌──────────────┐
  │ Tracks   │───────────────────────▶│ Playlist      │
  │ (list)   │                        │ (track list)  │
  │          │◀───────────────────────│               │
  │          │     Esc / Back         └──────────────┘
  │          │     Enter favourite
  │          │───────────────────────▶┌──────────────┐
  │          │                        │ Favourites   │
  │          │◀───────────────────────│ (track list) │
  └──────────┘     Esc / Back         └──────────────┘

Navigation:
  j/k or ↑/↓            Move cursor
  Enter                 Play selected track
  Space                 Add to queue (append)
  a                     Add to queue (next)
  /                     Enter filter mode
  Esc                   Exit filter / go back
  Tab, Shift+Tab       Cycle header tabs
  d                     Show track detail overlay
  f                     Toggle favourite
  p                     Add to playlist prompt
  s                     Sort by (artist/album/title/duration/year)
  S (Shift+s)           Reverse sort
  r                     Rescan directory
  / (in filter)         Clear filter
```

### Playlist sub-view

```
┌──────────────────────────────────────────────────────────────┐
│  [Tracks] │ [Playlists] │ Favourites │ Recent                │
├──────────────────────────────────────────────────────────────┤
│  ┌──────────────────────────────────────────────────────┐   │
│  │  My Chill Mix                    12 tracks           │   │
│  │  Road Trip                                        ← │   │
│  │  2024 Favourites                   42 tracks        │   │
│  │  Workout Energy                     8 tracks        │   │
│  │  Jazz for Studying                 23 tracks        │   │
│  ├──────────────────────────────────────────────────────┤   │
│  │  n  New playlist /  d  Delete selected              │   │
│  └──────────────────────────────────────────────────────┘   │
└──────────────────────────────────────────────────────────────┘
```

---

## QueueTab

```
┌──────────────────────────────────────────────────────────────┐
│  Queue (5 tracks)                                             │
├──────────────────────────────────────────────────────────────┤
│  ▶ Now Playing                                                │
│    ┌──────────────────────────────────────────────────────┐   │
│    │  ♪  Currently Playing Artist - Playing Song          │   │
│    │  ⏵  1:23 / 4:56  ████████░░░░░ 28%                  │   │
│    └──────────────────────────────────────────────────────┘   │
│                                                               │
│  ▸ Up Next                                                    │
│    ┌──────────────────────────────────────────────────────┐   │
│    │  1.  Artist A - Song One                 5:01        │   │
│    │  2.  Artist B - Song Two                 3:45    ←   │   │
│    │  3.  Artist C - Song Three               4:12        │   │
│    │  4.  Artist A - Song Four                3:22        │   │
│    │                                                    │   │
│  ═══════════════════════════════════════════════════════════   │
│                                                               │
│  (end of queue — 3 more in library)                          │
│                                                               │
├──────────────────────────────────────────────────────────────┤
│  Total: 5  |  D: delete  M: move  C: clear  S: save playlist │
└──────────────────────────────────────────────────────────────┘
```

### Navigation

```
j/k or ↑/↓            Move cursor
Enter                 Play track (jump to)
d                     Remove from queue
m                     Enter move mode (select source, then destination)
M                     Enter move mode (same, intuitive key)
C                     Clear queue
s                     Save queue as playlist
Space                 Play now (jump to)
```

### Move Mode

```
┌──────────────────────────────────────────────────────────────┐
│  Move mode: j/k to position, Enter to confirm, Esc to cancel │
│                                                               │
│  1.  Artist A - Song One                 5:01                │
│  →  2.  Artist B - Song Two             3:45   ◄─ source    │
│  3.  Artist C - Song Three               4:12                │
│   ↑                                                            │
│   └─ cursor shows insertion point                              │
└──────────────────────────────────────────────────────────────┘
```

---

## NowPlayingTab

```
┌──────────────────────────────────────────────────────────────┐
│  ┌────────────────────┐  ◀── Album art (Kitty image or box)  │
│  │                    │                                       │
│  │    ♫   Album      │  Song Title                            │
│  │        Art         │  Artist Name                          │
│  │                    │  Album Name                    2014   │
│  │    (Kitty image)   │  ─────────────────────────────         │
│  │                    │  ████████████████░░░░  2:34 / 4:20    │
│  │                    │                                       │
│  └────────────────────┘  ◀◀  ⏸  ▶▶  │◀  ▶│  🔀  🔁  🔂     │
│                                                               │
│  ┌─ Lyrics ───────────────────────────────────────────────┐   │
│  │                                                         │   │
│  │  🎤  This is the first line of the song                │   │
│  │  🎤  Second line goes here                             │   │
│  │  🎤  Third line — current position → green highlight   │   │
│  │  🎤  Fourth line                                       │   │
│  │  🎤  Fifth line of lyrics                              │   │
│  │                                                         │   │
│  └─────────────────────────────────────────────────────────┘   │
│                                                               │
│  ▸ Up Next                                                    │
│    1. Next Artist - Next Song                                 │
│    2. Another - Different Song                                │
└──────────────────────────────────────────────────────────────┘
```

### Without album art (narrow terminal)

```
┌──────────────────────────────────────────────────────────────┐
│  Now Playing                                                  │
│                                                               │
│  ♪   Song Title                                               │
│  ─── by Artist Name ───                                       │
│                                                               │
│  ████████████████░░░░░░░  2:34 / 4:20  75%                   │
│                                                               │
│  ◀◀  ⏸  ▶▶  │◀  ▶│  🔀  🔁 All  🔂                         │
│                                                               │
│  ┌─ Lyrics ───────────────────────────────────────────────┐   │
│  │  🎤  This is the first line of the song                │   │
│  │  🎤  Second line — green highlight                     │   │
│  └─────────────────────────────────────────────────────────┘   │
│                                                               │
│  Up Next:  Next Song — Artist                 3:45            │
└──────────────────────────────────────────────────────────────┘
```

### Navigation

```
Space              Play / Pause
→ / l             Seek forward 5s
← / h             Seek backward 5s
s                 Toggle shuffle
r                 Cycle repeat (Off → One → All → Off)
m                 Toggle mute
↑ / ↓ / j / k     Scroll lyrics
+ / =             Volume up 5
-                 Volume down 5
q                 Toggle Up Next panel
```

---

## YouTubeTab

```
┌──────────────────────────────────────────────────────────────┐
│  [Search] │ Playlists                                          │
├──────────────────────────────────────────────────────────────┤
│  ┌──────────────────────────────────────────────────────┐    │
│  │  🔍 Search YouTube (press / to search)               │    │
│  └──────────────────────────────────────────────────────┘    │
│                                                               │
│  Search results for "jazz lofi chill":                        │
│  ┌──────────────────────────────────────────────────────┐    │
│  │  ▶  Lofi Jazz Beats for Studying 2024               │    │
│  │     by Chill Music Lab         • 1,234,567 views    │    │
│  │     ──────────────────────────────────────────────── │    │
│  │     ▶  Smooth Jazz Playlist — 5 Hours             ← │    │
│  │     by Jazz Vibes          • 892,451 views          │    │
│  │     ──────────────────────────────────────────────── │    │
│  │     ▶  Late Night Jazz — Relaxing Mix               │    │
│  │     by Coffee Shop Jazz    • 2,103,456 views        │    │
│  │     ──────────────────────────────────────────────── │    │
│  │     ▶  Jazz for Sleep — Calm Piano & Sax           │    │
│  │     by Relaxing Music      • 567,890 views          │    │
│  └──────────────────────────────────────────────────────┘    │
│                                                               │
│  Results 1-4 of 10  (page 1/3)                                │
├──────────────────────────────────────────────────────────────┤
│  Enter: play stream  |  a: add to queue  |  /: search        │
└──────────────────────────────────────────────────────────────┘
```

### Search loading state

```
┌──────────────────────────────────────────────────────────────┐
│  [Search] │ Playlists                                          │
├──────────────────────────────────────────────────────────────┤
│                                                               │
│                    🔍 Searching...                             │
│                    ░░░░░░░░░░░░░░░░░░░░░░░░░░░                │
│                    Fetching from YouTube...                    │
│                                                               │
│                    (esc to cancel)                             │
│                                                               │
└──────────────────────────────────────────────────────────────┘
```

### Navigation

```
/                   Enter search (opens filter bar)
Enter               Play stream (resolve URL → daemon → play)
a                   Add stream URL to local queue
d                   Show stream details
l                   Toggle playlist subtab
j/k or ↑/↓          Move cursor
r                   Refresh / re-search
Esc                 Clear search / back
```

---

## SettingsTab

```
┌──────────────────────────────────────────────────────────────┐
│  Settings                                                     │
├──────────────────────────────────────────────────────────────┤
│  ⚙  Volume              ░░░░░░░░▓▓▓▓▓▓▓▓  75%               │
│  ⚙  Crossfade           [x]  3s                              │
│  ⚙  Theme               Light v Dark  ▶  ← cursor           │
│  ⚙  Icons               Nerd Font  ▶                        │
│  ⚙  Audio Backend       Symphonia  ▶                        │
│  ⚙  Tab Order           [Library] [Queue] [Now Playing] ...  │
│  ⚙  Library Paths       ~/Music/  ▶                         │
│  ⚙  Search History      Clear                                │
│  ⚙  Cache Size          500 items                            │
│  ────────────────────────────────────────────────────────     │
│  ⚙  About gtm v0.2.0                                         │
│  ⚙  Check for Updates                                        │
│                                                               │
└──────────────────────────────────────────────────────────────┘
```

### Navigation

```
j/k or ↑/↓          Move cursor
Enter               Toggle / open sub-setting
← / →               Decrease / increase value
Esc                 Back
```

### Theme Picker (inline)

```
│  ⚙  Theme               ▶ Catppuccin Mocha               │
│                               Catppuccin Latte            │
│                               Nord                         │
│                               Gruvbox Dark                ←│
│                               Gruvbox Light                │
│                               Tokyo Night                  │
│                               Custom (random seed)        │
│                               Solarized Dark               │
```

---

## HelpTab

```
┌──────────────────────────────────────────────────────────────┐
│  Help — Keybindings                                           │
├──────────────────────────────────────────────────────────────┤
│                                                               │
│  Global                                                       │
│  ┌──────────────────────────────────────────────────────┐    │
│  │  Tab / Shift+Tab     Next/Previous tab              │    │
│  │  1-6                 Switch to tab #                │    │
│  │  Ctrl+c / q          Quit                           │    │
│  │  :                   Command palette                │    │
│  │  ?                   Toggle help tab                │    │
│  │  Ctrl+r              Reload config                  │    │
│  └──────────────────────────────────────────────────────┘    │
│                                                               │
│  Navigation                                                   │
│  ┌──────────────────────────────────────────────────────┐    │
│  │  j / ↓                Move cursor down               │    │
│  │  k / ↑                Move cursor up                 │    │
│  │  g / G                Top / bottom                    │    │
│  │  Ctrl+d / Ctrl+u      Page down / page up            │    │
│  │  /                    Enter filter mode              │    │
│  └──────────────────────────────────────────────────────┘    │
│                                                               │
│  Playback                                                     │
│  ┌──────────────────────────────────────────────────────┐    │
│  │  Space                Play / Pause                   │    │
│  │  s                    Toggle shuffle                 │    │
│  │  r                    Cycle repeat mode              │    │
│  │  n / p                Next / Previous track          │    │
│  │  → / l                Seek forward 5s                │    │
│  │  ← / h                Seek backward 5s               │    │
│  │  + / -                Volume up / down               │    │
│  │  m                    Toggle mute                    │    │
│  └──────────────────────────────────────────────────────┘    │
│                                                               │
│  Press any key to close                                       │
└──────────────────────────────────────────────────────────────┘
```

### Alternative (compact) layout for small terminals:

```
┌──────────────────────────────────────────────────────────────┐
│  Help — Keybindings                                           │
├──────────────────────────────────────────────────────────────┤
│   Global: Tab↔next  :cmd  ?help  q/Ctrl+c=quit               │
│   Nav:    j↓ k↑ g↖ G↘  /filter  Ctrl+d/pgdn                  │
│   Play:   Space↕  sno shuf  rrep  n/p→prev  ←→seek          │
│   Vol:    +/- up/down  m mute                                 │
│   Lib:    Enter play  a enqueue  d detail  f fav             │
│   Queue:  d del  m move  C clear  s save playlist            │
│   YT:     / search  Enter play  a enqueue                    │
│   Now:    j/k lyrics  q toggle upnext                        │
│                                                               │
│  Press any key to close                                       │
└──────────────────────────────────────────────────────────────┘
```

## File Structure

```
gtm-tui/src/tabs/
├── mod.rs             # TabWidget trait + Tab enum
├── library.rs         # LibraryTab
├── queue.rs           # QueueTab
├── now_playing.rs     # NowPlayingTab
├── youtube.rs         # YouTubeTab
├── settings.rs        # SettingsTab
└── help.rs            # HelpTab
```
