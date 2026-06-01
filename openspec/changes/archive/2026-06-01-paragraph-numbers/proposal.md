## Why

Technical and academic documents benefit from hierarchical section numbers (1., 1.1, 1.1.1). mip currently renders headings without numbering, making it harder to reference sections. The numbering should appear in both the rendered preview and the TOC views.

Bean: mip.rs-6eev

## What Changes

- Add `paragraph_numbers` config setting (bool, default false)
- Add `paragraph_numbers_start` config setting (integer, default 1) — which heading level starts the numbering (e.g. 2 means H2 = "1.", H3 = "1.1")
- Compute hierarchical section numbers from TocEntry list
- Inject numbers into HTML heading text in the rendered preview
- Show numbers in both sidetoc and quicktoc TreeView displays
- Update `--initconf` template with new settings

## Capabilities

### New Capabilities
- `paragraph-numbers`: Hierarchical section numbering for headings in preview and TOC

### Modified Capabilities
- `config`: Add `paragraph_numbers` and `paragraph_numbers_start` settings

## Impact

- `src/markdown.rs`: add numbering computation, inject into HTML headings and TocEntry titles
- `src/config.rs`: add two new fields + accessors
- `src/view.rs`: pass numbering settings through to markdown functions
- `src/main.rs`: pass config values through
