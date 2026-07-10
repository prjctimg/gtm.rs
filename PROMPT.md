## Status (all fixed)

- ~~Crossfade inaudible / no easing~~ ✅ Fixed: `force_complete_crossfade` no longer emits `Finished` for the new track. `AudioEvent::Finished` emits `PlaybackStarted` when crossfading. 5 easing variants (Linear, SlowFadeInFastFadeOut, FastFadeInSlowFadeOut, Logarithmic, Smoothstep) implemented in mixer and wired through `CrossfadeConfig.easing`.
- ~~TUI state not restored on reattach~~ ✅ Fixed: `TrackEnded` now handled by `apply_event()`. Clock-skewing seeded from `GetStatus` snapshot via `seed_clock_from_state()` called after `fetch_state()`.
- ~~100% CPU / OOM on playback start~~ ✅ Fixed: 30Hz rate-limited daemon poll, no PositionChanged events, client-side clock skewing.
- ~~TUI unresponsive as soon as playback begins~~ ✅ Fixed: background auto-scan, no blocking calls in event loop.
- ~~Can't start playback from TUI in library tab~~ ✅ Fixed: Enter plays selected track in library right pane.
- ~~Random panics on blocking_lock()~~ ✅ Fixed: all `blocking_lock()` replaced with async `.lock().await`.
