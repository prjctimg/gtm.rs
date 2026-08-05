# CLI Reference

Use `gtm -c` or `gtm --cli` to run in CLI mode. Commands are sent to the daemon via IPC and the result is printed to stdout. Use `--json` for machine-readable output.

## Playback

| Command | Description |
|---------|-------------|
| `play *path* [*start_pos*] | Play a track by filesystem path or URL. Optionally start at a given position in seconds. |
| `play-pause` | Toggle between play and pause |
| `pause` | Pause playback |
| `stop` | Stop playback entirely |
| `next` | Skip to the next track |
| `prev` | Return to the previous track |
| `seek *position_secs* | Seek to a position in the current track (seconds) |
| `volume *volume* | Set volume (0-100) |
| `mute` | Toggle mute |
| `shuffle` | Toggle shuffle mode |
| `repeat {Off\|One\|All}` | Set repeat mode |
| `crossfade *enabled* [*duration_secs*] | Enable/disable crossfade |

## Queue

| Command | Description |
|---------|-------------|
| `queue` | Display the current queue |
| `queue-add *path* [*position*] | Add a track to the queue |
| `queue-add-many *paths*...` | Add multiple tracks at once |
| `queue-add-folder *path*` | Add all tracks in a folder |
| `queue-remove *index*` | Remove a track by index |
| `queue-move *from* *to*` | Move a track between positions |
| `queue-clear` | Clear the entire queue |
| `queue-set *paths*... *start_idx*` | Replace the entire queue |

## Library

| Command | Description |
|---------|-------------|
| `scan *path*` | Scan a directory for music files |
| `tracks [*filter*] [*sort*]` | List tracks in the library |
| `playlists` | List saved playlists |
| `create-playlist *name*` | Create a new playlist |
| `delete-playlist *id*` | Delete a playlist by ID |
| `add-to-playlist *playlist_id* *track_ids*...` | Add tracks to a playlist |
| `import-m3u *path*` | Import an M3U playlist file |
| `export-m3u *playlist_id* *path*` | Export a playlist to an M3U file |
| `recent *count*` | Show recently added tracks |
| `search *query*` | Search the library |
| `lyrics *query*` | Fetch lyrics via lrclib |

## Favourites

| Command | Description |
|---------|-------------|
| `favourites` | List favourite tracks |
| `favourite-add *track_id*` | Add a track to favourites |
| `favourite-remove *track_id*` | Remove a track from favourites |

## YouTube

| Command | Description |
|---------|-------------|
| `yt-search *query* [*filter*]` | Search YouTube |
| `yt-poll` | Poll for pending YouTube results |
| `yt-cancel` | Cancel a YouTube search |
| `yt-resolve *url*` | Resolve a YouTube URL to a playable stream |

## System

| Command | Description |
|---------|-------------|
| `status` | Show daemon status |
| `check-health` | Check daemon connectivity and return version info |
| `ping` | Ping the daemon |
| `quit` | Shut down the daemon |

## Options

| Option | Description |
|--------|-------------|
| `--socket *path*` / `-s` | Daemon Unix socket path |
| `--cli` / `-c` | Run in CLI mode |
| `--json` / `-j` | Output as JSON (CLI mode only) |
| `--version` / `-V` | Show version |
| `--help` / `-h` | Show help |