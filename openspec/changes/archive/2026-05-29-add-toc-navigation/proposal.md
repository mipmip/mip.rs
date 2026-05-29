## Why

mip.rs has no way to see or navigate a document's structure. For long markdown files — the primary use case when previewing alongside vim — you lose orientation quickly. A Table of Contents gives both a map of the document and a way to jump between sections.

Bean: [mip.rs-sdmg](/home/pim/cLinden/mip.rs/.beans/mip.rs-sdmg--side-panel-with-toc.md)

## What Changes

- Extract heading data (level, title, anchor ID) from pulldown-cmark events during markdown parsing, returning a `Vec<TocEntry>` alongside the HTML body
- Inject anchor `id` attributes on heading elements in the generated HTML
- Add a GTK `TreeView` + `TreeStore` widget displaying headings as a collapsible hierarchy
- Support two configurable display modes:
  - **side**: TOC as a persistent side panel (GTK `Paned`) alongside the WebView
  - **zathura**: TOC replaces the document view on `<Tab>`, selecting a heading jumps there and returns to the document (like Zathura's PDF TOC)
- Add `toc_mode` config option (`"side"` | `"zathura"` | `"off"`) to `~/.config/miprs/config.toml` and `--toc` CLI flag
- Update the TOC on file reload within the existing `glib::timeout_add_local` callback
- Clicking/selecting a TOC entry scrolls the WebView to the corresponding anchor via `evaluate_javascript()`

### Keyboard interaction (Zathura mode)

- `<Tab>` — toggle TOC view
- `j`/`k` or `↑`/`↓` — navigate entries
- `Enter` or `<Tab>` — select entry, jump to heading, return to document
- `<Esc>` — close TOC without jumping

### Design decisions

- **TreeView over ListView**: heading hierarchy is naturally tree-shaped; TreeView gives collapse/expand and keyboard navigation for free. Though "legacy" in GTK4, it's stable and avoids reinventing tree behavior.
- **Heading-level jumps** (e.g. h1→h3 skipping h2): parent under the nearest higher-level heading, no phantom nodes.
- **Runtime mode switching** is deferred to the command mode feature ([mip.rs-2t32](/home/pim/cLinden/mip.rs/.beans/mip.rs-2t32--colons-to-open-the-command-mode.md)), but the architecture supports reparenting the TOC widget between Stack and Paned.

## Capabilities

### New Capabilities
- `toc-navigation`: GTK-native Table of Contents with hierarchical heading display, two display modes (side panel / zathura-style), and keyboard navigation

### Modified Capabilities
- `markdown-rendering`: Heading elements gain anchor `id` attributes; `md_to_html_body` returns TOC data alongside HTML

## Impact

- **Code**: `markdown.rs` (heading extraction + anchor IDs), `view.rs` (TOC widget, layout containers, keyboard handling, scroll-to-anchor), `config.rs` (new `toc_mode` field), `main.rs` (CLI flag)
- **New files**: none expected — all changes fit in existing modules
- **Dependencies**: none — `TreeView`/`TreeStore` are part of `gtk4`
- **Config**: new `toc_mode` key in `config.toml`
