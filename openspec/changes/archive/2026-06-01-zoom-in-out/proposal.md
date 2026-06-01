## Why

There's no way to zoom in or out in the preview window (GitHub issue #6). WebKitGTK has a built-in `zoom_level` API — this is a small feature that adds standard Ctrl+/- zoom controls using the existing command and keybinding infrastructure.

Bean: [mip.rs-2l50](/home/pim/cLinden/mip.rs/.beans/mip.rs-2l50--zoom-inout.md)

## What Changes

- Add three new commands: `zoom_in`, `zoom_out`, `zoom_reset`
- Register default keybindings: Ctrl+= (zoom in), Ctrl+- (zoom out), Ctrl+0 (reset)
- Zoom is session-only (resets when mip closes)

## Capabilities

### New Capabilities
- `zoom`: Keyboard zoom in/out/reset via WebKitGTK's zoom_level API

### Modified Capabilities

_(none)_

## Impact

- `src/view.rs`: add `zoom_in`, `zoom_out`, `zoom_reset` command handlers (3 lines each)
- `src/command.rs`: register commands in `COMMANDS` list and default keybindings
- `src/config.rs`: add zoom keybindings to default config template
