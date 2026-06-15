# Changelog mip.rs

## Unreleased

- add default close keybindings: Ctrl+Q, Ctrl+W, and Alt+F4 quit mip (mapped to the `close` command, overridable via `[keybindings]` config)
- remove the "Document URL" link from the top of rendered pages

## v0.5.0 - 15 jun 2026

- fix theme build: replace the abandoned `inliner` dependency (hangs on Node 24) with a dependency-free `scripts/inline-theme.mjs` generator; `make compthemes` now runs reliably and offline
- back-port theme styling (CSS variables, dark mode, `@media print`, frontmatter table, section numbers, `#{THEME_CLASS}`/`#{CUSTOM_CSS}`) from the generated `asset/theme1/template.html` into `theme_src/`, the single source of truth
- add `make check-themes` (wired into `make check` and CI) that fails when the committed theme artifact is out of sync with `theme_src/`, preventing silent regeneration drift
- add configurable default zoom: `zoom` config setting and `--zoom <factor>` CLI flag set the startup zoom level (default 1.0, clamped 0.3–5.0); `:set zoom <factor>` changes it live

## v0.4.3 - 2 jun 2026

## v0.4.2 - 2 jun 2026

## v0.4.1 - 2 jun 2026

## v0.4.0 - 2 jun 2026

- add custom styles: load user CSS from `~/.config/miprs/styles/<name>/style.css` with live-reload
- add `style` config setting to select a custom style
- add `--initstyle <name>` CLI flag to scaffold a new style with documented default CSS
- add `:set style <name>` for runtime style switching, `:set style` (empty) reverts to default
- custom CSS injected after default styles via `<style id="custom-css">`, supports dark/light mode overrides
- add `make lint` (cargo fmt --check + cargo clippy -D warnings) and `make check` (lint + test) targets
- fix all clippy warnings across codebase (collapsible_if, manual_strip, redundant_closure, while_let_loop, etc.)
- apply cargo fmt to fix formatting drift
- add Mermaid diagram support: render ` ```mermaid ` code blocks as interactive SVG diagrams (flowcharts, sequence, Gantt, class, state, ER, pie, journey, git graphs)
- bundle mermaid.min.js (~3.3MB) in binary via rust-embed, served from local warp server at `/mermaid/*`
- Mermaid diagrams respect light/dark theme and re-render on theme switch
- add `mermaid` config option (bool, default `true`) and `--no-mermaid` CLI flag
- add `export_html` command: export current document as a self-contained HTML file (DOM capture with scripts/localhost refs stripped)
- add vim-style navigation: j/k scroll, Ctrl+f/b page scroll, Ctrl+d/u half-page, g,g/shift+G top/bottom, n/N heading jump
- add key sequence support in keybinding system (comma-separated sequences like `g,g` with 500ms timeout)
- add desktop icon: SVG icon in taskbar, `.desktop` file for app launchers, Nix packaging installs icon and desktop entry
- add dynamic window title: shows frontmatter `title` or filename as `<title> - MiP`
- `:open` now reloads file in-place instead of spawning a new process; preserves runtime settings, updates file watcher and server directory
- remove `string_to_static_str` / `Box::leak` memory leaks in main.rs
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
- add Ctrl+P print dialog via WebKitGTK PrintOperation (also supports "Print to File" for PDF export)
- add `@media print` CSS that forces light theme colors regardless of screen theme
- fix NixOS crash in file chooser by adding GTK4 gsettings schemas to `XDG_DATA_DIRS` in flake.nix
- add hierarchical section numbers (`paragraph_numbers` + `paragraph_numbers_start` config), shown in preview and TOC
- fix arrow/page keys stolen by GTK Paned divider
- add sidetoc keyboard navigation: arrow up/down, left/right collapse/expand, Enter, Escape
- add quicktoc left/right collapse/expand
- add `sidetoc_focus` and `document_focus` commands
- sidetoc auto-focuses on open, returns focus to document on close
- add `:set` command for changing settings at runtime (theme, frontmatter, paragraph_numbers, paragraph_numbers_start)
- runtime setting changes trigger immediate re-render
- add zoom: Ctrl+= (in), Ctrl+- (out), Ctrl+0 (reset), 10% steps, clamped 0.3–5.0
- Tab completion for setting names after `:set `
- fix Ctrl+key keybindings not matching when GDK sends control character keyvals
- add persistent command bar history with ↑/↓ navigation and prefix filtering (`history_size` config, default 50)
- add TeX math rendering via KaTeX: inline `$...$` and display `$$...$$` with pulldown-cmark ENABLE_MATH
- KaTeX JS, CSS, and fonts bundled offline (~600KB), served from embedded assets
- math re-renders on file change without page reload
- add `math` config option (default true) and `--no-math` CLI flag
- fix GTK init panic when `set_default_icon_name` is called before Application::run

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


