## Bugs (all fixed)

- ~~The TUI is unresponsive as soon as playback begins.~~ ✅ Fixed: background auto-scan, no blocking calls in event loop.
- ~~The user can't start playback from the TUI in the library tab.~~ ✅ Fixed: Enter plays selected track in library tab right pane.
- ~~The app randomly panics when a command that blocks the thread is executed.~~ ✅ Fixed: `blocking_lock()` → `.lock().await` in `parse()`.
