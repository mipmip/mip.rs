## math-rendering

Render TeX math notation in the document preview using KaTeX.

### Requirements

#### Parsing
- MUST parse inline math delimited by `$...$` into `<span class="math math-inline">` elements
- MUST parse display math delimited by `$$...$$` into `<span class="math math-display">` elements
- MUST NOT parse dollar signs inside code blocks (fenced or indented) as math
- MUST NOT parse dollar signs inside inline code as math
- MUST use pulldown-cmark's `ENABLE_MATH` option for parsing (not regex)

#### Rendering
- MUST render math client-side in the WebView using KaTeX
- MUST render inline math inline with surrounding text
- MUST render display math as centered block elements
- MUST re-render math after content reload (file change) without full page reload
- MUST show raw TeX as fallback if KaTeX fails to render an expression
- MUST NOT show a flash of unrendered TeX on page load or reload
- SHOULD render legibly in both light and dark themes

#### Offline
- MUST work fully offline — KaTeX JS, CSS, and fonts bundled in the binary
- MUST serve KaTeX assets from the local warp server
- MUST NOT reference any external CDN or network resource

#### Configuration
- MUST support `math` key in `~/.config/miprs/config.toml` (bool, default `true`)
- MUST support `--no-math` CLI flag to disable math rendering
- MUST NOT load KaTeX JS/CSS/fonts when math is disabled
- MUST NOT enable `ENABLE_MATH` parser option when math is disabled (dollar signs stay as text)

#### Performance
- KaTeX assets SHOULD add no more than ~300KB to the binary size
- Math rendering SHOULD complete within 100ms for documents with <50 math expressions
