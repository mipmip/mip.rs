## 1. Config

- [x] 1.1 Add `style: Option<String>` to Config struct with accessor
- [x] 1.2 Add `pub fn styles_dir() -> PathBuf` helper (returns `~/.config/miprs/styles/`)
- [x] 1.3 Add `pub fn style_css_path(name: &str) -> PathBuf` helper
- [x] 1.4 Update `default_config_template()` with `style` setting documentation
- [x] 1.5 Add `style` to RuntimeSettings as `RefCell<String>`

## 2. Template

- [x] 2.1 Add `<style id="custom-css">#{CUSTOM_CSS}</style>` after the default `<style>` in template.html
- [x] 2.2 Update `build_html` to replace `#{CUSTOM_CSS}` placeholder

## 3. Custom CSS loading

- [x] 3.1 In main.rs / view.rs: if `style` is configured, read the CSS file content
- [x] 3.2 Pass custom CSS content to `build_html` / `to_file` for initial render
- [x] 3.3 Handle missing file: print warning, use empty CSS

## 4. Live-reload

- [x] 4.1 In poll loop: track custom CSS file mtime
- [x] 4.2 On mtime change: read new CSS, inject via JS `document.getElementById('custom-css').textContent = '...'`
- [x] 4.3 Escape CSS content for JS injection (backticks, backslashes)

## 5. --initstyle CLI

- [x] 5.1 Add `--initstyle` option to Cli struct (Option<String>)
- [x] 5.2 Handle early in main: create styles dir, extract default CSS with comments, write, exit
- [x] 5.3 Refuse to overwrite existing style directory
- [x] 5.4 Add `pub fn default_style_css() -> &'static str` to config.rs with documented default CSS

## 6. Runtime switching

- [x] 6.1 Add `style` to SETTINGS list in command.rs
- [x] 6.2 Handle `set style <name>` in execute_command: load new CSS, inject via JS
- [x] 6.3 Handle `set style` (empty): clear custom CSS

## 7. Tests

- [x] 7.1 Test `style_css_path` returns correct path
- [x] 7.2 Test config parsing with `style` field
- [x] 7.3 Test `default_style_css()` is valid CSS (non-empty, no syntax errors)
- [x] 7.4 Test `build_html` replaces `#{CUSTOM_CSS}` placeholder

## 8. Verify

- [x] 8.1 `cargo build` succeeds
- [x] 8.2 `cargo test` passes
- [x] 8.3 `mip --initstyle academic` creates style directory and CSS file
- [x] 8.4 `style = "academic"` in config loads the custom CSS
- [x] 8.5 Editing the CSS file live-reloads in the preview
- [x] 8.6 `:set style academic` switches style at runtime
- [x] 8.7 `:set style` (empty) reverts to default
- [x] 8.8 Dark/light mode still works with custom styles
