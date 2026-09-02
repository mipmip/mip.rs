## Why

mip burns **100% of one CPU core, permanently, on any document** — including a
three-line file in an empty directory, with nothing touching it. Measured and
reproduced on v0.5.2.

The cause is an inotify feedback loop. `watch()` (`src/main.rs:83-124`) reacts to
every notify event on the watched file and never inspects `event.kind`. On Linux,
`notify` subscribes to an inotify mask of `0xfee` — every event except `IN_ACCESS`,
which includes **`IN_OPEN` (0x20)**. And `to_html()` *opens the markdown file to
read it*:

```
to_html() → read_to_string(file) → inotify IN_OPEN → event → to_html() → …
```

Self-feeding, rate-limited only by how long a full render takes: **~1070 renders
per second**, each parsing the markdown and writing a 104 KB `.temp.html` that
nothing reads.

Evidence:

| Probe                              | Result                                             |
|------------------------------------|----------------------------------------------------|
| `top -H -p <pid>`                  | one `tokio-rt-worker` at 94-101%, state R          |
| `/proc/<tid>/wchan`, `syscall`     | `0`, `running` — userspace spin, not blocked       |
| `strace -c -p <tid>`, 5s           | 16716 `openat`, 11143 `write`, 5572 `statx`        |
| `strace` filenames, 2s             | 2143x read the `.md`, 2143x write `.temp.html`     |
| `/proc/<pid>/fdinfo/<inotify fd>`  | `mask:fee` — `IN_OPEN` is subscribed               |
| raw inotify read buffer            | `wd=1 mask=0x20 cookie=0 len=16 "with-math.md"`    |
| froze `WebKitWebProcess`           | CPU stayed at 100% — unrelated to the webview      |

The loop survived to release because `watch()` and the reload poll are the only
untested code in the project: `watch()` is private to the `main.rs` binary crate
(unreachable from `tests/`) and the poll is an anonymous closure inside
`window()`. 248 tests, none touching either.

The deeper problem is architectural. There are **two parallel change-detection
systems**: a filesystem watcher that renders to disk, and a 500 ms GTK poll that
re-renders in memory and ignores the disk output. The watcher's product is thrown
away; the poll's trigger is a random token the watcher writes to a file. Using a
file as an IPC channel is what forced a poll on the other end.

Bean: [mip.rs-0nha](/home/pim/gh.mipmip/mip.rs/.beans/mip.rs-0nha--mip-is-using-a-lot-of-cpu.md)

## What Changes

Three independently shippable phases.

**Phase A — stop the loop.** Extract a pure `should_rerender(&Event, &Path) -> bool`
and allowlist event kinds: `Modify(Data)`, `Modify(Name)`, `Create`, `Remove`.
Never `Access(_)`, never `Modify(Metadata(_))`. Replace the
`teststr.contains(&current_file)` substring match with canonicalized path
equality. ~20 lines, releasable on its own.

**Phase B — collapse the two change-detection paths.** The watcher sends a typed
message over a channel to the GTK main loop; the 500 ms poll is deleted. With the
seed no longer needed as a signal, `to_html()`, `to_file()`, the random seed,
`.temp.html`, `.temp.seed`, both warp routes, `strip_seed_scripts()` and the dead
`bridge.js` poll all go away. The initial page is built in memory via the existing
`build_html()`.

**Phase C — remove the remaining idle cost.** `is_system_dark()` currently
fork/execs `gsettings` from inside the 500 ms timer — two process spawns per
second, forever. Replace with `gio::Settings` and a `changed::color-scheme`
signal. Fold custom-CSS watching into the same watcher instead of `stat`-polling
it. Move `watch()` off the tokio worker it permanently blocks onto a plain
`std::thread`.

End state: **zero periodic timers at idle**, 0% CPU.

## Capabilities

### New Capabilities
- `live-reload`: the document/CSS change-detection pipeline — event filtering,
  debouncing, and delivery to the GTK main loop. Currently unspecified anywhere.

### Modified Capabilities
- `gtk4-webview`: the WebView is loaded from an in-memory HTML string instead of
  `http://localhost:{port}/.temp.html`.
- `custom-styles`: CSS live-reload becomes watcher-driven rather than polled, and
  is specified as debounced rather than "within ~500ms".
- `theming`: adds a requirement for live system-theme switching (existing but
  unspecified behaviour), now event-driven and safe when the GSettings schema is
  absent.
- `test-suite`: adds watcher tests, removes the `.temp.html`/`.temp.seed` route
  and `strip_seed_scripts()` scenarios.

## Impact

- `src/main.rs`: `watch()` gains kind filtering, debouncing, canonical path
  matching, and a channel sender; moves to `std::thread`. Logic extracted into
  `src/watch.rs` so it is reachable from `tests/`.
- `src/watch.rs` (new): `should_rerender()`, `Debouncer`, `WatchMessage`, and the
  watch loop — pure and testable.
- `src/view.rs`: `glib::timeout_add_local` replaced by `glib::spawn_future_local`
  over the channel receiver; `strip_seed_scripts()` deleted; initial load from
  `build_html()`; `gio::Settings` theme signal.
- `src/markdown.rs`: `to_html()` and `to_file()` deleted; `build_html()` loses its
  `seed`/`seed_url` parameters.
- `src/lib.rs`: `is_system_dark()` becomes a schema-guarded `gio::Settings` read
  with the current `gsettings` exec as a one-shot fallback.
- `src/server.rs`: `.temp.html` and `.temp.seed` routes removed. The server stays
  for `katex/`, `mermaid/` and the `docroot` symlink.
- `theme_src/theme1/{template-src.html,bridge.js}`: seed script block and the dead
  seed poll removed; `make compthemes` regenerates `asset/theme1/template.html`.
- `tests/`: 2 server tests and 2 seed assertions removed; new `tests/watch_test.rs`.
- Adds `futures-channel = "0.3"` to `Cargo.toml` — already in the lock tree as a
  glib dependency, so no new crate is compiled.
