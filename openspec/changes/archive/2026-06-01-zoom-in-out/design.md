## Context

mip uses a `CommandContext` struct in `view.rs` with an `execute_command` function that dispatches string commands. The `command.rs` module has a `COMMANDS` list for tab-completion and a `KeybindingRegistry` with defaults. WebKitGTK's `WebView` has `set_zoom_level(f64)` and `zoom_level() -> f64`.

## Goals / Non-Goals

**Goals:**
- Ctrl+=, Ctrl+-, Ctrl+0 zoom the preview
- Zoom steps of 10% (0.1 increment), matching browser convention
- Commands available in command bar too (`:zoom_in`, etc.)

**Non-Goals:**
- Persisting zoom level across sessions
- Per-file zoom memory
- Zoom level indicator/overlay
- Pinch-to-zoom (may already work via WebKit natively)

## Decisions

### Zoom step size: 10%

**Choice**: Each zoom_in/zoom_out changes by 0.1 (10%). Clamp between 0.3 and 5.0.

**Why**: Matches browser behavior. Fine-grained enough without being too slow.

### Session-only state

**Choice**: Zoom level lives on the `WebView` itself (it already stores it). No need for extra state.

**Why**: `webview.zoom_level()` and `webview.set_zoom_level()` are the getter/setter. No additional tracking needed.

### Keybindings

**Choice**: `ctrl+=` → zoom_in, `ctrl+-` → zoom_out, `ctrl+0` → zoom_reset. Registered as defaults in `KeybindingRegistry::with_defaults()`.

**Why**: Universal browser/editor convention. Note: Ctrl+= (not Ctrl+Shift+=) because most keyboards produce `=` on that key, and GTK reports the unshifted keyval.

## Risks / Trade-offs

- **[Risk] Ctrl+0 might conflict** → No existing binding uses Ctrl+0. Safe.
- **[Risk] Zoom affects print output** → WebKit's print operation uses its own scaling. Not affected.
