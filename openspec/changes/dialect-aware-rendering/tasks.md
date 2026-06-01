## 1. Preprocessor Function

- [ ] 1.1 Create `preprocess_dialects(content: &str) -> String` function in `src/markdown.rs`
- [ ] 1.2 Implement Hugo self-closing shortcode regex: `{{<` and `{{%` variants → `<span class="dialect-inline dialect-hugo">`
- [ ] 1.3 Implement Hugo paired shortcode regex: `{{% name %}}...{{% /name %}}` → `<div class="dialect-block dialect-hugo">` wrapping preserved inner content
- [ ] 1.4 Implement Quarto fenced div regex: `:::{.class}...:::` → `<div class="dialect-block dialect-quarto">` wrapping preserved inner content
- [ ] 1.5 Implement Quarto inline attribute regex: `{.class}`, `{#id}`, `{key=value}` → `<span class="dialect-inline dialect-quarto">`

## 2. Pipeline Integration

- [ ] 2.1 Insert `preprocess_dialects()` call in `md_to_html_body_with_toc` between frontmatter parsing and pulldown-cmark parsing

## 3. CSS Styling

- [ ] 3.1 Add base `.dialect-block` styles: light gray background, left border, rounded corners, padding, monospace label
- [ ] 3.2 Add `.dialect-inline` styles: subtle gray background, rounded corners, monospace, reduced font size
- [ ] 3.3 Add `.dialect-hugo` accent color (warm/orange left border)
- [ ] 3.4 Add `.dialect-quarto` accent color (cool/blue left border)
- [ ] 3.5 Add `.dialect-label` styles: muted color, small font

## 4. Verification

- [ ] 4.1 Add unit tests for Hugo shortcode preprocessing (self-closing and paired)
- [ ] 4.2 Add unit tests for Quarto fenced div and inline attribute preprocessing
- [ ] 4.3 Test with a markdown file containing mixed Hugo and Quarto syntax alongside regular markdown
