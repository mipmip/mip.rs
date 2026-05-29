## markdown-rendering (modified)

Changes to the existing markdown rendering capability.

### Requirements

#### Anchor IDs on headings
- MUST add `id` attributes to all heading HTML elements (h1–h6)
- Anchor IDs MUST be deterministic: slugified heading text (lowercase, non-alphanumeric → `-`, collapse consecutive dashes, trim leading/trailing dashes)
- Duplicate heading text MUST produce unique IDs via `-1`, `-2` suffix
- MUST NOT change any other rendering behavior (frontmatter, video embeds, GFM extensions)

#### API change
- `md_to_html_body_with_toc()` MUST return `(String, Vec<TocEntry>)` where `TocEntry` contains `level`, `title`, and `anchor_id`
- Existing `md_to_html_body()` MUST continue to work (can delegate to the new function and discard TOC data)
- `build_html()` and `to_html()` MUST NOT change signature (they don't need TOC data)
