## Why

The TOC panels (sidetoc and quicktoc) lacked proper keyboard navigation. Arrow keys and PageUp/Down were being stolen by the GTK Paned widget for divider adjustment instead of scrolling the document. The sidetoc had no way to collapse/expand subtrees, focus management between sidetoc and document was missing, and there were no commands to switch focus programmatically.

## What Changes

- Fix: prevent GTK Paned from stealing arrow/page keys by disabling its focusability
- Sidetoc TreeView keyboard navigation: arrow up/down to navigate, left/right to collapse/expand subtrees, Enter to scroll to heading, Escape to close, `l` to focus document
- Quicktoc TreeView: add left/right collapse/expand (matching sidetoc behavior)
- New commands: `sidetoc_focus` (focus the sidetoc treeview), `document_focus` (focus the webview)
- Sidetoc auto-focuses its TreeView on open, returns focus to WebView on close
- Updated initconf template with new commands in the documentation

## Capabilities

### New Capabilities

### Modified Capabilities
- `command-mode`: Add `sidetoc_focus` and `document_focus` commands

## Impact

- `src/view.rs`: paned focusable fix, sidetoc/quicktoc key handlers, focus management in open/close commands
- `src/command.rs`: new command names in COMMANDS list
- `src/config.rs`: updated initconf template command documentation
