## Context

mip.rs renders markdown to HTML inside a GTK4 WebView. There is no way to see or navigate document structure. The document is re-rendered on every file change via a `glib::timeout_add_local` callback that polls a seed file every 500ms. All rendering state lives on the GTK main thread.

## Goals / Non-Goals

**Goals:**
- Extract heading hierarchy from markdown during parsing
- Display TOC as a native GTK TreeView with collapsible heading tree
- Support two display modes: persistent side panel and Zathura-style toggle
- Navigate to headings by selecting TOC entries
- Update TOC on file reload
- Vim-style keyboard navigation (j/k, Enter, Esc)

**Non-Goals:**
- Runtime mode switching (deferred to command mode, mip.rs-2t32)
- Nested scroll position sync (highlighting current heading in side panel mode) — nice-to-have, not in first pass
- Custom TOC styling/theming beyond GTK defaults

## Decisions

### 1. Heading extraction in pulldown-cmark

**Decision**: Intercept `Event::Start(Tag::Heading { level, .. })` and collect subsequent text events to build `Vec<TocEntry>`. Generate deterministic anchor IDs (slugified title) and inject `id` attributes into heading HTML output.

**Why**: pulldown-cmark's `html::push_html` consumes the iterator, so we need a custom event processing step that both collects TOC data and produces HTML. We'll use a wrapper iterator that passes events through while capturing headings.

**Alternative considered**: Post-processing the HTML with regex to find `<h1>`..`<h6>` tags. Rejected — fragile, doesn't give us control over ID generation, and duplicates parsing work.

### 2. Anchor ID generation

**Decision**: Slugify heading text: lowercase, replace non-alphanumeric with `-`, collapse consecutive dashes, trim. Append `-1`, `-2` for duplicates.

**Why**: Deterministic IDs mean the TOC can reference anchors reliably. The scheme matches common markdown renderers (GitHub, mdBook).

### 3. GTK TreeView + TreeStore for the TOC widget

**Decision**: Use `gtk4::TreeView` with a `gtk4::TreeStore` (columns: title as String, anchor_id as String, level as u32).

**Why**: Headings are hierarchical. TreeView handles expand/collapse, keyboard navigation (arrow keys), and indentation natively. ListView would require reimplementing tree behavior.

**Trade-off**: TreeView is "legacy" in GTK4 (won't get new features), but it's stable, works correctly, and provides exactly the widget semantics we need. Migration to ListView can happen later if GTK5 drops TreeView.

### 4. Heading-level jumps

**Decision**: When a heading skips levels (e.g. h1 → h3), parent it under the nearest ancestor with a lower level. No phantom/placeholder nodes.

**Why**: Markdown authors frequently skip heading levels. Inserting phantom nodes would clutter the tree and confuse navigation. The "nearest higher ancestor" approach is what Zathura and most TOC generators use.

```
# Title         → root
### Deep        → child of "Title" (skipped h2)
## Section      → root (sibling of "Title")
#### Sub-deep   → child of "Section" (skipped h3)
```

### 5. Layout containers

**Decision**: Use `gtk::Paned` for side panel mode, `gtk::Stack` for Zathura mode. The TOC TreeView is the same widget, parented in whichever container the mode requires. The container is chosen at startup based on config; runtime switching (reparenting) is deferred to command mode.

**Why**: Both containers are lightweight and support the exact interaction model needed. Paned gives a draggable divider. Stack gives instant swap with no animation overhead.

```
Side mode:                    Zathura mode:
┌─────────┬──────────┐       ┌─────────────────────┐
│ TreeView│ WebView  │       │  Stack               │
│ (Paned) │          │       │  ┌─────────────────┐ │
│         │          │       │  │ WebView (page 0) │ │
│         │          │       │  │ TreeView (page 1)│ │
└─────────┴──────────┘       │  └─────────────────┘ │
                              └─────────────────────┘
```

### 6. TOC update on reload

**Decision**: Extend the existing `glib::timeout_add_local` callback. When the seed changes, call a new `md_to_html_body_with_toc()` that returns `(String, Vec<TocEntry>)`. Update WebView HTML as before, then clear and repopulate the TreeStore.

**Why**: The callback already runs on the GTK main thread and re-parses markdown. Adding TOC extraction here requires zero cross-thread coordination.

### 7. Scroll-to-heading

**Decision**: On TOC row activation, call `webview.evaluate_javascript()` with `document.getElementById('<anchor_id>').scrollIntoView({behavior: 'smooth'})`.

**Why**: Simple, reliable, and uses the existing WebView JS injection pattern already used for content reload and theme switching.

### 8. Keyboard handling

**Decision**: TreeView gets `j`/`k` mapped to cursor movement, `Enter` to activate (navigate + close in Zathura mode), `Esc` to close without navigating (Zathura mode). In Zathura mode, `<Tab>` on the WebView opens the TOC; on the TreeView, `<Tab>` activates the selected row.

**Why**: Vim-style is the user's preference. Arrow keys work natively in TreeView; j/k are added via a `connect_key_pressed` handler that translates them to cursor movement signals.

## Risks / Trade-offs

- **[Risk] pulldown-cmark event interception adds complexity to markdown.rs** → Mitigation: the wrapper iterator is ~30 lines, well-isolated, and testable in isolation.
- **[Risk] TreeView is legacy GTK4** → Accepted: stable API, no removal planned, exact fit for the use case. Can migrate later.
- **[Risk] Duplicate heading IDs from repeated heading text** → Mitigation: suffix counter (`-1`, `-2`) tracked during extraction.
- **[Trade-off] No scroll-position sync in side panel mode initially** → The TOC shows structure and allows jumping, but won't highlight where you currently are. This can be added later with an IntersectionObserver in JS calling back to Rust.
- **[Trade-off] TreeStore rebuild on every file change** → For typical markdown files (<200 headings) this is negligible. If it becomes an issue, diff-based updates can be added later.
