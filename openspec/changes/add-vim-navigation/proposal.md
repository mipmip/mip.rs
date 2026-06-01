## Why

mip.rs is designed to preview markdown alongside vim, but the document itself has no vim-style navigation. Users must reach for the mouse or arrow keys to scroll. Adding hjkl scrolling, page movement, and heading jumping makes the preview feel native to a vim workflow.

Bean: [mip.rs-tctx](/home/pim/cLinden/mip.rs/.beans/mip.rs-tctx--vim-navigation-hjkl-and-ctrl-fcrtl-b.md)

## What Changes

### New scroll commands

Add commands to `execute_command()` that inject scroll JS into the WebView:

| Command | Default binding | Action |
|---|---|---|
| `scroll_down` | `j`, `down` | Scroll down one step (~60px) |
| `scroll_up` | `k`, `up` | Scroll up one step (~60px) |
| `scroll_page_down` | `ctrl+f`, `pagedown` | Scroll down one viewport height |
| `scroll_page_up` | `ctrl+b`, `pageup` | Scroll up one viewport height |
| `scroll_half_down` | `ctrl+d` | Scroll down half viewport |
| `scroll_half_up` | `ctrl+u` | Scroll up half viewport |
| `scroll_top` | `home`, `g,g` | Scroll to top of document |
| `scroll_bottom` | `shift+g`, `end` | Scroll to bottom of document |
| `scroll_next_heading` | `n` | Jump to next heading below viewport |
| `scroll_prev_heading` | `shift+n` | Jump to previous heading above viewport |

All commands only fire when the WebView is focused (not in command bar, not in TOC). All configurable via `[keybindings]`.

### Key sequence support in keybinding registry

Extend the keybinding system to support multi-key sequences using comma syntax:

```toml
[keybindings]
"g,g" = "scroll_top"
```

This requires:
- Extending `parse_key_combo()` to detect comma-separated sequences and store them as `KeySequence` (ordered list of `KeyCombo`)
- Adding pending-key state to the key handler: when the first key of a sequence matches, consume it and wait for the next key within 500ms
- If the sequence completes, execute the command; if it times out or a non-matching key arrives, discard the pending state

### Heading navigation via JS

`scroll_next_heading` / `scroll_prev_heading` use JS to find all `h1-h6[id]` elements, determine which is currently at/above the viewport top, and `scrollIntoView` the next/previous one.

## Capabilities

### New Capabilities
- `vim-navigation`: Vim-style document scrolling (j/k, Ctrl+f/b/d/u, G/gg) and heading jumping (n/N)

### Modified Capabilities
- `keybindings`: Support key sequences (`"g,g"`) in addition to single key combos

## Impact

- **Code**: `view.rs` (new commands in `execute_command()`, pending-key state in key handler), `command.rs` (sequence parsing, sequence lookup, registry changes)
- **Config**: New default keybindings for all scroll commands, `g,g` sequence example in `--initconf` template
- **Dependencies**: None
- **JS**: Heading navigation needs a small JS function injected to find and scroll to headings
