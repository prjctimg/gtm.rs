# 02 — gtm-audio: Audio Backend Abstraction

## Purpose

Defines the `AudioBackend` trait and provides implementations using `symphonia` (pure Rust)
and optionally `ffmpeg-next` (C bindings). Audio output via `cpal` for cross-platform playback.

Depends on: `gtm-core`, `symphonia`, `cpal`, `rubato` (resampling), `log`

Used by: `gtm-daemon`, `gtm-tui`

## Trait

```rust
#[async_trait]
pub trait AudioBackend: Send {
    /// Load a file at path, optionally seeking to start_pos
    async fn load(&mut self, path: &str, start_pos: f64) -> Result<()>;

    /// Start/resume playback
    async fn play(&mut self) -> Result<()>;

    /// Pause without unloading
    async fn pause(&mut self) -> Result<()>;

    /// Stop and unload
    async fn stop(&mut self) -> Result<()>;

    /// Seek to absolute position in seconds
    async fn seek(&mut self, position_secs: f64) -> Result<()>;

    /// Set volume 0-100
    async fn set_volume(&mut self, volume: u8) -> Result<()>;

    /// Poll for pending events (non-blocking)
    async fn poll(&mut self) -> Result<Option<AudioEvent>>;

    // Getters
    fn current_position(&self) -> f64;
    fn duration(&self) -> f64;
    fn is_playing(&self) -> bool;
    fn volume(&self) -> u8;
}

pub enum AudioEvent {
    Position(f64),
    Duration(f64),
    Finished,
    Volume(u8),
    Error(String),
}
```

## SymphoniaBackend (Primary)

```
┌────────────────────   SymphoniaBackend   ────────────────────┐
│                                                                │
│  ┌─────────┐   ┌──────────┐   ┌─────────┐   ┌────────────┐   │
│  │ File    │──▶│ Symphonia│──▶│ rubato  │──▶│ cpal       │   │
│  │ (path)  │   │ Decoder  │   │ resample│   │ audio out  │   │
│  └─────────┘   └──────────┘   └─────────┘   └────────────┘   │
│                                                    │          │
│                                                    ▼          │
│                                           ┌──────────────┐    │
│                                           │ volume ramp  │    │
│                                           │ (linear f64) │    │
│                                           └──────────────┘    │
│                                                                │
│  ┌────────────────────────────────────────────────────────┐   │
│  │ Audio Thread (dedicated)                               │   │
│  │  • decodes packets in a loop                           │   │
│  │  • writes samples to cpal output stream                │   │
│  │  • updates AtomicF64 position                          │   │
│  │  • signals Finished via channel                        │   │
│  └────────────────────────────────────────────────────────┘   │
└────────────────────────────────────────────────────────────────┘
```

The `SymphoniaBackend` spawns a dedicated audio thread:

```
┌──────────────┐     ┌──────────────────┐     ┌────────────────┐
│ main thread  │     │ audio thread     │     │ cpal callback  │
│              │     │                  │     │                │
│ load() ──────┼────▶│ init decoder     │     │                │
│              │     │ start stream ────┼────▶│ stream.run()   │
│ play() ──────┼────▶│ resume flag = T  │     │                │
│ pause() ─────┼────▶│ resume flag = F  │     │                │
│ seek() ──────┼────▶│ flush, seek      │     │                │
│              │     │                  │     │                │
│ poll() ◀─────┼────▶│ check event ch   │     │                │
│              │     │                  │     │                │
│              │     │  ┌────────────┐  │     │                │
│              │     │  │ decode loop│  │     │                │
│              │     │  │ (while T)  │  │     │                │
│              │     │  └─────┬──────┘  │     │                │
│              │     │        │ samples │     │                │
│              │     │        ▼         │     │                │
│              │     │  ┌────────────┐  │     │                │
│              │     │  │ ring buffer│──┼────▶│ write samples  │
│              │     │  └────────────┘  │     │                │
│              │     │        │         │     │                │
│              │     │  finished ───────┼────▶│ poll() returns │
│              │     │                  │     │ Finished       │
└──────────────┘     └──────────────────┘     └────────────────┘
```

## FfmpegBackend (Subprocess)

```
┌────────────────────   FfmpegBackend   ─────────────────────┐
│                                                              │
│  ┌──────────┐    stdin (WAV)    ┌──────────────────┐        │
│  │ ffmpeg   │ ─────────────────▶│ cpal stream      │        │
│  │ process  │   pipe            │ (raw PCM)        │        │
│  └──────────┘                   └──────────────────┘        │
│       │                              │                      │
│       │ stderr (progress lines)      │                      │
│       ▼                              ▼                      │
│  ┌──────────────┐             ┌──────────────┐             │
│  │ progress     │             │ position     │             │
│  │ parser       │             │ AtomicF64    │             │
│  │ (time=xxx)   │             └──────────────┘             │
│  └──────────────┘                                          │
│                                                              │
│  Strategy: spawn `ffmpeg -i <file> -f wav -`                │
│  Pipe raw WAV to cpal input. Parse stderr for position.     │
│  Good for exotic formats symphonia can't decode.            │
└──────────────────────────────────────────────────────────────┘
```

## Volume Handling

Volume is applied as a linear multiplier to PCM samples before sending to `cpal`:

```
sample_out = sample_in * (volume / 100.0)
```

For `SymphoniaBackend`: applied in the audio thread on the read side of the ring buffer.
For `FfmpegBackend`: applied before writing to the cpal output stream.

## File Structure

```
gtm-audio/
├── Cargo.toml
└── src/
    ├── lib.rs          # re-exports
    ├── backend.rs      # AudioBackend trait, AudioEvent enum
    ├── symphonia.rs    # SymphoniaBackend
    └── ffmpeg.rs       # FfmpegBackend (optional, feature-gated)
```

## Errors

```rust
#[derive(Debug, thiserror::Error)]
pub enum AudioError {
    #[error("failed to open file: {0}")]
    OpenFailed(String),
    #[error("decode error: {0}")]
    DecodeError(String),
    #[error("output error: {0}")]
    OutputError(String),
    #[error("unsupported format: {0}")]
    UnsupportedFormat(String),
    #[error("ffmpeg not found")]
    FfmpegNotFound,
    #[error("ffmpeg error: {0}")]
    FfmpegError(String),
}
```
