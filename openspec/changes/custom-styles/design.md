## Context

The template.html (97K) contains all CSS inlined in a `<style>` tag. It's embedded via rust-embed at compile time. Custom styles need to load CSS from the filesystem at runtime and inject it after the default styles. The existing 500ms poll loop can check for CSS file changes.

## Goals / Non-Goals

**Goals:**
- Custom CSS loaded from `~/.config/miprs/styles/<name>/style.css`
- CSS injected after default styles (override via specificity)
- Live-reload: CSS changes reflected instantly without restart
- `--initstyle <name>` scaffolds a new style with documented default CSS
- Works seamlessly with dark/light/system color modes
- Runtime switchable via `:set style <name>`

**Non-Goals:**
- Custom HTML templates (fragile, breaks on mip updates)
- Custom JavaScript
- Theme marketplace or package manager
- Renaming the existing `theme` setting (that stays as color mode)

## Decisions

### CSS-only override, not full template

**Choice**: Users provide a CSS file that's injected after the default styles in a separate `<style id="custom-css">` tag.

**Rationale**: Safe and forward-compatible. CSS overrides via specificity. Users can change colors, fonts, spacing, code styling — anything visual. The HTML structure stays under mip's control.

### Style directory structure

```
~/.config/miprs/styles/
  academic/
    style.css
  github/
    style.css
```

Config:
```toml
style = "academic"
```

Resolved to: `~/.config/miprs/styles/academic/style.css`

A directory (not just a file) allows future expansion (custom fonts, images).

### Injection via `<style id="custom-css">`

**Choice**: Add an empty `<style id="custom-css">#{CUSTOM_CSS}</style>` after the default `<style>` in template.html. At render time, replace `#{CUSTOM_CSS}` with the file contents. For live-reload, replace via JS: `document.getElementById('custom-css').textContent = '...'`.

**Rationale**: Clean separation. The ID makes JS injection trivial. No re-render needed for CSS changes — just swap the style content.

### Live-reload via mtime check

**Choice**: In the poll loop, track the custom CSS file's modification time. On change, read the file and inject via JS.

**Rationale**: Same pattern as the seed file check. Cheap (one stat call per 500ms). No file watcher needed.

### `--initstyle` scaffolding

**Choice**: Extract the default CSS from the embedded template, add comments explaining each section, write to `~/.config/miprs/styles/<name>/style.css`. Refuse to overwrite existing.

**Rationale**: Same UX as `--initconf`. The extracted CSS gives users a starting point to customize.

### Runtime switching via `:set style`

**Choice**: Add `style` to `RuntimeSettings` as `RefCell<String>`. When changed, load the new CSS file and inject via JS. Empty string means no custom style.

## Risks / Trade-offs

- [CSS specificity] → User CSS might not override default if selectors aren't specific enough. Mitigation: document that custom CSS loads after defaults; use same selectors or add `!important`.
- [Large CSS files] → Injecting large CSS via JS template literal could be slow. Mitigation: CSS files are tiny in practice.
- [Style not found] → If configured style directory doesn't exist, print warning and continue without custom CSS.
