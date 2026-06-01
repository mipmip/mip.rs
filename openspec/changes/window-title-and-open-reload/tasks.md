## 1. Extract frontmatter title

- [x] 1.1 Update `md_to_html_body_with_toc` to return `Option<String>` for frontmatter title (third tuple element)
- [x] 1.2 Extract `title` from `result.data` Pod hash if present
- [x] 1.3 Update all call sites for the new return type

## 2. Window title

- [x] 2.1 Set initial window title to `<title or filename> - MiP` after first render
- [x] 2.2 Update window title in the poll loop when document content changes
- [x] 2.3 Add `window` reference to CommandContext (already present)
- [x] 2.4 Store current filename in RuntimeSettings as `RefCell<String>`

## 3. Refactor leaked static strings

- [x] 3.1 Remove `string_to_static_str` / `Box::leak` usage in main.rs
- [x] 3.2 Change `run_bro` to take owned `String` instead of `&'static str`
- [x] 3.3 Change `watch` function to take owned `String` / `PathBuf`

## 4. Watcher restart on :open

- [x] 4.1 Add a channel (`mpsc::Sender<PathBuf>`) for sending new file paths to the watcher thread
- [x] 4.2 In the watcher loop: check channel for new paths, restart watching the new directory
- [x] 4.3 Pass the sender to CommandContext for `:open` to use

## 5. Server directory change on :open

- [x] 5.1 Create a `docroot` symlink in the temp directory pointing to the document's parent directory
- [x] 5.2 Serve from the symlink path instead of the raw document directory
- [x] 5.3 On `:open` to a different directory: update the symlink target
- [x] 5.4 Handle symlink creation failure gracefully (warn, continue)

## 6. In-process :open command

- [x] 6.1 Add `infile` to RuntimeSettings as `RefCell<String>`
- [x] 6.2 Rewrite `:open` handler: update infile in settings, update symlink, send new path to watcher channel, set force_render, update window title
- [x] 6.3 Remove the old spawn-new-process + quit logic
- [x] 6.4 Poll loop reads infile from settings instead of captured variable

## 7. Tests

- [x] 7.1 Test frontmatter title extraction: title present, title missing, no frontmatter
- [x] 7.2 Test window title format: `<title> - MiP`, `<filename> - MiP`
- [x] 7.3 Test `run_bro` with owned String (existing server tests should still pass)

## 8. Verify

- [x] 8.1 `cargo build` succeeds
- [x] 8.2 `cargo test` passes
- [x] 8.3 Window title shows frontmatter title when present
- [x] 8.4 Window title shows filename when no frontmatter title
- [x] 8.5 Title updates on live-reload (edit frontmatter title)
- [x] 8.6 `:open examples/with-front-matter.md` reloads in-place
- [x] 8.7 `:open` to file in different directory: images still render
- [x] 8.8 Runtime settings preserved after `:open`
- [x] 8.9 File watcher picks up changes to newly opened file
