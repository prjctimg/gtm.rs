# 11 — gtm-mpris: MPRIS D-Bus Server

## Purpose

Exposes the daemon's playback state and controls via the [MPRIS D-Bus specification](https://specifications.freedesktop.org/mpris-spec/latest/).
Allows media keys, GNOME/KDE lock screen controls, and external tools (playerctl) to
control gtm.

Depends on: `gtm-core`, `zbus`, `zvariant`

## Architecture

```
┌───────────────────────────────────────────────────────────────┐
│  gtmd Process                                                  │
│                                                                │
│  ┌────────────────────────────────────────┐                   │
│  │  Daemon                                │                   │
│  │  (main loop)                           │                   │
│  │  ┌──────────────────────────────────┐  │                   │
│  │  │  daemon_state: Arc<RwLock<...>>  │──┼── shared state   │
│  │  │  event_tx: broadcast::Sender     │──┼── event stream   │
│  │  └──────────────────────────────────┘  │                   │
│  └─────────────┬──────────────────────────┘                   │
│                │ Arc + broadcast rx                             │
│                ▼                                               │
│  ┌────────────────────────────────────────┐                   │
│  │  MprisServer                            │                   │
│  │  (spawned task)                         │                   │
│  │                                         │                   │
│  │  • Connects to session D-Bus            │                   │
│  │  • Registers:                           │                   │
│  │    org.mpris.MediaPlayer2.gtm           │                   │
│  │    /org/mpris/MediaPlayer2              │                   │
│  │                                         │                   │
│  │  • Serves:                              │                   │
│  │    org.mpris.MediaPlayer2               │                   │
│  │    org.mpris.MediaPlayer2.Player        │                   │
│  │                                         │                   │
│  │  • Listens for property changes and      │                   │
│  │    emits PropertiesChanged signals       │                   │
│  │  • On daemon event: update properties    │                   │
│  │  • On MPRIS method call: send IPC        │                   │
│  │    request to daemon                     │                   │
│  └────────────────────────────────────────┘                   │
└───────────────────────────────────────────────────────────────┘
```

## D-Bus Interface — `org.mpris.MediaPlayer2`

### Properties

| Property | Type | Value |
|----------|------|-------|
| `CanQuit` | Boolean | `true` |
| `CanRaise` | Boolean | `false` |
| `HasTrackList` | Boolean | `false` |
| `Identity` | String | `"gtm"` |
| `DesktopEntry` | String | `"gtm"` |
| `SupportedUriSchemes` | Array of String | `["file"]` |
| `SupportedMimeTypes` | Array of String | `["audio/flac","audio/mpeg","audio/ogg","audio/wav","audio/x-m4a"]` |

### Methods

| Method | Implementation |
|--------|---------------|
| `Raise()` | No-op (no window) |
| `Quit()` | Send `DaemonRequest::Quit` to daemon |

---

## D-Bus Interface — `org.mpris.MediaPlayer2.Player`

### Properties

| Property | Type | Source |
|----------|------|--------|
| `PlaybackStatus` | String | `daemon_state.status` → `"Playing"` `"Paused"` `"Stopped"` |
| `LoopStatus` | String | `daemon_state.repeat` → `"None"` `"Track"` `"Playlist"` |
| `Shuffle` | Boolean | `daemon_state.shuffle` |
| `Metadata` | Dict (a{sv}) | Built from `current_track` |
| `Volume` | Double | `daemon_state.volume / 100.0` |
| `Position` | Int64 (μs) | `daemon_state.time_pos * 1_000_000` |
| `MinimumRate` | Double | `1.0` |
| `MaximumRate` | Double | `1.0` |
| `CanGoNext` | Boolean | `true` |
| `CanGoPrevious` | Boolean | `true` |
| `CanPlay` | Boolean | `true` |
| `CanPause` | Boolean | `true` |
| `CanSeek` | Boolean | `true` |
| `CanControl` | Boolean | `true` |

### Metadata Map Construction

```
"mpris:trackid"   → "/org/mpris/MediaPlayer2/gtm/track/<id>"
"mpris:length"    → duration * 1_000_000  (μs)
"mpris:artUrl"    → "file:///path/to/cover.jpg" (if cover_path)
"xesam:title"     → track.title
"xesam:artist"    → [track.artist]
"xesam:album"     → track.album
"xesam:albumArtist" → [track.artist]
"xesam:trackNumber" → track.track_number
"xesam:genre"     → [track.genre]
"xesam:year"      → track.year
"xesam:url"       → "file://" + track.path
```

### Methods

| Method | Implementation |
|--------|---------------|
| `Next()` | Send `DaemonRequest::Next` |
| `Previous()` | Send `DaemonRequest::Prev` |
| `Pause()` | Send `DaemonRequest::PlayPause` (if playing) |
| `PlayPause()` | Send `DaemonRequest::PlayPause` |
| `Stop()` | Send `DaemonRequest::Stop` |
| `Play()` | Send `DaemonRequest::PlayPause` (if paused/stopped) |
| `Seek(offset)` | Get current pos + offset/1_000_000, send `Seek` |
| `SetPosition(track_id, position_us)` | Validate track_id matches, send `Seek(position_us/1_000_000)` |
| `OpenUri(uri)` | Strip `file://`, send `Play{path}` |

### Signals

| Signal | Trigger |
|--------|---------|
| `PropertiesChanged` | On any player property change (via event stream) |
| `Seeked(position)` | On `evPositionChanged` from daemon (rate-limited to 100ms) |

## Event → Signal bridge

```
DaemonEvent                     → MPRIS Action
────────────────────────────────────────────────────────
PlaybackStarted                 Update Metadata, PlaybackStatus→Playing, emit Signal
PlaybackPaused                  PlaybackStatus→Paused, emit Signal
PlaybackStopped                 PlaybackStatus→Stopped, Metadata→empty, emit Signal
PositionChanged                 Update Position property, emit Seeked (throttled)
VolumeChanged                   Update Volume property, emit Signal
ShuffleChanged                  Update Shuffle property, emit Signal
RepeatModeChanged               Update LoopStatus property, emit Signal
```

## File Structure

```
gtm-mpris/
├── Cargo.toml       # deps: zbus (with "tokio" feature), zvariant, gtm-core
└── src/
    └── lib.rs       # MprisServer struct, root + player impls
```
