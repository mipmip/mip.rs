## toc-navigation

Display a navigable Table of Contents extracted from document headings.

### Requirements

#### Heading extraction
- MUST extract all headings (h1–h6) from the markdown document during parsing
- MUST preserve heading hierarchy (h2 under h1, h3 under h2, etc.)
- MUST handle heading-level jumps gracefully (e.g. h1→h3 parents h3 under h1, no phantom nodes)
- MUST generate deterministic anchor IDs from heading text (slug format: lowercase, dashes)
- MUST handle duplicate heading text by appending `-1`, `-2`, etc.
- MUST inject anchor `id` attributes on heading elements in the HTML output

#### Display modes
- MUST support three modes via config/CLI: `side`, `zathura`, `off`
- `off` — no TOC displayed (default, preserves current behavior)
- `side` — persistent side panel to the left of the document
- `zathura` — TOC replaces document view on toggle, selecting a heading returns to the document

#### Side panel mode
- MUST display TOC as a collapsible tree in a GTK TreeView
- MUST allow resizing the panel (draggable divider)
- SHOULD use a sensible default width (~250px)

#### Zathura mode
- MUST toggle TOC view with `<Tab>` key
- MUST return to document view and scroll to heading on entry selection (Enter or Tab)
- MUST return to document view without navigation on `<Esc>`
- MUST focus the TOC widget when it becomes visible

#### Navigation
- MUST scroll the document to the selected heading's anchor
- MUST support keyboard navigation: `j`/`k` (vim) and arrow keys
- MUST support `Enter` to activate the selected heading
- MUST support expand/collapse of tree nodes (arrow keys, native TreeView behavior)

#### Reload
- MUST update the TOC when the document file changes
- MUST preserve tree expand/collapse state across reloads where headings haven't changed (SHOULD, not MUST)

#### Configuration
- MUST support `toc` key in `~/.config/miprs/config.toml` with values `"side"`, `"zathura"`, `"off"`
- MUST support `--toc` CLI flag that overrides the config file value
- MUST default to `"off"` when neither config nor CLI specifies a value
