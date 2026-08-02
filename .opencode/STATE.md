# Loop State — gtm.rs

Last run: 2026-07-31

## High Priority (loop is acting or waiting on human)

- Phase 3 (one-time user queue + default-list auto-advance) is **code-complete and test-green** in the worktree:
  - `gtmd/tests/daemon_test.rs` all 7 pass. Fixed test harness to speak the real wire protocol: `TestReader` now parses `WireRes` envelopes (skipping binary event frames), `send_req` sends `WireReq { id, cmd, params }` via `cmd_name()`, and `connect()` performs the required handshake.
  - Genuine bugs fixed along the way (all in `gtm-core`/`gtmd`, no auto-merge):
    1. `DaemonRes::ok_from_data`: a plain ack (`{"id":..,"ok":true}`) deserializes as `data = Some(Object{})` via serde flatten, so typed cmds misparsed acks as `Value{Object{}}`. Now Null/empty-object payloads decode to `DaemonRes::Ok` (except `ping` → `Pong`). This was breaking the real client's `send_ok` too.
    2. `queue::insert_at`: boundary `pos == ulen` inserted into `default_list` instead of the user queue (adding to an empty queue landed the track in the default list). Changed `<` to `<=`.
    3. `serde_json` cannot deserialize `u128` through internally-tagged enum content buffering ("u128 is not supported"). Added `u128_serde`/`opt_u128_serde` helpers (carry as `u64`, widen to `u128`) on `QueueAction::{Remove,Move,Add,Set}` and `LibraryAction::GetRecent::count`.
  - Verified: `cargo test` all pass; `cargo clippy --all-targets` warnings 91 → 86 (none introduced); `cargo fmt --check` clean for worktree files (pre-existing diffs remain only in files not touched this session: gtm/src/app.rs, gtm/src/ui.rs, gtm-core/src/client.rs, gtm-audio/src/mixer.rs, gtmd/src/daemon.rs).
  - `test_mixer_poll_finished` (gtm-audio) is flaky under full-suite parallel load (passes 5/5 solo, 3/3 full-binary); pre-existing timing/audio-device contention, no code change made.

## Watch List

- `gtm/src/app.rs` + `gtm/src/ui.rs` still have pre-existing `cargo fmt`/`cargo clippy` debt (Phase 4 TUI queue consumption target). Do not format/merge them in this phase.

## Recent Noise (ignored this run)

- Mixer underrun-grace test flake (see High Priority).

---
Run log: 2026-07-31 — completed gtmd daemon_test harness rewrite (WireRes envelope + handshake), fixed ok_from_data ack decode, insert_at boundary bug, u128 wire helpers; all 7 daemon tests green; full workspace green.
