# 01 — gtm-core: Shared Types & IPC Protocol

## Purpose

Defines every type shared across the workspace: IPC request/response enums, daemon event structs,
the binary wire protocol, track metadata, LRC lyrics data, and daemon state snapshot.

Depends on: `serde`, `serde_json`, `bincode`, `thiserror`, `chrono`, `uuid`

Used by: all crates

## IPC Protocol — Request/Response

```
 ┌─────────┐          JSON line            ┌─────────┐
 │  gtm    │  ──── DaemonRequest ────────▶ │  gtmd   │
 │ (client)│                               │ (daemon)│
 │         │  ◀──── DaemonResponse ──────── │         │
 └─────────┘          JSON line            └─────────┘
```

### DaemonRequest (client → daemon)

```rust
pub enum DaemonRequest {
    // Playback
    Play { path: String },
    PlayPause,
    Stop,
    Next,
    Prev,
    Seek { position_secs: f64 },
    SetVolume { volume: u8 },         // 0-100
    ToggleShuffle,
    CycleRepeat { mode: RepeatMode },
    ToggleMute,
    Crossfade { enabled: bool, duration_secs: u8 },

    // Queue
    Queue { action: QueueAction },
    Library { action: LibraryAction },

    // Discovery
    Search { query: String },
    GetFavourites,
    AddFavourite { track_id: i64 },
    RemoveFavourite { track_id: i64 },
    YtSearch { query: String, filter: Option<YtFilter> },
    YtSearchPoll,
    YtSearchCancel,
    YtResolveStream { url: String, title: Option<String>, channel: Option<String> },

    // System
    GetStatus,
    Ping,
    Quit,
}
```

Queue actions:

```rust
pub enum QueueAction {
    List,
    Clear,
    Remove { index: usize },
    Move { from: usize, to: usize },
    Add { path: String, position: Option<usize> },
    AddMany { paths: Vec<String> },
    Set { paths: Vec<String>, start_index: usize },
}
```

Library actions:

```rust
pub enum LibraryAction {
    Scan { path: String },
    GetTracks { filter: Option<String>, sort: Option<String> },
    GetPlaylists,
    CreatePlaylist { name: String },
    DeletePlaylist { id: i64 },
    AddToPlaylist { playlist_id: i64, track_ids: Vec<i64> },
    ImportM3u { path: String },
    GetRecent { count: usize },
}
```

## Binary Wire Protocol (DaemonEvents)

```
 ┌─────────┐    binary frames over same socket    ┌─────────┐
 │  gtm    │  ◀══════ DaemonEvent stream ═══════   │  gtmd   │
 │ (client)│                                       │ (daemon)│
 └─────────┘                                       └─────────┘
```

Frame encoding:

```
┌─────────────────────────────────────────────────────────┐
│ Byte 0..3    │  total_len: u32 BE                        │
│              │  (includes count + all events,             │
│              │   NOT including these 4 bytes)             │
├──────────────┼──────────────────────────────────────────┤
│ Byte 4       │  event_count: u8                          │
├──────────────┼──────────────────────────────────────────┤
│ Byte 5..     │  event[0]                                 │
│              │  ┌─ kind: u8                              │
│              │  ├─ version: u32 BE                       │
│              │  ├─ payload: bincode(DaemonEvent)         │
│              │  └─ (variable length)                     │
│              │  event[1] ...                             │
└──────────────┴──────────────────────────────────────────┘
```

### DaemonEvent (daemon → client, streamed)

```rust
pub enum DaemonEvent {
    PlaybackStarted {
        track: TrackInfo,
        auto_advanced: bool,
        time_pos: f64,
        duration: f64,
    },
    PlaybackPaused,
    PlaybackStopped,
    TrackEnded,
    PositionChanged { time_pos: f64 },
    DurationChanged { duration: f64 },
    VolumeChanged { volume: u8 },
    MetadataChanged { event: String },
    QueueChanged { queue: Vec<TrackInfo>, cursor: usize },
    QueueIndexChanged { index: usize },
    RepeatModeChanged { mode: RepeatMode },
    ShuffleChanged { enabled: bool },
    SleepTimerTick { remaining_secs: u32 },
    Custom { name: String, data: HashMap<String, String> },
}
```

## Track & Library Types

```rust
pub struct TrackInfo {
    pub id: i64,
    pub path: String,
    pub title: String,
    pub artist: String,
    pub album: String,
    pub duration: f64,
    pub track_number: Option<i32>,
    pub genre: Option<String>,
    pub year: Option<i32>,
    pub bitrate: Option<i32>,
    pub samplerate: Option<i32>,
    pub hash: String,
    pub cover_path: Option<String>,
}

pub struct Playlist {
    pub id: i64,
    pub name: String,
    pub created_at: String,
    pub track_count: usize,
}

pub struct LrcLine {
    pub timestamp: f64,
    pub text: String,
}

pub struct LrcData {
    pub title: Option<String>,
    pub artist: Option<String>,
    pub album: Option<String>,
    pub lines: Vec<LrcLine>,
}
```

## DaemonState (snapshot)

```rust
pub struct DaemonState {
    pub version: u32,              // monotonic counter
    pub status: PlaybackStatus,    // Stopped | Playing | Paused
    pub queue: Vec<TrackInfo>,
    pub queue_cursor: usize,
    pub volume: u8,                // 0-100
    pub repeat: RepeatMode,        // Off | One | All
    pub shuffle: bool,
    pub mute: bool,
    pub crossfade: Option<CrossfadeConfig>,
    pub current_track: Option<TrackInfo>,
    pub time_pos: f64,
    pub duration: f64,
    pub sleep_timer: Option<u32>,
}
```

File: `gtm-core/src/ipc.rs`, `gtm-core/src/state.rs`, `gtm-core/src/track.rs`, `gtm-core/src/wire.rs`
