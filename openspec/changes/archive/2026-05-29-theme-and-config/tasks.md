## 1. Dependencies

- [x] 1.1 Add `toml` and `serde` (with derive) to Cargo.toml
- [x] 1.2 Run `cargo update`

## 2. Config module

- [x] 2.1 Create `src/config.rs` with `Config` struct (theme: String, frontmatter: bool) with serde Deserialize
- [x] 2.2 Implement config file loading from `$XDG_CONFIG_HOME/miprs/config.toml` with `~/.config` fallback
- [x] 2.3 Handle missing file (return defaults), invalid TOML (warn + defaults), invalid values (warn + defaults)

## 3. CLI changes

- [x] 3.1 Add `--theme` as `Option<String>` to `Cli` struct
- [x] 3.2 Validate theme value is `system`, `light`, or `dark` after parsing
- [x] 3.3 Merge config: load config file, overlay CLI flags (only when explicitly provided)

## 4. Template and CSS refactor

- [x] 4.1 Add `#{THEME_CLASS}` placeholder to `<html>` tag in template.html
- [x] 4.2 Extract hardcoded colors into CSS variables on `:root` (--bg, --fg, --heading, --link, --border, --code-bg, --blockquote-bg, --table-odd-bg, --frontmatter-th-bg, --frontmatter-border)
- [x] 4.3 Replace all hardcoded color values with `var()` references
- [x] 4.4 Add `@media (prefers-color-scheme: dark)` block that redefines variables for system mode
- [x] 4.5 Add `.dark` class that overrides variables for explicit dark mode
- [x] 4.6 Add `.light` class that overrides variables for explicit light mode (locks to light even if OS is dark)

## 5. Wire theme through Rust

- [x] 5.1 Pass theme class string to `to_file` and `to_html` (empty for system, "light" or "dark" for explicit)
- [x] 5.2 Replace `#{THEME_CLASS}` in template with theme class string
- [x] 5.3 Pass theme to `view::window` for live-reload HTML injection

## 6. Verify

- [x] 6.1 `cargo build` succeeds
- [x] 6.2 `mip README.md` renders with system theme (respects OS preference)
- [x] 6.3 `mip --theme dark README.md` renders with dark theme
- [x] 6.4 `mip --theme light README.md` renders with light theme
- [x] 6.5 `mip --theme neon README.md` prints error and exits
- [x] 6.6 Config file with `theme = "dark"` applies dark theme
- [x] 6.7 Config file with `frontmatter = true` shows frontmatter without CLI flag
- [x] 6.8 CLI `--theme` overrides config file theme
- [x] 6.9 Missing config file works without error
- [x] 6.10 `mip --frontmatter --theme dark` shows frontmatter table with dark styling
