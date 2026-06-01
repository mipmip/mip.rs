## Context

The GTK Paned widget has built-in keyboard handling: when focused, arrow keys move the divider and PageUp/PageDown jump it. This was interfering with normal document scrolling. Additionally, the sidetoc and quicktoc TreeViews needed proper keyboard navigation for collapsing/expanding subtrees, and there was no way to programmatically switch focus between the sidetoc panel and the document.

## Goals / Non-Goals

**Goals:**
- Arrow keys and PageUp/Down work normally in the document
- Sidetoc TreeView: full keyboard navigation with collapse/expand
- Focus management commands for switching between sidetoc and document
- Consistent left/right behavior in both sidetoc and quicktoc

**Non-Goals:**
- Custom key bindings for individual TOC navigation actions (handled by the existing keybinding system at the command level)

## Decisions

### Disable paned focusability

**Choice**: `paned.set_focusable(false)` prevents the GTK Paned from receiving keyboard focus and stealing arrow/page keys for divider adjustment.

**Rationale**: The divider is controlled via commands (`sidetoc_expand_width`, `sidetoc_shrink_width`), not by keyboard-focusing the paned widget.

### Custom left/right handlers on TreeViews

**Choice**: Add explicit Left/Right key handlers on both sidetoc and quicktoc TreeViews that collapse/expand rows and navigate to parent/child.

**Rationale**: GTK TreeView's built-in left/right handling didn't work reliably. Custom handlers give consistent behavior: Left = collapse or go to parent, Right = expand or go to first child.

### Focus commands as first-class commands

**Choice**: `sidetoc_focus` and `document_focus` are registered commands, bindable via config keybindings.

**Rationale**: Follows the same pattern as all other mip commands. Users can bind them to any key combo (e.g. `"ctrl+h" = "sidetoc_focus"`).

### Auto-focus on open/close

**Choice**: `sidetoc_open` focuses the TreeView, `sidetoc_close` focuses the WebView.

**Rationale**: The user expects to interact with whatever panel they just opened/closed. No extra keystroke needed.

## Risks / Trade-offs

- [Paned not focusable] → Users can't adjust the divider with keyboard. Mitigation: `sidetoc_expand_width` and `sidetoc_shrink_width` commands serve this purpose.
