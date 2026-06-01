## Context

mip.rs uses `pulldown-cmark` with `ENABLE_TASKLISTS` to parse markdown task lists. The parser emits standard HTML: `<input type="checkbox" disabled>` inside `<li>` elements. Currently no CSS targets these elements, so they render with browser defaults — small, platform-dependent, and visually inconsistent with the rest of the theme.

## Goals / Non-Goals

**Goals:**
- Match GitHub's checkbox visual style (rounded corners, blue checked state, proper sizing)
- Remove bullet markers from task list items
- Work across light theme (dark theme support is future work)

**Non-Goals:**
- Making checkboxes interactive (they remain `disabled`)
- Changing the HTML output from pulldown-cmark
- Dark mode checkbox variants

## Decisions

**CSS-only approach** — Since pulldown-cmark already emits `<input type="checkbox" disabled>`, we style these elements directly with `appearance: none` and custom properties. No Rust code changes needed.

Alternative considered: Modifying the pulldown-cmark event stream to emit custom HTML (e.g., SVG icons). Rejected because CSS-only is simpler, more maintainable, and sufficient for the visual goal.

**`:has()` selector for list item styling** — Use `li:has(> input[type="checkbox"])` to remove bullets from task list items specifically. This avoids affecting regular list items.

Alternative considered: Adding a class to task list `<ul>` elements via Rust code. Rejected because `:has()` is widely supported now and avoids touching the Rust layer.

## Risks / Trade-offs

- **`:has()` browser support** → Supported in all modern browsers since Dec 2023. mip.rs renders in WebKit (GTK WebView), which supports `:has()`. No risk.
- **`appearance: none` cross-platform rendering** → Well-established CSS property. The custom styling replaces all native rendering, so platform differences are eliminated.
