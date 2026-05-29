## 1. Update Cargo.toml dependencies

- [x] 1.1 Replace `tao`, `wry`, `gtk` with `gtk4 = "0.11"` and `webkit6 = "0.6"`
- [x] 1.2 Update `pulldown-cmark` from 0.9 to 0.12
- [x] 1.3 Update `rust-embed` from 6.4 to 8.x
- [x] 1.4 Update `rand` from 0.8 to 0.9
- [x] 1.5 Update `notify` from 5.0 to 7.x
- [x] 1.6 Remove commented-out platform-specific dependencies
- [x] 1.7 Update Rust edition to 2024
- [x] 1.8 Run `cargo update` and verify Cargo.lock resolves

## 2. Rewrite view.rs with gtk4 + webkit6

- [x] 2.1 Replace tao EventLoop + WindowBuilder with gtk4::Application + ApplicationWindow
- [x] 2.2 Replace wry WebViewBuilder with webkit6::WebView widget
- [x] 2.3 Remove all `#[cfg]` platform-conditional blocks
- [x] 2.4 Implement temp directory cleanup on application shutdown signal
- [x] 2.5 Verify window opens and loads the local server URL

## 3. Update main.rs for GTK4 lifecycle

- [x] 3.1 Replace manual event loop setup with gtk4::Application::run()
- [x] 3.2 Move server and watcher spawning into the application activate handler
- [x] 3.3 Pass temp_dir to the GTK4 window for cleanup

## 4. Fix content pipeline for updated crates

- [x] 4.1 Update markdown.rs for pulldown-cmark 0.12 API changes
- [x] 4.2 Update markdown.rs for rust-embed 8.x derive macro changes
- [x] 4.3 Update markdown.rs for rand 0.9 API changes
- [x] 4.4 Update main.rs for notify 7.x watcher API changes

## 5. Update Nix packaging

- [x] 5.1 Replace `webkitgtk_4_1` with `webkitgtk_6_0` in flake.nix dev shell
- [x] 5.2 Replace `webkitgtk_4_1` with `webkitgtk_6_0` in package.nix
- [x] 5.3 Add `gtk4` to nix build inputs
- [x] 5.4 Verify `nix develop` + `cargo build` succeeds
- [x] 5.5 Verify `nix build` succeeds

## 6. Verify and clean up

- [x] 6.1 Run `cargo clippy` and fix warnings
- [x] 6.2 Test: mip opens and renders README.md
- [x] 6.3 Test: editing the markdown file triggers auto-reload
- [x] 6.4 Test: closing the window cleans up temp directory
- [x] 6.5 Bump version to 0.3.0 in Cargo.toml and package.nix
- [x] 6.6 Update CHANGELOG.md
