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
- `.opencode/STATE.md` — current loop memory
- `.opencode/LOOP.md` — active loops and cadence
- `.opencode/loop-budget.md` — token caps
- `.opencode/loop-constraints.md` — binding agent rules
- `.opencode/loop-run-log.md` — run history
- `.opencode/gate.yaml` — path denylist + auto-merge allowlist
- `.opencode/skills/` — triage and verifier skills

Start a loop: `opencode run "Run loop-triage. Update .opencode/STATE.md."`
Verify changes: `opencode run "Verify diff in worktree" --agent verifier`
