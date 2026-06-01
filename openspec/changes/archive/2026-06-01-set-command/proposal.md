## Why

Settings like theme, frontmatter display, and paragraph numbers can only be changed by editing the config file and restarting mip. A `:set` command allows changing these at runtime from the command bar, making the workflow much faster for experimentation and per-document tweaking.

Bean: mip.rs-k7cm

## What Changes

- Add `RuntimeSettings` struct with `Cell`/`RefCell` fields for mutable settings
- Add `set` command: `:set <name> <value>` to change settings at runtime
- Refactor poll loop and render calls to read from shared RuntimeSettings instead of captured variables
- Add "force re-render" flag so `:set` changes trigger an immediate re-render without file changes
- Tab completion for setting names after `:set `
- Runtime-changeable settings: `theme`, `frontmatter`, `paragraph_numbers`, `paragraph_numbers_start`

## Capabilities

### New Capabilities

### Modified Capabilities
- `command-mode`: Add `set` command with setting name completion

## Impact

- `src/view.rs`: add RuntimeSettings to CommandContext, refactor poll loop to read from it, add force-render flag, implement `set` command
- `src/command.rs`: add `set` to COMMANDS list, add SETTINGS list for completion
- `src/main.rs`: initialize RuntimeSettings from config values
