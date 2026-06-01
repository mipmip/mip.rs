## Why

Markdown files authored for Hugo or Quarto contain dialect-specific syntax (Hugo shortcodes, Quarto divs/attributes) that pulldown-cmark doesn't understand. Currently these constructs appear as raw broken text in the preview. They should instead render as subtle, non-distracting styled blocks so the user can see them without confusion.

Beans: mip.rs-k6uh (display quarto dialect), plus Hugo dialect support.

## What Changes

- Add a preprocessing step before pulldown-cmark that detects Hugo and Quarto dialect syntax
- Replace detected constructs with styled HTML blocks that pulldown-cmark passes through
- Hugo: `{{< shortcode >}}` and `{{% shortcode %}}` (paired and self-closing)
- Quarto: `:::{.class}` fenced divs, `{.class}` attribute syntax, callout blocks
- Add CSS styling for dialect blocks (subtle background, monospace, labeled)

## Capabilities

### New Capabilities
- `hugo-shortcode-rendering`: Detect and render Hugo shortcodes as styled blocks
- `quarto-syntax-rendering`: Detect and render Quarto-specific syntax as styled blocks
- `dialect-block-styling`: CSS styling for dialect-specific rendered blocks

### Modified Capabilities

## Impact

- `src/markdown.rs` — new preprocessing function inserted before pulldown-cmark parsing
- `theme_src/theme1/style.css` — new CSS rules for dialect blocks
