## Context

`md_to_html_body_with_toc` already extracts `TocEntry` structs with `level`, `title`, and `anchor_id`. The TOC TreeView and HTML headings both derive from this data. Section numbers can be computed as a pure function over the TocEntry list.

## Goals / Non-Goals

**Goals:**
- Compute hierarchical numbers (1., 1.1, 1.1.1) from heading levels
- Configurable start level (skip H1 title if desired)
- Numbers appear in both HTML preview and TOC views
- Off by default, enabled via config

**Non-Goals:**
- Runtime toggle via `:set` command (depends on mip.rs-k7cm, not yet implemented)
- Custom numbering styles (roman numerals, letters)
- Numbering non-heading elements (paragraphs, figures)

## Decisions

### Pure numbering function on TocEntry list

**Choice**: Add `compute_section_numbers(entries: &[TocEntry], start_level: u8) -> Vec<String>` that returns a parallel vec of number strings ("1.", "1.1", "1.1.2", etc.). Headings below `start_level` get empty strings.

**Rationale**: Pure function, easily testable. The caller decides where to use the numbers (HTML injection, TOC display).

**Algorithm**:
```
counters = [0; 6]  // one per h1-h6
for each entry:
    if entry.level < start_level: yield ""
    else:
        depth = entry.level - start_level
        counters[depth] += 1
        // reset all deeper counters
        for d in (depth+1)..6: counters[d] = 0
        // build "1.2.3" from counters[0..=depth]
        yield counters[0..=depth].join(".")
```

### Inject numbers into HTML via post-processing

**Choice**: After `push_html`, find `<h{n} id="...">` tags and prepend the section number with a `<span class="section-number">` wrapper.

**Rationale**: Same post-processing pattern as `rewrite_media_embeds`. The span class allows CSS styling (e.g. lighter color, margin).

### Inject numbers into TocEntry titles

**Choice**: When `paragraph_numbers` is true, modify the `TocEntry.title` to include the number prefix before returning from `md_to_html_body_with_toc`.

**Rationale**: The TreeView displays `title` directly. Modifying it at the source means both sidetoc and quicktoc get numbers automatically.

### Config

```toml
paragraph_numbers = false
paragraph_numbers_start = 1
```

`start_level` is 1-indexed matching heading levels (1 = H1, 2 = H2).

## Risks / Trade-offs

- [Number drift on live reload] → Numbers are recomputed on every content change, so they stay correct.
- [Start level edge cases] → If `start_level = 3`, only H3+ get numbers. H1 and H2 headings show without numbers. This is intentional.
