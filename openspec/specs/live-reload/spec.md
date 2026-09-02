# live-reload Specification

## Purpose
TBD - created by archiving change event-driven-live-reload. Update Purpose after archive.

## Requirements

### Requirement: Filesystem events are filtered by kind
The system SHALL rerender the document only for filesystem events that represent a
content or existence change. Events representing access to the file SHALL be
ignored.

Rerendering SHALL be triggered by `Create(_)`, `Remove(_)`, and any `Modify(_)`
other than `Modify(Metadata(_))`. It SHALL NOT be triggered by any `Access(_)`
event, by `Modify(Metadata(_))`, or by `EventKind::Any`/`EventKind::Other`.

Accepting the whole `Modify(_)` family rather than only `Data`/`Name` keeps live
reload working if a backend reports a content change as `Modify(Any)`. It costs
nothing in loop safety: reading a file only ever produces `Access(_)`.

#### Scenario: File is opened for reading
- **WHEN** the watched markdown file is opened for reading, producing an
  `Access(Open(_))` event
- **THEN** the system SHALL NOT rerender the document

#### Scenario: File content is modified in place
- **WHEN** the watched markdown file is written in place, producing a
  `Modify(Data(_))` event
- **THEN** the system SHALL rerender the document

#### Scenario: File is replaced by rename
- **WHEN** an editor writes a temporary file and renames it over the watched file,
  producing `Create(_)` and `Modify(Name(_))` events
- **THEN** the system SHALL rerender the document

#### Scenario: File metadata changes
- **WHEN** only the watched file's permissions or timestamps change, producing a
  `Modify(Metadata(_))` event
- **THEN** the system SHALL NOT rerender the document

### Requirement: Rendering does not retrigger the watcher
The act of rendering SHALL NOT produce a filesystem event that causes another
render. The system SHALL NOT enter a self-sustaining render loop.

#### Scenario: Idle document
- **WHEN** a document is open and nothing modifies it
- **THEN** the system SHALL perform no renders, and process CPU usage SHALL be
  indistinguishable from idle

#### Scenario: Reading the watched file
- **WHEN** any process reads the watched markdown file
- **THEN** the system SHALL NOT rerender the document

### Requirement: Event paths are matched exactly
The system SHALL match filesystem events against the canonicalized path of the
watched document, by path equality. Substring matching SHALL NOT be used.

#### Scenario: Sibling file with a matching prefix
- **WHEN** `doc.md` is being watched and `doc.md.bak` is modified in the same
  directory
- **THEN** the system SHALL NOT rerender the document

#### Scenario: Same filename in a subdirectory
- **WHEN** `doc.md` is being watched and `notes/doc.md` is modified under the
  watched directory
- **THEN** the system SHALL NOT rerender the document

#### Scenario: The watched file itself
- **WHEN** the watched file is modified, whether it was given as a relative or an
  absolute path on the command line
- **THEN** the system SHALL rerender the document

### Requirement: Events are debounced
The system SHALL coalesce filesystem events into a single render. A render SHALL
fire 100 ms after the last qualifying event.

#### Scenario: Single save producing multiple events
- **WHEN** one save produces a burst of qualifying events within 100 ms of each
  other
- **THEN** the system SHALL rerender the document exactly once

#### Scenario: Two saves in quick succession
- **WHEN** two saves occur more than 100 ms apart
- **THEN** the system SHALL rerender the document twice

### Requirement: Change notifications are delivered by channel
The watcher SHALL notify the GTK main loop of changes over a typed channel. The
system SHALL NOT use a file on disk as a change-notification signal, and SHALL NOT
poll for changes on a timer.

#### Scenario: Document change
- **WHEN** the watched markdown file changes
- **THEN** the watcher SHALL send a document-changed message, and the GTK main
  loop SHALL rerender and update the DOM, TOC and window title

#### Scenario: Style change
- **WHEN** the active custom CSS file changes
- **THEN** the watcher SHALL send a style-changed message, and the GTK main loop
  SHALL reinject the CSS

#### Scenario: No periodic work
- **WHEN** the application is idle
- **THEN** there SHALL be no recurring GTK timeout or idle source performing
  change detection

### Requirement: No intermediate render artifacts on disk
The system SHALL render directly into memory. It SHALL NOT write the rendered HTML
or a change token to the temporary directory.

#### Scenario: Temporary directory contents
- **WHEN** a document is open
- **THEN** the temporary directory (`$TMPDIR/mip-{pid}`) SHALL contain only the
  `docroot` symlink, and SHALL NOT contain `.temp.html` or `.temp.seed`

### Requirement: Watching survives a document switch
When the open document changes at runtime, the system SHALL rewatch accordingly.

#### Scenario: Open a file in another directory
- **WHEN** the user runs `:open` on a file in a different directory
- **THEN** the system SHALL unwatch the previous directory, watch the new one, and
  rerender only for changes to the newly opened file

#### Scenario: Change the active style at runtime
- **WHEN** the user changes the active style with `:set style`
- **THEN** the system SHALL watch the newly active CSS file for changes

### Requirement: The watch is not recursive
The system SHALL watch only the document's own directory, non-recursively.

#### Scenario: Document in a large tree
- **WHEN** the document's directory contains subdirectories such as `target/` or
  `.git/`
- **THEN** the system SHALL NOT establish watches on them, and activity inside
  them SHALL produce no events for mip to process
