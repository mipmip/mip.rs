## Why

Command mode Tab completion currently cycles through matches invisibly — the user can't see what options are available. Also, partial command names (`:op<Tab>`) don't complete to the full command. Both make the command bar harder to use than it should be.

Bean: mip.rs-v8fn

## What Changes

- Add command name completion: Tab on partial command name completes to the matching command (e.g. `:op<Tab>` → `:open `)
- Add wildmenu-style completion display: a Label widget above the command bar shows all matching options, with the current match highlighted via Pango markup
- Wildmenu appears on Tab when multiple matches exist, hides on Enter/Escape/typing
- Works for both command name completion and path completion

## Capabilities

### New Capabilities
- `completion-popup`: Wildmenu-style completion display for command bar with command name and path completion

### Modified Capabilities
- `command-mode`: Tab now completes command names, wildmenu label shows matches

## Impact

- `src/view.rs`: add Label widget to outer Box (between content and entry), command name matching in Tab handler, Pango markup for highlighting current match
