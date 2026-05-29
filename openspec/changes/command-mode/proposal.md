## Why

mip has no way to interact with the preview beyond CLI flags at startup. A vim-style command mode (`:`) gives users runtime access to commands like opening a different file, quitting, and (in future) changing settings — all without leaving the keyboard.

Bean: mip.rs-2t32

## What Changes

- Add a command bar (GTK Entry widget) at the bottom of the window, hidden by default
- `:` keypress shows the command bar and focuses it
- Escape dismisses, Enter executes the command
- Built-in commands:
  - `:q` / `:close` — quit the application
  - `:open <path>` / `:o <path>` — open a different markdown file, with Tab path completion
- Extensible command infrastructure for adding more commands later (`:set` planned as separate bean)

## Capabilities

### New Capabilities
- `command-mode`: Vim-style command bar with `:` activation, command parsing, and file path tab completion

### Modified Capabilities

## Impact

- `src/view.rs`: add GTK Box layout (WebView + Entry), key event handling for `:` and Escape, command parsing and execution
- The window child changes from bare WebView to a Box containing WebView + Entry
