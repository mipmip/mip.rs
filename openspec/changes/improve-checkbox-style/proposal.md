## Why

Markdown task list checkboxes (`- [ ]` / `- [x]`) currently render with browser-default styling — small, inconsistent across platforms, and visually out of place. GitHub-style checkboxes are the de-facto standard users expect in rendered markdown.

Bean: mip.rs-zb10

## What Changes

- Add CSS styling for task list checkboxes to match GitHub's visual style
- Remove default bullet markers from task list items
- Style checked checkboxes with a filled blue background and white checkmark
- Style unchecked checkboxes with a gray border and rounded corners

## Capabilities

### New Capabilities
- `github-checkbox-style`: CSS-only styling for task list checkboxes to match GitHub's appearance

### Modified Capabilities

## Impact

- `theme_src/theme1/style.css` — new CSS rules for checkbox styling
- No Rust code changes needed — pulldown-cmark already emits the correct HTML
