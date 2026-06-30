# 00 — Project Overview

Rewrite **gtm** (terminal music player) from Nim → Rust.

| Aspect | Nim Original | Rust Rewrite |
|--------|-------------|--------------|
| TUI framework | `nimwave` / `illwave` | `ratatui` + `crossterm` |
| Audio decode | FFmpeg subprocess | `symphonia` (pure Rust) + optional `ffmpeg-next` |
| Audio output | ALSA (Linux only) | `cpal` (cross-platform) |
| SQLite | vendored `sqlite3.c` | `rusqlite` (bundled) |
| HTTP | `httpclient` stdin pipe | `reqwest` (async) |
| Serialization | `json` module | `serde` + `serde_json` + `bincode` |
| IPC | Unix sockets (manual) | `tokio::net::UnixStream` |
| Async | single-threaded poll | `tokio` (multi-task) |
| D-Bus | `dbus` bindings | `zbus` + `zvariant` |

## Crate Dependency Graph

```
┌─────────────────────────────────────────────────┐
│                 gtm (TUI binary)                 │
│  ┌──────────┐  ┌──────────┐  ┌───────────────┐  │
│  │ ratatui  │  │ crossterm│  │ gtm-core      │  │
│  │ (widgets)│  │ (input)  │  │ (types, IPC)  │  │
│  └──────────┘  └──────────┘  └───────┬───────┘  │
│                                      │          │
│                             ┌────────▼───────┐  │
│                             │ gtm-audio      │  │
│                             │ (symphonia)    │  │
│                             └────────────────┘  │
└─────────────────────────────────────────────────┘
                        │ IPC (Unix socket, JSON + binary)
                        ▼
┌─────────────────────────────────────────────────┐
│              gtmd (daemon binary)                 │
│  ┌──────────┐  ┌──────────┐  ┌───────────────┐  │
│  │ gtm-core │  │ gtm-audio│  │ gtm-mpris     │  │
│  │ (types)  │  │ (backend)│  │ (zbus D-Bus)  │  │
│  └──────────┘  └──────────┘  └───────────────┘  │
│  ┌──────────┐  ┌──────────┐                     │
│  │ rusqlite │  │ reqwest  │                     │
│  │ (lib)    │  │ (HTTP)   │                     │
│  └──────────┘  └──────────┘                     │
└─────────────────────────────────────────────────┘
```

## Crate Legend

| Crate | Type | Deps In | Deps Out | Key Crates |
|-------|------|---------|----------|------------|
| `gtm-core` | lib | all | — | `serde`, `bincode`, `thiserror` |
| `gtm-audio` | lib | daemon, TUI | `gtm-core` | `symphonia`, `cpal` |
| `gtm-daemon-lib` | lib | gtmd | `gtm-core`, `gtm-audio`, `gtm-mpris` | `rusqlite`, `tokio`, `reqwest` |
| `gtmd` (bin) | bin | — | `gtm-daemon-lib` | `clap` |
| `gtm-tui` | bin | — | `gtm-core`, `gtm-audio` | `ratatui`, `crossterm`, `tokio` |
| `gtm-cli` | bin | — | `gtm-core` | `clap`, `tokio` |
| `gtm-mpris` | lib | gtmd | `gtm-core` | `zbus`, `zvariant` |

## Workspace Layout

```
gtm-rs/
├── Cargo.workspace.toml
├── Cargo.lock
├── README.md
├── docs/
├── assets/
├── gtm-core/
├── gtm-audio/
├── gtm-daemon/
├── gtm-tui/
├── gtm-cli/
├── gtm-mpris/
├── tests/
└── specs/
```
