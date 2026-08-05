# Development

## Building

```bash
# Build all crates
cargo build

# Build release binary
cargo build --release

# Build with PulseAudio support
cargo build --features pulseaudio
```

## Testing

```bash
# Run all tests
cargo test

# Run tests for a specific crate
cargo test -p gtm
cargo test -p gtmd
cargo test -p gtm-core

# Run inline tests only (no integration tests)
cargo test --lib -p gtm
```

## Linting & Formatting

```bash
# Clippy lints
cargo clippy

# Format check
cargo fmt --check

# Auto-format
cargo fmt
```

## Project Structure

```
gtm.rs/
├── gtm-core/      Shared types, IPC protocol, state machine, DaemonClient
├── gtm-audio/     Audio playback backend (rodio + symphonia), EQ, reverb
├── gtmd/          Background daemon — playback, queue, library, IPC
├── gtm/           Client — TUI (ratatui) + CLI (clap)
├── gtm-mpris/     MPRIS D-Bus integration
├── release-gen/   Build-time tool for shell completions
├── docs/          Documentation (manpages, wiki)
└── scripts/       Build scripts (manpage generation)
```

## Key Source Files

| File | Purpose |
|------|---------|
| `gtm/src/ui.rs` | TUI rendering (ratatui) |
| `gtm/src/app.rs` | Application state and event handling |
| `gtm/src/keymap.rs` | Context-aware keybinding dispatch |
| `gtm/src/theme.rs` | Color theme system |
| `gtm/src/footer.rs` | Footer bar rendering |
| `gtm/src/cli.rs` | CLI command definitions |
| `gtm/src/picker.rs` | Picker overlay system |
| `gtmd/src/daemon.rs` | Main daemon logic and event loop |
| `gtmd/src/library.rs` | SQLite-backed music library |
| `gtm-core/src/ipc.rs` | IPC protocol implementation |
| `gtm-core/src/state.rs` | Daemon state and playback status |

## Manpage Generation

Manpages are generated from Markdown source in `docs/man/`:

```bash
# Generate manpages
make man

# Install manpages
make install

# Build .deb package (includes manpages)
make deb
```

The generation script (`scripts/gen-manpages.sh`) prefers `gtm.spec/man/` over `docs/man/` if the external spec repo is cloned alongside this repository.

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make changes with tests
4. Run `cargo clippy`, `cargo fmt`, `cargo test`
5. Submit a pull request