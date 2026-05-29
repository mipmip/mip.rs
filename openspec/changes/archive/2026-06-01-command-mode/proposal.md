## Why

mip has no way to interact with the preview beyond CLI flags at startup. A vim-style command mode (`:`) gives users runtime access to commands like opening a different file, quitting, and (in future) changing settings — all without leaving the keyboard.

Bean: mip.rs-2t32

## What Changes

- Add a command bar (GTK Entry widget) at the bottom of the window, hidden by default
- `:` keypress shows the command bar as a modal input — it captures all keyboard focus
- The command bar has no `:` prefix in the text — users type commands directly (e.g. `q`, `open file.md`)
- Escape is the only way to dismiss without executing; Enter executes
- Styled with grey background, no borders, no focus ring, monospace font
- Built-in commands:
  - `q` / `close` — quit the application
  - `open <path>` / `o <path>` — open a different markdown file, with Tab path completion
- Extensible command infrastructure for adding more commands later (`:set` planned as separate bean)

## Capabilities

### New Capabilities
- `command-mode`: Vim-style command bar with `:` activation, modal focus, command parsing, and file path tab completion

### Modified Capabilities

## Impact

- `src/view.rs`: add GTK Box layout wrapping existing content + Entry, key event handling with capture phase for `:`, Escape, Tab
- Works with all TOC modes (none, side, zathura)
