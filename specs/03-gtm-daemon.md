# 03 — gtm-daemon: Background Audio Daemon

## Purpose

The daemon (`gtmd`) owns all audio state, manages the playback queue, communicates with the
library database, handles yt-dlp streaming, and broadcasts state changes to connected clients.
It is the single source of truth.

Depends on: `gtm-core`, `gtm-audio`, `gtm-mpris`, `rusqlite`, `tokio`, `reqwest`, `tracing`

## Daemon Architecture

```
┌──────────────────────────────────────────────────────────────────┐
│                        gtmd Process                              │
│                                                                  │
│  ┌──────────────────────────────────────────────────────────┐   │
│  │ main()                                                   │   │
│  │  1. parse CLI args (--socket, --library, --verbose)      │   │
│  │  2. load config                                          │   │
│  │  3. init Daemon struct                                   │   │
│  │  4. daemon.run().await                                   │   │
│  └─────────────────────┬────────────────────────────────────┘   │
│                        │                                        │
│                        ▼                                        │
│  ┌──────────────────────────────────────────────────────────┐   │
│  │ Daemon::run() (main loop)                                │   │
│  │                                                          │   │
│  │  loop {                                                  │   │
│  │      tokio::select! {                                    │   │
│  │          conn = listener.accept()    → accept_client()   │   │
│  │          req  = read_request()        → dispatch()       │   │
│  │          ev   = backend.poll()        → handle_event()   │   │
│  │          _    = sleep_timer.tick()    → check_timer()    │   │
│  │      }                                                    │   │
│  │      flush_events();  // broadcast to all clients        │   │
│  │  }                                                        │   │
│  └──────────────────────────────────────────────────────────┘   │
│                                                                  │
│  ┌──────────────────────────────────────────────────────────┐   │
│  │ Daemon struct fields                                    │   │
│  │  ┌──────────────────┐  ┌──────────────────┐             │   │
│  │  │ state:           │  │ backend:         │             │   │
│  │  │ Arc<RwLock<      │  │ Box<dyn          │             │   │
│  │  │ DaemonState>>    │  │ AudioBackend>    │             │   │
│  │  └──────────────────┘  └──────────────────┘             │   │
│  │  ┌──────────────────┐  ┌──────────────────┐             │   │
│  │  │ library:         │  │ listener:        │             │   │
│  │  │ Option<Library>  │  │ UnixListener     │             │   │
│  │  └──────────────────┘  └──────────────────┘             │   │
│  │  ┌──────────────────┐  ┌──────────────────┐             │   │
│  │  │ clients:         │  │ event_tx:        │             │   │
│  │  │ Vec<ClientConn>  │  │ broadcast::Sender│             │   │
│  │  └──────────────────┘  └──────────────────┘             │   │
│  │  ┌──────────────────┐  ┌──────────────────┐             │   │
│  │  │ yt_manager:      │  │ cover_cache:     │             │   │
│  │  │ YoutubeManager   │  │ CoverCache       │             │   │
│  │  └──────────────────┘  └──────────────────┘             │   │
│  │  ┌──────────────────┐  ┌──────────────────┐             │   │
│  │  │ lyrics_manager:  │  │ sleep_timer:     │             │   │
│  │  │ LyricsManager    │  │ Option<Instant>  │             │   │
│  │  └──────────────────┘  └──────────────────┘             │   │
│  └──────────────────────────────────────────────────────────┘   │
└──────────────────────────────────────────────────────────────────┘
```

## State Machine

```
                ┌──────────┐
                │  STOPPED │◀──────────────────┐
                └─────┬────┘                    │
                      │ play()                  │ stop() / track ends
                      ▼                         │
                ┌──────────┐      pause()    ┌──┴───────┐
         ┌─────▶│ PLAYING  │────────────────▶│  PAUSED  │
         │      └────┬─────┘◀────────────────└────┬─────┘
         │           │ seek()/next()/prev()        │
         │           │   (stay in PLAYING)         │ seek()/next()/prev()
         │           │                             │ (may resume PLAYING)
         │           │ track ends (auto_advance)   │
         │           └─────────────────────────────┘
         │
         │  (repeat_one = true → restart from 0)
         └──────────────────────────────────────────
```

## IPC Server

```
┌──────────────────────────────────────────────────────────────┐
│  Unix Socket: /run/user/$UID/gtmd.socket                     │
│  (or $XDG_RUNTIME_DIR/gtmd.socket, fallback /tmp/gtmd.sock) │
│                                                              │
│  ┌────────────┐    ┌────────────┐    ┌────────────┐         │
│  │ listener   │───▶│ accept     │───▶│ ClientConn │         │
│  │ (UnixList.)│    │ (tokio)    │    │            │         │
│  └────────────┘    └────────────┘    │  reader:   │         │
│                                       │  LineReader│         │
│  Request flow:                       │  writer:   │         │
│  ┌────────┐    ┌─────────┐           │  BufWriter │         │
│  │ client │───▶│ parse   │           │  event_buf:│         │
│  │ sends  │    │ JSON    │           │  Vec<u8>   │         │
│  │ line   │    │ request │           └────────────┘         │
│  └────────┘    └────┬────┘                                   │
│                     ▼                                        │
│              ┌──────────────┐                                │
│              │ dispatch()   │                                │
│              │  → play()    │                                │
│              │  → pause()   │                                │
│              │  → next()    │                                │
│              │  → ...       │                                │
│              └──────────────┘                                │
│                                                              │
│  Event broadcast:                                           │
│  ┌────────────┐    ┌──────────────┐    ┌──────────────┐    │
│  │ Daemon     │───▶│ serialize()  │───▶│ write to all │    │
│  │ produces   │    │ binary frame │    │ clients      │    │
│  │ event      │    │ (bincode)    │    │ (BufWriter)  │    │
│  └────────────┘    └──────────────┘    └──────────────┘    │
└──────────────────────────────────────────────────────────────┘
```

## Request Dispatch Table

```
 DaemonRequest          → Handler
 ─────────────────────────────────────────────────────
 Play{path}             │ load_and_play(path, 0.0)
 PlayPause              │ if playing → pause() else → play()
 Stop                   │ stop()
 Next                   │ advance_queue(1)
 Prev                   │ advance_queue(-1)
 Seek{pos}              │ backend.seek(pos)
 SetVolume{v}           │ backend.set_volume(v); state.volume = v
 ToggleShuffle          │ state.shuffle = !state.shuffle; reshuffle_queue()
 CycleRepeat{m}         │ state.repeat = m
 ToggleMute             │ if muted → restore vol else → vol=0
 Crossfade{enab, dur}   │ state.crossfade = Some/None
 Queue{action}          │ dispatch_queue_action(action)
 Library{action}        │ dispatch_library_action(action)
 Search{query}          │ library.search_tracks(query)
 GetFavourites          │ library.get_favourites()
 AddFavourite{id}       │ library.add_favourite(id)
 RemoveFavourite{id}    │ library.remove_favourite(id)
 YtSearch{query}        │ yt_manager.search(query)
 YtSearchPoll           │ yt_manager.poll_results()
 YtSearchCancel         │ yt_manager.cancel()
 YtResolveStream{url}   │ yt_manager.resolve_stream(url)
 Ping                   │ respond with Pong
 Quit                   │ shutdown()
```

## Queue Logic

```
Queue (Vec<TrackInfo> + cursor: usize):

  Normal mode:
    next() → cursor = (cursor + 1) % len  [if repeat_all]
    next() → cursor = (cursor + 1)        [if repeat_off, stop at end]
    next() → cursor stays, restart track   [if repeat_one]

  Shuffle mode:
    play_order: Vec<usize>  // indices into queue
    shuffle_cursor: usize    // position in play_order
    shuffle_next() → shuffle_cursor += 1; play(play_order[shuffle_cursor])
    Reshuffle: Fisher-Yates, preserve current track at position 0

  Queue set/add:
    Set{paths, start_idx} → build TrackInfo list, cursor = start_idx
    Add{path, pos=N} → insert at N (or end)
    Remove{idx} → shift cursor if needed
    Move{from, to} → reorder, update cursor
```

## File Structure

```
gtm-daemon/
├── Cargo.toml
└── src/
    ├── main.rs                # gtmd binary: CLI, daemon init, run
    ├── daemon.rs              # Daemon struct, main loop
    ├── ipc.rs                 # IpcServer, ClientHandle
    ├── dispatch.rs            # request dispatch logic
    ├── queue.rs               # QueueManager (shuffle, repeat, cursor)
    ├── library.rs             # Library wrapper (rusqlite)
    ├── youtube.rs             # YoutubeManager (yt-dlp subprocess)
    ├── cover_art.rs           # CoverCache (Deezer API)
    ├── lyrics.rs              # LyricsManager (LRC sidecar + LRCLIB)
    └── config.rs              # Config loading from XDG paths
```
