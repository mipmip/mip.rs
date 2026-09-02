use mip::watch::{WatchControl, WatchMessage, canonical, is_change_kind, should_rerender};
use notify::event::{
    AccessKind, AccessMode, CreateKind, DataChange, MetadataKind, ModifyKind, RemoveKind,
    RenameMode,
};
use notify::{Event, EventKind};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::thread;
use std::time::{Duration, Instant};

const WATCHED: &str = "/tmp/mip-watch-test/doc.md";

fn event(kind: EventKind, path: &str) -> Event {
    Event::new(kind).add_path(PathBuf::from(path))
}

// ---------------------------------------------------------------- event kinds

#[test]
fn access_open_does_not_rerender() {
    // The regression this whole module exists for: rendering opens the watched
    // file, so reacting to IN_OPEN is a self-sustaining render loop.
    let e = event(
        EventKind::Access(AccessKind::Open(AccessMode::Any)),
        WATCHED,
    );
    assert!(!should_rerender(&e, Path::new(WATCHED)));
}

#[test]
fn every_access_kind_is_rejected() {
    for kind in [
        AccessKind::Any,
        AccessKind::Read,
        AccessKind::Open(AccessMode::Any),
        AccessKind::Open(AccessMode::Read),
        AccessKind::Close(AccessMode::Read),
        AccessKind::Close(AccessMode::Write),
        AccessKind::Other,
    ] {
        assert!(
            !is_change_kind(&EventKind::Access(kind)),
            "Access({kind:?}) must not trigger a rerender"
        );
    }
}

#[test]
fn modify_data_rerenders() {
    let e = event(
        EventKind::Modify(ModifyKind::Data(DataChange::Any)),
        WATCHED,
    );
    assert!(should_rerender(&e, Path::new(WATCHED)));
}

#[test]
fn modify_name_rerenders() {
    // Write-then-rename saves (vim with backupcopy=no) land here.
    let e = event(EventKind::Modify(ModifyKind::Name(RenameMode::To)), WATCHED);
    assert!(should_rerender(&e, Path::new(WATCHED)));
}

#[test]
fn create_rerenders() {
    let e = event(EventKind::Create(CreateKind::File), WATCHED);
    assert!(should_rerender(&e, Path::new(WATCHED)));
}

#[test]
fn remove_rerenders() {
    let e = event(EventKind::Remove(RemoveKind::File), WATCHED);
    assert!(should_rerender(&e, Path::new(WATCHED)));
}

#[test]
fn modify_metadata_does_not_rerender() {
    for kind in [
        MetadataKind::Any,
        MetadataKind::AccessTime,
        MetadataKind::WriteTime,
        MetadataKind::Permissions,
        MetadataKind::Ownership,
    ] {
        assert!(
            !is_change_kind(&EventKind::Modify(ModifyKind::Metadata(kind))),
            "Modify(Metadata({kind:?})) must not trigger a rerender"
        );
    }
}

#[test]
fn ambiguous_event_kinds_do_not_rerender() {
    assert!(!is_change_kind(&EventKind::Any));
    assert!(!is_change_kind(&EventKind::Other));
}

// ---------------------------------------------------------------- path matching

#[test]
fn sibling_with_matching_prefix_does_not_rerender() {
    // `contains()` matching used to fire for this.
    let e = event(
        EventKind::Modify(ModifyKind::Data(DataChange::Any)),
        "/tmp/mip-watch-test/doc.md.bak",
    );
    assert!(!should_rerender(&e, Path::new(WATCHED)));
}

#[test]
fn same_filename_in_subdirectory_does_not_rerender() {
    let e = event(
        EventKind::Modify(ModifyKind::Data(DataChange::Any)),
        "/tmp/mip-watch-test/notes/doc.md",
    );
    assert!(!should_rerender(&e, Path::new(WATCHED)));
}

#[test]
fn unrelated_path_does_not_rerender() {
    let e = event(
        EventKind::Modify(ModifyKind::Data(DataChange::Any)),
        "/tmp/mip-watch-test/other.md",
    );
    assert!(!should_rerender(&e, Path::new(WATCHED)));
}

#[test]
fn multi_path_event_matches_on_any_path() {
    // Rename events carry both the old and the new path.
    let e = Event::new(EventKind::Modify(ModifyKind::Name(RenameMode::Both)))
        .add_path(PathBuf::from("/tmp/mip-watch-test/tmpfile"))
        .add_path(PathBuf::from(WATCHED));
    assert!(should_rerender(&e, Path::new(WATCHED)));
}

#[test]
fn canonical_resolves_a_relative_path_without_requiring_the_file() {
    // The document may be given relative on the command line, while inotify
    // always reports absolute paths. A remove-then-recreate save also means the
    // file may not exist at the moment we resolve it.
    let dir = tempfile::tempdir().unwrap();
    let canon_dir = std::fs::canonicalize(dir.path()).unwrap();
    let missing = canon_dir.join("not-created-yet.md");

    assert_eq!(canonical(&missing), missing);
    assert_eq!(
        canonical(&canon_dir.join("./doc.md")),
        canon_dir.join("doc.md")
    );
}

// ---------------------------------------------------------------- the live loop

/// Start the real watch loop over `doc`, returning a counter of how many
/// document rerenders it asked for. The callback reads the watched file,
/// exactly as rendering does — that read is what used to feed the loop.
fn spawn_watcher(doc: &Path) -> Arc<AtomicUsize> {
    spawn_watcher_with_style(doc, None).0
}

/// As [`spawn_watcher`], also watching `style` and counting its rerenders
/// separately. Returns `(document_count, style_count)`.
fn spawn_watcher_with_style(
    doc: &Path,
    style: Option<PathBuf>,
) -> (Arc<AtomicUsize>, Arc<AtomicUsize>) {
    let docs = Arc::new(AtomicUsize::new(0));
    let styles = Arc::new(AtomicUsize::new(0));
    let (doc_counter, style_counter) = (docs.clone(), styles.clone());
    let doc = doc.to_path_buf();
    let watched_doc = doc.clone();
    thread::spawn(move || {
        // The sender lives in the thread, so the loop never sees a control message.
        let (_tx, rx) = std::sync::mpsc::channel();
        let _ = mip::watch::run(doc, style, rx, move |message| match message {
            WatchMessage::Document => {
                let _ = std::fs::read_to_string(&watched_doc);
                doc_counter.fetch_add(1, Ordering::SeqCst);
            }
            WatchMessage::Style => {
                style_counter.fetch_add(1, Ordering::SeqCst);
            }
        });
    });
    // Give the watcher time to establish its inotify watches.
    thread::sleep(Duration::from_millis(400));
    (docs, styles)
}

/// How long to allow for an expected render to arrive. Generous, because these
/// tests also run inside a parallel release build where the watcher thread can
/// be starved for a while; the assertions are on counts, never on timing.
const SETTLE: Duration = Duration::from_secs(5);

/// How long to keep watching after the expected count is reached, to catch a
/// render that should *not* have happened.
const QUIET: Duration = Duration::from_millis(700);

/// Wait until `counter` reaches `want`, then confirm it stays there.
fn settled_at(counter: &AtomicUsize, want: usize) -> usize {
    let deadline = Instant::now() + SETTLE;
    while Instant::now() < deadline && counter.load(Ordering::SeqCst) < want {
        thread::sleep(Duration::from_millis(20));
    }
    thread::sleep(QUIET);
    counter.load(Ordering::SeqCst)
}

/// Confirm `counter` stays at `want` for the quiet period. Used to prove a
/// render did *not* happen, where there is nothing to wait for.
fn stays_at(counter: &AtomicUsize, want: usize) -> usize {
    thread::sleep(SETTLE.min(Duration::from_millis(1500)));
    assert_eq!(counter.load(Ordering::SeqCst), want);
    counter.load(Ordering::SeqCst)
}

fn temp_doc() -> (tempfile::TempDir, PathBuf) {
    let dir = tempfile::tempdir().unwrap();
    let canon = std::fs::canonicalize(dir.path()).unwrap();
    let doc = canon.join("doc.md");
    std::fs::write(&doc, "# hello\n").unwrap();
    (dir, doc)
}

#[test]
fn reading_the_watched_file_never_triggers_a_rerender() {
    let (_dir, doc) = temp_doc();
    let count = spawn_watcher(&doc);

    for _ in 0..20 {
        std::fs::read_to_string(&doc).unwrap();
    }

    assert_eq!(
        stays_at(&count, 0),
        0,
        "reading the watched file must not trigger a rerender"
    );
}

#[test]
fn writing_the_watched_file_triggers_exactly_one_rerender() {
    let (_dir, doc) = temp_doc();
    let count = spawn_watcher(&doc);

    std::fs::write(&doc, "# changed\n").unwrap();

    assert_eq!(
        settled_at(&count, 1),
        1,
        "a write must trigger exactly one rerender"
    );
}

#[test]
fn a_rerender_does_not_cascade() {
    // The loop test: after one write settles, the render's own read of the file
    // must not have produced further rerenders.
    let (_dir, doc) = temp_doc();
    let count = spawn_watcher(&doc);

    std::fs::write(&doc, "# changed\n").unwrap();
    let settled = settled_at(&count, 1);

    thread::sleep(Duration::from_millis(1500));
    assert_eq!(
        count.load(Ordering::SeqCst),
        settled,
        "render count kept climbing with nothing touching the file"
    );
}

#[test]
fn a_sibling_file_does_not_trigger_a_rerender() {
    let (_dir, doc) = temp_doc();
    let parent = doc.parent().unwrap().to_path_buf();
    let count = spawn_watcher(&doc);

    std::fs::write(parent.join("doc.md.bak"), "backup").unwrap();
    std::fs::write(parent.join("other.md"), "# other\n").unwrap();

    assert_eq!(
        stays_at(&count, 0),
        0,
        "only the watched document should trigger a rerender"
    );
}

#[test]
fn a_burst_of_writes_coalesces_into_one_render() {
    let (_dir, doc) = temp_doc();
    let count = spawn_watcher(&doc);

    // Three saves well inside the 100ms debounce window. Each one also emits
    // several inotify events of its own, so without coalescing this would be
    // closer to nine renders than one.
    for i in 0..3 {
        std::fs::write(&doc, format!("# change {i}\n")).unwrap();
        thread::sleep(Duration::from_millis(10));
    }

    assert_eq!(
        settled_at(&count, 1),
        1,
        "a burst of saves must produce exactly one render"
    );
}

#[test]
fn writes_outside_the_debounce_window_render_separately() {
    let (_dir, doc) = temp_doc();
    let count = spawn_watcher(&doc);

    std::fs::write(&doc, "# first\n").unwrap();
    // Wait for the first render to actually land before the second save, so the
    // test measures the debounce rather than the scheduler.
    assert_eq!(settled_at(&count, 1), 1);

    std::fs::write(&doc, "# second\n").unwrap();
    assert_eq!(
        settled_at(&count, 2),
        2,
        "saves separated by more than the debounce window must render separately"
    );
}

// ---------------------------------------------------------------- style watching

#[test]
fn a_stylesheet_change_is_reported_separately() {
    let (_dir, doc) = temp_doc();
    let style_dir = tempfile::tempdir().unwrap();
    let style = std::fs::canonicalize(style_dir.path())
        .unwrap()
        .join("style.css");
    std::fs::write(&style, "body { color: red; }").unwrap();

    let (docs, styles) = spawn_watcher_with_style(&doc, Some(style.clone()));

    std::fs::write(&style, "body { color: blue; }").unwrap();

    assert_eq!(settled_at(&styles, 1), 1, "the stylesheet changed");
    assert_eq!(docs.load(Ordering::SeqCst), 0, "the document did not");
}

#[test]
fn document_and_stylesheet_debounce_independently() {
    let (_dir, doc) = temp_doc();
    let style_dir = tempfile::tempdir().unwrap();
    let style = std::fs::canonicalize(style_dir.path())
        .unwrap()
        .join("style.css");
    std::fs::write(&style, "body { color: red; }").unwrap();

    let (docs, styles) = spawn_watcher_with_style(&doc, Some(style.clone()));

    // Both change inside one debounce window; neither may swallow the other.
    std::fs::write(&doc, "# changed\n").unwrap();
    std::fs::write(&style, "body { color: blue; }").unwrap();

    assert_eq!(settled_at(&docs, 1), 1);
    assert_eq!(settled_at(&styles, 1), 1);
}

// ---------------------------------------------------------------- retargeting

/// Start the watch loop with a control channel, so the test can retarget it the
/// way `:open` and `:set style` do.
fn spawn_controllable(
    doc: &Path,
    style: Option<PathBuf>,
) -> (
    std::sync::mpsc::Sender<WatchControl>,
    Arc<AtomicUsize>,
    Arc<AtomicUsize>,
) {
    let (tx, rx) = std::sync::mpsc::channel();
    let docs = Arc::new(AtomicUsize::new(0));
    let styles = Arc::new(AtomicUsize::new(0));
    let (doc_counter, style_counter) = (docs.clone(), styles.clone());
    let doc = doc.to_path_buf();
    thread::spawn(move || {
        let _ = mip::watch::run(doc, style, rx, move |message| match message {
            WatchMessage::Document => {
                doc_counter.fetch_add(1, Ordering::SeqCst);
            }
            WatchMessage::Style => {
                style_counter.fetch_add(1, Ordering::SeqCst);
            }
        });
    });
    thread::sleep(Duration::from_millis(400));
    (tx, docs, styles)
}

#[test]
fn opening_another_document_moves_the_watch() {
    let (_dir_a, doc_a) = temp_doc();
    let dir_b = tempfile::tempdir().unwrap();
    let doc_b = std::fs::canonicalize(dir_b.path())
        .unwrap()
        .join("other.md");
    std::fs::write(&doc_b, "# other\n").unwrap();

    let (control, docs, _) = spawn_controllable(&doc_a, None);

    // `:open` the second document, in a different directory.
    control.send(WatchControl::Document(doc_b.clone())).unwrap();
    thread::sleep(Duration::from_millis(400));

    // The old document is no longer watched.
    std::fs::write(&doc_a, "# a changed\n").unwrap();
    thread::sleep(Duration::from_millis(400));
    assert_eq!(
        docs.load(Ordering::SeqCst),
        0,
        "the previously open document must no longer trigger renders"
    );

    // The new one is.
    std::fs::write(&doc_b, "# b changed\n").unwrap();
    thread::sleep(Duration::from_millis(400));
    assert_eq!(
        docs.load(Ordering::SeqCst),
        1,
        "the newly opened document must trigger renders"
    );
}

#[test]
fn setting_a_style_starts_watching_it() {
    let (_dir, doc) = temp_doc();
    let style_dir = tempfile::tempdir().unwrap();
    let style = std::fs::canonicalize(style_dir.path())
        .unwrap()
        .join("style.css");
    std::fs::write(&style, "body{}").unwrap();

    // No style at first.
    let (control, _, styles) = spawn_controllable(&doc, None);
    std::fs::write(&style, "body{color:red}").unwrap();
    assert_eq!(stays_at(&styles, 0), 0, "not watched yet");

    // `:set style <name>`
    control
        .send(WatchControl::Style(Some(style.clone())))
        .unwrap();
    thread::sleep(Duration::from_millis(500));
    std::fs::write(&style, "body{color:blue}").unwrap();
    assert_eq!(settled_at(&styles, 1), 1, "now watched");

    // `:set style` with no value clears it again.
    control.send(WatchControl::Style(None)).unwrap();
    thread::sleep(Duration::from_millis(500));
    std::fs::write(&style, "body{color:green}").unwrap();
    assert_eq!(stays_at(&styles, 1), 1, "no longer watched");
}
