## 1. Register default keybindings

- [x] 1.1 In `KeybindingRegistry::with_defaults()` (`src/command.rs`), add `register_str("ctrl+q", "close")`, `register_str("ctrl+w", "close")`, and `register_str("alt+f4", "close")` alongside the other defaults

## 2. Documentation

- [x] 2.1 Add the close shortcuts (Ctrl+Q / Ctrl+W / Alt+F4 → close) to the commented shortcut/command list in `default_config_template()` (`src/config.rs`)
- [x] 2.2 Add a CHANGELOG entry under `## Unreleased` noting the new default close keybindings

## 3. Tests

- [x] 3.1 Add a unit test that builds `KeybindingRegistry::with_defaults()` and asserts `ctrl+q`, `ctrl+w`, and `alt+f4` each resolve to the `close` command

## 4. Verification

- [x] 4.1 `make check` passes (fmt, clippy, check-themes, tests)
- [x] 4.2 Verified via unit test (`test_registry_default_close_keybindings`: the three combos resolve to `close` through the real `lookup` API) and the unchanged `close`→`app.quit()` dispatch; override precedence is covered by existing `register_from_config` tests. Live window keypress not driven headlessly (no xdotool); covered by automated tests.
