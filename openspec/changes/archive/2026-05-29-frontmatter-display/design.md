## Context

`gray_matter` already parses frontmatter into `result.data` (a `Pod` enum) but this data is discarded. The rendering pipeline is: `to_html` → `to_file` → pulldown-cmark → template substitution. The `md_to_html_body` public function also strips frontmatter independently.

## Goals / Non-Goals

**Goals:**
- Add `--frontmatter` flag to show frontmatter as a styled key-value table
- Default remains hidden (no behavioral change without the flag)
- Handle scalar, array, and nested values gracefully

**Non-Goals:**
- Per-document frontmatter display config (in-frontmatter toggle)
- Custom frontmatter styling/themes
- Frontmatter editing

## Decisions

### Render as HTML table, not code block

**Choice**: Generate an HTML `<table>` with a `frontmatter` CSS class, prepended before the markdown body.

**Rationale**: A table gives structured, readable output. A raw YAML code block would just be showing the source — not useful for previewing blog metadata. The table can be styled via the existing theme CSS.

**Alternatives considered**:
- YAML code fence: too raw, defeats the purpose of "display"
- Definition list (`<dl>`): semantically nice but less visually structured

### Thread `show_frontmatter: bool` through function signatures

**Choice**: Add a `show_frontmatter` parameter to `to_html`, `to_file`, and `md_to_html_body`.

**Rationale**: Minimal change. These functions already accept multiple parameters. A config struct could come later if more options accumulate.

### Value rendering strategy

- **Strings/numbers/booleans**: render directly
- **Arrays**: comma-separated inline
- **Objects/nested**: render as inline YAML string (keep it simple, avoid recursive table)

`gray_matter`'s `Pod` type provides `.as_string()`, `.as_vec()`, and `.as_hashmap()` for this.

## Risks / Trade-offs

- [Pod API surface] → gray_matter's `Pod` enum may not cover all edge cases. Mitigation: fall back to debug format for unknown variants.
- [CSS styling] → The frontmatter table needs to look decent in the existing theme. Mitigation: add minimal inline CSS or a `.frontmatter` class with basic styling in the template.
