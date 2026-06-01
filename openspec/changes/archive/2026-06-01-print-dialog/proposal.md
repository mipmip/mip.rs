## Why

There's no way to export a PDF or print from mip. The WebView already renders the full styled HTML — we can leverage WebKitGTK's built-in print API to give users a familiar Ctrl+P → print dialog workflow, including "Print to File" for PDF export.

Bean: mip.rs-73lh

## What Changes

- Add `Ctrl+P` keyboard shortcut in the preview window
- Open GTK print dialog via `webkit6::PrintOperation`
- Add `@media print` CSS that forces light theme colors regardless of screen theme
- No new CLI flags, no headless mode — interactive only

## Capabilities

### New Capabilities
- `print`: Ctrl+P keyboard shortcut opens GTK print dialog for printing/PDF export

### Modified Capabilities
- `theming`: Add `@media print` CSS block that forces light colors for print output

## Impact

- `src/view.rs`: add key event controller for Ctrl+P, create PrintOperation and run dialog
- `asset/theme1/template.html`: add `@media print` CSS block
