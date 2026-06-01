## Context

The command bar in `view.rs` uses a GTK `Entry` widget with an `EventControllerKey` that handles Escape, Backspace, Tab, and character keys. ↑/↓ are not currently captured. Commands are executed on Enter via `connect_activate`. The tab-completion system uses `Rc<RefCell<Vec<String>>>` for match state — history will follow the same pattern.

## Goals / Non-Goals

**Goals:**
- ↑/↓ cycles history, filtered by current prefix
- Persistent across sessions
- Deduplicated (most recent wins)
- Configurable size (default 50)
- Vim-style: history replaces entry text inline

**Non-Goals:**
- Search/fuzzy find in history (`:` + `/` pattern)
- Separate history per command type
- History for keybinding-invoked commands (only command bar)

## Decisions

### 1. History file format and location

**Choice**: Plain text, one command per line (without the leading `:`), at `~/.local/state/miprs/history`. Most recent entry last.

**Why**: XDG-compliant (`XDG_STATE_HOME`). Simple format, easy to inspect or clear manually. Same approach as vim's `~/.viminfo` and fish's `~/.local/share/fish/fish_history`.

### 2. History struct in src/history.rs

**Choice**: A `CommandHistory` struct with:
- `entries: Vec<String>` — all entries, most recent last
- `max_size: usize` — from config
- `load(path) -> Self`, `save(&self, path)`, `push(&mut self, cmd)` (dedup + trim), `filter(&self, prefix) -> Vec<&str>`

**Why**: Keeps history logic testable and separate from GTK code. Pure functions, no GTK dependencies.

### 3. Browse state in view.rs

**Choice**: Three `Rc` cells alongside existing tab-completion state:
- `history_index: Rc<Cell<Option<usize>>>` — current position in filtered list, None = not browsing
- `history_matches: Rc<RefCell<Vec<String>>>` — filtered entries for current prefix
- `saved_input: Rc<RefCell<String>>` — text user typed before pressing ↑

**Why**: Matches the existing pattern used for tab-completion state.

### 4. Interaction flow

**Choice**:
1. User opens command bar (`:`)
2. Optionally types a prefix (e.g., `op`)
3. Presses ↑: saves current text to `saved_input`, builds filtered history, shows most recent match
4. ↑/↓ cycles through filtered matches
5. ↓ past the newest entry restores `saved_input`
6. Enter executes and pushes to history
7. Escape dismisses without pushing
8. Any character key resets browse state (history_index = None)

**Why**: Standard vim/shell behavior.

### 5. Deduplication

**Choice**: On `push()`, remove any existing occurrence of the same command before appending. This keeps history ordered by recency without duplicates.

**Why**: Matches shell behavior. Running `:open foo.md` 5 times keeps one entry.

## Risks / Trade-offs

- **[Risk] History file corruption on crash** → Mitigation: write on clean shutdown (app.connect_shutdown). If file is corrupted, start fresh — it's just convenience data.
- **[Risk] ↑/↓ conflict with GTK Entry cursor movement** → The entry is single-line, so ↑/↓ have no default GTK behavior. Safe to capture.
