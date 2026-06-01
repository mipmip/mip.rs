## Context

WebKitGTK exposes `PrintOperation` which wraps the GTK print dialog. It can print to physical printers or "Print to File" (PDF). The WebView already renders the complete styled HTML. mip uses CSS variables for theming (light/dark), so print styling can be handled purely in CSS.

## Goals / Non-Goals

**Goals:**
- Ctrl+P opens the GTK print dialog
- Print output is always light theme (regardless of current screen theme)
- Works for both physical printing and PDF export via "Print to File"

**Non-Goals:**
- Headless/CLI PDF export (`mip --export-pdf`)
- Custom print settings (page size, margins) — GTK dialog handles all of this
- Print preview (GTK print dialog has its own)

## Decisions

### `@media print` CSS for forced light theme

**Choice**: Add a `@media print` block in template.html that redefines all CSS variables to light values.

**Rationale**: This is the cleanest approach — no JS class toggling, no need to save/restore state. The browser's print engine automatically applies `@media print` rules. The screen stays in whatever theme the user chose while the print output is always light.

### GTK4 EventControllerKey for Ctrl+P

**Choice**: Add a `gtk4::EventControllerKey` to the window, check for `Ctrl+P`, create `PrintOperation::new(&webview)` and call `run_dialog(Some(&window))`.

**Rationale**: Standard GTK4 pattern for keyboard shortcuts. The PrintOperation is created fresh each time — no need to keep a reference around.

### No page setup customization

**Choice**: Use default page setup. The GTK print dialog lets the user change paper size, margins, orientation themselves.

**Rationale**: Keep it simple. Power users can configure in the dialog.

## Risks / Trade-offs

- [Dark backgrounds in print] → The `@media print` CSS forces light. If we miss a color variable, some elements might print dark. Mitigation: use the same variable set as `.light` class.
