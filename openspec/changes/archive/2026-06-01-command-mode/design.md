## Context

The window currently has a WebView as its only child. Keyboard events are handled by an EventControllerKey (added by the print-dialog change for Ctrl+P). The command bar needs to coexist with this.

## Goals / Non-Goals

**Goals:**
- Vim-style `:` command activation
- Command bar appears/disappears at bottom of window
- `:q`, `:close` to quit
- `:open`/`:o` with Tab file path completion
- Clean infrastructure for adding commands later

**Non-Goals:**
- `:set` command (separate bean mip.rs-k7cm)
- Command history (up/down arrow)
- Fuzzy matching or search

## Decisions

### GTK Box layout with hidden Entry

**Choice**: Replace the bare WebView child with a vertical `gtk4::Box` containing the WebView (expand=true) and a `gtk4::Entry` (visible=false). On `:`, show the entry and focus it. On Escape/Enter, hide and return focus to the WebView.

**Rationale**: Simple GTK4 pattern. The entry takes zero space when hidden. No overlay complexity.

### Command parsing

**Choice**: Split the entry text on whitespace. First token is the command name, rest is the argument. A `match` on the command name dispatches to handlers.

```rust
match command {
    "q" | "close" => app.quit(),
    "open" | "o" => open_file(arg, ...),
    _ => show_error("Unknown command"),
}
```

**Rationale**: Minimal parsing. Easy to extend with new arms.

### Tab completion for `:open`

**Choice**: On Tab keypress while the entry is visible and text starts with `open ` or `o `, complete the file path by listing directory entries that match the current prefix. Cycle through matches on repeated Tab.

**Implementation**:
1. Extract the path fragment after the command
2. Split into directory part and prefix part
3. Read directory entries matching the prefix
4. Replace the path fragment with the first match
5. Subsequent Tabs cycle through matches

**Rationale**: Standard shell-style tab completion. No external dependencies — `std::fs::read_dir` is sufficient.

### Focus management

- `:` on window → show entry, set text to ":", place cursor at end, grab focus
- Escape in entry → hide entry, clear text, return focus to WebView
- Enter in entry → execute command, hide entry, clear text, return focus to WebView
- The entry's key handler should consume `:` initial, Escape, Tab (for completion)

## Risks / Trade-offs

- [WebView losing focus] → GTK4 focus management can be tricky. The entry must explicitly grab focus on show, and the WebView should regain it on hide. Test carefully.
- [Tab completion edge cases] → Paths with spaces, symlinks, permission errors. Mitigation: silently ignore errors, don't complete if read_dir fails.
- [Command bar styling] → The entry should look like a vim command line — monospace, minimal. Can be styled with CSS class.
