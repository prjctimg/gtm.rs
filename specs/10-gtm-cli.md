# 10 — gtm-cli: CLI Controller

## Purpose

Lightweight binary that reads DaemonRequest/Response JSON from stdin/stdout of
the daemon Unix socket. No TUI, no Ratatui. Used for scripting, keybindings,
and headless control.

Depends on: `gtm-core`, `clap`, `tokio`

## Subcommand Tree

```
gtm-cli
├── play [targets...]        Play file(s) or URL(s)
├── pause                    Pause playback
├── stop                     Stop playback
├── next                     Next track
├── prev                     Previous track
├── seek <seconds>           Seek to position
├── volume [level]           Get/set volume (0-100)
├── shuffle [on|off]         Toggle or set shuffle
├── repeat <off|one|all>     Set repeat mode
├── mute                     Toggle mute
├── status                   Show daemon status (JSON)
├── now                      Show current track info
├── queue                    Queue management
│   ├── list                 List queue
│   ├── clear                Clear queue
│   ├── add <path> [pos]     Add to queue
│   ├── remove <index>       Remove from queue
│   └── move <from> <to>     Move in queue
├── library                  Library management
│   ├── scan <path>          Scan directory
│   ├── list [filter]        List tracks
│   ├── search <query>       Search tracks
│   └── recent [count]       Recent tracks
├── favourite                Favourites
│   ├── list                 List favourites
│   ├── add <id>             Add favourite
│   └── remove <id>          Remove favourite
├── playlist                 Playlist management
│   ├── list                 List playlists
│   ├── create <name>        Create playlist
│   ├── delete <id>          Delete playlist
│   └── add <id> <track_ids..> Add to playlist
├── crossfade [secs]         Get/set crossfade
├── sleep <minutes>          Set sleep timer
├── kill                     Kill daemon
├── daemon                   Start daemon (fork)
├── lyrics [track_id]        Get lyrics for track
└── help                     Print help
```

## Usage Examples

```
# Play a file
gtm play ~/Music/song.flac

# Toggle pause
gtm pause

# Show current status
gtm status | jq '.state.volume'

# Queue management
gtm queue add ./another_song.flac
gtm queue list
gtm queue remove 2

# Library operations
gtm library scan ~/Music
gtm library search "jazz"
gtm library recent 10

# Volume
gtm volume 80
```

## Completion Output

```
# Shell completions generated via clap_complete
# Built-in subcommand:
gtm completions bash     # bash
gtm completions zsh      # zsh
gtm completions fish     # fish
gtm completions elvish   # elvish
gtm completions powershell  # powershell
```

## IPC Flow

```
┌──────────┐     connect     ┌──────────────────┐
│ gtm-cli  │ ───────────────▶│ Unix Socket       │
│          │                 │ /run/user/$UID/   │
│          │                 │ gtmd.socket       │
│          │                 │                   │
│          │  {"cmd":"play", │                   │
│          │   "path":"..."}─┼─────────────────▶│ gtmd (daemon)
│          │                 │                   │
│          │  {"ok":true,    │                   │
│          │   "status":...}─┼◀─────────────────│
│          │                 │                   │
└──────────┘                 └──────────────────┘

Implementation:
  DaemonClient::request() → await JSON response
  Same DaemonClient used by gtm-tui, but without event polling
```

## File Structure

```
gtm-cli/
├── Cargo.toml         # deps: clap={features=["derive"]}, gtm-core, tokio
└── src/
    ├── main.rs        # CLI dispatch, IPC connector
    └── completions.rs # Shell completion generation (optional, feature-gated)
```
