//! Filesystem watching for live reload.
//!
//! The decision of *whether* a filesystem event warrants a rerender lives in
//! [`should_rerender`], separate from the loop that acts on it, so it can be
//! unit-tested. That separation matters: mip previously reacted to every event
//! notify reported, and because rendering opens the watched file to read it,
//! inotify's `IN_OPEN` fed straight back into another render — a self-sustaining
//! loop that pegged a CPU core for the lifetime of the process.

use notify::event::ModifyKind;
use notify::{Event, EventKind, RecommendedWatcher, RecursiveMode, Watcher};
use std::path::{Path, PathBuf};
use std::sync::mpsc::{Receiver, RecvTimeoutError, channel};
use std::time::{Duration, Instant};

/// How long the watch loop waits between checks for a new document path.
const TICK: Duration = Duration::from_millis(200);

/// How long to wait for the event stream to go quiet before rendering.
///
/// One save typically emits several events — an in-place write produces
/// `IN_MODIFY` twice (truncate, then write) plus `IN_CLOSE_WRITE`, and a
/// write-then-rename save produces a create/rename pair. Coalescing them into
/// one render is the difference between one render per save and three.
pub const DEBOUNCE: Duration = Duration::from_millis(100);

/// What the watcher tells the GTK main loop to do.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WatchMessage {
    /// The watched markdown document changed.
    Document,
    /// The active custom CSS file changed.
    Style,
}

/// What the GTK main loop tells the watcher to watch.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WatchControl {
    /// `:open` switched to a different document.
    Document(PathBuf),
    /// `:set style` changed (or cleared) the active custom CSS file.
    Style(Option<PathBuf>),
}

/// Does this event kind represent a change to a file's content or existence?
///
/// `Access(..)` is rejected unconditionally: rendering reads the watched file,
/// so treating a read as a change is a feedback loop. `Modify(Metadata(..))` is
/// rejected because permission and timestamp churn is not a content change.
pub fn is_change_kind(kind: &EventKind) -> bool {
    match kind {
        EventKind::Access(_) => false,
        EventKind::Modify(ModifyKind::Metadata(_)) => false,
        EventKind::Modify(_) | EventKind::Create(_) | EventKind::Remove(_) => true,
        EventKind::Any | EventKind::Other => false,
    }
}

/// Should `event` cause `watched` to be rerendered?
///
/// True only when the event is a content/existence change *and* names exactly
/// `watched`. The path comparison is by equality, not substring: a sibling
/// `doc.md.bak` or a nested `notes/doc.md` must not trigger a rerender of
/// `doc.md`.
pub fn should_rerender(event: &Event, watched: &Path) -> bool {
    is_change_kind(&event.kind) && event.paths.iter().any(|p| p == watched)
}

/// Keep `watcher` subscribed to exactly the directories in `wanted`.
fn resync(watcher: &mut RecommendedWatcher, watched: &mut Vec<PathBuf>, wanted: Vec<PathBuf>) {
    for dir in watched.iter() {
        if !wanted.contains(dir) {
            let _ = watcher.unwatch(dir);
        }
    }
    for dir in wanted.iter() {
        if !watched.contains(dir) {
            let _ = watcher.watch(dir, RecursiveMode::NonRecursive);
        }
    }
    *watched = wanted;
}

/// The directories that must be watched to see changes to `doc` and `style`.
fn wanted_dirs(doc: &Path, style: Option<&PathBuf>) -> Vec<PathBuf> {
    let mut dirs = Vec::with_capacity(2);
    if let Some(parent) = doc.parent() {
        dirs.push(parent.to_path_buf());
    }
    if let Some(parent) = style.and_then(|p| p.parent()) {
        let parent = parent.to_path_buf();
        if !dirs.contains(&parent) {
            dirs.push(parent);
        }
    }
    dirs
}

/// Watch the document and the active custom CSS file, calling `on_change` when
/// either changes.
///
/// Files are watched through their parent directory so that write-then-rename
/// saves are seen; only the exact paths are matched. Control messages arriving
/// on `control_rx` retarget the watch when `:open` or `:set style` runs.
pub fn run<F>(
    doc: PathBuf,
    style: Option<PathBuf>,
    control_rx: Receiver<WatchControl>,
    mut on_change: F,
) -> notify::Result<()>
where
    F: FnMut(WatchMessage),
{
    let (tx, rx) = channel();
    let mut watcher = RecommendedWatcher::new(tx, notify::Config::default())?;

    let mut doc = doc;
    let mut style = style;
    let mut watched: Vec<PathBuf> = Vec::new();
    resync(
        &mut watcher,
        &mut watched,
        wanted_dirs(&doc, style.as_ref()),
    );

    // Trailing-edge debounce state, tracked per target so a document save and a
    // stylesheet save cannot swallow one another.
    let mut pending_doc: Option<Instant> = None;
    let mut pending_style: Option<Instant> = None;

    loop {
        // Retarget on `:open` / `:set style`.
        while let Ok(control) = control_rx.try_recv() {
            match control {
                WatchControl::Document(path) => doc = canonical(&path),
                WatchControl::Style(path) => style = path.map(|p| canonical(&p)),
            }
            resync(
                &mut watcher,
                &mut watched,
                wanted_dirs(&doc, style.as_ref()),
            );
        }

        // While something is pending, wait only for the rest of its debounce
        // window. A zero wait returns immediately and the flush below fires; it
        // cannot spin, because flushing clears the pending slot.
        let wait = [pending_doc, pending_style]
            .into_iter()
            .flatten()
            .map(|since| DEBOUNCE.saturating_sub(since.elapsed()))
            .min()
            .unwrap_or(TICK);

        match rx.recv_timeout(wait) {
            Ok(Ok(event)) => {
                if should_rerender(&event, &doc) {
                    pending_doc = Some(Instant::now());
                } else if let Some(ref style) = style
                    && should_rerender(&event, style)
                {
                    pending_style = Some(Instant::now());
                }
            }
            Ok(Err(e)) => eprintln!("watch error: {:?}", e),
            Err(RecvTimeoutError::Timeout) => {}
            Err(RecvTimeoutError::Disconnected) => break,
        }

        if let Some(since) = pending_doc
            && since.elapsed() >= DEBOUNCE
        {
            pending_doc = None;
            on_change(WatchMessage::Document);
        }
        if let Some(since) = pending_style
            && since.elapsed() >= DEBOUNCE
        {
            pending_style = None;
            on_change(WatchMessage::Style);
        }
    }
    Ok(())
}

/// Resolve `path` to the absolute form inotify reports, without requiring the
/// file itself to exist (a remove-then-recreate save briefly deletes it).
pub fn canonical(path: &Path) -> PathBuf {
    let parent = path.parent().filter(|p| !p.as_os_str().is_empty());
    match (parent, path.file_name()) {
        (Some(parent), Some(name)) => match std::fs::canonicalize(parent) {
            Ok(dir) => dir.join(name),
            Err(_) => path.to_path_buf(),
        },
        _ => std::fs::canonicalize(path).unwrap_or_else(|_| path.to_path_buf()),
    }
}
