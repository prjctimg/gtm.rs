## Bugs

- The time shown in the footer is 2 hours behind. Use the system provided time to stay consistent.
- In Neovim the images appear blocky and in Zellij they don't render at all. Can we include this in the documentation`or at least design around this.
- The contrast between the footer text and the background isterrible, create a function to ensure that good contrast is maintained i.e using black text on bright backgrounds.
- Improve the center scrolling on the Library right pane and in overlays so that it is the list that moves up but the highlighter stays stationery.
- When you press previous/next in the Now Playing tab, the daemon may stop responding though playback and TUI responsiveness remains. Add daemon tolerance so that it restarts the client connection or the client tries to reconnect so that normal behaviour is restored.
- Remove the list numbers in the Library left pane (we don't need those) and remove the redundant icon right beside the 'Liked' list title. Also change the library tab icon because the current one has a weird 'vertical bar' artifact in dark mode.

## State synchronisation

### Track info floating window

When the window moves down the list, it briefly shows the cover art and then it disappears, almost as if it got replaced by another frame and was not skipped or preserved.

When you play a track, the currently playing track's cover is the only one that gets shown even if you scroll the list.

### Now Playing tab

This tab suffers from non deterministic behaviour where sometimes the TUI updates as expected and sometimes it keeps stale even  across TUI restarts.
The client MUST ALWAYS get track change or crossfade events so that the track details, elapsed time (setting it into the position of the track we are transitioning into AND setting the duration of the track we are transitioning into as the new duration) and the cover art are updated robustly and that there's no data races elsewhere.

## Improved YT search

- Add one playlist entry for every 3 track entries and ensure that the icons between individual tracks and playlist results are different. Ensure that the drill down list loads the playlist entries as expected and that it inherits the keybindings from the parent.

## Improved library 'motions'

- Add the following motions to the library right pane for common CRUD operations. For example 'add to queue','add to playlist' (shows an overlay of playlists available and a default option to create a new one), delete (permanently)/delete (from list),jump to end/start of list, edit track metadata (shows an overlay with tab navigatable fields for the user to modify the desired fields). Allow the user to engage multiselect mode with 'v' and have the Tab key toggle selection and move one position down the list after an item has been selected (better UX, removes manual need to press Down)
Take inspiration from vim.
