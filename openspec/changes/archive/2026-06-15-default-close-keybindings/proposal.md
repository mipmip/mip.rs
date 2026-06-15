## Why

`mip` has no keyboard shortcut to close the window — users must use the `:q`/`:close` command bar or the window manager's controls. The common desktop shortcuts (Ctrl+Q, Ctrl+W, Alt+F4) do nothing. A community PR (#12, `cuducos/close-bindings`) added these as *hardcoded* handlers, but that approach is not present in the current branch and would not be remappable. Adding them as **default keybindings** mapped to the existing `close` command gives the same shortcuts while letting users override or disable them via config like every other binding.

## What Changes

- Register three default keybindings, all mapped to the existing `close` command:
  - `ctrl+q` → `close`
  - `ctrl+w` → `close`
  - `alt+f4` → `close`
- Because they go through the keybinding registry, users can override them in the `[keybindings]` config section (e.g. rebind `ctrl+w` to something else).
- Document the shortcuts in the config template's commented shortcut list and the changelog.

No new command is introduced — `close` (and its alias `q`) already calls `app.quit()`. No hardcoded key handling is added; this relies entirely on the existing registry and combo-string parser (which already accept `ctrl+q`, `ctrl+w`, `alt+f4`).

## Capabilities

### New Capabilities
<!-- None. -->

### Modified Capabilities
- `keybindings`: Adds Ctrl+Q, Ctrl+W, and Alt+F4 as default keybindings for the `close` command (overridable via config, consistent with existing defaults like Tab → quicktoc).

## Impact

- **Code**: add three `register_str(...)` lines for `close` to `KeybindingRegistry::with_defaults()` in `src/command.rs`. No changes to dispatch (`"q" | "close"` arm in `src/view.rs` already quits).
- **Docs**: add the shortcuts to the commented shortcut list in `default_config_template()` (`src/config.rs`), and a CHANGELOG entry under `## Unreleased`.
- **Tests**: a unit test asserting the three combos resolve to `close` from the default registry.
- **Relationship to PR #12**: this supersedes the hardcoded approach from PR #12 (which is not in the current branch) with a configurable equivalent.
