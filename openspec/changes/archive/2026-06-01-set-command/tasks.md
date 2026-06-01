## 1. RuntimeSettings struct

- [x] 1.1 Create `RuntimeSettings` struct with `Cell<bool>` for frontmatter, paragraph_numbers; `Cell<u8>` for paragraph_numbers_start; `RefCell<String>` for theme; `Cell<bool>` for force_render
- [x] 1.2 Add `RuntimeSettings` to `CommandContext`
- [x] 1.3 Initialize from config values in `connect_activate`

## 2. Refactor poll loop

- [x] 2.1 Replace captured `show_frontmatter` with `ctx.settings.frontmatter.get()` in poll loop render call
- [x] 2.2 Replace captured `paragraph_numbers` / `paragraph_numbers_start` with settings reads
- [x] 2.3 Add force_render check: if `settings.force_render.get()` is true, re-render even if seed unchanged, then reset flag
- [x] 2.4 Also refactor the initial TOC extraction to read from settings

## 3. Set command

- [x] 3.1 Add `set` to COMMANDS list in command.rs
- [x] 3.2 Add `SETTINGS` list in command.rs: `["frontmatter", "paragraph_numbers", "paragraph_numbers_start", "theme"]`
- [x] 3.3 Implement `set` command handler in execute_command: parse setting name + value, validate, update RuntimeSettings
- [x] 3.4 For theme changes: also inject CSS class via JS on the WebView
- [x] 3.5 Set `force_render = true` after any setting change that affects rendering

## 4. Tab completion for settings

- [x] 4.1 Add `match_settings(prefix) -> Vec<String>` in command.rs (same pattern as match_commands)
- [x] 4.2 Update `handle_tab_completion` to detect `:set ` prefix and complete against SETTINGS
- [x] 4.3 Tests for match_settings: prefix match, no match, empty prefix

## 5. Tests

- [x] 5.1 Tests for RuntimeSettings: default values, get/set roundtrip
- [x] 5.2 Tests for set command value validation: valid bools, valid theme, invalid values, integer clamping

## 6. Verify

- [x] 6.1 `cargo build` succeeds
- [x] 6.2 `cargo test` passes
- [x] 6.3 `:set frontmatter true` toggles frontmatter display immediately
- [x] 6.4 `:set theme dark` switches theme immediately
- [x] 6.5 `:set paragraph_numbers true` enables section numbers immediately
- [x] 6.6 Tab completion shows setting names after `:set `
- [x] 6.7 Invalid values print warning, don't crash
