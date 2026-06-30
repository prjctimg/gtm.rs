# 12 — Full Workspace File Structure

```
gtm-rs/
│
├── Cargo.workspace.toml          # workspace root, members
├── Cargo.lock
├── README.md
├── .gitignore
├── .github/
│   └── workflows/
│       ├── ci.yml                # cargo test, cargo clippy, cargo fmt
│       └── release.yml           # build + deploy binaries
│
├── docs/
│   ├── ipc.md                    # IPC protocol specification
│   ├── architecture.md           # high-level architecture diagram
│   └── theme.md                  # theme customization guide
│
├── assets/
│   └── default_cover.png         # fallback album art image
│
├── specs/                        # ← this directory
│   ├── INDEX.md
│   ├── 00-overview.md
│   ├── 01-gtm-core.md
│   ├── 02-gtm-audio.md
│   ├── 03-gtm-daemon.md
│   ├── 04-gtm-daemon-library.md
│   ├── 05-gtm-daemon-features.md
│   ├── 06-gtm-tui-architecture.md
│   ├── 07-gtm-tui-tabs.md
│   ├── 08-gtm-tui-overlays.md
│   ├── 09-gtm-tui-features.md
│   ├── 10-gtm-cli.md
│   ├── 11-gtm-mpris.md
│   ├── 12-file-structure.md
│   ├── 13-development-phases.md
│   └── 14-migration-decisions.md
│
├── gtm-core/
│   ├── Cargo.toml                # serde, serde_json, bincode, thiserror, chrono, uuid
│   └── src/
│       ├── lib.rs                # re-exports all public types
│       ├── ipc.rs                # DaemonRequest, DaemonResponse, DaemonEvent, enums
│       ├── wire.rs               # encode_frame, decode_frame, WireFrame
│       ├── track.rs              # TrackInfo, Playlist, LrcLine, LrcData, RepeatMode
│       └── state.rs              # DaemonState, PlaybackStatus, CrossfadeConfig
│
├── gtm-audio/
│   ├── Cargo.toml                # symphonia, cpal, rubato, thiserror, log
│   └── src/
│       ├── lib.rs                # re-exports AudioBackend, AudioEvent
│       ├── backend.rs            # AudioBackend trait, AudioEvent enum, AudioError
│       ├── symphonia.rs          # SymphoniaBackend (pure Rust decode + cpal output)
│       └── ffmpeg.rs             # FfmpegBackend (ffmpeg subprocess, feature-gated)
│
├── gtm-daemon/
│   ├── Cargo.toml                # gtm-core, gtm-audio, gtm-mpris, rusqlite, tokio, reqwest, tracing
│   └── src/
│       ├── main.rs               # gtmd binary: CLI (clap), daemon init, run loop
│       ├── daemon.rs             # Daemon struct, main loop, state machine
│       ├── ipc.rs                # IpcServer, ClientHandle, read/write helpers
│       ├── dispatch.rs           # request → handler dispatch table
│       ├── library.rs            # Library (rusqlite wrapper, schema, queries)
│       ├── queue.rs              # QueueManager (cursor, shuffle, repeat)
│       ├── youtube.rs            # YoutubeManager (yt-dlp subprocess, search/resolve)
│       ├── cover_art.rs          # CoverCache (Deezer API, LRU memory + disk cache)
│       ├── lyrics.rs             # LyricsManager (sidecar, LRCLIB, cache)
│       └── config.rs             # Config loading (XDG paths, CLI overrides)
│
├── gtm-tui/
│   ├── Cargo.toml                # ratatui, crossterm, tokio, gtm-core, gtm-audio, image, base64
│   └── src/
│       ├── main.rs               # gtm binary: args, terminal init, event loop
│       ├── app.rs                # App struct, render(), terminal.draw()
│       ├── state.rs              # AppState, per-tab view states
│       ├── daemon_client.rs      # DaemonClient (IPC transport + state mirror + extrapolation)
│       ├── keymap.rs             # Keybindings, parse_keycode, KeyContext, KeyboardAction
│       ├── theme.rs              # Theme struct, presets, hsl_to_rgb, random generation
│       ├── graphics.rs           # KittyGraphics (probe, transmit, delete, place)
│       ├── icons.rs              # IconSet, NERD_FONT, EMOJI constants
│       ├── footer.rs             # FooterBar, FooterModule enum, rendering
│       ├── tabs/
│       │   ├── mod.rs            # TabWidget trait, Tab enum
│       │   ├── library.rs        # LibraryTab (tracks, playlists, favs, recent)
│       │   ├── queue.rs          # QueueTab (now playing, up next, move mode)
│       │   ├── now_playing.rs    # NowPlayingTab (album art, progress, controls, lyrics)
│       │   ├── youtube.rs        # YouTubeTab (search, results, stream play)
│       │   ├── settings.rs       # SettingsTab (all config options)
│       │   └── help.rs           # HelpTab (keybindings reference)
│       └── overlays/
│           ├── mod.rs            # Overlay enum, render/dispatch
│           ├── command_palette.rs # CommandPaletteState (modal command input)
│           ├── fuzzy_finder.rs   # FuzzyFinderState (fuzzy search overlay)
│           ├── queue_picker.rs   # QueuePickerState (insert position selector)
│           ├── theme_picker.rs   # ThemePickerState (theme selection with preview)
│           ├── confirm_dialog.rs # ConfirmState (yes/no with action callback)
│           └── track_detail.rs   # TrackDetailState (metadata, cover, lyrics)
│
├── gtm-cli/
│   ├── Cargo.toml                # clap (derive), gtm-core, tokio
│   └── src/
│       ├── main.rs               # CLI dispatch, DaemonClient::request() calls
│       └── completions.rs        # shell completion generation (clap_complete)
│
├── gtm-mpris/
│   ├── Cargo.toml                # zbus (tokio), zvariant, gtm-core
│   └── src/
│       └── lib.rs                # MprisServer, root + player D-Bus interfaces
│
└── tests/
    ├── integration/
    │   ├── daemon_ipc.rs         # spawn daemon, send requests, verify responses
    │   └── wire_protocol.rs      # binary frame encode/decode roundtrips
    └── fixtures/
        ├── sample.mp3
        ├── sample.flac
        ├── sample.ogg
        ├── sample.wav
        └── sample.lrc
```

## Summary

| Area | Files | Lines (est.) |
|------|-------|-------------|
| Specs | 15 | ~3,000 |
| gtm-core | 5 | ~800 |
| gtm-audio | 4 | ~600 |
| gtm-daemon | 10 | ~3,000 |
| gtm-tui | 20+ | ~5,000+ |
| gtm-cli | 3 | ~300 |
| gtm-mpris | 2 | ~400 |
| Tests | 3+ | ~500 |
| **Total** | **~62** | **~13,000** |

## Key Configuration Paths

```
Socket:     /run/user/$UID/gtmd.socket           (XDG_RUNTIME_DIR/gtmd.socket)
Database:   $XDG_DATA_HOME/gtm/library.db        (~/.local/share/gtm/)
Config:     $XDG_CONFIG_HOME/gtm/config.toml     (~/.config/gtm/)
Cache:      $XDG_CACHE_HOME/gtm/                 (~/.cache/gtm/)
Covers:     $XDG_CACHE_HOME/gtm/covers/
Log:        $XDG_STATE_HOME/gtm/gtmd.log         (~/.local/state/gtm/)
```
