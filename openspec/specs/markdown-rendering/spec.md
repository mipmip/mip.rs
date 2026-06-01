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

#### Mermaid code blocks
- MUST NOT change how pulldown-cmark parses or renders ` ```mermaid ` fenced code blocks — the default `<pre><code class="language-mermaid">` output is correct
- MUST NOT modify the Rust markdown pipeline for Mermaid support — all transformation happens client-side in JS
- Mermaid code blocks MUST NOT interfere with heading extraction, TOC generation, or other markdown features

#### Template changes
- `build_html()` MUST support a `#{MATH_SCRIPTS}` placeholder
- When math is enabled, `#{MATH_SCRIPTS}` MUST be replaced with KaTeX `<link>` and `<script>` tags
- When math is disabled, `#{MATH_SCRIPTS}` MUST be replaced with empty string
- `build_html()` MUST support a `#{MERMAID_SCRIPTS}` placeholder
- When mermaid is enabled, `#{MERMAID_SCRIPTS}` MUST be replaced with the Mermaid `<script>` tag, initialization code, and `renderMermaid()` function
- When mermaid is disabled, `#{MERMAID_SCRIPTS}` MUST be replaced with empty string

#### Reload JS
- The content reload JS string MUST call `renderMermaid()` after innerHTML replacement (with `typeof` guard)
- The theme change JS MUST re-initialize Mermaid and re-render diagrams when the theme class changes
