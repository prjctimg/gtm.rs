## Daemon reliability

Carefully study the current architecture and implementation of our daemon based audio backend and assess the perfomance, reliability and correctness of the program. Pay special attention to the following:

- How it initializes and what tasks are prioritized to make the TUI or client get fast load times and library data
- How state change from the daemon is handled, persisted and/or passed to the client. What current gaps exist that can create impossible UI states and partially initialized/bootstrapped components.
- How the TUI communicates requests, and processes or awaits responses from the daemon. Ensure that the client is always ready to reserve updates from the daemon and that the daemon is also always ready to respond to client commands.
- To improve stability, remove the need to sync state between multiple clients so that the daemon does not have to maintain complex state. Just ensure that when the multiple clients connect they are are responsible for fetching the session data and then maintaining state changes and subscribed to changes on the daemon state. The daemon itself must not disconnect and reconnect with clients since it doesn't need them.
- How it keeps alive in the background and syncs state changes to the client.
- How it recovers from internal errors like crashes without causing impossible states to the clients i.e making the TUI freeze during daemon interaction.

### Daemon health check

Look at what metrics are important for debugging the daemon and show them in status overlay similar to what `:checkhealth` does in neovim. It must check the health of all important daemon tasks or their related threads or past tasks such as logging the success of the past YT download requests, start times and resource usages of the components and their current operational state.

This can be extended to the TUI client to also check the healthiness of the TUIs components and the start times of different widgets/components.

In short we need good diagnostics support to provide better debugging feedback.
