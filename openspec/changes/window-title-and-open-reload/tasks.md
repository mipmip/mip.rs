## 1. Extract frontmatter title

- [ ] 1.1 Update `md_to_html_body_with_toc` to return `Option<String>` for frontmatter title (third tuple element)
- [ ] 1.2 Extract `title` from `result.data` Pod hash if present
- [ ] 1.3 Update all call sites for the new return type

## 2. Window title

- [ ] 2.1 Set initial window title to `<title or filename> - MiP` after first render
- [ ] 2.2 Update window title in the poll loop when document content changes
- [ ] 2.3 Add `window` reference to CommandContext (already present)
- [ ] 2.4 Store current filename in RuntimeSettings as `RefCell<String>`

## 3. Refactor leaked static strings

- [ ] 3.1 Remove `string_to_static_str` / `Box::leak` usage in main.rs
- [ ] 3.2 Change `run_bro` to take owned `String` instead of `&'static str`
- [ ] 3.3 Change `watch` function to take owned `String` / `PathBuf`

## 4. Watcher restart on :open

- [ ] 4.1 Add a channel (`mpsc::Sender<PathBuf>`) for sending new file paths to the watcher thread
- [ ] 4.2 In the watcher loop: check channel for new paths, restart watching the new directory
- [ ] 4.3 Pass the sender to CommandContext for `:open` to use

## 5. Server directory change on :open

- [ ] 5.1 Create a `docroot` symlink in the temp directory pointing to the document's parent directory
- [ ] 5.2 Serve from the symlink path instead of the raw document directory
- [ ] 5.3 On `:open` to a different directory: update the symlink target
- [ ] 5.4 Handle symlink creation failure gracefully (warn, continue)

## 6. In-process :open command

- [ ] 6.1 Add `infile` to RuntimeSettings as `RefCell<String>`
- [ ] 6.2 Rewrite `:open` handler: update infile in settings, update symlink, send new path to watcher channel, set force_render, update window title
- [ ] 6.3 Remove the old spawn-new-process + quit logic
- [ ] 6.4 Poll loop reads infile from settings instead of captured variable

## 7. Tests

- [ ] 7.1 Test frontmatter title extraction: title present, title missing, no frontmatter
- [ ] 7.2 Test window title format: `<title> - MiP`, `<filename> - MiP`
- [ ] 7.3 Test `run_bro` with owned String (existing server tests should still pass)

## 8. Verify

- [ ] 8.1 `cargo build` succeeds
- [ ] 8.2 `cargo test` passes
- [ ] 8.3 Window title shows frontmatter title when present
- [ ] 8.4 Window title shows filename when no frontmatter title
- [ ] 8.5 Title updates on live-reload (edit frontmatter title)
- [ ] 8.6 `:open examples/with-front-matter.md` reloads in-place
- [ ] 8.7 `:open` to file in different directory: images still render
- [ ] 8.8 Runtime settings preserved after `:open`
- [ ] 8.9 File watcher picks up changes to newly opened file
