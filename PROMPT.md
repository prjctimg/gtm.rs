## NvChad themes

Change the themes to use NvChad color schemes. Repurpose the colors used for syntax highlighting for each module as the accent colors for the footer modules.

Be sure to take note of the background color of each theme and apply it as a background color as well

- Add build information about the host machine e.g (system, rust version used and versions for the symphonia,rodio and ratatui deps)
- Add information about the memory used by the daemon and CPU count as well.
- Show the storage usage of tracks and cover images.

## Cleaning the UI

- The Up next notification is redundant, let's rely on the footer module to tell us the track coming next
- Remove the redundant 'Title/Artist' and duration column headings (just show one column with no title) and the redundant inner 'Queue' heading inside the queue overlay

## Quirks

- Esc should NOT close the TUI, the only keys that can close the TUI are 'q' and 'Q' (which tells the daemon to save state and kill itself, the TUI doe not wait for the daemon, it fires and quits)
-On cold start, the playback starts automatically (which shouldn'thappen since the user must manually resume) but the TUI hangs and I have to manually kill both processes to use the TUI again
- The equalizer presets are not being applied as the user navigates the list, even when you pick one,no change is  audible.
-Ensure that the icons have the same size e.g the Settings icon on the settings tab is too small.
-Fix the 'gtm status' command to just show the playback  information instead of dumping raw JSON as a response. Add ANSI color coding to the output.
- The TUI should update EVERY related widget at once when the track advances automatically to the next (detected when crossfade starts). This includes, track details, elapsed time and the cover art. Always ensure that they are updated when the crossfade begins.
- When the user moves the highlight in a long scrolling list (overlay or not) apply the same approach we have in the Library right pane so that the highlighted item is always visible.
- As the user moves up and down the list in the Library, ensure that the album art is updated as well instead of only being updated after we click Enter on the current item.
- Improve the responsiveness of the youtube search by discarding all previous results when the user types in a whitespace character which means that our query is more specific than the results we previously fetched. Don't show stale results in the list.
