# gtm.rs - Agent Notes

## Build & Test

```bash
cargo build                    # build all crates
cargo test                     # run all tests
cargo clippy                   # lint check
cargo fmt --check              # format check
```

## Architecture

- Modular crate suite: gtm-core, gtm-audio, gtmd, gtm, gtm-mpris
- MPRIS D-Bus integration for media player controls
- PulseAudio support for Android/Termux
- Systemd user service support

## Coding Guidelines

- Follow Rust idioms and error handling patterns
- Use `cargo clippy` warnings as guide
- Write tests for new functionality
- Follow conventional commit guidelines

## Loop Engineering

This repo uses loop engineering patterns. See:
- `STATE.md` — current loop memory
- `LOOP.md` — active loops and cadence
- `loop-budget.md` — token caps
- `loop-constraints.md` — binding agent rules
- `loop-run-log.md` — run history
- `gate.yaml` — path denylist + auto-merge allowlist
- `skills/` — triage and verifier skills

Start a loop: `opencode run "Run loop-triage. Update STATE.md."`
Verify changes: `opencode run "Verify diff in worktree" --agent verifier`
