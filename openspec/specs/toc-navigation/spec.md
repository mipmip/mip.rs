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
- MUST support two TOC modes, both always available and hidden by default:
  - `sidetoc` — persistent side panel (left or right)
  - `quicktoc` — full-screen TOC overlay that replaces the document view
- Both modes are controlled via runtime commands, not CLI flags

#### Sidetoc mode
- MUST display TOC as a collapsible tree in a GTK TreeView
- MUST support configurable width (`sidetoc_width`, default 250px)
- MUST support configurable position (`sidetoc_position`: "left" or "right")
- MUST auto-focus the TreeView when opened
- MUST return focus to the document when closed
- Commands: `sidetoc_open`, `sidetoc_close`, `sidetoc_toggle`, `sidetoc_expand_width`, `sidetoc_shrink_width`, `sidetoc_focus`

#### Quicktoc mode
- MUST toggle between document view and TOC view via `quicktoc` command
- MUST return to document view and scroll to heading on entry selection (Enter)
- MUST return to document view without navigation on Escape

#### Keyboard navigation (both modes)
- MUST support arrow up/down for cursor navigation
- MUST support arrow left to collapse subtree or move to parent
- MUST support arrow right to expand subtree or move to first child
- MUST support Enter to activate the selected heading (scroll to it)
- MUST support Escape to close/dismiss
- Quicktoc additionally supports `j`/`k` (vim-style) navigation

#### Focus management
- `sidetoc_focus` command focuses the sidetoc TreeView (when open)
- `document_focus` command focuses the document WebView
- Sidetoc auto-focuses TreeView on open, returns focus to WebView on close

#### Reload
- MUST update the TOC when the document file changes
- MUST only rebuild TOC when headings actually change (avoid unnecessary rebuilds)

#### Configuration
- Startup commands via `runcmd` config or `--runcmd` CLI flag (e.g. `runcmd = "sidetoc_open"`)
- `sidetoc_width` and `sidetoc_position` in config.toml
- Keybindings configurable via `[keybindings]` section
