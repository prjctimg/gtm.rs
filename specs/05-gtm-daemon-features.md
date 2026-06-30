# 05 — gtm-daemon: Features (yt-dlp, Cover Art, Lyrics, Queue)

## Purpose

Additional subsystems that live inside the daemon process: YouTube/yt-dlp streaming manager,
Deezer cover art cache, LRC lyrics resolution, and the queue manager.

## yt-dlp Manager

```
┌────────────────────  YoutubeManager  ──────────────────────┐
│                                                             │
│  ┌──────────────────┐     ┌──────────────────────┐         │
│  │ search(query)    │────▶│ yt-dlp process       │         │
│  │                  │     │ yt-dlp ytsearch10:... │         │
│  │                  │     │ --dump-json           │         │
│  │                  │     │ --flat-playlist       │         │
│  │                  │     └──────────┬───────────┘         │
│  │                  │                │ stdout lines         │
│  │                  │                ▼                      │
│  │                  │     ┌──────────────────────┐         │
│  │                  │     │ parse JSON lines     │         │
│  │                  │     │ → Vec<YtSearchResult> │         │
│  │                  │     └──────────────────────┘         │
│  └──────────────────┘                                      │
│                                                             │
│  ┌──────────────────┐     ┌──────────────────────┐         │
│  │ resolve(url)     │────▶│ yt-dlp --get-url     │         │
│  │                  │     │ → direct stream URL  │         │
│  │                  │     └──────────────────────┘         │
│  │                  │                                      │
│  │                  │     ┌──────────────────────┐         │
│  │                  │     │ returns StreamInfo    │         │
│  │                  │     │ { url, title, ext,   │         │
│  │                  │     │   duration }          │         │
│  │                  │     └──────────────────────┘         │
│  └──────────────────┘                                      │
│                                                             │
│  Subprocess management:                                     │
│  • spawn on search/resolve                                  │
│  • kill on cancel() or timeout (30s)                        │
│  • cap concurrent: max 2 subprocesses                       │
│  • stdout parse: Lines starting with "{" are JSON           │
│  • stderr: logged at debug level                            │
│                                                             │
└─────────────────────────────────────────────────────────────┘
```

### Data types

```rust
pub struct YtSearchResult {
    pub id: String,
    pub title: String,
    pub url: String,
    pub channel: String,
    pub duration: f64,
    pub views: u64,
    pub thumbnail: Option<String>,
}

pub struct StreamInfo {
    pub url: String,       // direct media URL
    pub title: String,
    pub ext: String,       // e.g. "webm", "m4a"
    pub duration: f64,
}
```

## Cover Art Cache

```
┌───────────────────  CoverCache  ───────────────────────┐
│                                                          │
│  ┌────────────┐   ┌──────────────┐   ┌──────────────┐   │
│  │ lookup()   │──▶│ memory cache │──▶│ return        │   │
│  │ (artist,   │   │ (LRU, 500)   │   │ Option<(bytes,│   │
│  │  album)    │   └──────┬───────┘   │  mime)>      │   │
│  │            │          │ miss      └──────────────┘   │
│  │            │          ▼                              │
│  │            │   ┌──────────────┐   ┌──────────────┐   │
│  │            │   │ disk cache   │──▶│ load from     │   │
│  │            │   │ ($CACHE/gtm/ │   │ disk          │   │
│  │            │   │  covers/)    │   └──────────────┘   │
│  │            │   └──────┬───────┘                      │
│  │            │          │ miss                          │
│  │            │          ▼                              │
│  │            │   ┌──────────────┐   ┌──────────────┐   │
│  │            │   │ Deezer API   │──▶│ fetch from    │   │
│  │            │   │ search       │   │ https://api-  │   │
│  │            │   │              │   │ deezer.com    │   │
│  │            │   └──────────────┘   └──────────────┘   │
│  │            │          │                               │
│  │            │          ▼                               │
│  │            │   ┌──────────────┐                       │
│  │            │   │ save to disk │                       │
│  │            │   │ + memory     │                       │
│  │            │   └──────────────┘                       │
└──────────────────────────────────────────────────────────┘

Deezer API:
  GET https://api.deezer.com/search?q=artist:"<artist>" album:"<album>"
  → parse response for cover_medium / cover_big URL
  → download image bytes
  → cache as JPEG/PNG with hash key filename
```

Cover data sent to TUI as raw bytes + MIME string via `CoverData { data: Vec<u8>, mime: String }`
in a `MetadataChanged` event or a dedicated `Custom` event.

## Lyrics Manager

```
┌─────────────────  LyricsManager  ─────────────────────────┐
│                                                             │
│  Resolution order:                                          │
│                                                             │
│  1. Sidecar file (fastest, local)                           │
│     ┌─────────────────────────────────────────────────┐    │
│     │ Look for <track_path>.lrc                        │    │
│     │ If found, parse and return LrcData               │    │
│     └─────────────────────────────────────────────────┘    │
│                                                             │
│  2. LRCLIB (by artist + title + album, exact match)        │
│     ┌─────────────────────────────────────────────────┐    │
│     │ GET https://lrclib.net/api/get?                  │    │
│     │   artist=<artist>&title=<title>&album=<album>   │    │
│     │ If 200, parse synced/nonsynced lyrics           │    │
│     └─────────────────────────────────────────────────┘    │
│                                                             │
│  3. LRCLIB search (fuzzy fallback)                         │
│     ┌─────────────────────────────────────────────────┐    │
│     │ GET https://lrclib.net/api/search?               │    │
│     │   q=<artist> <title>                            │    │
│     │ Pick best match from results                     │    │
│     └─────────────────────────────────────────────────┘    │
│                                                             │
│  4. SQLite cache check (on subsequent plays)               │
│     ┌─────────────────────────────────────────────────┐    │
│     │ Key = SHA256(artist+album+title)[:16]           │    │
│     │ Store/retrieve from lyrics_cache table          │    │
│     └─────────────────────────────────────────────────┘    │
│                                                             │
└─────────────────────────────────────────────────────────────┘
```

### LRC Parser

```
[00:12.34]First line of lyrics
[00:15.67]Second line
[00:18.90]Third line
[00:22.10]Fourth line
[00:25.43]Fifth line

→ Vec<LrcLine>:
   0: { timestamp: 12.34, text: "First line of lyrics" }
   1: { timestamp: 15.67, text: "Second line" }
   2: { timestamp: 18.90, text: "Third line" }
   3: { timestamp: 22.10, text: "Fourth line" }
   4: { timestamp: 25.43, text: "Fifth line" }

Current line lookup: binary search for largest timestamp ≤ current_position
```

## Queue Manager

```
┌──────────────────  QueueManager  ─────────────────────┐
│                                                         │
│  struct QueueManager {                                  │
│      queue: Vec<TrackInfo>,                             │
│      cursor: usize,            // index into queue      │
│      shuffle_order: Vec<usize>,// shuffled indices      │
│      shuffle_cursor: usize,    // index into shuffle    │
│      repeat: RepeatMode,       // Off | One | All       │
│      shuffle: bool,                                     │
│  }                                                       │
│                                                         │
│  Methods:                                               │
│  ┌─────────────────────────────────────────────────┐   │
│  │ fn current(&self) -> Option<&TrackInfo>         │   │
│  │   → queue[cursor] (or shuffle_order[sc] if sh) │   │
│  ├─────────────────────────────────────────────────┤   │
│  │ fn advance(&mut self) -> Option<usize>          │   │
│  │   → new cursor, or None if at end (repeat=Off) │   │
│  │   Logic:                                        │   │
│  │     if repeat=One → return same index           │   │
│  │     if shuffle → sc += 1; if past end, reshuffle│   │
│  │     else → cursor += 1; if past end, wrap/stop  │   │
│  ├─────────────────────────────────────────────────┤   │
│  │ fn prev(&mut self) -> usize                     │   │
│  │   → reverse direction                          │   │
│  ├─────────────────────────────────────────────────┤   │
│  │ fn reshuffle(&mut self)                        │   │
│  │   → Fisher-Yates, keep current at front        │   │
│  ├─────────────────────────────────────────────────┤   │
│  │ fn set_queue(&mut self, tracks, start)         │   │
│  │ fn add(&mut self, track, position?)            │   │
│  │ fn remove(&mut self, index)                    │   │
│  │ fn move_item(&mut self, from, to)              │   │
│  └─────────────────────────────────────────────────┘   │
│                                                         │
│  Shuffle visual:                                        │
│    queue = [A, B, C, D, E]                              │
│    shuffle_order = [2, 0, 4, 1, 3]                     │
│    shuffle_cursor = 0 → queue[2] = C                   │
│    shuffle_cursor = 1 → queue[0] = A                   │
│    shuffle_cursor = 2 → queue[4] = E                   │
│    ...                                                   │
└─────────────────────────────────────────────────────────┘
```

## Crossfade

```rust
pub struct CrossfadeConfig {
    pub enabled: bool,
    pub duration_secs: u8,    // default 3
}
```

Crossfade flow:
```
track_1 at position (duration - crossfade_secs)
  → pre-start track_2 at volume 0
  → ramp track_1 volume 100% → 0% over crossfade_secs
  → ramp track_2 volume 0% → 100% over crossfade_secs
  → stop track_1, full play track_2
```

Implementation: `crossfade_start()` called from `handle_auto_advance` timer.
Uses AudioBackend's volume control + separate preload of next track.

## File Structure

```
gtm-daemon/src/
├── youtube.rs
├── cover_art.rs
├── lyrics.rs
└── queue.rs
```
