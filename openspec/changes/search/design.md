## Context

mip has a command bar (GTK Entry) that opens with `:`. The same widget can be reused for search by opening with `/` prefix instead. The main window key controller already handles `:` in capture phase. WebKitGTK's `WebView::find_controller()` provides `search()`, `search_next()`, `search_previous()`, and `count_matches()` with built-in highlighting. The TOC is a `TreeStore` backing two `TreeView` widgets (sidetoc and quicktoc).

## Goals / Non-Goals

**Goals:**
- `/` opens search bar, behaves differently based on focus context (document vs TOC)
- Document search: live highlighting via FindController, n/N for next/prev
- TOC search: live filtering of headings
- Escape clears everything

**Non-Goals:**
- Regex search (plain text only)
- Case-sensitive toggle (always case-insensitive)
- Search-and-replace
- Persistent search state across sessions

## Decisions

### 1. Reuse the command bar for search input

**Choice**: The same GTK Entry widget used for `:` commands is reused for `/` search. The prefix determines the mode.

**Why**: Avoids a second input widget. The entry already has key handling, focus management, and the wildmenu label above it. The `/` prefix visually distinguishes search from command mode.

### 2. Determine mode by focus context at `/` press time

**Choice**: When `/` is pressed, check what is focused:
- If document (webview) or nothing specific → document search mode
- If sidetoc or quicktoc TreeView → TOC filter mode

Store the mode in a cell so the entry's key handler knows which behavior to use.

**Why**: Matches the bean's requirement: "when sidetoc or quicktoc is focussed let / be a filter". Natural — you search what you're looking at.

### 3. Document search: WebKit FindController with live highlight

**Choice**: On each keystroke in search mode, call `find_controller.search(text, CASE_INSENSITIVE | WRAP_AROUND, 0)`. This highlights all matches live. Enter closes the search bar. `n` calls `search_next()`, `N` calls `search_previous()`.

**Why**: FindController handles all rendering, scrolling to match, and highlight management. Zero custom rendering needed.

### 4. TOC filter: rebuild store with matching entries

**Choice**: On each keystroke in TOC filter mode, rebuild the TreeStore with only entries whose title contains the search text (case-insensitive). On Escape/clear, restore the full TOC.

**Why**: Simpler than `TreeModelFilter` (which requires careful lifetime management with two TreeViews sharing one model). Rebuilding is fast — TOCs are small (dozens of entries at most).

### 5. n/N navigation in main window key controller

**Choice**: Add `n` and `N` (shift+n) handlers in the main window key controller. They call `find_controller.search_next()` / `search_previous()`. Only active when a search has been performed (track last search text in a cell).

**Why**: Standard vim behavior. These are unmodified letter keys, so they must not fire when command bar or TOC is focused — the existing `is_visible` check on the command bar handles this, and TreeView key handlers consume their own keys.

### 6. Match count display

**Choice**: Show match count in the wildmenu label: "3/17 matches". Use `FindController::count_matches()` and connect to `counted-matches` signal.

**Why**: Provides useful feedback without a new widget. Reuses the existing wildmenu label.

## Risks / Trade-offs

- **[Risk] `/` conflicts with typing in command bar** → No conflict: `/` only triggers from the main window key controller, which skips when command bar is visible.
- **[Risk] `n` conflicts with regular typing** → Only fires when command bar is hidden and a search is active. If no search has been performed, `n` does nothing.
- **[Risk] TOC rebuild flickers** → Unlikely with small data sets. If needed, can batch updates.
