## 1. Config

- [ ] 1.1 Add `style: Option<String>` to Config struct with accessor
- [ ] 1.2 Add `pub fn styles_dir() -> PathBuf` helper (returns `~/.config/miprs/styles/`)
- [ ] 1.3 Add `pub fn style_css_path(name: &str) -> PathBuf` helper
- [ ] 1.4 Update `default_config_template()` with `style` setting documentation
- [ ] 1.5 Add `style` to RuntimeSettings as `RefCell<String>`

## 2. Template

- [ ] 2.1 Add `<style id="custom-css">#{CUSTOM_CSS}</style>` after the default `<style>` in template.html
- [ ] 2.2 Update `build_html` to replace `#{CUSTOM_CSS}` placeholder

## 3. Custom CSS loading

- [ ] 3.1 In main.rs / view.rs: if `style` is configured, read the CSS file content
- [ ] 3.2 Pass custom CSS content to `build_html` / `to_file` for initial render
- [ ] 3.3 Handle missing file: print warning, use empty CSS

## 4. Live-reload

- [ ] 4.1 In poll loop: track custom CSS file mtime
- [ ] 4.2 On mtime change: read new CSS, inject via JS `document.getElementById('custom-css').textContent = '...'`
- [ ] 4.3 Escape CSS content for JS injection (backticks, backslashes)

## 5. --initstyle CLI

- [ ] 5.1 Add `--initstyle` option to Cli struct (Option<String>)
- [ ] 5.2 Handle early in main: create styles dir, extract default CSS with comments, write, exit
- [ ] 5.3 Refuse to overwrite existing style directory
- [ ] 5.4 Add `pub fn default_style_css() -> &'static str` to config.rs with documented default CSS

## 6. Runtime switching

- [ ] 6.1 Add `style` to SETTINGS list in command.rs
- [ ] 6.2 Handle `set style <name>` in execute_command: load new CSS, inject via JS
- [ ] 6.3 Handle `set style` (empty): clear custom CSS

## 7. Tests

- [ ] 7.1 Test `style_css_path` returns correct path
- [ ] 7.2 Test config parsing with `style` field
- [ ] 7.3 Test `default_style_css()` is valid CSS (non-empty, no syntax errors)
- [ ] 7.4 Test `build_html` replaces `#{CUSTOM_CSS}` placeholder

## 8. Verify

- [ ] 8.1 `cargo build` succeeds
- [ ] 8.2 `cargo test` passes
- [ ] 8.3 `mip --initstyle academic` creates style directory and CSS file
- [ ] 8.4 `style = "academic"` in config loads the custom CSS
- [ ] 8.5 Editing the CSS file live-reloads in the preview
- [ ] 8.6 `:set style academic` switches style at runtime
- [ ] 8.7 `:set style` (empty) reverts to default
- [ ] 8.8 Dark/light mode still works with custom styles
