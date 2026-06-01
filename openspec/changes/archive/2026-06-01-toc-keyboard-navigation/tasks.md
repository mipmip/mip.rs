## 1. Fix paned keyboard stealing

- [x] 1.1 Set `paned.set_focusable(false)` to prevent arrow/page key interception

## 2. Sidetoc keyboard navigation

- [x] 2.1 Add key handler on sidetoc TreeView: Left collapses or moves to parent
- [x] 2.2 Right expands or moves to first child
- [x] 2.3 Escape closes sidetoc and returns focus to WebView
- [x] 2.4 `l` key moves focus to document (vim-style)

## 3. Quicktoc left/right

- [x] 3.1 Add Left/Right key handlers to quicktoc TreeView (same collapse/expand behavior)

## 4. Focus commands

- [x] 4.1 Add `sidetoc_focus` command (focuses TreeView when sidetoc is open)
- [x] 4.2 Add `document_focus` command (focuses WebView)
- [x] 4.3 Add both to COMMANDS list in command.rs

## 5. Auto-focus on open/close

- [x] 5.1 `sidetoc_open` grabs focus on the TreeView
- [x] 5.2 `sidetoc_close` returns focus to WebView

## 6. Documentation

- [x] 6.1 Update initconf template with `sidetoc_focus` and `document_focus` in command list

## 7. Verify

- [x] 7.1 `cargo build` succeeds
- [x] 7.2 Arrow keys and PageUp/Down work normally in document
- [x] 7.3 Sidetoc: up/down/left/right/Enter/Escape all work
- [x] 7.4 Quicktoc: left/right collapse/expand works
- [x] 7.5 `:sidetoc_focus` and `:document_focus` commands work
- [x] 7.6 Sidetoc auto-focuses on open, WebView on close
