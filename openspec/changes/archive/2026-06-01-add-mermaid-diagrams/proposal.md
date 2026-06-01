## Why

Mermaid diagrams are widely used in markdown for flowcharts, sequence diagrams, Gantt charts, and more. Currently mip.rs renders ` ```mermaid ` code blocks as plain text. Supporting Mermaid completes mip's coverage of the most common markdown extensions (alongside GFM and TeX math).

Bean: [mip.rs-rc19](/home/pim/cLinden/mip.rs/.beans/mip.rs-rc19--add-mermaid-support.md)

## What Changes

- Bundle `mermaid.min.js` (~1.5MB) into the binary via `rust-embed`, served from the local warp server at `/mermaid/*`
- Add a JS shim that transforms `<pre><code class="language-mermaid">` into `<pre class="mermaid">` and calls `mermaid.run()` on page load and content reload
- Initialize Mermaid with the correct theme (light/dark) based on the document's current theme class
- Re-initialize Mermaid theme on system theme change
- Add `mermaid` config option (bool, default `true`) and `--no-mermaid` CLI flag
- When mermaid is disabled, ` ```mermaid ` blocks render as regular code blocks (syntax-highlighted text)
- Add a `#{MERMAID_SCRIPTS}` template placeholder, conditionally replaced like `#{MATH_SCRIPTS}`

### Why bundled JS over CLI (mmdc)

- **Zero install friction** — mmdc requires npm + Puppeteer + Chromium (~300MB). Antithetical to mip's "fast and bloatless" philosophy.
- **Instant re-render** — JS rendering takes ~50ms vs 1-3s per diagram for CLI. Critical for the "preview while editing in vim" workflow.
- **Offline by default** — bundled in binary, no external dependencies.
- **Binary size trade-off** — +1.5MB (current ~5MB → ~6.5MB) is acceptable for full Mermaid support.

### Why JS shim over Rust-side HTML rewriting

- pulldown-cmark emits `<pre><code class="language-mermaid">` for fenced code blocks — this is standard and correct
- A small JS function transforms these into `<pre class="mermaid">` before `mermaid.run()` — 5 lines of JS
- No changes needed to the Rust markdown pipeline, heading extractor, or event processing
- Same pattern as KaTeX: Rust produces correct HTML, JS renders it client-side

## Capabilities

### New Capabilities
- `mermaid-diagrams`: Render Mermaid diagram code blocks as interactive SVG diagrams in the preview

### Modified Capabilities
- `markdown-rendering`: ` ```mermaid ` code blocks gain visual rendering (no parsing change — pulldown-cmark already handles fenced code blocks)

## Impact

- **Code**: `view.rs` (append `renderMermaid()` to reload JS), `build_html()` in `markdown.rs` (new placeholder replacement), `config.rs` (new field), `main.rs` (new CLI flag)
- **Template**: `template-src.html` gains `#{MERMAID_SCRIPTS}` placeholder
- **Assets**: New `asset/mermaid/mermaid.min.js` (~1.5MB)
- **Server**: New warp route for `/mermaid/*`
- **Binary size**: +~1.5MB
- **Dependencies**: None in Cargo.toml — Mermaid is a JS asset
