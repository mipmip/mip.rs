## 1. Rename zathura → quicktoc, side → sidetoc

- [x] 1.1 Rename all "zathura" references to "quicktoc" in view.rs
- [x] 1.2 Rename all "side" TOC references to "sidetoc" in view.rs
- [x] 1.3 Rename config `toc` values and update config.rs
- [x] 1.4 Update COMMANDS list in command.rs with new command names

## 2. Command composition

- [x] 2.1 Add `execute_commands(text, app)` that splits on `;` and calls `execute_command` for each
- [x] 2.2 Use `execute_commands` in command bar activate handler
- [x] 2.3 Tests for command splitting: single, multiple, whitespace, empty parts

## 3. Always-available TOC layout

- [x] 3.1 Refactor view.rs: always create Paned (sidetoc) + Stack (quicktoc), hidden by default
- [x] 3.2 Remove the `toc_mode` match that creates different layouts — single layout always
- [x] 3.3 Remove hardcoded Tab keybinding for quicktoc

## 4. Navigation commands

- [x] 4.1 Implement `sidetoc_open` command (show paned start_child)
- [x] 4.2 Implement `sidetoc_close` command (hide paned start_child)
- [x] 4.3 Implement `sidetoc_toggle` command
- [x] 4.4 Implement `sidetoc_expand_width` command (increase paned position by step)
- [x] 4.5 Implement `sidetoc_shrink_width` command (decrease paned position by step)
- [x] 4.6 Implement `quicktoc` command (toggle Stack visible child between document and toc)

## 5. Sidetoc config settings

- [x] 5.1 Add `sidetoc_width` (u32, default 250) and `sidetoc_position` (String, default "left") to Config struct
- [x] 5.2 Apply `sidetoc_width` as initial paned position
- [x] 5.3 Apply `sidetoc_position` to control which side of the Paned the TOC is on
- [x] 5.4 Remove old `toc` config field
- [x] 5.5 Tests for new config fields

## 6. --runcmd CLI option

- [x] 6.1 Replace `--toc` with `--runcmd` in Cli struct (Option<String>)
- [x] 6.2 Add `runcmd` to Config struct
- [x] 6.3 Execute runcmd (CLI overrides config) in connect_activate after widget setup
- [x] 6.4 Show helpful error if `--toc` is used (suggest --runcmd equivalent)

## 7. Verify

- [x] 7.1 `cargo build` succeeds
- [x] 7.2 `cargo test` passes
- [x] 7.3 `mip README.md` opens with no TOC (default)
- [x] 7.4 `mip --runcmd sidetoc_open README.md` opens with sidetoc
- [x] 7.5 `mip --runcmd quicktoc README.md` opens with quicktoc
- [x] 7.6 `:sidetoc_toggle` shows/hides sidetoc at runtime
- [x] 7.7 `:quicktoc` toggles quicktoc at runtime
- [x] 7.8 `:sidetoc_open; quicktoc` composes correctly
- [x] 7.9 Config `runcmd = "sidetoc_open"` works
- [x] 7.10 Config `sidetoc_width = 300` applies
