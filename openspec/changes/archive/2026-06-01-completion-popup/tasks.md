## 1. Wildmenu label widget

- [x] 1.1 Add a `gtk4::Label` to the outer Box between the content widget and the command entry
- [x] 1.2 Style it with same grey background and monospace font as command bar, hidden by default
- [x] 1.3 Set label to use Pango markup for bold highlighting

## 2. Command name completion

- [x] 2.1 Define command name list: `["q", "close", "open", "o"]`
- [x] 2.2 On Tab when text after `:` has no space, match prefix against command names
- [x] 2.3 Single match: complete to command name + trailing space, no wildmenu
- [x] 2.4 Multiple matches: complete to first, show wildmenu, cycle on repeated Tab, Shift+Tab cycles backward

## 3. Wildmenu display logic

- [x] 3.1 Create `update_wildmenu(label, matches, current_index)` that sets Pango markup with current match bolded
- [x] 3.2 Show wildmenu label when matches > 1, hide when matches <= 1
- [x] 3.3 Truncate display to ~10 items if many matches, append "(+N more)"

## 4. Integrate with path completion

- [x] 4.1 Update existing path Tab handler to also show/update wildmenu label with path matches
- [x] 4.2 Path matches in wildmenu show only filenames (not full path) for readability

## 5. Hide wildmenu on events

- [x] 5.1 Hide wildmenu on Escape (dismiss command bar)
- [x] 5.2 Hide wildmenu on Enter (execute command)
- [x] 5.3 Hide wildmenu on any non-Tab keypress (typing resets)

## 6. Refactor and test pure logic

- [x] 6.1 Extract `expand_tilde` into a testable location (e.g. a `command` module or keep in view.rs with `pub(crate)`)
- [x] 6.2 Extract `parse_command(text) -> (cmd, arg)` pure function (strip `:`, split on whitespace)
- [x] 6.3 Extract `match_commands(prefix, commands) -> Vec<String>` pure function for command name matching
- [x] 6.4 Extract `match_paths(path_fragment) -> Vec<String>` pure function for path completion (reads filesystem but no GTK)
- [x] 6.5 Extract `wildmenu_markup(matches, current_index) -> String` pure function that returns Pango markup string
- [x] 6.6 Tests for `expand_tilde`: `~` expands, non-tilde unchanged, `~/foo` works
- [x] 6.7 Tests for `parse_command`: strips `:`, splits cmd/arg, handles no arg, handles extra whitespace
- [x] 6.8 Tests for `match_commands`: unique prefix, ambiguous prefix, no match, exact match
- [x] 6.9 Tests for `match_paths`: existing dir, partial filename, empty dir, nonexistent path
- [x] 6.10 Tests for `wildmenu_markup`: single item (bold), multiple items (one bold), index wrapping, truncation with "(+N more)"

## 7. Verify

- [x] 7.1 `cargo build` succeeds
- [x] 7.2 `cargo test` passes (all new unit tests)
- [x] 7.3 `:op<Tab>` completes to `:open `
- [x] 7.4 `:o<Tab>` shows wildmenu with `o` and `open`
- [x] 7.5 `:open src/<Tab>` shows wildmenu with directory contents
- [x] 7.6 Tab cycles forward, Shift+Tab cycles backward through matches, wildmenu highlight updates
- [x] 7.7 Escape/Enter/typing hides wildmenu
- [x] 7.8 Wildmenu styling matches command bar
