## Why

The TOC navigation is currently controlled by `--toc side|zathura` CLI flags, locking the user into one mode at startup. The naming ("zathura") is an implementation reference, not a user-facing concept. Both TOC modes should always be available as runtime commands, and the hardcoded Tab keybinding should be removed in favor of configurable keybindings (separate change).

Bean: mip.rs-6iiu

## What Changes

- **Rename**: "zathura" → "quicktoc", "side" → "sidetoc" throughout the codebase, config, and specs
- **Remove `--toc` CLI flag**: replace with `--runcmd` for running commands at startup
- **Add `--runcmd` CLI option**: takes a command string (same format as command bar), supports `;` for composing multiple commands
- **New commands**: `sidetoc_open`, `sidetoc_close`, `sidetoc_toggle`, `quicktoc`, `sidetoc_expand_width`, `sidetoc_shrink_width`
- **Command composition**: `;` separates multiple commands (works in command bar, --runcmd, and future keybindings)
- **Sidetoc config settings**: `sidetoc_width`, `sidetoc_position` (left/right) in config.toml
- **Remove hardcoded Tab keybinding**: Tab no longer toggles quicktoc (will be handled by configurable keybindings change)
- Commands use no colon prefix internally — `:` is the command bar UI activation key only

## Capabilities

### New Capabilities
- `runcmd`: `--runcmd` CLI option for executing commands at startup, with `;` composition

### Modified Capabilities
- `command-mode`: Add navigation commands, `;` command composition
- `config`: Add `sidetoc_width` and `sidetoc_position` settings

## Impact

- `src/view.rs`: rename all "zathura"/"side" references, remove hardcoded Tab handler, sidetoc/quicktoc always built (hidden by default), new commands in execute_command
- `src/config.rs`: add `sidetoc_width`, `sidetoc_position` fields
- `src/main.rs`: replace `--toc` with `--runcmd`, execute runcmd commands after window creation
- `src/command.rs`: add `;` splitting, new command names in COMMANDS list
- `openspec/specs/`: rename specs to match new naming
