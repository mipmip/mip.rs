## Context

mip.rs renders markdown via pulldown-cmark. The pipeline is: frontmatter parse → pulldown-cmark → event stream → HTML → post-processing (media embeds, section numbers). Dialect-specific syntax from Hugo and Quarto is not recognized by pulldown-cmark and renders as broken raw text.

## Goals / Non-Goals

**Goals:**
- Gracefully display Hugo shortcodes and Quarto syntax as recognizable, non-distracting blocks
- Insert preprocessing before pulldown-cmark so the parser receives clean markdown + HTML blocks

**Non-Goals:**
- Actually executing shortcodes or rendering their output (no Hugo/Quarto engine integration)
- Supporting every possible Quarto/Pandoc extension — focus on the most common constructs
- Auto-detecting which dialect a file uses (process all patterns unconditionally)

## Decisions

**Preprocessing approach** — Add a `preprocess_dialects(content: &str) -> String` function that runs regex replacements on the raw markdown before it reaches pulldown-cmark. Detected constructs are replaced with HTML `<div>` blocks (which pulldown-cmark passes through verbatim).

Alternative considered: Post-processing the HTML output. Rejected because dialect syntax may confuse pulldown-cmark's parser (e.g., `{{% %}}` could interfere with emphasis parsing). Pre-processing is more predictable.

**Unconditional processing** — Run both Hugo and Quarto regex patterns on every file rather than detecting dialect. The patterns are distinct enough that false positives are extremely unlikely (`{{< >}}` doesn't appear in normal markdown). This avoids needing a dialect detection mechanism or user configuration.

Alternative considered: Dialect detection via frontmatter hints or filename patterns. Rejected as unnecessary complexity — the regex patterns don't conflict.

**HTML block output format** — Each detected construct becomes:
```html
<div class="dialect-block dialect-hugo">
  <span class="dialect-label">shortcode-name</span>
  <code>original syntax</code>
</div>
```
For inline shortcodes (e.g., `{{< ref >}}`), use `<span class="dialect-inline dialect-hugo">` instead.

**Paired vs self-closing** — Hugo shortcodes can be self-closing (`{{< figure ... >}}`) or paired (`{{% note %}}...{{% /note %}}`). Paired shortcodes wrap their content; the preprocessor SHALL preserve the inner content as markdown (rendered normally) and wrap the opening/closing tags as dialect labels.

## Risks / Trade-offs

- **False positives** → The `{{< >}}` and `{{% %}}` patterns are highly specific to Hugo. Quarto's `:::` fenced divs could theoretically match other markdown extensions, but the `{.class}` attribute requirement makes false matches unlikely.
- **Regex complexity for nested constructs** → Quarto allows nested `:::` blocks. A simple regex won't handle arbitrary nesting. Mitigation: handle one level of nesting, which covers the vast majority of real-world usage.
- **Performance** → Regex runs on every render. Mitigation: the patterns are simple and markdown files are small; negligible impact.
