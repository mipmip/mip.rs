## Why

mip strips YAML frontmatter before rendering, which is correct for preview. But sometimes you want to see the metadata (title, date, tags) rendered nicely — e.g. when previewing blog posts. There's no way to toggle this.

Bean: mip.rs-yzkg

## What Changes

- Add `--frontmatter` CLI flag to show frontmatter as a styled HTML table above the content
- Default remains hidden (current behavior unchanged)
- Thread the flag through `to_html` / `to_file` / `md_to_html_body`
- Render `gray_matter` parsed data as key-value table; arrays comma-joined, nested values YAML-stringified

## Capabilities

### New Capabilities
- `frontmatter-display`: Frontmatter rendering as styled HTML table, controlled by CLI flag

### Modified Capabilities
- `cli`: Adding `--frontmatter` switch to the CLI

## Impact

- `src/main.rs`: add `--frontmatter` flag to `Cli` struct, pass to markdown functions
- `src/markdown.rs`: accept frontmatter flag, render `result.data` as HTML table when enabled
