## markdown-rendering (modified)

Changes to the existing markdown rendering capability.

### Requirements

#### Mermaid code blocks
- MUST NOT change how pulldown-cmark parses or renders ` ```mermaid ` fenced code blocks — the default `<pre><code class="language-mermaid">` output is correct
- MUST NOT modify the Rust markdown pipeline for Mermaid support — all transformation happens client-side in JS
- Mermaid code blocks MUST NOT interfere with heading extraction, TOC generation, or other markdown features

#### Template changes
- `build_html()` MUST support a `#{MERMAID_SCRIPTS}` placeholder
- When mermaid is enabled, `#{MERMAID_SCRIPTS}` MUST be replaced with the Mermaid `<script>` tag, initialization code, and `renderMermaid()` function
- When mermaid is disabled, `#{MERMAID_SCRIPTS}` MUST be replaced with empty string

#### Reload JS
- The content reload JS string MUST call `renderMermaid()` after innerHTML replacement (with `typeof` guard)
- The theme change JS MUST re-initialize Mermaid and re-render diagrams when the theme class changes
