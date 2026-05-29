## Context

mip is Linux-only with a single inlined CSS theme in `template.html`. Settings are CLI-only — no persistence. The `--frontmatter` flag was recently added and would benefit from config file persistence.

## Goals / Non-Goals

**Goals:**
- Config file at `~/.config/miprs/config.toml` for persisting preferences
- Three theme modes: system (default), light, dark
- CSS variable-based theming with `prefers-color-scheme` for system mode
- CLI flags override config file values

**Non-Goals:**
- Custom color schemes or user-defined themes
- Config file creation wizard or `mip init` command
- Hot-reloading config changes (requires restart)
- Config for anything beyond theme and frontmatter for now

## Decisions

### Config file format: TOML

**Choice**: TOML at `~/.config/miprs/config.toml`.

**Rationale**: TOML is the Rust ecosystem standard for config. Linux-only means we can hardcode `~/.config` via `$XDG_CONFIG_HOME` fallback without needing the `directories` crate. The `toml` crate with `serde` gives derive-based deserialization.

**Config structure**:
```toml
theme = "system"       # "system" | "light" | "dark"
frontmatter = false    # show frontmatter table
```

**Alternatives considered**:
- JSON/YAML: less idiomatic for Rust CLI tools
- `directories` crate: unnecessary for Linux-only

### CSS variables + class toggle + prefers-color-scheme

**Choice**: Define all colors as CSS variables on `:root`. For "system" mode, use `@media (prefers-color-scheme: dark)` to swap variables. For explicit light/dark, add a class on `<html>` that overrides the variables.

**Rationale**: This gives all three modes with minimal template changes. System mode is zero-JS. Explicit modes need only a class injected from Rust.

**Template change**: Add `#{THEME_CLASS}` placeholder on `<html>` tag:
```html
<html class="#{THEME_CLASS}">
```
- system mode: empty string (let media query decide)
- light mode: `"light"`
- dark mode: `"dark"`

### Config resolution order

```
default → config file → CLI flag
```

Create a `Config` struct with defaults. Overlay config file values if present (missing file is not an error). Then overlay CLI flags (only if explicitly provided). The `--theme` flag uses argh's string option with validation.

### argh option type for theme

**Choice**: `--theme` as `Option<String>` validated to `system|light|dark` after parsing.

**Rationale**: argh doesn't support enums natively. Parsing as `Option<String>` and validating manually is simplest. `None` means "not provided via CLI" (don't override config).

## Risks / Trade-offs

- [Config file missing] → Not an error. Fall back to defaults silently.
- [Invalid config values] → Print warning, use default for that field.
- [Template placeholder in existing HTML] → Need to be careful that `#{THEME_CLASS}` doesn't break the current single-line template. Straightforward string replacement like existing placeholders.
