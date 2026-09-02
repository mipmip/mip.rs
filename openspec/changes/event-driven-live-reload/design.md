## Context

mip's live reload is built from two halves that were added at different times and
never joined up.

The **watcher half** is original: `watch()` runs a blocking `recv_timeout(200ms)`
loop, and on any filesystem event calls `markdown::to_html()`, which renders the
whole document into `$TMPDIR/mip-<pid>/.temp.html` plus a random 7-character token
in `.temp.seed`. Warp serves both. `bridge.js` polls `.temp.seed` over XHR every
500 ms and calls `location.reload()` when it changes.

The **in-process half** was added later to remove the reload flicker: a
`glib::timeout_add_local(500ms)` in `view::window()` reads `.temp.seed`, and when
it changes re-renders the markdown *in memory* with
`md_to_html_body_with_toc()` and injects the result via `evaluate_javascript`.
`strip_seed_scripts()` removes the browser-side poll before `load_html()` so the
two do not fight.

The result is that `.temp.html` is written on every render and read exactly once
(at startup), `.temp.seed` exists only as a change-notification token between a
worker thread and the GTK main thread, and `bridge.js`'s poll is dead code. The
`IN_OPEN` feedback loop is a direct consequence: the watcher's reaction to an
event is to open the file it is watching.

Relevant facts established while diagnosing:

- `build_html()` (`src/markdown.rs:410`) is already `pub` and returns a `String`.
  Nothing about the initial load requires disk.
- `strip_seed_scripts()` matches `<script>document.addEventListener("keydown"`,
  which is the first line of the *inlined* `bridge.js`, and cuts to the next
  `</script>`. It therefore deletes the entire inlined bridge, including the 94 KB
  highlight.js bundle.
- That bundle is never invoked: `hljs.initHighlightingOnLoad()` sits inside a
  `/* ... */` comment (`theme_src/theme1/bridge.js:44-51`). Syntax highlighting is
  not active today.
- `glib::MainContext::channel` was removed in glib 0.19. The project is on glib
  0.22, so the current idiomatic cross-thread pattern is an async channel plus
  `glib::spawn_future_local`.
- `futures-channel 0.3.32` is already in `Cargo.lock` as a glib dependency.
- `.temp.seed` was observed at **0 bytes** mid-loop: `fs::write` truncates before
  writing, so a reader can see an empty or partial token. This is a plausible cause
  of bean `mip.rs-1n46` (".temp.seed error").

## Goals / Non-Goals

**Goals**
- Idle CPU at 0%: no feedback loop, and no periodic timer of any kind.
- One change-detection path, not two.
- The reload decision becomes a pure function with tests, so this class of bug is
  caught by `cargo test` rather than by `top`.
- Live reload stays fully automatic — no user-visible behaviour change.

**Non-Goals**
- Changing when or how the document *looks* updated. Rendering, TOC, and title
  behaviour are unchanged.
- Restoring syntax highlighting. highlight.js is dead code today; removing it is
  in scope, re-adding highlighting is a separate change.
- Removing the warp server. It is still needed for `katex/`, `mermaid/`, and the
  `docroot` symlink that makes relative image and video paths resolve.
- Watching anything outside the document's directory and the active style file.

## Decisions

### 1. Filter on `event.kind` with an allowlist, not a denylist

**Choice**: rerender for `Create(_)`, `Remove(_)` and any `Modify(_)` except
`Modify(Metadata(_))`. Everything else — all of `Access(_)`, plus
`EventKind::Any`/`Other` — is ignored.

Accepting the whole `Modify(_)` family rather than enumerating `Data` and `Name`
is deliberate: if notify ever reports a content change as `Modify(Any)` on some
filesystem, an enumerated allowlist would silently stop reloading, whereas the
loop-safety property only requires that `Access(_)` never triggers a render.
Reading a file cannot produce a `Modify(_)` event.

**Why**: a denylist ("everything except `Access(Open)`") leaves the next
read-triggered event kind free to reintroduce the loop. Note that `IN_CLOSE_WRITE`
maps to `Access(Close(Write))`, which *is* a genuine save signal but sits under
`Access`; it is safe to drop because `IN_MODIFY` → `Modify(Data(_))` also fires on
every in-place save. Editors that write-then-rename (vim with `backupcopy=no`)
emit `IN_CREATE`/`IN_MOVED_TO` → `Create(_)`/`Modify(Name(To))`, which is why the
watch must stay on the **directory**, not the file. It already does.

### 2. Debounce at 100 ms, coalescing to a single render

**Choice**: events that survive the filter set a pending flag; the render fires
100 ms after the last surviving event.

**Why**: one save typically emits `IN_MODIFY` + `IN_CLOSE_WRITE` + `IN_ATTRIB`, and
write-then-rename emits a `Create`/`Rename` pair. Without coalescing that is three
renders per keystroke-save in editors with autosave. 100 ms is below the threshold
of perception for a preview and comfortably above a single editor's write burst.

Implemented in-loop with `recv_timeout` rather than by adding
`notify-debouncer-full`: the loop already has a timeout-driven shape, and the
dependency would pull in new crates for ~15 lines of logic.

### 3. Replace the seed file with a typed channel to the GTK main loop

**Choice**:

```rust
enum WatchMessage {
    Document,          // the watched markdown file changed
    Style,             // the active custom CSS file changed
}
```

sent over `futures_channel::mpsc::UnboundedSender`, received in
`glib::spawn_future_local` inside `view::window()`.

**Why**: the seed file exists solely because the watcher thread cannot touch
non-`Send` GTK widgets. A channel solves that directly — only the message crosses
the thread boundary, the widgets stay captured in the local future. `MainContext`
channels were removed in glib 0.19, and `MainContext::invoke`/`idle_add_once`
require `Send` closures, which the `WebView`/`Window`/`TreeStore` are not.
`futures-channel` is already in the tree, so this costs one manifest line.

**Consequence**: with no seed to signal through, `to_html()`, `to_file()`, the
random seed, `.temp.html`, `.temp.seed`, and both warp routes have no remaining
purpose and are deleted rather than refactored.

```
 ── BEFORE ──────────────────────────────────────────────────────────────
  watcher thread                          GTK main thread
  (blocking, on a tokio worker)           glib::timeout 500ms  <-- forever
       |                                       |
       | to_html()                             |- read .temp.seed  <- signal
       |- read doc.md ---+                     |- stat style.css
       |                 | IN_OPEN             |- exec gsettings   <- 2/sec
       |  <--------------+  loop 1070x/sec     '- re-parse md, inject DOM
       |- write .temp.seed   (a token)
       '- write .temp.html   (104 KB, read by nobody)

 ── AFTER ───────────────────────────────────────────────────────────────
  watcher thread (std::thread)            GTK main thread
       |                                  spawn_future_local(async {
       |- filter on event.kind ---+         while let Some(m) = rx.next().await {
       |- debounce 100ms          |           |- Document -> re-parse, inject DOM
       '- tx.send(msg) -----------+---------> '- Style    -> inject <style>
                                            })
  gio::Settings "changed::color-scheme" --> handler -> swap theme class

                       idle: zero timers, zero wakeups, 0% CPU
```

### 4. Build the initial page in memory

**Choice**: `view::window()` calls `build_html()` directly and passes the result to
`load_html()`. No file is written or read.

**Why**: `build_html()` is already pure and public; the disk round-trip was only
there because `to_file()` was the single render entry point. Removing it also
removes the truncate-then-write race that was observed producing a 0-byte
`.temp.seed`.

### 5. Delete the `bridge.js` include rather than trimming it

**Choice**: remove the seed poll from `theme_src/theme1/bridge.js`, remove the
`<script>var seedUrl…</script>` block from `template-src.html`, and remove the
`<script src="bridge.js"></script>` include. `strip_seed_scripts()` and its two
tests are deleted.

**Why**: after the poll is gone, the only remaining content of `bridge.js` is a
highlight.js bundle whose initialiser is commented out — it is never invoked, and
`strip_seed_scripts()` deletes it before load anyway. Keeping the include would
inline 94 KB of never-executed JavaScript into every page (roughly doubling the
template), to no effect. This is behaviourally a no-op: nothing that runs today
stops running.

**Reversible**: if syntax highlighting is wanted later, it comes back as its own
change with a live initialiser and a `renderHighlight()` hook alongside the
existing `renderMath()`/`renderMermaid()` ones.

### 6. Guard `gio::Settings` behind a schema lookup

**Choice**:

```rust
gio::SettingsSchemaSource::default()
    .and_then(|src| src.lookup("org.gnome.desktop.interface", true))
```

before constructing `gio::Settings`. If the schema is absent, fall back to the
current `gsettings` exec — but **once at startup only**, never on a timer.

**Why**: `gio::Settings::new()` does not return an error when the schema is not
installed, it **aborts the process**. On non-GNOME desktops and on NixOS that is a
hard crash, not a degraded path. The fallback keeps today's behaviour for those
systems, minus the live switching they never had.

### 7. Move `watch()` onto a plain `std::thread`

**Choice**: drop the `tokio::spawn` wrapper; spawn the watch loop with
`std::thread::spawn` and leave only the warp server on the tokio runtime.

**Why**: `watch()` is a synchronous loop that never `.await`s, so
`tokio::spawn(async move { watch(...) })` permanently occupies a runtime worker
thread. It also made `top` blame a thread named `tokio-rt-worker`, which sent the
initial diagnosis toward the HTTP server. `spawn_blocking` would be correct too,
but the thread has no relationship to the async runtime at all.

### 8. Watch the directory non-recursively

**Choice**: `RecursiveMode::NonRecursive` on the document's parent directory.

**Why**: the watch has to be on the directory rather than the file so that
write-then-rename saves are seen (decision 1), but only the exact document path
is ever matched. A recursive watch therefore adds an inotify watch descriptor per
subdirectory and a stream of events that can never match — `mip README.md` at a
repo root was watching all of `target/` and `.git/`, turning any build into an
event storm.

### 9. Make the application NON_UNIQUE

**Choice**: build the `gtk4::Application` with
`gio::ApplicationFlags::NON_UNIQUE`, and make the render-channel handover in
`connect_activate` non-fatal when the receiver is already taken.

**Why**: found while verifying this change. `Application` was built with a
shared `application_id` and no flags, so GTK gave it single-instance semantics
over D-Bus. Launching `mip b.md` while `mip a.md` runs therefore does **not**
open `b`: the second process forwards an `Activate` call to the first and exits,
and the first fires `activate` a second time — building a duplicate window of
its *own* document. Verified against released v0.5.2: two launches leave one
live process.

mip is built one-process-per-document — a per-pid temp directory, its own server
port, its own watcher — so the uniqueness was never intended. It also breaks the
new code specifically: the render receiver can only be handed to one window, and
taking it twice took the process down with a non-unwinding panic. `NON_UNIQUE`
removes the double activation at the source; the graceful fallback ensures a
missing receiver can only cost that window its live reload, never the process.

## Risks / Trade-offs

- **A filtered event kind turns out to matter on some filesystem.** The allowlist
  is derived from inotify semantics. On a network or fuse mount, notify may report
  different kinds. Mitigation: `Remove(_)` and `Create(_)` are both in the
  allowlist, which covers the write-then-rename and delete-then-recreate patterns;
  the `:reload`/`:e` command remains as a manual escape hatch.
- **Debouncing adds up to 100 ms of latency** to a save-to-preview cycle that is
  currently immediate. Accepted: it is imperceptible in a preview, and today's
  "immediate" is immediate 1070 times over.
- **No timer means a missed event is permanent** until the next event, where
  previously the 500 ms poll would eventually notice. Mitigation: the watch is on
  the directory and covers create/rename/remove, so the realistic miss modes
  (editor swap files, atomic rename) are covered. `:reload` covers the rest.
- **Deleting `.temp.html` removes an accidental debugging affordance** — you could
  previously `cat` the rendered HTML. Nothing depends on it, and export-to-HTML
  captures the live DOM rather than reading this file.

## Migration Plan

Three commits, each green on its own:

1. **A** — event filter + canonical path match + `should_rerender()` tests. Fixes
   the 100% CPU. Shippable as a patch release without any of B or C.
2. **B** — channel, in-memory initial load, deletion of the seed/temp-file/route/
   `bridge.js` machinery, spec and test updates.
3. **C** — `gio::Settings` theme signal, CSS watching folded into the watcher,
   `watch()` onto `std::thread`. Removes the last timer.

No config, CLI, or user-visible behaviour changes in any phase, so there is no
migration for users and no changelog entry beyond the fix itself.

## Open Questions

None blocking. Decision 5 (dropping the dead `bridge.js` include) is the one
judgement call that is reversible without cost if the preference is to keep the
bundle in place.
