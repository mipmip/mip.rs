## Context

The window title is hardcoded to "MiP". The watcher and server use `&'static str` paths created via `Box::leak`, making them immutable at runtime. `:open` currently spawns a new mip process and quits. `gray_matter` already parses frontmatter but the `title` field isn't extracted.

## Goals / Non-Goals

**Goals:**
- Dynamic window title: frontmatter `title` or filename, updated on reload
- `:open` reloads in-place, preserving runtime settings
- File watcher restarts on new directory
- Server serves images from the new document's directory

**Non-Goals:**
- Multiple documents open at once (tabs/splits)
- Undo/history of opened files

## Decisions

### Window title format

**Choice**: `<title> - MiP` when frontmatter has `title`, `<filename> - MiP` otherwise.

**Rationale**: Standard convention (like `vim`, `code`, browsers). The app name at the end is consistent.

### Extract frontmatter title in markdown.rs

**Choice**: Add `document_title` field to the return from `md_to_html_body_with_toc` — return `(String, Vec<TocEntry>, Option<String>)` where the third element is the frontmatter title if present.

**Rationale**: The frontmatter is already parsed. Extracting one field is trivial. Returning it alongside the HTML avoids re-parsing.

### Infile path in RuntimeSettings

**Choice**: Add `infile: RefCell<String>` to RuntimeSettings. The poll loop reads from it. `:open` updates it and sets `force_render = true`.

**Rationale**: Same pattern as other runtime settings. The poll loop already reads settings from the context.

### Watcher restart via channel

**Choice**: The watcher thread receives file paths via a channel (`mpsc::Sender<PathBuf>`). When `:open` changes the file, send the new path. The watcher loop checks for new paths alongside file events, and restarts watching the new directory.

**Alternative considered**: Kill and respawn the watcher thread. Messier — channels are cleaner.

### Server directory change

**Choice**: The warp server serves the temp directory (fixed) and the document directory (needs to change). Since warp's `fs::dir` captures the path at creation time, we can't change it dynamically.

**Pragmatic solution**: Use a symlink. Create a symlink `<temp_dir>/docroot` → document's parent directory. The server serves from the symlink path. When `:open` changes the file, update the symlink target.

**Alternative considered**: Restart the server. Too disruptive — WebView would lose connection. The symlink approach is transparent to warp.

### Removing Box::leak

**Choice**: Replace `string_to_static_str` / `Box::leak` with owned `String` passed to threads. The server function takes `String` instead of `&'static str`. The watcher takes owned paths.

**Rationale**: `Box::leak` was a shortcut. Owned strings are cleaner and support the new architecture.

## Risks / Trade-offs

- [Symlink on temp filesystem] → Some temp filesystems might not support symlinks. Mitigation: fall back to copying or warn if symlink fails.
- [Watcher channel complexity] → Adds a communication channel to the watcher thread. Mitigation: simple `try_recv` in the existing event loop.
- [Race condition on `:open`] → File might be read before watcher is updated. Mitigation: `force_render` ensures a re-render after settings change; watcher catches subsequent edits.
