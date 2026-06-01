## 1. KeyCombo type and parsing

- [x] 1.1 Create `KeyCombo` struct in command.rs (keyval + modifiers)
- [x] 1.2 Implement `parse_key_combo(s: &str) -> Option<KeyCombo>` to parse strings like "ctrl+p", "tab", "shift+tab"
- [x] 1.3 Support modifier names: ctrl, shift, alt, super
- [x] 1.4 Support common key names: tab, escape, return, space, a-z, 0-9, f1-f12
- [x] 1.5 Tests for parse_key_combo: modifiers, plain keys, invalid names

## 2. Keybinding registry

- [x] 2.1 Create `KeybindingRegistry` in command.rs: HashMap<KeyCombo, String>
- [x] 2.2 Add `register_defaults()` with default bindings (tab→quicktoc, ctrl+p→print)
- [x] 2.3 Add `register(combo, command)` and `lookup(keyval, modifiers) -> Option<&str>`
- [x] 2.4 Tests for registry: lookup hit, lookup miss, override

## 3. Config integration

- [x] 3.1 Add `keybindings: Option<HashMap<String, String>>` to Config struct
- [x] 3.2 Parse `[keybindings]` TOML section
- [x] 3.3 Build registry: load defaults, then overlay config keybindings
- [x] 3.4 Warn on invalid key names in config
- [x] 3.5 Tests for config keybindings parsing

## 4. Wire into view.rs

- [x] 4.1 Create KeybindingRegistry in connect_activate, pass config keybindings
- [x] 4.2 Replace hardcoded Ctrl+P handler with registry lookup
- [x] 4.3 Add window-level key handler: if command bar hidden, look up in registry, execute if found
- [x] 4.4 Ensure `:` is always handled separately (not rebindable)
- [x] 4.5 Remove any remaining hardcoded key handlers (except `:` and command bar entry handlers)

## 5. Verify

- [x] 5.1 `cargo build` succeeds
- [x] 5.2 `cargo test` passes
- [x] 5.3 Default Tab → quicktoc works
- [x] 5.4 Default Ctrl+P → print works
- [x] 5.5 Custom config binding `ctrl+y = "open ~/todo.md"` works
- [x] 5.6 Config override `tab = "sidetoc_toggle"` replaces default
- [x] 5.7 `:` still opens command bar regardless of config
- [x] 5.8 Keybindings don't fire when command bar is open
