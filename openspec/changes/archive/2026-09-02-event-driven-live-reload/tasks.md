## Phase A — Stop the feedback loop

- [x] A.1 Create `src/watch.rs`, declare `pub mod watch;` in `src/lib.rs` so the
      logic is reachable from `tests/`
- [x] A.2 Add `pub fn should_rerender(event: &notify::Event, watched: &Path) -> bool`:
      allowlist `EventKind::Modify(ModifyKind::Data(_))`,
      `Modify(ModifyKind::Name(_))`, `Create(_)`, `Remove(_)`; reject all
      `Access(_)`, `Modify(Metadata(_))`, `Any`, `Other`
- [x] A.3 Path match: canonicalize the watched file once at startup and compare
      each `event.paths` entry by path equality, replacing
      `teststr.contains(&current_file)` (`src/main.rs:110`)
- [x] A.4 Wire `should_rerender()` into the `watch()` loop; keep `to_html()` as the
      action for now so this phase stands alone
- [x] A.5 `tests/watch_test.rs`: `Access(Open(Any))` on the watched path returns
      false; `Modify(Data(Any))`, `Modify(Name(To))`, `Create(File)`, `Remove(File)`
      return true; `Modify(Metadata(Any))` returns false
- [x] A.6 `tests/watch_test.rs`: a sibling `doc.md.bak` and a nested
      `notes/doc.md` do NOT match the watched `doc.md` (the substring-match
      regression)
- [x] A.7 Loop regression test: run the real watcher on a `tempfile::TempDir`,
      `read_to_string` the watched file, assert **zero** renders fire; then write
      to the file and assert exactly **one** fires
- [x] A.8 Verify by hand: `mip <file>` idles at 0% CPU in `top`; editing the file
      still updates the preview
- [x] A.9 Switch the watch from `RecursiveMode::Recursive` to `NonRecursive`:
      only the exact document path is ever matched, so a recursive watch adds
      watch descriptors and event traffic for the whole tree (`target/`, `.git/`)
      without ever matching anything extra

## Phase B — Collapse the two change-detection paths

- [x] B.1 Add `futures-channel = "0.3"` to `Cargo.toml`
- [x] B.2 Define `pub enum WatchMessage { Document, Style }` in `src/watch.rs`
- [x] B.3 Add a 100 ms debounce to the watch loop: surviving events set a pending
      flag, the message is sent 100 ms after the last one
- [x] B.4 Replace the `to_html()` call in the watch loop with
      `tx.unbounded_send(WatchMessage::Document)`
- [x] B.5 In `view::window()`, replace `glib::timeout_add_local` (`src/view.rs:1425`)
      with `glib::spawn_future_local` over the receiver; move the existing
      re-render / title / TOC body into the `WatchMessage::Document` arm unchanged
- [x] B.6 Keep the `force_render` path from `:set` working — have the command
      handler send `WatchMessage::Document` instead of setting the flag, and drop
      `force_render` from settings
- [x] B.7 Initial load in memory: call `markdown::build_html()` directly in
      `view::window()` and pass the string to `load_html()`; delete the
      `.temp.html` read (`src/view.rs:826`)
- [x] B.8 Delete `strip_seed_scripts()` (`src/view.rs:19-39`) and its two unit tests
- [x] B.9 Delete `markdown::to_html()` and `markdown::to_file()`
      (`src/markdown.rs:438-509`); drop the `seed` and `seed_url` parameters from
      `build_html()` and the `#{SEEDURL}` / `#{INITIALSEED}` substitutions
- [x] B.10 Remove the seed poll, `checkSeed`, and the Ctrl+R keydown handler from
      `theme_src/theme1/bridge.js` (lines 1-38)
- [x] B.11 Remove the `<script>var seedUrl…</script>` block and the
      `<script src="bridge.js"></script>` include from
      `theme_src/theme1/template-src.html`; delete `bridge.js` (see design
      decision 5)
- [x] B.12 `make compthemes` to regenerate `asset/theme1/template.html`; confirm
      `make check-themes` passes
- [x] B.13 Remove the `.temp.html` and `.temp.seed` routes from `src/server.rs:38-40`
- [x] B.14 Delete `test_route_temp_html` and `test_route_temp_seed` from
      `tests/server_test.rs`
- [x] B.15 Update `test_build_html_replaces_placeholders` in `tests/markdown_test.rs`
      for the new `build_html()` signature; drop the seed assertions
- [x] B.16 Test: `WatchMessage::Document` is sent exactly once for a burst of three
      events inside the debounce window, and twice when they straddle it
- [x] B.17 Verify by hand: startup renders correctly with no `$TMPDIR/mip-*/.temp.*`
      files present; `:open <other file>` still reloads and rewatches; relative
      images and videos still resolve through `docroot`

## Phase C — Remove the remaining idle cost

- [x] C.1 Rewrite `is_system_dark()` (`src/lib.rs:8-25`) to read
      `org.gnome.desktop.interface color-scheme` via `gio::Settings`, guarded by
      `gio::SettingsSchemaSource::default()` + `lookup(…, true)`; fall back to the
      existing `gsettings` exec once at startup when the schema is absent
- [x] C.2 In `view::window()`, connect `changed::color-scheme` on the `gio::Settings`
      object to swap the theme class, replacing the polled check
      (`src/view.rs:1450-1465`); only active when the theme setting is `system`
- [x] C.3 Watch the active custom CSS file in the watcher and send
      `WatchMessage::Style`; handle it in the future by injecting into
      `#custom-css`, replacing the `stat` poll (`src/view.rs:1427-1447`)
- [x] C.4 Re-watch the new style file when `:set style` changes it at runtime
- [x] C.5 Move the watch loop from `tokio::spawn` (`src/main.rs:322`) to
      `std::thread::spawn`; leave only the warp server on the tokio runtime
- [x] C.9 Build the `Application` with `gio::ApplicationFlags::NON_UNIQUE` and make
      the render-receiver handover in `connect_activate` non-fatal — the shared
      application id made `activate` fire twice (see design decision 9)
- [x] C.6 Confirm no `glib::timeout_add*` or `glib::idle_add*`-with-`Continue`
      remains in `src/view.rs`
- [x] C.7 Test: `is_system_dark()` returns without aborting when the GSettings
      schema is unavailable (exercise the fallback path)
- [x] C.8 Verify by hand on a GNOME session: toggling the desktop colour scheme
      switches the preview theme immediately with no polling

## Spec and documentation updates

- [x] S.1 Add the new `live-reload` capability spec
- [x] S.2 Update `gtk4-webview`: WebView loads from an in-memory string, not
      `http://localhost:{port}/.temp.html`
- [x] S.3 Update `custom-styles`: CSS live-reload is watcher-driven and debounced
- [x] S.4 Update `theming`: add live system-theme switching, schema-absent fallback
- [x] S.5 Update `test-suite`: add watcher scenarios, remove the temp-route and
      `strip_seed_scripts()` scenarios
- [x] S.6 Add a CHANGELOG entry under a new version heading

## Verify

- [x] V.1 `make check` passes (`cargo fmt --check`, `cargo clippy -D warnings`,
      `make check-themes`, `cargo test`)
- [x] V.2 Idle CPU is 0.0% in `top` for a plain document, a math document, and a
      mermaid document
- [x] V.3 `strace -c -p <watch tid>` over 5 s shows no `openat` of the watched file
      while idle
- [x] V.4 Editing and saving the document updates the preview once per save, within
      ~100 ms, in both an in-place editor and a write-then-rename editor (`vim` with
      `backupcopy=no`)
- [x] V.5 `ls $TMPDIR/mip-*` contains only `docroot` — no `.temp.html`, no `.temp.seed`
- [x] V.6 No stale `$TMPDIR/mip-*` directory is left after a clean window close
- [x] V.7a Verified headlessly: the `katex/`, `mermaid/` and docroot-relative
      routes all answer 200 against a live instance; `render_page()` emits a
      complete document with the KaTeX/mermaid script tags and the custom-CSS
      hook; TOC extraction, `post_process_export()` and watcher retargeting for
      `:open` / `:set style` are covered by tests
- [x] V.7b Verified at the window by the maintainer: KaTeX formulae, mermaid
      diagrams, embedded media, the side TOC, print and `:export_html` all
      confirmed working (GTK4/Wayland cannot be screenshotted from the agent
      session, so this one needed a human)
