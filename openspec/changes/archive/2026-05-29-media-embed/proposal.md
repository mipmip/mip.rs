## Why

Links and image references to video files (`.webm`, `.mp4`) render as plain `<a>` or broken `<img>` tags. GitHub auto-embeds these as `<video>` elements, but pulldown-cmark doesn't. mip should match GitHub's behavior for local previewing.

Bean: mip.rs-m4ic

## What Changes

- Post-process HTML output to rewrite `<a>` links pointing to video files into `<video>` elements
- Post-process HTML output to rewrite `<img>` tags pointing to video files into `<video>` elements
- Supported video extensions: `.webm`, `.mp4`, `.mov`, `.ogv`
- No new dependencies — simple string-based HTML rewriting

## Capabilities

### New Capabilities
- `media-embed`: Auto-embed video files referenced via link or image markdown syntax as playable `<video>` elements

### Modified Capabilities

## Impact

- `src/markdown.rs`: add post-processing step between pulldown-cmark output and template injection
