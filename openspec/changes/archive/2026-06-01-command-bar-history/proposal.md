## Why

The command bar has no history — every command must be typed from scratch. Vim, shell, and every command-line interface provides ↑/↓ history navigation, optionally filtered by a typed prefix. This is especially useful for `:open` paths that are tedious to retype.

Bean: [mip.rs-2b2j](/home/pim/cLinden/mip.rs/.beans/mip.rs-2b2j--command-bar-history.md)

## What Changes

- Add persistent command history (stored at `~/.local/state/miprs/history`)
- ↑/↓ in the command bar cycles through history
- Prefix filtering: if you've typed `:op`, ↑/↓ only show history entries starting with `op`
- Deduplicate: repeated commands keep only the most recent occurrence
- History size configurable via `history_size` config option (default 50)
- Vim-style inline replacement: history entry replaces text in the command bar, editable before executing

## Capabilities

### New Capabilities
- `command-history`: Persistent command bar history with ↑/↓ navigation and prefix filtering

### Modified Capabilities

_(none)_

## Impact

- **New files**: `src/history.rs` (history loading/saving/filtering logic)
- **Modified files**: `src/view.rs` (↑/↓ key handling in command bar), `src/config.rs` (add `history_size` option + default config), `src/lib.rs` (add `pub mod history`)
- **New state file**: `~/.local/state/miprs/history` (one command per line)
