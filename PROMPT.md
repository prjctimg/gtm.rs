## Bugs (all fixed)

- ~~TUI freezes / 120MB OOM on playback start~~ ✅ Fixed: 30Hz rate-limited daemon poll, no PositionChanged events, client-side clock skewing.
- ~~TUI unresponsive as soon as playback begins~~ ✅ Fixed: background auto-scan, no blocking calls in event loop.
- ~~Can't start playback from TUI in library tab~~ ✅ Fixed: Enter plays selected track in library right pane.
- ~~Random panics on blocking_lock()~~ ✅ Fixed: all `blocking_lock()` replaced with async `.lock().await`.
