## Features

### Fault resistant track metadata extraction

- When there's an active internet connection and a track without a cover art is played, fetch the image (png) from the Deezer API and update the default cover art in the NowPlaying tab. Follow the existing convention to the path where cover arts are cached.
- If a song has an embedded cover image, then extract and use that instead. The user can still request a refresh of the cover it if it is, for example a promotional banner of some dj or music channel.
- Always download the cover art of a song immediately after the audio download succeeds. This means that deleting a song in the TUI will also delete the associated cover art of that song. This helps clean the storage.
- During audio download, find a robust way to get the correct Artist, Track title and album (if it's not a single), rather than just loading the filename as the title.
- Use ratatui-image for handling image rendering in the TUI to keep our code clean.

### Documentation as tests

The examples included in the manpages should be ran as part of the test suite, this means we need a generated .wav file(s) that we can use to test playback features, like crossfade/audio transitions.

Also generate completions for the 5 popular shells during the release.

### APT packaging

Refer to relevant documentation on how to create a .deb file so that users can install via APT. The package must have the docs (if its gtm-full then it has both manpages for the daemon and tui).
Look at how one creates an apt repository and integrates updates via CI on every release as well.

During installation, it must do the following:

- Register the daemon service to the system.
- If cron exists, prompt the user if they'd like to add an automated periodical indexing of media directories to update the index if new files were added.
- Setup completions for all the valid shells in /etc/shells except the non-interactive ones.

### Smarter completions

When the user types in a command that expects predefined options, paths etc as arguments, show the suggestions when the user presses Tab.
Do this for all shells and use carapace for this.

### ANSI colored output

Instead of just printing an "okay" response from the daemon, show nicely colored and formatted output. Also

## UI

### Themes

Include the following themes (include all the variants):

- Catpuccin
- Tokyonight
- Gruvbox
- Ayu
- A reimplementation of mini.colorschemes including the random and default colors it provides.

The TUI must not use hardcoded tokens so that the UI stays consistently themeable.

### Overlays

These are windows that can float over other views (higher z-index) like the YTSearch, SpotifySearch, ThemePicker etc. These overlays should be accessible from any tab in the TUI and they are accessible using a modifier key (the default is Alt).

OVERLAYS ARE NOT TABS. The current implementation made tabs out of views that should be overlays.

Here's an exhaustive list of the overlays:

- ThemePicker => Live preview theme picker that applies the theme as the user searches or navigates options.
- YTSearch => Allows the user to search and download tracks and even entire Youtube playlists. The user can also play,add to queue and download with Ctrl modified keybindings within the overlay
- SearchLibrary => Allows the user to fuzzy find the local files indexed in the library
- SpotifySearch => Allow the user to interact with spotify via spot-dl.
- Equalizer => Overlay that shows the list of equalizer presets and an ascii based graph that is updated as the user navigates the options to show the line graph representation of the wave form. It must apply the equalizer effects as the user navigates the list.
- CommandPalette => Lists all the possible actions in the player.
- About => Show information,versions,stats and health of the player and daemon e.g current memory/cpu/storage usage etc.
- Queue => Allows the player to see the currently playing tracks, add more, change playback order and remove/clear it.
- SleepTimer => Allows the user to set the sleep timer
- SoundEffects => Allows the user to adjust audio settings like reverb preset, playback speed, crossfade settings and more.

You can make the overlay a generic container that contain an optional fuzzy finder and take an object of keymaps to increase code reuse.

Overlays have semi transparent backgrounds by default and can have their blend adjusted too.

### Tabs

The TUI should only have the following tabs:

- NowPlaying
- Library
- Settings

#### NowPlaying

- Don't always show the volume slider on the tab. It is only shown briefly when the user adjusts the volume as a toast notification from the right.
- On the left side before the track details are shown, show a square cover image of the currently playing track.

#### Library

- The library should have the lists shown on the left pane and the contents of that list displayed on the right pane.
- The list types are: All tracks, Albums, Artists, Playlists, Recently Added, Most Played, Least Played, Spotify,Downloads. Show the number of elements  in each list at the right end of the list type in the left pane.
- Add support for fetching recommended playlists, tracks based on my listening pattern e.g most played artist or genre (mix the results)
- Include other library stats based on the amount of tracks we have, e.g top genre,most played genre, listening time, longest listening session, the number of tracks in the library and put it in human terms (e.g "You have 500 songs' that's a full day and 3 hours worth of unrepeated playback")

### Settings

- The setting categories are shown on the left pane and the options in that category are shown in the right pane, the user toggles between panes with Tab.
- As the user navigates either the left or right pane, help information about the highlighted item is shown. For example, if the user is on the 'Crossfade easing' option it will show what this means, and the current choice and how it sounds like. This allows the user to understand what's going on as they navigate the options.
- The categories of settings are: Audio, YouTube, Appearance, System, Spotify.

### Responsive design

- The Library and settings tab share the same layout, on very small width viewports only show one pane at a time and the Tab key should cycle between the tabs.

### Aesthetics

- Use rounded borders and give overlays adjustable opacity, the default is 90%.
- Use braille characters for loading state spinners
- Use nerd icons and fallback to emojis for the TUI.
- Use your own background and have a 'transparent' option for users that may want to inherit the terminal background. It should have an option to adjust opacity and blend as well.
- Use a line progress bar for the track progress that has an oscillating head like the new design being used in material design.

### Notifications and command feedback

- Show an up next notification in the TUI when the next track has started fading in. It should show from the top right and elastic bounce out of the view or similar (whichever easing has the better UX).
- Show a toast on the top right side of the TUI when the user adjusts volume and let the indicator have different colors for different levels (quiet,medium,loud).

### Customizable footer

- The footer should have customizable presets, with each preset being customizable. It should contain playback state,date/time, current listening time, sleep timer count down, active settings that affect playback like, shuffle/repeat, relevant system information like audio playback device or backend. Ask me for suggestions after you search for inspirations.
- Modules can be individually themed do that for instance related mdules (e.g playback state) can share the same background and be put side by side for visual consistency.
- It must show the pressed keybinding's action in the footer (similar to whichkey), keeping the module active as long as the overlay is active.

### Customizable progress bar

- Add options for different track progress indicators like braille, waveform and more ASCII and other symbols. For the waveform, it must be unique for each track is it is actually computed from the active track or the track we're transitioning to.

## Daemon client communication redesign

The approach of sending both binary encoded frames and JSON respones on the same line feels unreliable.

### Pulse thread

Can we have a separate thread for updating the TUI when events affect the UI are broadcast from the daemon. This would allow us to keep a seperate thread ready to handle user commands so that the latency is kept low.  It should be binary encoded so that only the necessary info is sent.

### snake_case IPC commands

The IPC commands should be in snake_case which makes them easier to pass around and it feels more  conventional. Let me if they're any added costs.

## Coding conventions

- Favor pattern matching for defining commands or functions that have the same parent e.g LibraryActions or QueueActions. This means that the user can just pass any valid queue path (single file or a folder) and they'll be handled accorgingly. Use space delimiting when the user is specifying multiple paths in the CLI.
- ALWAYS comment complex flows and document every public symbol.
- Prefer methods that don't panic to avoid crashing the entire program, e.g `unwrap_or` instead of `unwrap`.

## Audio improvements

### Crossfade support and audio transition effects

- Any transition between tracks should trigger a cross dissolve (or crossfade) either manual or when a track ends normaly.
- Add different easing options for the crossfades and give them user friendly names  like 'Slow fade in, fast fade out' etc. This will allow us to, for instance, increase the tempo of the channel with the track that  is exiting etc.
- Add an option to add a reverb into a song as it fades in/out to the main channel so that it feels like it is being mixed in realtime.
- Change the default crossfade duration to 7s.
- When the user presses pause the track should not stop abruptly but dip the volume then pause. When the user presses resume, ease the volume back to the original state so that it feels like the volume was being increased.
- Add a prompt in the TUI that warns  the user when they're increasing the volume to unsafe levels. This means that only the TUI can increase the volume beyond safe levels, the daemon will have to respond with a challenge that needs the client session id before it approves. The client session id is shared discreetly between client and daemon on initial connection and is valid for as long as the daemon is alive.

### Smart volume

Songs may not be equally loud even if we're playing them at the same volume levels, to fix this we want the user to hear the same level of loudness they selected when they last adjust the volume so  that the next track follow suite.

### Sound effects

Allow the user to adjust playback speed, use reverb presets and adjust other sound properties that are easy for beginners to play with.

Do all the needed research for the requests in this section and follow best practices in acoustic and audio mixing.

## Error handling

- When the daemon gets to the end of a file or queue it is showing a buffer over/under flow error. Guard against such edge cases and assert that this never happens.

## Feature compliance with legacy

### Client

Refer to the ./docs-legacy/gtm.1.md file and ensure that the client matches the features, flags and commands shown in the document. Also respect the user name and license declaration in the document.

### Daemon

Refer to the ./docs-legacy/gtmd.1.md file and ensure that the daemon follows the conventions, commands and features shown here. It  should also respect the license declaration and project author (prjctimg).

### Daemon IPC

Refer to the ./docs-legacy/gtmd-ipc.md file and ensure that the daemon ipc follows the conventions, commands and features shown here.

Generate new and updated spec files to reflect these changes and list them in implementation phases.

### Copyright notice

Use the one below as the template and update version fields to be dynamic and based on the latest tag+7charGitHash or with the -dirty flag prepended if we're running a build with unsaved changes.

```text


gtm 0.7.34
Copyright (C) 2026, prjctimg <prjctimg@outlook.com>
Website: https://prjctimg.me
License GPL-3.0
This is free software: you are free to change and redistribute it.
There is NO WARRANTY, to the extent permitted by law.



```
