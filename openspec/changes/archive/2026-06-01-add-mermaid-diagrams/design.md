## Context

mip.rs renders markdown in a GTK4 WebView. Assets are embedded in the binary via `rust-embed` and served by a local warp server. The KaTeX math feature (in progress) establishes the pattern: embed JS/CSS assets, serve from warp, render client-side, re-render after innerHTML swap.

pulldown-cmark renders ` ```mermaid ` fenced code blocks as `<pre><code class="language-mermaid">...diagram source...</code></pre>`. This is standard HTML — no custom parsing needed.

Mermaid.js 10+ expects `<pre class="mermaid">` elements (no `<code>` wrapper). A small JS shim bridges the gap.

## Goals / Non-Goals

**Goals:**
- Render all Mermaid diagram types (flowcharts, sequence, Gantt, class, state, ER, pie, etc.)
- Diagrams render offline, bundled in the binary
- Diagrams re-render on file change without full page reload
- Diagrams respect light/dark theme
- Config option to disable Mermaid rendering
- When disabled, diagram source shows as a regular code block

**Non-Goals:**
- Mermaid editor/live-edit integration
- Exporting diagrams as standalone SVG/PNG files
- Syntax highlighting of Mermaid source when rendering is disabled (plain text is fine)
- Supporting other diagram languages (PlantUML, D2, etc.) — separate features

## Decisions

### 1. Bundle mermaid.min.js, serve via warp

**Decision**: Embed `mermaid.min.js` (~1.5MB) in `asset/mermaid/` via `rust-embed`. Serve at `/mermaid/*` from the warp server. Reference in the template as `<script src="/mermaid/mermaid.min.js">`.

**Why**: Same pattern as KaTeX. Keeps the HTML template lean, avoids base64-inlining a 1.5MB file. Offline by default.

**Alternative considered**: CDN loading. Rejected — violates the offline-first requirement.

**Alternative considered**: CLI rendering via `mmdc`. Rejected — requires npm + Puppeteer + Chromium (~300MB external dependency), 1-3s render time per diagram, blocks the file watcher. See proposal for full comparison.

### 2. JS shim to transform code blocks, no Rust pipeline changes

**Decision**: A JS function `renderMermaid()` transforms `<pre><code class="language-mermaid">` into `<pre class="mermaid">` (removing the `<code>` wrapper, moving text content up), then calls `mermaid.run()`.

```javascript
function renderMermaid() {
  document.querySelectorAll('code.language-mermaid').forEach(function(code) {
    var pre = code.parentElement;
    pre.classList.add('mermaid');
    pre.textContent = code.textContent;
  });
  mermaid.run({ querySelector: '.mermaid' });
}
```

**Why**: Zero changes to the Rust markdown pipeline. pulldown-cmark's output is correct — we just need Mermaid to find and render it. The 5-line shim is simpler and more maintainable than intercepting CodeBlock events in the Rust iterator or post-processing HTML with string manipulation.

**Alternative considered**: Intercept `Event::Start(Tag::CodeBlock(Fenced("mermaid")))` in `extract_headings_and_inject_ids` and emit `<div class="mermaid">` instead. Rejected — adds complexity to the event processing for no benefit. The JS shim achieves the same result with less code and less risk.

### 3. Theme-aware initialization

**Decision**: Initialize Mermaid with the current theme on load. Re-initialize when the theme changes.

```javascript
mermaid.initialize({
  startOnLoad: false,
  theme: document.documentElement.classList.contains('dark') ? 'dark' : 'default'
});
```

The existing system theme change detection in `glib::timeout_add_local` already injects JS to update `document.documentElement.className`. After that class change, we re-run `renderMermaid()` to pick up the new theme.

**Why**: Mermaid generates inline SVGs with hard-coded colors. If the theme changes, the SVGs need to be re-generated with the new color scheme.

### 4. Re-render after content reload

**Decision**: Append `renderMermaid()` to the innerHTML injection JS (same pattern as KaTeX's `renderMath()`). Use `typeof` guard.

```rust
let js = format!(
    "document.querySelector('.section').innerHTML = `{}`; \
     if(typeof renderMath==='function')renderMath(); \
     if(typeof renderMermaid==='function')renderMermaid();",
    escaped
);
```

**Why**: The innerHTML swap destroys rendered SVGs and replaces them with the raw `<pre><code>` blocks from the new markdown. `renderMermaid()` transforms and renders them again.

### 5. Conditional loading via template placeholder

**Decision**: Add `#{MERMAID_SCRIPTS}` placeholder in the template. When mermaid is enabled, replace with `<script src="/mermaid/mermaid.min.js">` + the `renderMermaid` function + initialization. When disabled, replace with empty string.

**Why**: Same pattern as `#{MATH_SCRIPTS}`. Avoids loading 1.5MB of JS when mermaid is disabled.

### 6. Mermaid re-rendering needs element cleanup

**Decision**: Before transforming code blocks, remove any previously rendered `.mermaid` SVGs to avoid duplicates. Mermaid adds a `data-processed` attribute to rendered elements — skip those or clear them.

**Why**: On content reload, the innerHTML swap replaces everything with fresh `<pre><code>` blocks. But if the same diagram appears unchanged, Mermaid might encounter stale state. The cleanest approach: `renderMermaid()` always works on fresh DOM from the innerHTML swap, so no cleanup is needed — the swap itself is the cleanup.

Actually, this is simpler than it seems: since we replace the entire `.section` innerHTML, all previous Mermaid SVGs are destroyed. The fresh `<pre><code>` elements are then transformed and rendered. No cleanup needed.

## Risks / Trade-offs

- **[Risk] Binary size +1.5MB** → Accepted trade-off for full Mermaid support. Binary goes from ~5MB to ~6.5MB. Still reasonable for a desktop tool.
- **[Risk] Mermaid.js rendering errors on malformed diagrams** → Mitigation: Mermaid shows an error message in the SVG by default. `renderMermaid()` can catch errors and leave the code block as-is.
- **[Risk] Theme change re-renders all diagrams** → For most documents (<10 diagrams) this is instant. For diagram-heavy documents, there may be a brief flicker. Acceptable.
- **[Risk] Large diagrams cause slow rendering** → Mermaid.js renders most diagrams in <100ms. Very large diagrams (100+ nodes) can take 500ms+. This is a Mermaid.js limitation, not something we can fix.
- **[Trade-off] Mermaid.js version locked at bundle time** → Users can't update Mermaid independently. We update it when we update the mip binary. This is fine — same as KaTeX.
