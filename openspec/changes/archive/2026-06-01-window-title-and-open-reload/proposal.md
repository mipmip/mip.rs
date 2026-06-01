## Why

The window title is hardcoded to "MiP" — there's no way to tell which file you're viewing. And `:open` spawns a new process instead of reloading in-place, which is clumsy and loses runtime settings.

Bean: mip.rs-tnv1

## What Changes

### Window title
- Show `<title or filename> - MiP` in the window title bar
- If YAML frontmatter has a `title` field, use that
- Otherwise use the filename (e.g. `README.md`)
- Update on live-reload when frontmatter title changes
- Update when `:open` switches files

### In-process `:open` reload
- `:open <path>` reloads the document in the same window instead of spawning a new process
- Update the file path in RuntimeSettings
- Re-render the document from the new file
- Restart the file watcher on the new directory
- Update the warp server's asset directory for image serving
- The temp directory stays the same (it's process-scoped)

## Capabilities

### New Capabilities
- `window-title`: Dynamic window title from frontmatter title or filename

### Modified Capabilities
- `command-mode`: `:open` reloads in-process instead of spawning

## Impact

- `src/view.rs`: set window title from frontmatter/filename, update on render, add infile to RuntimeSettings
- `src/main.rs`: refactor watcher/server to use shared paths (Arc<Mutex> or channels), remove `string_to_static_str` leaks
- `src/server.rs`: support changing the document directory at runtime
- `src/markdown.rs`: extract frontmatter title as part of the render result
