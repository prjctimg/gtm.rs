# 06 — gtm-tui: Architecture

## Purpose

The TUI binary (`gtm`) connects to the daemon via IPC, mirrors daemon state via events,
and renders a Ratatui-based terminal interface with 6 tabs and modal overlays.

Depends on: `gtm-core`, `gtm-audio`, `ratatui`, `crossterm`, `tokio`, `image`, `base64`

## Event Loop

```
┌──────────────────────  main()  ───────────────────────────┐
│                                                             │
│  1. parse CLI args (--socket, --tab, --theme)               │
│  2. init crossterm (raw mode, alternate screen)             │
│  3. create AppState + DaemonClient                          │
│  4. daemon_client.ensure_connected().await                  │
│  5. run event loop                                          │
│                                                             │
│  ┌──────────────────────────────────────────────────────┐  │
│  │ loop {                                               │  │
│  │     tokio::select! {                                 │  │
│  │         // Non-blocking IPC event read               │  │
│  │         events = client.poll_events() => {           │  │
│  │             for ev in events { process_event(ev) }   │  │
│  │         }                                            │  │
│  │                                                      │  │
│  │         // User keyboard input (10ms timeout)       │  │
│  │         key = crossterm::event::poll(10ms) => {     │  │
│  │             handle_key(key)                          │  │
│  │         }                                            │  │
│  │                                                      │  │
│  │         // 60fps render tick                         │  │
│  │         _ = render_throttle.tick() => {              │  │
│  │             render()                                 │  │
│  │         }                                            │  │
│  │     }                                                │  │
│  │ }                                                     │  │
│  └──────────────────────────────────────────────────────┘  │
│                                                             │
│  Render throttle: 16ms (60fps) using tokio::time::interval │
│  Position extrapolation between events:                    │
│    displayed_pos = last_event_pos + (now - event_time)     │
│    (if playing)                                             │
│                                                             │
└─────────────────────────────────────────────────────────────┘
```

## TUI Layout

```
┌──────────────────────────────────────────────────────────┐
│  ▶ Now Playing   Library   Queue   YouTube   Settings  H │  ← TabBar
│  ┌────────────────────────────────────────────────────┐  │
│  │                                                    │  │
│  │              Active Tab Content                    │  │
│  │              (fills entire area)                   │  │
│  │                                                    │  │
│  │                                                    │  │
│  │                                                    │  │
│  └────────────────────────────────────────────────────┘  │
│  ⏸ 2:34 / 4:20  ████████░░  Volume: 75%  🔀  🔁 All    │  ← Footer
└──────────────────────────────────────────────────────────┘

  TabBar: 6 tabs, left-aligned, active highlighted
  Footer: left section (status icon, position/progress)
          right section (volume, shuffle, repeat, clock)
```

## AppState

```rust
pub struct AppState {
    // Daemon connection
    pub daemon: DaemonClient,
    pub connected: bool,
    pub daemon_state: Arc<RwLock<DaemonState>>,
    pub daemon_state_version: u32,

    // Tab routing
    pub active_tab: Tab,        // Library | Queue | NowPlaying | YouTube | Settings | Help
    pub mode: UIMode,           // Normal | Filter | Command
    pub overlay: Option<Overlay>,

    // Shared view state
    pub filter_text: String,
    pub feedback_msg: Option<(String, Instant)>,
    pub select_mode: bool,
    pub selected_track_ids: HashSet<i64>,

    // Per-tab state
    pub library: LibraryViewState,
    pub queue: QueueViewState,
    pub now_playing: NowPlayingState,
    pub youtube: YouTubeViewState,
    pub settings: SettingsState,

    // Caches
    pub cover_image: Option<ImageData>,
    pub lyrics: Option<LrcData>,
    pub lyrics_line_idx: usize,

    // Scrolling
    pub viewer_scroll: u16,
    pub viewer_cursor: usize,

    // Theme
    pub theme: Theme,
    pub theme_mode: ThemeMode,
}
```

## DaemonClient (IPC Transport)

```
┌─────────────────  DaemonClient  ───────────────────────┐
│                                                          │
│  struct DaemonClient {                                   │
│      socket: Option<UnixStream>,                         │
│      reader: BufReader<UnixStream>,                      │
│      writer: BufWriter<UnixStream>,                      │
│      buf: String,               // read buffer          │
│      connected: bool,                                    │
│      extrapolated_pos: f64,     // smooth position      │
│      pos_base_time: Instant,                             │
│      pos_base_value: f64,                                │
│      extrapolating: bool,                                │
│      last_version: u32,         // last event version   │
│  }                                                       │
│                                                          │
│  Methods:                                                │
│  ┌─────────────────────────────────────────────────┐    │
│  │ async fn ensure_connected(&mut self)            │    │
│  │   • check socket is alive                       │    │
│  │   • if not: connect to Unix socket              │    │
│  │   • if daemon not running: spawn gtmd           │    │
│  │   • exponential backoff (100ms, 200ms, 400ms)   │    │
│  ├─────────────────────────────────────────────────┤    │
│  │ async fn request(&mut self, req) -> DaemonResponse│   │
│  │   • serialize req as JSON line                   │    │
│  │   • write to socket                              │    │
│  │   • read JSON response line                      │    │
│  │   • timeout: 5s                                  │    │
│  ├─────────────────────────────────────────────────┤    │
│  │ async fn send(&mut self, req)                    │    │
│  │   • fire-and-forget (no response)                │    │
│  ├─────────────────────────────────────────────────┤    │
│  │ async fn poll_events(&mut self) -> Vec<DaemonEvent│   │
│  │   • non-blocking read from socket                │    │
│  │   • parse binary frames                          │    │
│  │   • return accumulated events                    │    │
│  ├─────────────────────────────────────────────────┤    │
│  │ fn apply_event(&mut self, state, event)          │    │
│  │   • update state.daemon_state fields             │    │
│  │   • update extrapolation base for position       │    │
│  │   • set daemon_state_version                     │    │
│  └─────────────────────────────────────────────────┘    │
└─────────────────────────────────────────────────────────┘
```

## Render Pipeline

```
render() {
    1. let mut frame = terminal.get_frame()

    2. let layout = Layout::vertical([
           Constraint::Length(1),    // TabBar
           Constraint::Min(0),       // Content
           Constraint::Length(1),    // Footer
       ]).split(frame.area())

    3. // Render TabBar
       let tab_titles = ["Library", "Queue", "Now Playing", "YouTube", "Settings", "Help"]
       render_tab_bar(layout[0], buf, &tab_titles, active_tab)

    4. // Render active tab
       match active_tab {
           Library    => library_tab.render(layout[1], buf, &mut state)
           Queue      => queue_tab.render(...)
           NowPlaying => now_playing_tab.render(...)
           YouTube    => youtube_tab.render(...)
           Settings   => settings_tab.render(...)
           Help       => help_tab.render(...)
       }

    5. // Render overlay on top if active
       if let Some(ref overlay) = state.overlay {
           overlay.render(layout[1], buf, &state)
       }

    6. // Render Footer
       render_footer(layout[2], buf, &state)

    7. terminal.draw(|f| f.render_widget(buf))
}
```

## State Mirror Pattern

```
Daemon (source of truth)              TUI (cached mirror)
┌──────────────────────┐             ┌──────────────────────┐
│ DaemonState          │   events    │ AppState             │
│  .version = 42       │───────────▶ │  .daemon_state       │
│  .volume = 75        │  binary     │    .volume = 75      │
│  .shuffle = true     │  frames     │    .shuffle = true   │
│  .time_pos = 124.5   │  (async)    │    .time_pos = 124.5 │
│  .queue = [...]      │             │    .queue = [...]    │
│  .current_track = X  │             │  .daemon_state_ver=42│
└──────────────────────┘             │                      │
                                     │  Position extrpolation│
                                     │  displayed_pos =      │
                                     │    time_pos + elapsed │
                                     │    (only when playing)│
                                     └──────────────────────┘
```

## File Structure

```
gtm-tui/src/
├── main.rs              # binary entrypoint, terminal init, event loop
├── app.rs               # App struct, render function
├── state.rs             # AppState, per-tab view states
├── daemon_client.rs     # DaemonClient (IPC transport + state mirror)
├── keymap.rs            # Keybindings, parse_keycode, fuzzy match
├── theme.rs             # Theme generation, catppuccin presets, HSL→RGB
├── graphics.rs          # Kitty graphics protocol transmit/delete/place
├── icons.rs             # Nerd Font / Emoji icon constants
├── footer.rs            # FooterBar widget, FooterModule variants
└── tabs/
│   ├── mod.rs           # TabWidget trait
│   ├── library.rs       # LibraryTab
│   ├── queue.rs         # QueueTab
│   ├── now_playing.rs   # NowPlayingTab
│   ├── youtube.rs       # YouTubeTab
│   ├── settings.rs      # SettingsTab
│   └── help.rs          # HelpTab
└── overlays/
    ├── mod.rs           # Overlay enum + dispatch
    ├── command_palette.rs
    ├── fuzzy_finder.rs
    ├── theme_picker.rs
    ├── confirm_dialog.rs
    └── track_detail.rs
```
