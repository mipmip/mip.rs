## 1. Commands

- [x] 1.1 Add `zoom_in`, `zoom_out`, `zoom_reset` to `COMMANDS` list in `command.rs`
- [x] 1.2 Add `zoom_in`, `zoom_out`, `zoom_reset` handlers in `execute_command()` in `view.rs`
- [x] 1.3 Register default keybindings: `ctrl+=` → zoom_in, `ctrl+-` → zoom_out, `ctrl+0` → zoom_reset

## 2. Config

- [x] 2.1 Add zoom keybindings to the default config template in `config.rs`
- [x] 2.2 Add zoom commands to the command list comment in default config template

## 3. Verify

- [x] 3.1 `cargo build` succeeds
- [x] 3.2 Ctrl+=, Ctrl+-, Ctrl+0 work in preview
- [x] 3.3 `:zoom_in`, `:zoom_out`, `:zoom_reset` work from command bar
