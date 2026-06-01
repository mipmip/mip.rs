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
- add Table of Contents with two display modes: `--toc side` (persistent side panel) and `--toc zathura` (Tab-toggle, Zathura-style)
- TOC uses native GTK TreeView with collapsible heading hierarchy
- vim-style keyboard navigation in TOC (j/k, Enter, Esc)
- headings now get anchor `id` attributes for in-document navigation
- add vim-style command mode (`:`) with `:q`, `:open` commands and Tab path completion
- add wildmenu completion popup with command name and path completion, Shift+Tab backward cycling
- path completion filters to markdown files (.md, .markdown, .mkd, .qmd) and directories
- only rebuild TOC and re-inject HTML when content actually changes (reduces flicker)
- **BREAKING**: rename zathura → quicktoc, side → sidetoc; replace `--toc` with `--runcmd`
- add navigation commands: `sidetoc_open/close/toggle`, `sidetoc_expand/shrink_width`, `quicktoc`
- add `--runcmd` CLI option for executing commands at startup (replaces `--toc`)
- add command composition with `;` separator (works in command bar, --runcmd, config)
- add config settings: `runcmd`, `sidetoc_width`, `sidetoc_position`
- both sidetoc and quicktoc always available, hidden by default
- add configurable keybindings via `[keybindings]` section in config.toml
- default keybindings: Tab → quicktoc, Ctrl+P → print
- add `--initconf` flag to generate a documented default config file

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


