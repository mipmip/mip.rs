## 1. Config setting

- [x] 1.1 Add `zoom: Option<f64>` to the `Config` struct in `src/config.rs`
- [x] 1.2 Add a `zoom()` accessor that returns `self.zoom.unwrap_or(1.0).clamp(0.3, 5.0)`
- [x] 1.3 Add a documented `zoom = 1.0` line to `default_config_template()` in `src/config.rs`
- [x] 1.4 Add unit tests for `zoom()`: unset → 1.0, in-range passthrough, above-max clamps to 5.0, below-min clamps to 0.3

## 2. CLI flag

- [x] 2.1 Add a `--zoom <factor>` (`Option<f64>`) flag to the argh CLI struct in `src/main.rs`
- [x] 2.2 Resolve the effective zoom (`--zoom` if present, else `cfg.zoom()`) and pass it into the view/window setup

## 3. Apply at startup

- [x] 3.1 In `src/view.rs`, after `WebView::new()` (~line 795), call `webview.set_zoom_level(resolved_zoom)` with the resolved value clamped to 0.3–5.0

## 4. Runtime `:set zoom`

- [x] 4.1 Add `"zoom"` to the `SETTINGS` list in `src/command.rs` (for tab-completion)
- [x] 4.2 Add a `"zoom"` arm to the `:set` dispatch in `src/view.rs`: parse the value as `f64`, clamp to 0.3–5.0, call `set_zoom_level`; on parse failure print a warning and leave zoom unchanged

## 5. Verification

- [x] 5.1 `make check` passes (fmt, clippy, check-themes, tests)
- [x] 5.2 Manually verify: `mip --zoom 1.5 README.md` starts zoomed; `zoom = 1.4` in config starts zoomed; `:set zoom 2.0` rescales live; `:set zo` completes to `zoom`; out-of-range and non-numeric values are clamped/warned
