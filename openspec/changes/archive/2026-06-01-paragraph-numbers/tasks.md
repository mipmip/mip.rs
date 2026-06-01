## 1. Config

- [x] 1.1 Add `paragraph_numbers: Option<bool>` and `paragraph_numbers_start: Option<u8>` to Config struct
- [x] 1.2 Add accessor methods: `paragraph_numbers() -> bool` (default false), `paragraph_numbers_start() -> u8` (default 1, clamp 1-6)
- [x] 1.3 Update `default_config_template()` with new settings
- [x] 1.4 Config tests for new fields

## 2. Numbering computation

- [x] 2.1 Add `compute_section_numbers(entries: &[TocEntry], start_level: u8) -> Vec<String>` in markdown.rs
- [x] 2.2 Entries below start_level get empty string, others get hierarchical "1.2.3" format
- [x] 2.3 Reset deeper counters when a higher-level heading is encountered
- [x] 2.4 Tests: basic hierarchy, start_level=2, skipped levels, single heading, empty list

## 3. Inject into HTML

- [x] 3.1 Add `inject_section_numbers(html: &str, entries: &[TocEntry], numbers: &[String]) -> String` that finds `<h{n} id="...">` and prepends `<span class="section-number">N.N</span> `
- [x] 3.2 Add `.section-number` CSS to template.html (subtle styling, e.g. lighter color, small right margin)
- [x] 3.3 Call from `md_to_html_body_with_toc` when enabled

## 4. Inject into TOC entries

- [x] 4.1 When paragraph_numbers enabled, prepend number to `TocEntry.title` before returning from `md_to_html_body_with_toc`

## 5. Thread config through

- [x] 5.1 Pass `paragraph_numbers` and `paragraph_numbers_start` from config/CLI through main.rs to markdown functions
- [x] 5.2 Pass through view.rs to the md_to_html_body_with_toc calls (initial render + poll loop)

## 6. Verify

- [x] 6.1 `cargo build` succeeds
- [x] 6.2 `cargo test` passes
- [x] 6.3 With `paragraph_numbers = true`: headings show numbers in preview
- [x] 6.4 With `paragraph_numbers_start = 2`: H1 has no number, H2 starts at "1."
- [x] 6.5 TOC views show matching numbers
- [x] 6.6 Default (off): no numbers shown
- [x] 6.7 Numbers update correctly on live reload
