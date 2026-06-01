## Why

mip has no way to customize the visual appearance beyond light/dark mode. Users who want different fonts, colors, code block styling, or spacing are stuck with the built-in look. A custom CSS system lets users create and switch between visual styles while keeping the dark/light mode system intact.

"Theme" means color mode (system/light/dark). "Style" means custom CSS — separate concepts, separate names.

Bean: mip.rs-3lls

## What Changes

- Add `style` config setting — name of a style directory under `~/.config/miprs/styles/`
- Load custom CSS from `~/.config/miprs/styles/<name>/style.css` and inject after default styles
- Add `<style id="custom-css">` tag in template for custom CSS injection
- Auto-reload: poll CSS file mtime in the existing 500ms loop, re-inject via JS on change
- Add `--initstyle <name>` CLI flag to scaffold a new style with the default CSS extracted and commented
- Works with dark/light mode: custom CSS can override `:root`, `.dark`, `.light` variables
- Add `set style <name>` for runtime style switching

## Capabilities

### New Capabilities
- `custom-styles`: Custom CSS styles loaded from filesystem with live-reload

### Modified Capabilities
- `cli`: Add `--initstyle` flag
- `config`: Add `style` setting
- `command-mode`: Add `style` to settable values via `:set`

## Impact

- `src/config.rs`: add `style` field, `styles_dir()` helper, extract default CSS for initstyle
- `src/main.rs`: add `--initstyle` flag handler, load custom CSS, pass to view
- `src/view.rs`: inject custom CSS into template, poll CSS mtime, re-inject via JS, add `style` to RuntimeSettings
- `asset/theme1/template.html`: add `<style id="custom-css">` placeholder tag
