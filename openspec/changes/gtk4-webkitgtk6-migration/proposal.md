## Why

mip currently uses GTK3 (via `tao` + `wry`) with webkit2gtk-4.1. These libraries are maintained but aging — `tao` and `wry` have not yet migrated to GTK4, and the GTK3 stack has known EGL/Wayland issues. Since mip is now Linux-only (as of v0.2.3), we can drop the cross-platform abstraction layer and use `gtk4` + `webkit6` crates directly. This gives us a modern, actively maintained stack with better Wayland support.

Bean: mip.rs-qwkp

## What Changes

- **BREAKING**: Drop `tao` and `wry` dependencies — replace with `gtk4` (0.11) and `webkit6` (0.6)
- **BREAKING**: Remove macOS/Windows platform code (`#[cfg]` blocks in `view.rs`)
- Remove unused `gtk` (0.18, GTK3 bindings) dependency
- Update content pipeline crates: `pulldown-cmark` 0.9 → 0.12, `rust-embed` 6.4 → 8.x, `rand` 0.8 → 0.9, `notify` 5.0 → 7.x
- Update `flake.nix` and `package.nix` to use `webkitgtk_6_0` instead of `webkitgtk_4_1`
- Update Rust edition from 2021 to 2024

## Capabilities

### New Capabilities
- `gtk4-webview`: Native GTK4 application window with webkit6 WebView widget, replacing the tao/wry abstraction layer

### Modified Capabilities

## Impact

- `src/view.rs`: Complete rewrite — GTK4 Application + webkit6 WebView instead of tao EventLoop + wry WebViewBuilder
- `src/main.rs`: Simplify — GTK4 handles the application lifecycle, no need for manual event loop setup
- `src/server.rs`: No changes expected (warp server is independent)
- `src/markdown.rs`: Minor changes if `pulldown-cmark` API changed between 0.9 and 0.12
- `Cargo.toml`: Major dependency changes
- `flake.nix` / `package.nix`: Switch from `webkitgtk_4_1` to `webkitgtk_6_0`
- GitHub Actions / CI: Can remove macOS and Windows build targets
