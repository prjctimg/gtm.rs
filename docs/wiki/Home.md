# GTM — Terminal Music Player

GTM is a terminal-based music player with a TUI client, a background daemon, and optional MPRIS D-Bus integration. It supports local file playback, YouTube streaming, Spotify, and full library management.

## Quick Start

```bash
# Build
cargo build --release

# Run the TUI
./target/release/gtm

# CLI mode — play a track
./target/release/gtm -c play ~/Music/song.flac

# Start the daemon (usually via systemd user service)
gtmd
```

## Crate Overview

| Crate | Purpose |
|-------|---------|
| **gtm-core** | Shared types, IPC protocol, state machine, `DaemonClient` |
| **gtm-audio** | Audio playback backend (rodio + symphonia), EQ, reverb, crossfade |
| **gtmd** | Background daemon — playback, queue, library, IPC, YouTube/Spotify |
| **gtm** | Client — TUI (ratatui) + CLI (clap) |
| **gtm-mpris** | MPRIS D-Bus integration for desktop media keys |

## Key Features

- Full TUI with 3 tabs: Now Playing, Library, Settings
- 60+ keybindings with vim-style navigation
- Customizable themes (12 built-in + TOML user themes)
- Customizable footer presets (left/middle/right layout)
- Command palette, search, filter mode
- Queue management with add, remove, reorder, clear
- Library scanning, playlists, favourites, recent tracks
- YouTube search and stream resolution
- Spotify integration
- Lyrics fetching via lrclib
- Cover art display (Kitty, Sixel, iTerm2, half-block)
- Parametric EQ (15 bands), reverb, crossfade, sleep timer
- Dynamic mode, loudness compensation, gapless playback
- Scrobbling
- MPRIS D-Bus media player integration

## Installation

```bash
# From crates.io
cargo install gtm

# From source
git clone https://github.com/prjctimg/gtm.rs
cd gtm.rs
cargo install --path gtm
```

## Configuration

Configuration lives in `~/.config/gtm/`:

| Path | Purpose |
|------|---------|
| `~/.config/gtm/themes/` | TOML user themes |
| `~/.config/gtm/footer.toml` | TOML user footer presets |
| `$XDG_RUNTIME_DIR/gtm/gtmd.sock` | Daemon IPC socket |
| `$XDG_DATA_HOME/gtmd/library.db` | SQLite library database |

## Documentation

- [Architecture](Architecture.md) — Crate breakdown and data flow
- [TUI Guide](TUI-Guide.md) — Tab navigation and keybindings
- [CLI Reference](CLI-Reference.md) — All CLI commands
- [Configuration](Configuration.md) — Themes and footer presets
- [IPC Protocol](IPC-Protocol.md) — Daemon communication
- [Development](Development.md) — Building and contributing