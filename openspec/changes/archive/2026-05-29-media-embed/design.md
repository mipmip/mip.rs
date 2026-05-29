## Context

pulldown-cmark renders `[text](file.webm)` as `<a href="file.webm">text</a>` and `![](file.webm)` as `<img src="file.webm">`. Neither plays video in a browser. GitHub's renderer detects media file extensions and auto-embeds them as `<video>` tags. The warp server already serves the parent directory as static files, so video files are accessible — they just need the right HTML tag.

## Goals / Non-Goals

**Goals:**
- Rewrite links to video files as `<video>` elements with `controls` attribute
- Rewrite image tags pointing to video files as `<video>` elements
- Support `.webm`, `.mp4`, `.mov`, `.ogv` extensions
- Work for both local and remote URLs

**Non-Goals:**
- Audio file embedding (can be added later)
- Custom video player controls or styling
- Parsing complex HTML (no regex for general HTML — just the specific patterns pulldown-cmark produces)

## Decisions

### String replacement, not HTML parser

**Choice**: Use targeted string replacements to find `<a href="...video_ext">` and `<img src="...video_ext"` patterns, replacing them with `<video>` tags.

**Rationale**: pulldown-cmark produces predictable, well-formed HTML. The patterns are simple and known. Adding an HTML parser dependency (like `scraper` or `kuchiki`) would be overkill for rewriting two tag patterns.

**Alternatives considered**:
- Custom pulldown-cmark event handler: more complex, harder to maintain
- HTML parser crate: heavy dependency for a simple transform

### Post-process after pulldown-cmark, before template injection

**Choice**: Add a `rewrite_media_embeds` function called on `html_output` after `html::push_html` and before template substitution.

**Rationale**: Clean separation — pulldown-cmark does markdown→HTML, then we fix up media tags, then the template wraps it. Same pattern as the frontmatter prepend.

### Video tag format

```html
<video src="file.webm" controls style="max-width:100%"></video>
```

`controls` for playback UI, `max-width:100%` to match how `<img>` is styled (existing CSS: `img{ max-width:100%;}`).

## Risks / Trade-offs

- [False positives] → A link like `[click here](page.mp4)` where `.mp4` is not actually a video would get rewritten. Acceptable — file extension is the only signal available, same as GitHub does.
- [Remote video URLs] → Links to `https://...file.webm` will render as `<video src="https://...">` which may not play due to CORS. Acceptable — local files are the primary use case.
