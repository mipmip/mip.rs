## Context

mip.rs renders markdown to HTML via pulldown-cmark and displays it in a GTK4 WebView. The HTML template is compiled by inlining CSS/JS into a single file, then embedded in the binary via `rust-embed`. Content reloads happen via JS injection (`evaluate_javascript`) — the Rust side replaces `.section` innerHTML every 500ms when the source file changes.

pulldown-cmark 0.12 has `ENABLE_MATH` which parses `$...$` and `$$...$$` into `Event::InlineMath` / `Event::DisplayMath`. The default `html::push_html` already renders these as:
```html
<span class="math math-inline">x^2</span>
<span class="math math-display">\sum_{i=0}^n x_i</span>
```

The math text is HTML-escaped inside the spans.

## Goals / Non-Goals

**Goals:**
- Render inline and display math in the preview using KaTeX
- Math works offline (KaTeX bundled in the binary)
- Math re-renders on file change without full page reload
- Config option to disable math rendering
- No flash of unrendered TeX on load or reload

**Non-Goals:**
- Full TeX document support (TikZ, custom preambles, etc.)
- Server-side math rendering (all rendering is client-side in WebView)
- Custom KaTeX macros via config (can be added later)
- Math in the TOC (raw text is fine there)

## Decisions

### 1. Enable pulldown-cmark ENABLE_MATH, no custom event handling

**Decision**: Add `Options::ENABLE_MATH` to the parser options. Let `push_html` handle the HTML output. No changes to `extract_headings_and_inject_ids`.

**Why**: pulldown-cmark already emits the exact HTML we need (`<span class="math math-inline">` / `<span class="math math-display">`). Our heading extractor's catch-all `_ => {}` passes math events through untouched. Zero Rust-side rendering code needed.

**Trade-off**: Math inside headings won't appear in TOC titles (the InlineMath event isn't captured by the Text/Code handlers). This is acceptable — raw TeX in a TOC would be confusing anyway.

### 2. KaTeX over MathJax

**Decision**: Bundle KaTeX for client-side rendering.

**Why**: 7x smaller (~280KB vs ~2MB), synchronous rendering (no flash of unrendered math), sufficient TeX coverage for markdown use cases. See proposal for full comparison.

### 3. Bundle KaTeX as separate served assets, not inlined

**Decision**: Serve KaTeX JS, CSS, and fonts from the local warp server rather than inlining them into the HTML template.

**Why**: KaTeX includes ~40 font files (WOFF2). Inlining these as base64 in the HTML template would bloat it massively (~400KB+ of base64 fonts). Instead:
- Add KaTeX dist files to `asset/katex/` (embedded via `rust-embed` or a second embed struct)
- Serve them from the warp server at `/katex/*`
- Reference them in the template as `<link href="/katex/katex.min.css">` etc.
- The HTML template stays lean, and fonts load naturally via CSS `@font-face` URLs

```
asset/
├── theme1/
│   └── template.html        (stays lean, references /katex/*)
└── katex/
    ├── katex.min.js          (~75KB gzipped)
    ├── katex.min.css          (~25KB)
    └── fonts/
        ├── KaTeX_Main-Regular.woff2
        ├── KaTeX_Math-Italic.woff2
        └── ... (~40 font files, ~200KB total)
```

**Alternative considered**: Inline everything via the inliner tool. Rejected — font inlining produces enormous base64 strings.

### 4. Programmatic render call, not auto-render extension

**Decision**: Use `katex.render()` directly on `.math` spans instead of the auto-render extension.

**Why**: We already have well-defined spans from pulldown-cmark. The auto-render extension scans for delimiters in text content — unnecessary and could cause issues. Direct rendering is simpler and more reliable:

```javascript
function renderMath() {
  document.querySelectorAll('.math').forEach(function(el) {
    var math = el.textContent;
    var displayMode = el.classList.contains('math-display');
    try {
      katex.render(math, el, {
        displayMode: displayMode,
        throwOnError: false
      });
    } catch (e) {
      el.textContent = math; // fallback: show raw TeX
    }
  });
}
```

This also means we don't need `auto-render.min.js` — one less file.

### 5. Re-render after content reload

**Decision**: After the innerHTML injection in the `glib::timeout_add_local` callback, append a `renderMath()` call to the JS string.

**Why**: The content reload replaces `.section` innerHTML, which destroys the KaTeX-rendered elements. The math spans come back as raw text. A `renderMath()` call after the injection re-renders them.

```rust
// In the reload callback:
let js = format!(
    "document.querySelector('.section').innerHTML = `{}`; if(typeof renderMath==='function')renderMath();",
    escaped
);
```

The `typeof` guard ensures no error if KaTeX isn't loaded (e.g., math disabled).

### 6. Conditional loading via template placeholder

**Decision**: Add a `#{MATH_SCRIPTS}` placeholder in the template. When math is enabled, replace it with the KaTeX `<link>` and `<script>` tags + the `renderMath` function. When disabled, replace with empty string.

**Why**: Avoids loading KaTeX JS/CSS/fonts when math is disabled. The template stays the same, only the placeholder replacement changes.

**Alternative considered**: Always load KaTeX, just skip `ENABLE_MATH`. Rejected — wastes ~280KB of asset loading for users who don't need math.

## Risks / Trade-offs

- **[Risk] KaTeX fonts not loading from warp server** → Mitigation: serve fonts with correct MIME types (woff2 → `font/woff2`). Test with `@font-face` URLs.
- **[Risk] Math in code blocks** → pulldown-cmark correctly ignores `$` inside code fences and inline code. No risk of false positive math rendering.
- **[Risk] Binary size increase ~280KB** → Accepted trade-off for math support. Still well under 1MB total for all embedded assets.
- **[Trade-off] Math in headings shows as raw text in TOC** → Acceptable. Could be improved later by parsing InlineMath events in heading extraction if needed.
- **[Trade-off] KaTeX ~95% TeX coverage vs MathJax ~99%** → Acceptable for markdown math use case. Missing features are obscure.
