# TUI Guide

## Tabs

GTM has three tabs navigated with **Tab** and **Shift+Tab**:

### Now Playing (tab 1)

Displays the current track's cover art, title, artist, album, progress bar, volume gauge, sleep timer, and control hints.

| Key | Action |
|-----|--------|
| `Space` | Play / Pause |
| `n` / `Ctrl+N` | Next track |
| `p` / `Ctrl+P` | Previous track |
| `+` / `=` | Volume up |
| `-` | Volume down |
| `m` | Toggle mute |
| `r` | Cycle repeat mode (Off/One/All) |
| `s` | Toggle shuffle |
| `h` / `l` | Seek backward / forward |

### Library (tab 2)

Browse tracks by category: All Tracks, Playlists, Favourites, Recent. The left pane selects a category; the right pane lists tracks.

| Key | Action |
|-----|--------|
| `Tab` | Toggle left/right pane focus |
| `j` / `k` or `↑` / `↓` | Navigate up/down |
| `Enter` | Play selected / drill-down |
| `/` | Enter filter mode |
| `d` / `Del` | Remove item |
| `F` | Toggle favourite |
| `D` | Clear queue |

### Settings (tab 3)

Adjust playback settings and open overlays. Left pane selects a category; right pane shows options.

| Key | Action |
|-----|--------|
| `Tab` | Toggle left/right pane focus |
| `j` / `k` | Navigate up/down |
| `Enter` | Toggle / select option |

## Global Keys

| Key | Action |
|-----|--------|
| `?` | Toggle help buffer |
| `:` | Command palette |
| `q` | Quit |
| `Q` | Quit & stop daemon |
| `Ctrl+H` | Toggle help bar |

## Overlays (Alt+key)

| Key | Overlay |
|-----|---------|
| `Alt+Q` | Queue picker |
| `Alt+Y` | YouTube search |
| `Alt+F` | Search library |
| `Alt+A` | About dialog |
| `Alt+C` | Theme picker |
| `Alt+E` | Equalizer |
| `Alt+P` | Command palette |
| `Alt+Z` | Sleep timer |
| `Alt+X` | Sound effects |
| `Alt+S` | Spotify search |
| `Alt+F` | Cycle footer preset |

## Help Buffer

Press `?` to open the dedicated help buffer. It supports:

- **Vim motions**: `j`/`k` navigate, `gg`/`G` jump to top/bottom, `0`/`$` go to line start/end
- **Search**: `/` to search, `n`/`N` for next/previous match
- **Topic navigation**: Jump to Keybindings, Configuration, or Setup sections
- **Read-only**: Cannot modify any content
- **Close**: `Esc` or `q`

## Picker Overlays

When a picker is open (Queue, YouTube Search, Library Search, etc.):

- `j`/`k` or `↑`/`↓` to navigate
- Type to filter/search (where supported)
- `Enter` to select
- `Esc` to close

## Help Bar

The bottom bar on the Library tab shows a condensed keybinding reference. Toggle it with `Ctrl+H`.