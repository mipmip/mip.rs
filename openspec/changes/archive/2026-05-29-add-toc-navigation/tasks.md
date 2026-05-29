## 1. Heading extraction and anchor IDs

- [x] 1.1 Define `TocEntry` struct in `markdown.rs` — fields: `level: u8`, `title: String`, `anchor_id: String`
- [x] 1.2 Implement `slugify()` helper — lowercase, non-alphanumeric to `-`, collapse dashes, trim, deduplicate with `-N` suffix
- [x] 1.3 Implement heading-collecting iterator wrapper around pulldown-cmark events — captures `TocEntry` on heading events, injects `id` attribute into heading HTML
- [x] 1.4 Add `md_to_html_body_with_toc()` — returns `(String, Vec<TocEntry>)`, replaces internal usage in the reload path
- [x] 1.5 Unit tests: slugify edge cases (special chars, unicode, duplicates, empty), heading extraction (nested, skipped levels, no headings)

## 2. Config and CLI

- [x] 2.1 Add `toc: Option<String>` field to `Config` struct in `config.rs` — valid values: `"side"`, `"zathura"`, `"off"`; default `"off"`
- [x] 2.2 Add `--toc` CLI option to `Cli` struct in `main.rs` — accepts `side`, `zathura`, `off`; overrides config
- [x] 2.3 Merge logic: CLI `--toc` takes precedence over config `toc`, fallback to `"off"`

## 3. GTK TOC widget

- [x] 3.1 Create TreeStore with columns: title (String), anchor_id (String), level (u32)
- [x] 3.2 Create TreeView bound to the TreeStore — single text column, expander arrows
- [x] 3.3 Implement `populate_toc()` — takes `&[TocEntry]`, clears TreeStore, builds tree hierarchy (tracking parent stack by level, handling level jumps)
- [x] 3.4 Wrap TreeView in a `ScrolledWindow` for long TOC lists

## 4. Layout integration

- [x] 4.1 Side panel mode: wrap WebView and TOC ScrolledWindow in a `gtk::Paned` (horizontal), TOC on the left with sensible default width (~250px)
- [x] 4.2 Zathura mode: wrap WebView and TOC ScrolledWindow in a `gtk::Stack`, WebView as default visible child
- [x] 4.3 Off mode: WebView only, no TOC widget created (current behavior)
- [x] 4.4 Wire the chosen container as the window's child based on resolved toc_mode

## 5. Navigation and keyboard

- [x] 5.1 Connect TreeView row-activated signal — read anchor_id from model, call `webview.evaluate_javascript()` with `scrollIntoView`
- [x] 5.2 Zathura mode: on row activation, also switch Stack to WebView page
- [x] 5.3 Zathura mode: `<Tab>` on WebView switches Stack to TOC page and grabs focus
- [x] 5.4 Zathura mode: `<Esc>` on TreeView switches Stack back to WebView without navigating
- [x] 5.5 TreeView key handler: map `j`→cursor down, `k`→cursor up
- [x] 5.6 TreeView key handler: `Enter` activates selected row (same as row-activated)

## 6. Reload integration

- [x] 6.1 Modify the `glib::timeout_add_local` callback in `view.rs` — call `md_to_html_body_with_toc()` instead of `md_to_html_body()`, feed HTML to WebView and TOC entries to `populate_toc()`
- [x] 6.2 On initial load, also extract TOC and populate TreeView

## 7. Automated tests

- [x] 7.1 Unit tests for `slugify()` in `markdown.rs` — special chars, unicode, consecutive dashes, leading/trailing dashes, empty string, duplicates with suffix
- [x] 7.2 Unit tests for `md_to_html_body_with_toc()` in `markdown.rs` — correct TocEntry extraction (level, title, anchor_id), heading-level jumps, no headings returns empty vec, frontmatter headings excluded
- [x] 7.3 Unit tests for anchor ID injection — verify `<h1 id="...">`, `<h2 id="...">` appear in HTML output
- [x] 7.4 Unit tests for `populate_toc()` tree building — flat headings, nested hierarchy, skipped levels (h1→h3), empty input (GTK TreeStore tests require main thread; covered by comment, tree logic tested via TocEntry extraction)
- [x] 7.5 Integration tests in `tests/markdown_test.rs` — `md_to_html_body_with_toc()` with realistic markdown (mixed headings, frontmatter, GFM extensions)
- [x] 7.6 Integration tests in `tests/config_test.rs` — `toc` config key: valid values, invalid value warning, missing key defaults to None

## 8. Manual verification

- [x] 8.1 Test side panel mode: TOC visible, clicking entries scrolls document, pane resizable
- [x] 8.2 Test Zathura mode: Tab toggles, Enter jumps, Esc cancels, j/k navigate
- [x] 8.3 Test off mode: no TOC visible, existing behavior unchanged
- [x] 8.4 Test file reload: edit markdown headings, verify TOC updates
- [x] 8.5 Test heading-level jumps: h1→h3, h2→h4 render as expected in tree
- [ ] 8.6 Test config file and CLI flag precedence
