## 1. History module

- [x] 1.1 Create `src/history.rs` with `CommandHistory` struct: `entries: Vec<String>`, `max_size: usize`
- [x] 1.2 Implement `load(path) -> Self` — reads history file, one command per line
- [x] 1.3 Implement `save(&self, path)` — writes history file
- [x] 1.4 Implement `push(&mut self, cmd)` — dedup (remove existing), append, trim to max_size
- [x] 1.5 Implement `filter(&self, prefix) -> Vec<&str>` — return matching entries, most recent last
- [x] 1.6 Add `pub mod history` to `src/lib.rs`

## 2. Config

- [x] 2.1 Add `history_size: Option<u32>` to `Config` struct with default 50
- [x] 2.2 Add `history_size` to default config template with comment

## 3. Integration in view.rs

- [x] 3.1 Load `CommandHistory` on startup, wrap in `Rc<RefCell<...>>`
- [x] 3.2 Add history browse state: `history_index`, `history_matches`, `saved_input` as `Rc` cells
- [x] 3.3 Handle ↑ in command bar key handler: save input, build filtered matches, show previous entry
- [x] 3.4 Handle ↓ in command bar key handler: show next entry or restore saved input
- [x] 3.5 Reset browse state on character input (in the `_` catch-all arm)
- [x] 3.6 On Enter (connect_activate): push command to history
- [x] 3.7 On app shutdown: save history to disk

## 4. Tests

- [x] 4.1 Unit tests for `CommandHistory`: push, dedup, filter, max_size trimming, load/save round-trip

## 5. Verify

- [x] 5.1 `cargo build` succeeds
- [x] 5.2 ↑/↓ work in command bar with prefix filtering
- [x] 5.3 History persists across mip restarts
