## markdown-rendering (modified)

Changes to the existing markdown rendering capability.

### Requirements

#### Math parsing option
- MUST add `Options::ENABLE_MATH` to pulldown-cmark parser options when math is enabled
- MUST NOT add `Options::ENABLE_MATH` when math is disabled
- MUST NOT change any other parsing behavior (headings, frontmatter, GFM extensions, video embeds)

#### Math in headings
- Math events inside headings MUST pass through to HTML output (rendered by KaTeX)
- Math inside headings MUST NOT appear as raw TeX in TOC entry titles
- TOC titles for headings containing math SHOULD show the plain text portions only

#### Template changes
- `build_html()` MUST support a `#{MATH_SCRIPTS}` placeholder
- When math is enabled, `#{MATH_SCRIPTS}` MUST be replaced with KaTeX `<link>` and `<script>` tags
- When math is disabled, `#{MATH_SCRIPTS}` MUST be replaced with empty string
