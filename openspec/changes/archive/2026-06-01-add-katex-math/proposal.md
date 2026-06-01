## Why

Markdown without math support is a limitation for technical writing. Inline TeX (`$x^2$`) and display math (`$$\sum_{i=0}^n$$`) are expected in any serious markdown previewer. Currently mip.rs renders dollar-sign math as plain text.

Bean: [mip.rs-w5we](/home/pim/cLinden/mip.rs/.beans/mip.rs-w5we--add-simple-tex-support.md)
GitHub: issue #9

## What Changes

- Enable `ENABLE_MATH` in pulldown-cmark options to parse `$...$` (inline) and `$$...$$` (display) into `Event::InlineMath` / `Event::DisplayMath`
- Handle these events in the heading-collecting iterator to emit `<span class="math-inline">...</span>` and `<div class="math-display">...</div>` HTML wrappers
- Bundle KaTeX (JS + CSS + fonts, ~280KB) into the HTML template, loaded offline from the embedded assets
- Add a KaTeX auto-render call in `bridge.js` that renders `.math-inline` and `.math-display` elements on page load and content reload
- Add a `math` config option (bool, default `true`) to enable/disable math rendering
- Add `--no-math` CLI flag to disable math rendering

### Why KaTeX over MathJax

- 7x smaller bundle (~280KB vs ~2MB) — aligns with mip's "fast and bloatless" philosophy
- Synchronous rendering — no flash of unrendered TeX on reload
- Covers ~95% of TeX math, which is everything you'd write in markdown
- MathJax's extra coverage (complex `\newcommand`, `\definecolor`) is not relevant for markdown math

### Why pulldown-cmark math over JS auto-render

- Proper AST-level parsing, not regex-based dollar-sign detection
- No false positives with dollar signs in code blocks or inline code
- pulldown-cmark 0.12 already has `ENABLE_MATH` — we just need to enable it
- The math events flow through our existing event processing pipeline

## Capabilities

### New Capabilities
- `math-rendering`: TeX math support via pulldown-cmark math parsing + KaTeX rendering in the WebView

### Modified Capabilities
- `markdown-rendering`: Enable `ENABLE_MATH` option, handle `InlineMath`/`DisplayMath` events

## Impact

- **Code**: `markdown.rs` (enable math option, handle math events), `bridge.js` (KaTeX render call on load/reload)
- **Template**: `template-src.html` gains KaTeX CSS/JS includes
- **Assets**: New `theme_src/theme1/katex/` directory with KaTeX dist files (JS, CSS, fonts)
- **Build**: `make compthemes` inlines KaTeX into the template; binary grows ~280KB
- **Config**: New `math` key in `config.toml` (default `true`)
- **Dependencies**: None in Cargo.toml — KaTeX is a JS/CSS asset, not a Rust crate
- **Nix**: May need to add KaTeX source to flake.nix or vendor it
