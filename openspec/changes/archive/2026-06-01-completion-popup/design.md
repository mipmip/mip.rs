## Context

The command bar has Tab completion for file paths that cycles through matches silently. Commands are dispatched by name (`q`, `close`, `open`, `o`) but there's no command name completion. The outer Box layout is: content widget → command entry. A wildmenu label fits naturally between them.

## Goals / Non-Goals

**Goals:**
- Command name completion on Tab when no space in text yet
- Wildmenu label showing all matches with current highlighted
- Same wildmenu for both command names and path completions
- Vim-like UX: Tab cycles forward, Shift+Tab cycles backward, wildmenu updates highlight

**Non-Goals:**
- Fuzzy matching
- Dropdown/popover (too complex, not vim-like)

## Decisions

### Wildmenu as a Label with Pango markup

**Choice**: A `gtk4::Label` inserted in the outer Box between the content and the entry. Hidden by default. When Tab produces matches, set its markup to show all matches with the current one wrapped in `<b>` tags. Style with same grey background as command bar.

**Rationale**: Simplest possible implementation. A Label is cheap, Pango markup handles highlighting, no complex widget tree. Matches vim's wildmenu which is also just a text line.

**Example markup**: `:open R<Tab>` with matches `[README.md, rust-toolchain.toml]`:
```
<b>README.md</b>   rust-toolchain.toml
```

### Command name completion logic

**Choice**: When Tab is pressed and the text after `:` contains no space, match the typed prefix against the known command list. If one unique match, complete and append a space. If multiple, show wildmenu and cycle.

**Known commands**: `q`, `close`, `open`, `o`

**Edge case**: `:o<Tab>` — both `o` and `open` match. Since `o` is already a valid complete command, complete to `open` (longer match first). The user can just press Enter if they meant `o`.

### Shared wildmenu for commands and paths

**Choice**: The same Label and display logic handles both command name matches and path matches. The Tab handler decides which completion source to use (command names vs directory entries), gathers matches, then calls a shared `show_wildmenu(label, matches, index)` function.

## Risks / Trade-offs

- [Label height] → A long list of matches may wrap. Mitigation: truncate display to ~10 items, show count if more.
- [Styling consistency] → The wildmenu label needs to match the command bar's grey background and monospace font. Same CSS class.
