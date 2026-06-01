## 1. Print CSS

- [x] 1.1 Add `@media print` block to template.html that redefines all CSS variables to light theme values

## 2. Keyboard shortcut and print handler

- [x] 2.1 Add `EventControllerKey` to the ApplicationWindow in `view.rs`
- [x] 2.2 Detect Ctrl+P keypress in the key handler
- [x] 2.3 Create `PrintOperation::new(&webview)` and call `run_dialog(Some(&window))`

## 3. Verify

- [x] 3.1 `cargo build` succeeds
- [x] 3.2 Ctrl+P in preview opens GTK print dialog
- [x] 3.3 "Print to File" as PDF produces a readable PDF
- [x] 3.4 PDF output uses light colors even when previewing in dark mode
- [x] 3.5 Cancelling the print dialog returns to preview normally
