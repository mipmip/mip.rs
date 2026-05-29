## Context

mip is a markdown preview tool that opens a webview window, runs a local warp server, and watches the file for changes. The current stack uses `tao` (window management) + `wry` (webview abstraction) which internally depend on GTK3 and webkit2gtk-4.1 on Linux. As of v0.2.3, mip is Linux-only, making the cross-platform abstraction unnecessary.

The codebase is small: 4 source files (~200 lines total), a nix flake, and an embedded HTML theme.

## Goals / Non-Goals

**Goals:**
- Replace tao/wry with native gtk4 + webkit6 crates
- Update all outdated content pipeline crates
- Maintain identical user-facing behavior (open window, render markdown, auto-reload)
- Keep the nix flake and package.nix working

**Non-Goals:**
- Adding new features (this is a dependency modernization)
- Changing the server architecture (warp stays)
- Changing the theme/HTML template system
- Supporting non-Linux platforms

## Decisions

### 1. Use gtk4 + webkit6 directly instead of waiting for wry GTK4 support

**Choice**: Native `gtk4` (0.11) + `webkit6` (0.6) crates

**Alternatives considered**:
- Wait for wry/tao GTK4 migration (open issues, no timeline)
- Use webkit2gtk-rs (GTK3, same issues as current)

**Rationale**: wry/tao GTK4 migration has no completion date. The webkit6 crate is mature (v0.6.1, GNOME maintainer). mip's webview usage is minimal (load a URL), so the migration surface is small.

### 2. GTK4 Application model replaces manual event loop

**Choice**: Use `gtk4::Application` with `activate` signal instead of tao's `EventLoop`

**Rationale**: GTK4's Application handles the lifecycle (single instance, signal handling, clean shutdown). This simplifies `main.rs` — no manual `EventLoop::new()` + `run()` pattern. The webkit6 `WebView` is a native GTK4 widget, so it slots in directly as a child of `gtk4::ApplicationWindow`.

### 3. Update content pipeline crates in the same change

**Choice**: Update pulldown-cmark, rust-embed, rand, notify together with the GTK migration

**Rationale**: Doing it separately would mean two rounds of fixing compile errors. Since we're touching Cargo.toml anyway and the crate count is small, bundling is simpler.

### 4. Nix: webkitgtk_6_0 in flake and package

**Choice**: Replace `webkitgtk_4_1` with `webkitgtk_6_0` in both `flake.nix` (dev shell) and `package.nix` (build)

**Rationale**: Direct replacement — nixpkgs already packages both.

## Risks / Trade-offs

- **[API breakage in pulldown-cmark 0.9→0.12]** → The `html::push_html` API may have changed. Mitigation: check docs, likely minimal changes.
- **[rust-embed 6→8 breaking changes]** → The `RustEmbed` derive macro API may differ. Mitigation: small usage (one struct), easy to adapt.
- **[notify 5→7 API changes]** → Watcher creation API changed. Mitigation: still uses `RecommendedWatcher`, likely just constructor signature changes.
- **[webkit6 WebView API differs from wry]** → Mitigation: mip only uses `load_uri()` equivalent — the simplest possible webview operation.
- **[GTK4 temp dir cleanup on close]** → Currently done in tao's window close event. In GTK4, use the `shutdown` signal on `Application`. Same effect, different hook point.
