# Changelog mip.rs

## Unreleased

- add CLI argument parsing with `argh`: `--help`, `--version`, `--verbose` flags
- fix panic when running without arguments (now prints help and exits cleanly)
- add `--frontmatter` flag to display YAML frontmatter as a styled table
- add dark mode with `--theme system|light|dark` (system default, reactive to OS changes)
- add config file support at `~/.config/miprs/config.toml` (theme, frontmatter settings)
- auto-embed video files (.webm, .mp4, .mov, .ogv) as playable `<video>` elements from link or image syntax
- add GStreamer plugins to Nix packaging for WebKitGTK video playback
- external links now open in default browser instead of navigating the preview
- add test suite (35 tests): unit tests for markdown, view, port helpers; integration tests for config, server routes, markdown pipeline
- refactor: extract pure `build_html()` from `to_file()`, extract `routes()` from `run_bro()`, add `Config::load_from(path)`
- add `src/lib.rs` crate root for testability
- add coverage tooling: `scripts/update-coverage.sh` with `cargo-tarpaulin`, plain-text percentage in README

## v0.3.0 - 28 May 2026

Linux-only from this version onwards.

- **BREAKING**: drop macOS and Windows support
- migrate from tao/wry (GTK3 + webkit2gtk-4.1) to native gtk4 + webkit6 (WebKitGTK 6.0)
- update pulldown-cmark 0.9 → 0.12, rust-embed 6 → 8, rand 0.8 → 0.9, notify 5 → 7
- update Rust edition to 2024

## v0.2.3 - 28 May 2026

This is the last version with macOS and Windows support. Future versions will
be Linux-only, using GTK4 and WebKitGTK 6.0.

- temp files are now written to system temp directory instead of next to the markdown file (#2, #11)
- temp directory is cleaned up on window close
- update nixpkgs flake to fix webkitgtk EGL crash on wayland
- add flake.nix
- get it working again after cargo update - 2 april 2025

## v.0.2.0 - 28 Dec 2022
- readme: contrib
- readme: dev
- app: icon
- remove unwanted debug output
- bug: fix other than current path images
- webserver
- markdown parser
- websview wri (more mature)
- cli arguments
- free portfinder
- inotify
- images
- embedded template
- License
- remove frontmatter

## v.0.1.0 - 28 Sep 2022

- inital project setup
- webview working


