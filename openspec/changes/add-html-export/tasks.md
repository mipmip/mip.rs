## 1. Command registration

- [ ] 1.1 Add `"export_html"` to `COMMANDS` list in `command.rs` for tab completion
- [ ] 1.2 Add `export_html` match arm in `execute_command()` in `view.rs`

## 2. DOM capture

- [ ] 2.1 In the `export_html` command handler: expand tilde on the path argument, clone path into closure
- [ ] 2.2 Call `webview.evaluate_javascript("document.documentElement.outerHTML", ...)` with a result-capturing callback
- [ ] 2.3 In the callback: extract the HTML string from `javascriptcore::Value` via `to_string()`

## 3. Post-processing

- [ ] 3.1 Implement `post_process_export(html: &str) -> String` function that:
  - Strips all `<script>...</script>` tags (including inline and src-referenced)
  - Strips `<link>` tags referencing localhost URLs
  - Strips the header div (`<div id="header">...</div>`)
  - Ensures `<!DOCTYPE html>` is present at the top
- [ ] 3.2 Unit tests for post-processing: scripts removed, links removed, header removed, document content preserved, DOCTYPE present

## 4. File writing

- [ ] 4.1 Create parent directories if they don't exist (`std::fs::create_dir_all`)
- [ ] 4.2 Write the post-processed HTML string to the specified file path
- [ ] 4.3 Handle errors gracefully: print warning to stderr if file write fails, don't crash

## 5. Edge cases

- [ ] 5.1 Handle missing path argument: print warning "export_html requires a file path"
- [ ] 5.2 Handle empty DOM result: print warning, don't write empty file
- [ ] 5.3 Handle JS evaluation error: print warning from the error result

## 6. Automated tests

- [ ] 6.1 Unit tests for `post_process_export()`: strips scripts, strips localhost links, strips header div, preserves content, adds DOCTYPE
- [ ] 6.2 Unit test: post-processing with no scripts/links (passthrough)
- [ ] 6.3 Unit test: post-processing preserves inline SVGs (Mermaid diagrams)
- [ ] 6.4 Unit test: post-processing preserves KaTeX-rendered math spans
- [ ] 6.5 Integration test: `export_html` appears in command list for tab completion

## 7. Documentation

- [ ] 7.1 Add `export_html` to the command list comment in the `--initconf` template
- [ ] 7.2 Add `export_html` to CHANGELOG

## 8. Manual verification

- [ ] 8.1 Test `:export_html ~/test.html` produces a file
- [ ] 8.2 Test exported file opens in browser with correct styling
- [ ] 8.3 Test exported file shows rendered math (not raw TeX)
- [ ] 8.4 Test exported file shows rendered Mermaid diagrams (not source)
- [ ] 8.5 Test exported file has no script tags
- [ ] 8.6 Test exported file works without internet / without mip running
- [ ] 8.7 Test tilde expansion and relative paths work
- [ ] 8.8 Test error handling: invalid path, read-only directory
