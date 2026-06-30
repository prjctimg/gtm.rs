# 13 — Development Phases

## Phase 0 — Foundation (Week 1-2)

**Goal:** Shared types, IPC protocol, audio abstraction compile and pass tests.

| Step | File(s) | Deliverable |
|------|---------|-------------|
| 0.1 | `Cargo.workspace.toml` | Workspace skeleton with 7 members |
| 0.2 | `gtm-core/src/ipc.rs` | All enums: `DaemonRequest`, `DaemonResponse`, `DaemonEvent` |
| 0.3 | `gtm-core/src/track.rs` | `TrackInfo`, `Playlist`, `LrcLine`, `LrcData` |
| 0.4 | `gtm-core/src/state.rs` | `DaemonState`, `PlaybackStatus` |
| 0.5 | `gtm-core/src/wire.rs` | `encode_frame()`, `decode_frame()` with roundtrip tests |
| 0.6 | `gtm-audio/src/backend.rs` | `AudioBackend` trait, `AudioEvent` enum |
| 0.7 | `gtm-audio/src/symphonia.rs` | `SymphoniaBackend`: file load, decode, cpal output |
| 0.8 | `tests/wire_protocol.rs` | Binary encode/decode roundtrip verification |

```
┌─────────────────────────────────────────────────────────────────┐
│ Phase 0 Checklist                                                │
│                                                                  │
│ [ ] cargo new --workspace gtm-rs                                │
│ [ ] gtm-core compiles with serde + bincode                      │
│ [ ] gtm-audio compiles with symphonia + cpal                    │
│ [ ] encode/decode roundtrip test passes (all event variants)    │
│ [ ] SymphoniaBackend loads a real .mp3 file and reports duration │
│ [ ] cargo test --workspace passes                               │
│ [ ] cargo clippy --workspace is clean                            │
└─────────────────────────────────────────────────────────────────┘
```

---

## Phase 1 — Daemon Core (Week 3-4)

**Goal:** `gtmd` starts, accepts IPC connections, plays audio, manages queue.

| Step | File(s) | Deliverable |
|------|---------|-------------|
| 1.1 | `gtm-daemon/src/daemon.rs` | Daemon struct, main loop with `tokio::select!` |
| 1.2 | `gtm-daemon/src/ipc.rs` | Unix socket listener, JSON line reader/writer |
| 1.3 | `gtm-daemon/src/dispatch.rs` | Request → handler mapping for all commands |
| 1.4 | `gtm-daemon/src/queue.rs` | QueueManager: cursor, shuffle, repeat |
| 1.5 | `gtm-daemon/src/library.rs` | SQLite schema, add_track, scan_directory, search |
| 1.6 | `gtm-daemon/src/main.rs` | CLI args, daemonize, config load |
| 1.7 | `tests/daemon_ipc.rs` | Spawn daemon, send play/pause/next, verify events |

```
Integration test flow:
  ┌──────────────────────────────────────────────┐
  │  1. spawn gtmd (--test-mode, ephemeral sock)  │
  │  2. connect to Unix socket                     │
  │  3. send Play{path: "test/fixtures/sample.mp3"}│
  │  4. verify DaemonResponse.ok == true           │
  │  5. poll events → PlaybackStarted received     │
  │  6. send Pause                                 │
  │  7. poll events → PlaybackPaused received      │
  │  8. send Stop                                  │
  │  9. poll events → PlaybackStopped received     │
  │  10. send Quit                                  │
  │  11. verify process exited cleanly              │
  └──────────────────────────────────────────────┘
```

---

## Phase 2 — TUI Skeleton (Week 5-6)

**Goal:** TUI renders tabs, connects to daemon, displays now playing.

| Step | File(s) | Deliverable |
|------|---------|-------------|
| 2.1 | `gtm-tui/src/main.rs` | crossterm init, alt screen, raw mode, panic hook |
| 2.2 | `gtm-tui/src/app.rs` | App struct, render(), 60fps throttle |
| 2.3 | `gtm-tui/src/state.rs` | AppState, LibraryViewState, QueueViewState, etc. |
| 2.4 | `gtm-tui/src/daemon_client.rs` | Connect, poll_events, request, state mirror |
| 2.5 | `gtm-tui/src/tabs/mod.rs` | TabWidget trait + Tab enum |
| 2.6 | `gtm-tui/src/tabs/now_playing.rs` | Album art placeholder, progress bar, controls |
| 2.7 | `gtm-tui/src/tabs/library.rs` | Track list, cursor, scroll |
| 2.8 | `gtm-tui/src/tabs/help.rs` | Keybinding table |
| 2.9 | `gtm-tui/src/footer.rs` | Playback status, position, volume |
| 2.10 | `gtm-tui/src/keymap.rs` | Keybinding map, input dispatch |

```
Minimum viable TUI:
┌──────────────────────────────────────────┐
│  Library │ Queue │ ▶Now Playing │ ... H  │
├──────────────────────────────────────────┤
│  ┌─────────┐  Song Title                  │
│  │         │  Artist Name                 │
│  │ Album   │  ────────────────            │
│  │ Art     │  ████████████░░░░  2:34/4:20 │
│  │ (ascii) │  ◀◀ ⏸ ▶▶  🔊 75%            │
│  └─────────┘                              │
├──────────────────────────────────────────┤
│  ⏸ 2:34/4:20  ████████░░  Vol:75% 🔀 🔁  │
└──────────────────────────────────────────┘
```

---

## Phase 3 — Remaining Tabs & Overlays (Week 7-8)

**Goal:** All 6 tabs and 6 overlays functional.

| Step | File(s) | Deliverable |
|------|---------|-------------|
| 3.1 | `gtm-tui/src/tabs/queue.rs` | Queue display, move mode, delete, clear |
| 3.2 | `gtm-tui/src/tabs/youtube.rs` | Search UI, results, play stream |
| 3.3 | `gtm-tui/src/tabs/settings.rs` | All config options, inline editing |
| 3.4 | `gtm-tui/src/overlays/command_palette.rs` | `:` command input, autocomplete |
| 3.5 | `gtm-tui/src/overlays/fuzzy_finder.rs` | Fuzzy search over tracks |
| 3.6 | `gtm-tui/src/overlays/queue_picker.rs` | Insert position selector |
| 3.7 | `gtm-tui/src/overlays/theme_picker.rs` | Theme selection with preview |
| 3.8 | `gtm-tui/src/overlays/confirm_dialog.rs` | Yes/no confirmations |
| 3.9 | `gtm-tui/src/overlays/track_detail.rs` | Full track metadata + lyrics + cover |

---

## Phase 4 — Advanced Features (Week 9-10)

**Goal:** Lyrics syncing, album art, YouTube streaming, MPRIS.

| Step | File(s) | Deliverable |
|------|---------|-------------|
| 4.1 | `gtm-daemon/src/lyrics.rs` | LRC sidecar, LRCLIB API, SQLite cache |
| 4.2 | `gtm-tui/src/tabs/now_playing.rs` | Synced lyrics display (green highlight) |
| 4.3 | `gtm-daemon/src/cover_art.rs` | Deezer search + LRU cache |
| 4.4 | `gtm-tui/src/graphics.rs` | Kitty protocol transmit/delete/place |
| 4.5 | `gtm-tui/src/tabs/now_playing.rs` | Cover image rendering via Kitty |
| 4.6 | `gtm-daemon/src/youtube.rs` | yt-dlp subprocess, search, stream resolve |
| 4.7 | `gtm-mpris/src/lib.rs` | MPRIS D-Bus server, properties, methods |

---

## Phase 5 — CLI Client (Week 10-11)

**Goal:** Complete headless CLI controller.

| Step | File(s) | Deliverable |
|------|---------|-------------|
| 5.1 | `gtm-cli/src/main.rs` | All subcommands, IPC dispatch |
| 5.2 | `gtm-cli/src/completions.rs` | Shell completion scripts |
| 5.3 | `gtm-daemon/src/daemon.rs` | Daemon auto-launch from CLI |

---

## Phase 6 — Polish & Release (Week 12+)

**Goal:** Production-ready.

| Area | Tasks |
|------|-------|
| **Testing** | Unit tests for all modules, integration tests for IPC flow, property-based tests for shuffle |
| **Benchmarks** | Audio decode throughput, IPC event throughput, TUI frame time |
| **Cross-platform** | Linux primary, macOS (CoreAudio via cpal), Windows (WASAPI via cpal) |
| **Packaging** | `cargo install`, Docker, `.deb`/`.rpm`, Homebrew |
| **Documentation** | README, man pages, IPC protocol spec, architecture doc |
| **CI/CD** | GitHub Actions: `cargo test`, `cargo clippy`, `cargo fmt --check`, binary releases |

---

## Risks & Mitigations

| Risk | Impact | Mitigation | Phase |
|------|--------|------------|-------|
| **symphonia doesn't support format X** | Medium | Feature-gate `ffmpeg-next` fallback | 0 |
| **cpal audio glitches on some hardware** | Medium | Adjust buffer sizes, add `cpal` feature flags | 0 |
| **IPC protocol version skew** | Low | Include `version` in wire frames, graceful mismatch | 0 |
| **yt-dlp API changes** | Medium | Parse stderr for warnings, pin yt-dlp version in docs | 4 |
| **D-Bus connection failure** | Low | MPRIS is optional, daemon runs without it | 4 |
| **Large library scan blocks daemon** | Medium | `spawn_blocking`, progress channel, batch size config | 1 |
| **TUI render at 60fps on slow terminals** | Low | Adaptive throttle, skip frames, only re-render on changes | 2 |
| **Kitty protocol not supported** | Low | Fallback to ASCII box for album art | 4 |
