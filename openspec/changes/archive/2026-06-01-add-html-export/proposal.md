## Why

mip.rs renders beautiful markdown previews, but there's no way to share the result. For sharing with collaborators or prototyping, users need a portable HTML file they can open in any browser — with math, diagrams, and styling intact — without mip running.

Bean: [mip.rs-iva6](/home/pim/cLinden/mip.rs/.beans/mip.rs-iva6--export-html.md)

## What Changes

- Add an `export_html` command that captures the WebView's rendered DOM and writes it to a self-contained HTML file
- The export captures the fully rendered state: KaTeX math already as HTML spans, Mermaid diagrams already as inline SVGs, CSS already applied
- The exported file requires no JavaScript — it's pure HTML + CSS + inline SVG
- Triggered via command bar (`:export_html path/to/file.html`) or keybinding

### Why DOM capture over template rebuilding

Two approaches were considered:

| | **DOM capture** | **Template rebuild (inline everything)** |
|---|---|---|
| Output size | ~50-200KB | ~2-3MB (KaTeX + Mermaid JS) |
| JS in output | None | KaTeX + Mermaid must execute |
| Math rendering | Already baked in as HTML | Requires KaTeX JS to run |
| Diagrams | Already baked in as SVG | Requires Mermaid JS to run |
| Font handling | Fonts already rendered | Must base64-encode ~40 font files |
| WYSIWYG | Exact match to preview | Re-renders, may differ |
| Theme | Exports current theme | Must decide at export time |
| Complexity | Low — one JS call + file write | High — asset inlining, CSS rewriting |

DOM capture wins on every dimension: smaller output, no JS dependencies, exact WYSIWYG match, simpler implementation.

### How it works

```
`:export_html ~/shared/report.html`
     │
     ▼
  webview.evaluate_javascript(
    "document.documentElement.outerHTML"
  )
     │
     ▼
  Callback receives full rendered HTML string
  - KaTeX math → already rendered to styled <span>s
  - Mermaid diagrams → already inline <svg>s
  - CSS → already in <style> tags
     │
     ▼
  Post-process:
  - Strip mip-specific scripts (seed polling, bridge.js)
  - Strip localhost asset references
  - Ensure CSS is self-contained
  - Wrap in proper <!DOCTYPE html>
     │
     ▼
  Write to specified file path
```

## Capabilities

### New Capabilities
- `html-export`: Export the current preview as a self-contained HTML file via `export_html` command

### Modified Capabilities
- `command-mode`: New `export_html` command with path argument and tilde expansion

## Impact

- **Code**: `view.rs` (new `export_html` command in `execute_command()`, async JS callback for DOM capture, file write), `command.rs` (add `export_html` to command list for tab completion)
- **Dependencies**: None
- **Config**: No new config options — command-only feature
- **Binary size**: No change — no new assets
