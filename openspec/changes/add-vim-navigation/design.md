## Context

mip.rs has a capture-phase key handler on the window (view.rs:592-633) that routes keystrokes through a `KeybindingRegistry`. The registry maps single `KeyCombo`s (key + modifiers) to command strings. Commands are executed via `execute_command()` which dispatches to specific actions. The WebView is scrolled via `webview.evaluate_javascript()`.

The keybinding registry lives in `command.rs` and supports parsing combo strings like `"ctrl+p"`, `"tab"`, `"shift+g"`. It stores bindings in a `HashMap<KeyCombo, String>`.

## Goals / Non-Goals

**Goals:**
- Add scroll commands that inject JS scroll actions into the WebView
- Add heading jump commands (next/prev heading)
- Extend the keybinding registry to support key sequences (`"g,g"`)
- All navigation commands are configurable via `[keybindings]` in config
- Navigation only fires when the WebView/document is focused

**Non-Goals:**
- Horizontal scrolling (h/l) — most markdown doesn't overflow horizontally
- Visual mode, text selection, or copy — handled by WebView natively
- `/{search}` — separate bean (mip.rs search feature)
- Smooth scrolling option — can be added later, instant scroll for now (matches vim)

## Decisions

### 1. Scroll commands as JS injection

**Decision**: Each scroll command calls `webview.evaluate_javascript()` with the appropriate scroll JS. Add these to `execute_command()`:

| Command | JS |
|---|---|
| `scroll_down` | `window.scrollBy(0, 60)` |
| `scroll_up` | `window.scrollBy(0, -60)` |
| `scroll_page_down` | `window.scrollBy(0, window.innerHeight)` |
| `scroll_page_up` | `window.scrollBy(0, -window.innerHeight)` |
| `scroll_half_down` | `window.scrollBy(0, window.innerHeight/2)` |
| `scroll_half_up` | `window.scrollBy(0, -window.innerHeight/2)` |
| `scroll_top` | `window.scrollTo(0, 0)` |
| `scroll_bottom` | `window.scrollTo(0, document.body.scrollHeight)` |

**Why**: The WebView owns the scroll state. JS injection is the same pattern already used for content reload, theme switching, and anchor scrolling. No new mechanisms needed.

### 2. Heading navigation via JS

**Decision**: `scroll_next_heading` and `scroll_prev_heading` inject JS that:
1. Collects all `h1, h2, h3, h4, h5, h6` elements with `id` attributes
2. Finds the current viewport position (`window.scrollY`)
3. Finds the next/previous heading relative to current scroll position
4. Calls `element.scrollIntoView({behavior: 'instant'})` on it

```javascript
// scroll_next_heading
(function() {
  var headings = document.querySelectorAll('h1[id],h2[id],h3[id],h4[id],h5[id],h6[id]');
  var y = window.scrollY + 10;
  for (var i = 0; i < headings.length; i++) {
    if (headings[i].offsetTop > y) {
      headings[i].scrollIntoView({behavior:'instant'});
      return;
    }
  }
})()
```

```javascript
// scroll_prev_heading
(function() {
  var headings = document.querySelectorAll('h1[id],h2[id],h3[id],h4[id],h5[id],h6[id]');
  var y = window.scrollY - 10;
  for (var i = headings.length - 1; i >= 0; i--) {
    if (headings[i].offsetTop < y) {
      headings[i].scrollIntoView({behavior:'instant'});
      return;
    }
  }
})()
```

**Why**: Headings already have `id` attributes from the TOC feature. No new data structures needed. The small offset (+/-10px) prevents getting stuck on the current heading.

### 3. Key sequence support with comma syntax

**Decision**: Extend the keybinding system to support sequences. A binding string containing commas is a sequence:

- `"g,g"` → press `g`, then press `g` within 500ms
- `"z,z"` → press `z`, then press `z` (future: center screen)
- `"ctrl+p"` → single combo (no comma, unchanged behavior)

**Data model**:
```
enum Binding {
    Single(KeyCombo),
    Sequence(Vec<KeyCombo>),
}
```

The registry stores `HashMap<KeyCombo, BindingAction>` where:
```
enum BindingAction {
    Command(String),                           // single combo → execute
    SequencePrefix(HashMap<KeyCombo, String>),  // first key → wait for second
}
```

When registering `"g,g" = "scroll_top"`:
- Parse into `[KeyCombo("g"), KeyCombo("g")]`
- Store: key `g` → `SequencePrefix({g: "scroll_top"})`

**Why**: This naturally extends the existing `HashMap<KeyCombo, String>`. The lookup becomes: check if the keypress maps to a `Command` (execute immediately) or a `SequencePrefix` (enter pending state). Single-key bindings are unaffected.

### 4. Pending-key state in the window key handler

**Decision**: Add `pending_key: Rc<Cell<Option<(KeyCombo, std::time::Instant)>>>` to the key handler closure. On keypress:

```
1. If pending_key is set:
   a. Check if expired (>500ms) → clear pending, process current key fresh
   b. Look up (pending_key, current_key) as sequence → execute command, clear pending
   c. No sequence match → clear pending, process current key as standalone

2. If pending_key is not set:
   a. Look up current key in registry
   b. If Single → execute immediately
   c. If SequencePrefix → set pending_key, consume event
   d. If no match → Propagation::Proceed
```

**Why**: No timers or async needed. The timeout check is just `Instant::elapsed() > Duration::from_millis(500)` on the next keypress. This is how vim works — it doesn't use a timer, it resolves ambiguity on the next input.

**Edge case**: If you press `g` and then nothing, the `g` is "stuck" in pending state forever. This is fine — the next keypress (whatever it is) will clear it (either completing the sequence or discarding it). There's no observable effect since `g` alone doesn't do anything.

### 5. Default keybindings

**Decision**: Register all vim navigation bindings as defaults in `KeybindingRegistry::with_defaults()`:

```rust
registry.register_str("j", "scroll_down");
registry.register_str("k", "scroll_up");
registry.register_str("down", "scroll_down");
registry.register_str("up", "scroll_up");
registry.register_str("ctrl+f", "scroll_page_down");
registry.register_str("ctrl+b", "scroll_page_up");
registry.register_str("pagedown", "scroll_page_down");
registry.register_str("pageup", "scroll_page_up");
registry.register_str("ctrl+d", "scroll_half_down");
registry.register_str("ctrl+u", "scroll_half_up");
registry.register_str("home", "scroll_top");
registry.register_str("end", "scroll_bottom");
registry.register_str("shift+g", "scroll_bottom");
registry.register_str("g,g", "scroll_top");
registry.register_str("n", "scroll_next_heading");
registry.register_str("shift+n", "scroll_prev_heading");
```

Users can override any of these in their config. Setting a binding to an empty string disables it.

**Why**: Sensible defaults that work immediately. Power users can remap.

### 6. Only fire when WebView is focused

**Decision**: The existing key handler already skips when the command bar is visible. For scroll commands specifically, also skip when a TreeView (sidetoc or quicktoc) is focused — those have their own j/k handlers.

In practice this works naturally: the capture-phase handler on the window fires, looks up the binding, and executes the command. The `scroll_*` commands call `webview.evaluate_javascript()` which scrolls the WebView regardless of focus. But the j/k keys are also bound to TreeView navigation. The resolution: TreeView key handlers run at the bubble phase and consume j/k when the TreeView is focused. The capture-phase window handler should check if a TreeView descendant is focused and skip scroll commands in that case.

Actually simpler: the TreeView key handlers already return `Propagation::Stop` for j/k. Since they run on the TreeView at the default phase, and the window handler runs at capture phase... wait, capture fires first. So the window handler would intercept j/k before the TreeView sees them.

**Fix**: In the window key handler, check if the focused widget is a TreeView. If so, skip the keybinding lookup and let the event propagate to the TreeView's own handlers.

## Risks / Trade-offs

- **[Risk] j/k conflict with TreeView navigation** → Mitigation: window key handler checks focused widget before dispatching scroll commands. TreeView focus → skip registry lookup.
- **[Risk] Ctrl+f conflict with browser find** → WebKitGTK's WebView doesn't have a built-in find bar on Ctrl+f. No conflict.
- **[Risk] Key sequence state leaks across focus changes** → Mitigation: clear pending_key when command bar opens or focus changes to TreeView.
- **[Trade-off] 60px scroll step is fixed** → Could be configurable later via a `scroll_step` config option. 60px (~3 lines) is a reasonable default.
- **[Trade-off] No smooth scrolling** → Instant scroll matches vim behavior. Smooth scroll can be a config option later.
