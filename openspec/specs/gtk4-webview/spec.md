## Purpose
Present the rendered document in a native GTK4 window backed by a webkit6
WebView, with a local server supplying bundled assets and document-relative
media.

## Requirements

### Requirement: Application cleans up on close
The system SHALL remove the temporary directory when the application window is closed.

#### Scenario: Clean shutdown
- **WHEN** the user closes the application window
- **THEN** the temporary directory (`$TMPDIR/mip-{pid}`) SHALL be removed

#### Scenario: Forced termination
- **WHEN** the process is killed (SIGKILL)
- **THEN** temporary files MAY remain (no cleanup guarantee)

### Requirement: Application uses GTK4 lifecycle
The system SHALL use `gtk4::Application` for window management, replacing the tao EventLoop.

#### Scenario: Single application instance
- **WHEN** mip starts
- **THEN** a `gtk4::Application` SHALL be created and run with the `activate` signal handling window creation

### Requirement: Linux-only platform support
The system SHALL target Linux only. All platform-conditional compilation (`#[cfg]`) blocks for macOS, Windows, iOS, and Android SHALL be removed.

#### Scenario: No cross-platform code
- **WHEN** the codebase is compiled
- **THEN** there SHALL be no `#[cfg(target_os = "windows")]`, `#[cfg(target_os = "macos")]`, or similar platform gates

### Requirement: Application window renders markdown from memory
The system SHALL create a GTK4 application window with a webkit6 WebView that
displays rendered HTML built in memory. The local warp server SHALL remain in use
for bundled assets (`katex/`, `mermaid/`) and for document-relative media resolved
through the `docroot` symlink, which is why the HTML is loaded with a
`http://localhost:{port}/` base URI.

#### Scenario: Normal startup
- **WHEN** mip is launched with a markdown file path argument
- **THEN** a GTK4 window titled "MiP" opens displaying the rendered markdown
  content

#### Scenario: WebView loads from memory
- **WHEN** the application window is created
- **THEN** the webkit6 WebView SHALL load an HTML string produced by `build_html()`
  with a base URI of `http://localhost:{port}/`, without reading or writing any
  intermediate file

#### Scenario: Assets and relative media still resolve
- **WHEN** the document references bundled KaTeX or mermaid assets, or a
  document-relative image or video
- **THEN** those requests SHALL be served by the warp server via the base URI

### Requirement: Each invocation is its own application instance
The system SHALL run one independent application instance per invocation. A new
invocation SHALL NOT be forwarded to an already-running instance.

#### Scenario: A second document is opened while one is already open
- **WHEN** the user runs `mip b.md` while `mip a.md` is already running
- **THEN** a new window SHALL open showing `b.md`, and the existing window
  SHALL continue showing `a.md` unchanged

#### Scenario: Activation happens once per instance
- **WHEN** an application instance starts
- **THEN** its `activate` handler SHALL run exactly once, and a repeat
  activation SHALL NOT terminate the process
