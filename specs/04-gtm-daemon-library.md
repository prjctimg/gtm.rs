# 04 — gtm-daemon: Library (SQLite)

## Purpose

Manage the local music library database: track metadata, playlists, favourites, search history,
lyrics cache, download queue. All SQLite access goes through the `Library` struct.

Depends on: `rusqlite` (bundled sqlite3 via feature), `gtm-core`

## Schema

```sql
-- Core track table
CREATE TABLE tracks (
    id          INTEGER PRIMARY KEY AUTOINCREMENT,
    path        TEXT UNIQUE NOT NULL,
    title       TEXT NOT NULL DEFAULT '',
    artist      TEXT NOT NULL DEFAULT '',
    album       TEXT NOT NULL DEFAULT '',
    duration    REAL NOT NULL DEFAULT 0.0,
    track_no    INTEGER,
    genre       TEXT,
    year        INTEGER,
    bitrate     INTEGER,
    samplerate  INTEGER,
    hash        TEXT NOT NULL,
    added_at    TEXT NOT NULL DEFAULT (datetime('now'))
);
CREATE INDEX idx_tracks_path ON tracks(path);
CREATE INDEX idx_tracks_artist ON tracks(artist);
CREATE INDEX idx_tracks_album ON tracks(album);

-- Playlists
CREATE TABLE playlists (
    id          INTEGER PRIMARY KEY AUTOINCREMENT,
    name        TEXT NOT NULL,
    created_at  TEXT NOT NULL DEFAULT (datetime('now'))
);

-- Playlist-tracks junction
CREATE TABLE playlist_tracks (
    playlist_id INTEGER NOT NULL REFERENCES playlists(id) ON DELETE CASCADE,
    track_id    INTEGER NOT NULL REFERENCES tracks(id) ON DELETE CASCADE,
    position    INTEGER NOT NULL,
    added_at    TEXT NOT NULL DEFAULT (datetime('now')),
    PRIMARY KEY (playlist_id, position)
);
CREATE INDEX idx_pl_tracks_playlist ON playlist_tracks(playlist_id);
CREATE INDEX idx_pl_tracks_track ON playlist_tracks(track_id);

-- Favourites
CREATE TABLE favourites (
    track_id  INTEGER PRIMARY KEY REFERENCES tracks(id) ON DELETE CASCADE,
    added_at  TEXT NOT NULL DEFAULT (datetime('now'))
);

-- Move-to-trash (soft delete)
CREATE TABLE trash (
    id          INTEGER PRIMARY KEY AUTOINCREMENT,
    track_id    INTEGER NOT NULL REFERENCES tracks(id) ON DELETE CASCADE,
    deleted_at  TEXT NOT NULL DEFAULT (datetime('now'))
);

-- Lyrics cache (key = artist+album+title hash or path hash)
CREATE TABLE lyrics_cache (
    track_key   TEXT PRIMARY KEY,
    source      TEXT NOT NULL DEFAULT 'local',  -- 'local' | 'lrclib'
    lyrics_data TEXT NOT NULL,
    updated_at  TEXT NOT NULL DEFAULT (datetime('now'))
);

-- Search history
CREATE TABLE search_history (
    id          INTEGER PRIMARY KEY AUTOINCREMENT,
    query       TEXT NOT NULL,
    searched_at TEXT NOT NULL DEFAULT (datetime('now'))
);
CREATE INDEX idx_search_history_time ON search_history(searched_at DESC);

-- Downloads (yt-dlp queue)
CREATE TABLE downloads (
    id          INTEGER PRIMARY KEY AUTOINCREMENT,
    url         TEXT NOT NULL,
    title       TEXT,
    status      TEXT NOT NULL DEFAULT 'pending',  -- pending | downloading | done | failed
    progress    REAL NOT NULL DEFAULT 0.0,
    path        TEXT,
    added_at    TEXT NOT NULL DEFAULT (datetime('now'))
);
```

## Library Struct

```rust
pub struct Library {
    conn: Connection,
    watch_paths: Vec<PathBuf>,
    scan_batch_size: usize,        // default 50
    metadata_extractor: MetadataExtractor,
}

impl Library {
    /// Open or create database at path
    pub fn open(path: &Path) -> Result<Self>;

    // ─── Track CRUD ───

    /// Add a single track file. Returns track id.
    /// Extracts metadata via symphonia or ffmpeg probe.
    /// Skips duplicates (matched by path).
    pub fn add_track(&mut self, path: &Path) -> Result<i64>;

    /// Scan a directory recursively. Returns progress handle.
    pub fn scan_directory(&mut self, path: &Path) -> Result<ScanProgress>;

    /// Remove a track by id (hard delete, not trash)
    pub fn remove_track(&mut self, id: i64) -> Result<()>;

    // ─── Queries ───

    /// Get tracks with optional filter + sort
    pub fn get_tracks(&self, filter: Option<&str>, sort: Option<&str>) -> Result<Vec<TrackInfo>>;

    /// Full-text search across title, artist, album
    pub fn search_tracks(&self, query: &str) -> Result<Vec<TrackInfo>>;

    /// Get by id or path
    pub fn get_track_by_id(&self, id: i64) -> Result<Option<TrackInfo>>;
    pub fn get_track_by_path(&self, path: &str) -> Result<Option<TrackInfo>>;

    /// Recently added
    pub fn get_recent_tracks(&self, count: usize) -> Result<Vec<TrackInfo>>;

    // ─── Playlists ───

    pub fn create_playlist(&mut self, name: &str) -> Result<i64>;
    pub fn delete_playlist(&mut self, id: i64) -> Result<()>;
    pub fn get_playlists(&self) -> Result<Vec<Playlist>>;
    pub fn get_playlist_tracks(&self, id: i64) -> Result<Vec<TrackInfo>>;
    pub fn add_to_playlist(&mut self, playlist_id: i64, track_ids: &[i64]) -> Result<()>;
    pub fn remove_from_playlist(&mut self, playlist_id: i64, position: usize) -> Result<()>;

    // ─── Playlist I/O ───

    pub fn import_m3u(&mut self, path: &Path) -> Result<usize>;
    pub fn export_m3u(&self, playlist_id: i64, path: &Path) -> Result<()>;

    // ─── Favourites ───

    pub fn add_favourite(&mut self, track_id: i64) -> Result<()>;
    pub fn remove_favourite(&mut self, track_id: i64) -> Result<()>;
    pub fn get_favourites(&self) -> Result<Vec<TrackInfo>>;
    pub fn is_favourite(&self, track_id: i64) -> bool;
}
```

## Metadata Extraction Flow

```
File path
    │
    ▼
┌──────────────────────────┐
│ symphonia::probe()       │ ← pure Rust, fast, supports MP3/FLAC/OGG/WAV
│  (primary, fast path)    │
└────────┬─────────────────┘
         │ success?
         ├── YES ──▶ extract tags → TrackInfo
         │
         └── NO ───▶
                    ┌──────────────────────────┐
                    │ ffmpeg -i <file> -f null  │ ← fallback, slower
                    │  (parse stderr metadata)  │   supports exotic formats
                    └────────┬─────────────────┘
                             │
                             ▼
                        TrackInfo (with duration, tags)
```

## Hash Computation

```rust
/// SHA-256 of first 64KB of file content
/// Used for deduplication and cache keys
pub fn compute_hash(path: &Path) -> Result<String>;
```

## Scan Progress

```rust
pub struct ScanProgress {
    pub total: usize,
    pub scanned: usize,
    pub added: usize,
    pub skipped: usize,
    pub errors: Vec<String>,
    pub done: bool,
}
```

Scan runs on a `tokio::task::spawn_blocking` thread to avoid blocking the async main loop.
Progress is polled via a `watch::Receiver<ScanProgress>` channel.

## Config Paths

```
Database:  $XDG_DATA_HOME/gtm/library.db
Default:   ~/.local/share/gtm/library.db
```

## File Structure

```
gtm-daemon/src/library.rs
```
