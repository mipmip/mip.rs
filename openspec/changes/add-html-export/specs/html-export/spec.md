## html-export

Export the current preview as a self-contained HTML file.

### Requirements

#### Output
- MUST produce a single self-contained HTML file (no external asset dependencies)
- Exported file MUST work in any modern browser without mip running
- Exported file MUST NOT require an internet connection
- Exported file MUST NOT contain any `<script>` tags
- Exported file MUST NOT contain references to `localhost` URLs
- Exported file MUST contain a `<!DOCTYPE html>` declaration

#### Content preservation
- MUST preserve rendered KaTeX math as HTML (not raw TeX)
- MUST preserve rendered Mermaid diagrams as inline SVGs (not source code)
- MUST preserve document styling (CSS)
- MUST preserve the current light/dark theme as-is
- MUST preserve heading anchor IDs
- MUST preserve section numbers if enabled

#### DOM capture
- MUST capture the rendered DOM via `document.documentElement.outerHTML`
- MUST strip all `<script>` tags from the captured DOM
- MUST strip `<link>` tags referencing localhost URLs
- MUST strip the mip header div (`<div id="header">`)

#### File handling
- MUST expand tilde (`~`) in the output path
- MUST resolve relative paths against the current working directory
- MUST create parent directories if they don't exist
- MUST NOT crash on file write errors — print warning to stderr
- MUST NOT write an empty file if DOM capture fails

#### Trigger
- MUST be available as `export_html <path>` command in the command bar
- MUST support tab completion of the command name
- Path argument MUST support tilde expansion and relative paths
- MUST print warning if no path argument is provided
