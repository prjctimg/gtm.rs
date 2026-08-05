# Configuration

GTM stores user configuration in `~/.config/gtm/` (or `$XDG_CONFIG_HOME/gtm/`).

## Themes

### Built-in Themes

12 themes are bundled with GTM:

**Dark themes**: Chadrula, One Dark, Tokyo Night, Tokyo Night Storm, Catppuccin Mocha, Gruvbox Dark, Nord, Rose Pine, Everforest, Kanagawa

**Light themes**: Catppuccin Latte, Kanagawa Lotus

### User Themes (TOML)

Place `.toml` files in `~/.config/gtm/themes/`. Each file defines a theme with the same fields as `AppTheme`. Example:

```toml
name = "My Theme"
bg = "#1a1b26"
picker_bg = "#24283b"
fg = "#c0caf5"
fg_dim = "#565f89"
fg_bright = "#a9b1d6"
accent = "#7aa2f7"
error = "#f7768e"
warning = "#e0af68"
success = "#9ece6a"
selection_fg = "#1a1b26"
selection_bg = "#7aa2f7"
border = "#414868"
border_active = "#7aa2f7"
volume_low = "#9ece6a"
volume_medium = "#e0af68"
volume_high = "#f7768e"
sidebar_active_border = "#7aa2f7"
```

User themes **replace** built-in themes with the same name. The `light` flag is auto-detected from background luminance if omitted.

## Footer Presets

### Built-in Presets

| Preset | Left | Middle | Right |
|--------|------|--------|-------|
| **Default** | Playback, Queue, Repeat, Shuffle, Volume, EqPreset | KeyAction, SleepTimer | Clock |
| **Minimal** | Playback, EqPreset | KeyAction, SleepTimer | Clock |
| **Full** | Playback, Title, Volume, Repeat, Shuffle, EqPreset, Progress | KeyAction, SleepTimer | Clock |

### User Presets (TOML)

Place `~/.config/gtm/footer.toml` to define custom presets. Example:

```toml
[[presets]]
name = "My Preset"
left = ["Playback", "Queue", "Volume"]
middle = ["KeyAction", "SleepTimer"]
right = ["Clock"]
```

User presets **replace** built-in presets with the same name.

## Keybindings

All keybindings are hardcoded in the client. The `?` key opens the help buffer for a full reference. Key contexts:

- **Global**: active everywhere (quit, help, play/pause, next/prev)
- **Normal**: main view mode (tab switching, cursor, volume, filters, overlays)
- **List**: when a list widget has focus (navigation, selection, deletion)