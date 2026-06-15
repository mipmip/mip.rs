## Why

`mip` always starts the WebView at zoom level 1.0 and provides no way to change the startup zoom. On HiDPI / Wayland-scaled displays the rendered document can look too large or too small, and the only remedy today is pressing `Ctrl+=`/`Ctrl+-` several times after every launch — the adjustment does not persist. A configurable default zoom lets users set their preferred scale once.

## What Changes

- Add a `zoom` config setting (a float scale factor) that sets the WebView zoom level at startup. Default `1.0` (unchanged behavior when unset).
- Add a `--zoom <factor>` CLI flag that overrides the config value for a single run (consistent with `--frontmatter` / `--no-mermaid`).
- Add `zoom` to the runtime `:set` settings (with tab-completion), so `:set zoom 1.4` changes the live zoom without restart.
- Clamp the value to the existing zoom bounds (0.3–5.0) used by the `zoom_in`/`zoom_out` commands; warn and fall back to default on an invalid value.
- Document the setting in the generated config template.

The existing `zoom_in` / `zoom_out` / `zoom_reset` commands are unchanged; this adds a *persisted starting point* they continue to adjust from.

## Capabilities

### New Capabilities
<!-- None. -->

### Modified Capabilities
- `zoom`: Adds a configurable default/startup zoom level (config setting, `--zoom` CLI flag, and `:set zoom`), in addition to the existing relative zoom commands.

## Impact

- **Config**: `Config` gains `zoom: Option<f64>` plus a `zoom()` accessor (default `1.0`, clamped 0.3–5.0), mirroring the existing `theme`/`sidetoc_width` pattern in `src/config.rs`.
- **CLI**: a `--zoom <factor>` flag in `src/main.rs`, overriding the config value when present (flag presence semantics consistent with the other override flags).
- **Startup**: apply the resolved zoom via `webview.set_zoom_level(...)` right after `WebView::new()` in `src/view.rs` (~line 795).
- **Runtime**: add `"zoom"` to `SETTINGS` in `src/command.rs` and a `"zoom"` arm in the `:set` dispatch in `src/view.rs` (parse float, clamp, `set_zoom_level`).
- **Docs**: a `zoom` line in the config template (`src/config.rs` `default_config_template`).
- **Tests**: `config.rs` accessor/clamping unit tests, following the existing config test style.
