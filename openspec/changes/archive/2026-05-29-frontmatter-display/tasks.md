## 1. CLI flag

- [x] 1.1 Add `--frontmatter` switch to `Cli` struct in main.rs
- [x] 1.2 Pass `cli.frontmatter` bool to `markdown::to_html`

## 2. Render frontmatter as HTML table

- [x] 2.1 Add `show_frontmatter: bool` parameter to `to_html`, `to_file`, and `md_to_html_body`
- [x] 2.2 Implement `pod_to_html_value` helper to render Pod values (string, array comma-join, nested YAML-stringify)
- [x] 2.3 When `show_frontmatter` is true and `result.data` is present, generate `<table class="frontmatter">` HTML
- [x] 2.4 Prepend the frontmatter table HTML before the markdown body HTML

## 3. Styling

- [x] 3.1 Add `.frontmatter` table CSS to template.html (subtle styling, fits existing theme)

## 4. Verify

- [x] 4.1 `cargo build` succeeds
- [x] 4.2 `mip README.md` renders without frontmatter table (default behavior unchanged)
- [x] 4.3 `mip --frontmatter` on a file with YAML frontmatter shows key-value table
- [x] 4.4 `mip --frontmatter` on a file without frontmatter renders normally
- [x] 4.5 Array values render comma-separated
