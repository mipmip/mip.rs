## mermaid-diagrams

Render Mermaid diagram code blocks as interactive SVG diagrams in the preview.

### Requirements

#### Diagram support
- MUST render all standard Mermaid diagram types: flowcharts, sequence diagrams, Gantt charts, class diagrams, state diagrams, ER diagrams, pie charts, journey maps, git graphs
- MUST render diagrams as inline SVG in the document
- MUST show Mermaid's default error message for malformed diagram syntax (not crash or blank)

#### Rendering
- MUST render diagrams client-side in the WebView using Mermaid.js
- MUST re-render diagrams after content reload (file change) without full page reload
- MUST NOT show raw Mermaid source as a flash before rendering completes

#### Theming
- MUST initialize Mermaid with the correct theme matching the document's light/dark mode
- MUST re-render diagrams with updated colors when the theme changes (system theme switch)
- Diagrams MUST be legible in both light and dark themes

#### Offline
- MUST work fully offline — Mermaid.js bundled in the binary
- MUST serve Mermaid assets from the local warp server
- MUST NOT reference any external CDN or network resource

#### Configuration
- MUST support `mermaid` key in `~/.config/miprs/config.toml` (bool, default `true`)
- MUST support `--no-mermaid` CLI flag to disable Mermaid rendering
- MUST NOT load Mermaid.js when mermaid is disabled
- When disabled, ` ```mermaid ` blocks MUST render as regular code blocks (plain text)

#### Performance
- Mermaid.js asset SHOULD add no more than ~2MB to the binary size
- Diagrams with <20 nodes SHOULD render within 200ms
