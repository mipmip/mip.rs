## Why

mip has no dark mode and no config file. The single hardcoded light theme doesn't respect the user's system preference. Settings like `--frontmatter` can only be set per-invocation via CLI flags — there's no way to persist preferences.

Bean: mip.rs-x9sf

## What Changes

- Add config file support at `~/.config/miprs/config.toml` with `toml` crate
- Add `--theme system|light|dark` CLI flag (default: system)
- Refactor CSS to use CSS variables with `prefers-color-scheme` for system mode and class-based override for explicit light/dark
- Dark mode styles for all elements including `.frontmatter` table
- Make `frontmatter` setting configurable in config file (CLI overrides config, config overrides default)
- Config resolution order: default → config file → CLI flag

## Capabilities

### New Capabilities
- `config`: Config file loading from `~/.config/miprs/config.toml`, merged with CLI flags
- `theming`: Dark/light/system theme support via CSS variables and `prefers-color-scheme`

### Modified Capabilities
- `cli`: Adding `--theme system|light|dark` option

## Impact

- `Cargo.toml`: add `toml` and `serde` dependencies
- `src/main.rs`: config loading, merge CLI over config, pass theme to view/markdown
- `src/markdown.rs`: no changes (frontmatter bool already threaded)
- `src/view.rs`: inject theme class into HTML
- `asset/theme1/template.html`: refactor CSS to variables, add dark mode styles, add `#{THEME_CLASS}` placeholder
