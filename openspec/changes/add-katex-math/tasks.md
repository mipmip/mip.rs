## 1. pulldown-cmark math parsing

- [ ] 1.1 Add `Options::ENABLE_MATH` to parser options in `md_to_html_body_with_toc()`
- [ ] 1.2 Verify `InlineMath`/`DisplayMath` events pass through `extract_headings_and_inject_ids` untouched
- [ ] 1.3 Unit tests: inline math `$x^2$` produces `<span class="math math-inline">`, display math `$$...$$` produces `<span class="math math-display">`
- [ ] 1.4 Unit test: math inside code blocks/inline code is NOT rendered as math
- [ ] 1.5 Unit test: math inside headings — heading text in TOC is plain (no raw TeX)

## 2. Bundle KaTeX assets

- [ ] 2.1 Download KaTeX release (katex.min.js, katex.min.css, fonts/*.woff2) into `asset/katex/`
- [ ] 2.2 Add a `KatexAsset` rust-embed struct in an appropriate module (or extend server.rs) for `asset/katex/`
- [ ] 2.3 Add warp route to serve `/katex/*` files from the embedded KaTeX assets with correct MIME types
- [ ] 2.4 Verify fonts load correctly by checking `@font-face` URLs resolve through the warp server

## 3. Template and JS integration

- [ ] 3.1 Add `#{MATH_SCRIPTS}` placeholder to `template-src.html` in the `<head>` section
- [ ] 3.2 Implement `renderMath()` JS function: iterate `.math` spans, call `katex.render()` with `displayMode` based on class
- [ ] 3.3 Call `renderMath()` on initial page load (at end of body, after KaTeX script loads)
- [ ] 3.4 Run `make compthemes` to compile updated template

## 4. Reload integration

- [ ] 4.1 Append `renderMath()` call to the innerHTML injection JS in the `glib::timeout_add_local` callback (with `typeof` guard)
- [ ] 4.2 Verify math re-renders after file change without full page reload

## 5. Config and CLI

- [ ] 5.1 Add `math: Option<bool>` field to `Config` struct — default `true`
- [ ] 5.2 Add `--no-math` CLI flag (switch) to disable math rendering
- [ ] 5.3 Merge logic: CLI `--no-math` overrides config, fallback to `true`
- [ ] 5.4 Pass `math_enabled` to `build_html()` — conditionally replace `#{MATH_SCRIPTS}` with KaTeX tags or empty string
- [ ] 5.5 Conditionally enable `ENABLE_MATH` in parser options based on `math_enabled`

## 6. Automated tests

- [ ] 6.1 Integration tests in `tests/markdown_test.rs` — `md_to_html_body` with math: inline math produces correct spans, display math produces correct spans
- [ ] 6.2 Integration tests — math in fenced code blocks stays as plain text
- [ ] 6.3 Integration tests — `build_html()` with math enabled includes KaTeX script tags, with math disabled does not
- [ ] 6.4 Integration tests in `tests/config_test.rs` — `math` config key: true, false, missing (defaults to true)
- [ ] 6.5 Integration test in `tests/server_test.rs` — `/katex/katex.min.js` route serves content with correct MIME type

## 7. Manual verification

- [ ] 7.1 Test inline math renders correctly: `$x^2 + y^2 = z^2$`
- [ ] 7.2 Test display math renders correctly: `$$\int_0^\infty e^{-x} dx = 1$$`
- [ ] 7.3 Test math in code blocks is NOT rendered
- [ ] 7.4 Test math re-renders on file save
- [ ] 7.5 Test `--no-math` flag disables math (shows raw spans)
- [ ] 7.6 Test config `math = false` disables math
- [ ] 7.7 Test fonts render correctly (no missing glyphs or squares)
- [ ] 7.8 Test dark mode — math renders legibly on dark background
