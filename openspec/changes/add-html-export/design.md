## Context

mip.rs renders markdown in a WebView via `evaluate_javascript` for content updates. The fully rendered DOM contains KaTeX math as styled HTML spans, Mermaid diagrams as inline SVGs, and all CSS applied. The command system dispatches string commands via `execute_command()` in `view.rs`, with tab-completable command names listed in `COMMANDS` in `command.rs`.

The `evaluate_javascript` API accepts a callback of type `FnOnce(Result<javascriptcore::Value, glib::Error>)`. The `javascriptcore::Value` has a `to_string()` method that extracts a JS string result. All current uses ignore the callback result (`|_| {}`).

## Goals / Non-Goals

**Goals:**
- Export the current preview as a self-contained HTML file
- Exported file works in any browser without mip or a local server
- Math, diagrams, styling all preserved in the export
- Triggered via `:export_html <path>` command

**Non-Goals:**
- Export as PDF (already handled by Ctrl+P print dialog)
- Export as markdown (source file is already markdown)
- Batch export / CLI-only export mode (can be added later)
- Asset directory export (single file is simpler for sharing)

## Decisions

### 1. DOM capture via evaluate_javascript

**Decision**: Execute `document.documentElement.outerHTML` in the WebView and capture the result string via the callback.

```rust
ctx.webview.evaluate_javascript(
    "document.documentElement.outerHTML",
    None, None, None::<&gtk4::gio::Cancellable>,
    move |result| {
        if let Ok(value) = result {
            let html = value.to_string();
            // post-process and write to file
        }
    },
);
```

**Why**: The rendered DOM already contains everything — KaTeX math as styled `<span>`s, Mermaid diagrams as `<svg>`s, CSS in `<style>` tags, the current theme applied. No need to re-render or inline JS libraries. The result is a clean HTML+CSS+SVG document.

### 2. Post-processing: strip mip-specific content

**Decision**: After capturing the DOM, remove:
- All `<script>` tags (seed polling, bridge.js, KaTeX, Mermaid — none needed in exported file)
- The header div with the localhost document URL link
- Any `localhost:PORT` references in remaining attributes

**Why**: The exported file should be a clean document, not a mip preview. Scripts are unnecessary because all rendering is already baked into the DOM. The localhost link would be a dead link.

### 3. Inline CSS from external references

**Decision**: The DOM capture includes `<link>` tags referencing `localhost:PORT/katex/katex.min.css`. These need to be either:
- Left as-is if the CSS is already applied via `<style>` tags in the DOM (WebKitGTK may inline computed styles)
- Replaced with the CSS content read from rust-embed assets

In practice, the DOM capture from `outerHTML` preserves `<style>` tags but also keeps `<link>` references. The simplest approach: strip `<link>` tags pointing to localhost and verify the rendered appearance is preserved (KaTeX applies its styles inline on elements, so the `<link>` removal is safe for math). If needed, read the CSS from embedded assets and inject as a `<style>` block.

**Why**: External `<link>` tags pointing to localhost would be dead references in the exported file. KaTeX renders with inline styles on elements, so removing the CSS link is safe for math. The document's own CSS (from the template) is already in a `<style>` tag.

### 4. Font handling

**Decision**: KaTeX fonts referenced via `@font-face` in `katex.min.css` won't be available in the export. This is acceptable because:
- KaTeX applies inline styles with specific font-family declarations
- Most browsers will fall back to similar system fonts (serif/sans-serif)
- The math is still fully readable, just with slightly different font metrics
- Embedding fonts as base64 would add ~200KB for marginal visual improvement

If font fidelity becomes important, we can add a `--with-fonts` option later that base64-encodes WOFF2 files into the CSS.

**Why**: Pragmatic — the sharing use case cares about content and structure, not pixel-perfect font matching.

### 5. Path handling with tilde expansion

**Decision**: The command argument goes through `expand_tilde()` (already exists in `command.rs`). Relative paths are resolved against the current working directory. Parent directories are created if they don't exist.

**Why**: Consistent with the existing `open` command's path handling.

### 6. Async callback writes file

**Decision**: The `evaluate_javascript` callback is async (fires when JS evaluation completes). The file write happens inside the callback closure. The closure captures the file path.

```
User types :export_html ~/report.html
     │
     ▼
execute_command("export_html", "~/report.html", ctx)
     │
     ├─ expand_tilde → /home/user/report.html
     ├─ clone path into closure
     └─ webview.evaluate_javascript("document.documentElement.outerHTML", ...,
          move |result| {
            let html = post_process(result.unwrap().to_string());
            std::fs::write(&path, html);
          })
```

**Why**: `evaluate_javascript` is inherently async. The callback is the natural place to process the result. Since it runs on the GTK main thread, file I/O is safe (though blocking — acceptable for a one-off export).

## Risks / Trade-offs

- **[Risk] DOM capture may include WebKit-internal attributes or shadow DOM elements** → Mitigation: test with various content types, strip unwanted attributes if needed.
- **[Risk] KaTeX fonts missing in exported file** → Accepted trade-off: math is readable with fallback fonts, font embedding can be added later.
- **[Risk] Large documents produce large exported HTML** → For most markdown documents, the export will be 50-200KB. Documents with many Mermaid SVGs could be larger. Acceptable.
- **[Risk] Mermaid SVGs contain theme-specific colors** → The export captures the current theme. Users should switch to the desired theme before exporting. Could add `:export_html --theme light` later.
- **[Trade-off] Blocking file I/O in callback** → For a single file write of <1MB, the blocking time is negligible (<1ms). Async I/O would add complexity for no practical benefit.
