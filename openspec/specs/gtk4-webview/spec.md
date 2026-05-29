## ADDED Requirements

### Requirement: Application window displays rendered markdown
The system SHALL create a GTK4 application window with a webkit6 WebView that loads the rendered HTML from the local warp server.

#### Scenario: Normal startup
- **WHEN** mip is launched with a markdown file path argument
- **THEN** a GTK4 window titled "MiP" opens displaying the rendered markdown content

#### Scenario: WebView loads from local server
- **WHEN** the application window is created
- **THEN** the webkit6 WebView SHALL load `http://localhost:{port}/.temp.html` from the warp server

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
