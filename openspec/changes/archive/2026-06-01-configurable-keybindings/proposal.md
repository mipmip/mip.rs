## Why

Keybindings are currently hardcoded (`:` for command bar, Ctrl+P for print). Users can't customize shortcuts or bind keys to commands. Since the command system already supports arbitrary commands and `;` composition, keybindings just need to map key combos to command strings.

Bean: mip.rs-6iiu (keybinding part)

## What Changes

- Add `[keybindings]` section to config.toml
- Keybinding registry: maps key combo strings to command strings
- Default keybindings (hardcoded, overridable by config)
- Key handler looks up keyval+modifiers in the registry and executes the mapped command
- Key combo format: `ctrl+p`, `tab`, `ctrl+shift+t`, single keys like `j`, `k`
- Values are command strings: `print`, `quicktoc`, `open ~/todo.md`, `sidetoc_open; set theme dark`

## Capabilities

### New Capabilities
- `keybindings`: Configurable keyboard shortcuts mapping key combos to command strings

### Modified Capabilities
- `config`: Add `[keybindings]` section

## Impact

- `src/command.rs`: add keybinding registry (parse key combo strings, lookup by keyval+modifier)
- `src/config.rs`: add `keybindings` HashMap field, parse `[keybindings]` TOML section
- `src/view.rs`: replace hardcoded key handlers with registry lookup, keep `:` handler special (always active)
