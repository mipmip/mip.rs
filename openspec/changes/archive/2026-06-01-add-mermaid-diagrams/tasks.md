## 1. Bundle Mermaid assets

- [x] 1.1 Download `mermaid.min.js` from the latest Mermaid release into `asset/mermaid/`
- [x] 1.2 Add a `MermaidAsset` rust-embed struct (or extend existing asset serving) for `asset/mermaid/`
- [x] 1.3 Add warp route to serve `/mermaid/*` files from embedded Mermaid assets with correct MIME types
- [x] 1.4 Verify `mermaid.min.js` is served correctly by checking the route returns valid JS

## 2. Template and JS integration

- [x] 2.1 Add `#{MERMAID_SCRIPTS}` placeholder to `template-src.html`
- [x] 2.2 Implement `renderMermaid()` JS function: transform `<pre><code class="language-mermaid">` into `<pre class="mermaid">`, call `mermaid.run()`
- [x] 2.3 Add Mermaid initialization with `startOnLoad: false` and theme detection based on document class
- [x] 2.4 Call `renderMermaid()` on initial page load (after Mermaid script loads)
- [x] 2.5 Run `make compthemes` to compile updated template

## 3. Reload and theme integration

- [x] 3.1 Append `renderMermaid()` call to the innerHTML injection JS in the `glib::timeout_add_local` callback (with `typeof` guard)
- [x] 3.2 On theme change (system dark/light switch), re-initialize Mermaid with the new theme and re-render diagrams
- [x] 3.3 Verify diagrams re-render after file change without full page reload

## 4. Config and CLI

- [x] 4.1 Add `mermaid: Option<bool>` field to `Config` struct — default `true`
- [x] 4.2 Add `--no-mermaid` CLI flag (switch) to disable Mermaid rendering
- [x] 4.3 Merge logic: CLI `--no-mermaid` overrides config, fallback to `true`
- [x] 4.4 Pass `mermaid_enabled` to `build_html()` — conditionally replace `#{MERMAID_SCRIPTS}` with Mermaid script tag or empty string
- [x] 4.5 Add `mermaid` to the `--initconf` default config template with documentation

## 5. Automated tests

- [x] 5.1 Integration test: markdown with ` ```mermaid ` code block produces `<pre><code class="language-mermaid">` in HTML output (pulldown-cmark default behavior, sanity check)
- [x] 5.2 Integration test: `build_html()` with mermaid enabled includes Mermaid script tag, with mermaid disabled does not
- [x] 5.3 Integration tests in `tests/config_test.rs` — `mermaid` config key: true, false, missing (defaults to true)
- [x] 5.4 Integration test in `tests/server_test.rs` — `/mermaid/mermaid.min.js` route serves content with correct MIME type

## 6. Example document

- [x] 6.1 Create `examples/with-mermaid.md` with flowchart, sequence diagram, Gantt chart, class diagram, and pie chart examples

## 7. Manual verification

- [x] 7.1 Test flowchart diagram renders correctly
- [x] 7.2 Test sequence diagram renders correctly
- [ ] 7.3 Test diagram re-renders on file save
- [ ] 7.4 Test `--no-mermaid` flag shows raw source as code block
- [ ] 7.5 Test config `mermaid = false` disables rendering
- [ ] 7.6 Test dark mode — diagrams use dark theme colors
- [ ] 7.7 Test theme switch — diagrams re-render with new colors (fixed: renderMermaid now resets data-processed and restores source before re-rendering)
- [ ] 7.8 Test malformed Mermaid syntax shows error message (not crash)
- [ ] 7.9 Test document with both math and Mermaid renders both correctly
<!-- Note: Section 7 tasks require manual GUI testing -->
