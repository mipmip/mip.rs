## ADDED Requirements

### Requirement: .deb package is built in CI
The GitHub Actions release workflow SHALL build a `.deb` package when a version tag is pushed.

#### Scenario: Tag push triggers build
- **WHEN** a tag matching `v*` is pushed to GitHub
- **THEN** the CI workflow SHALL build a `.deb` package using `cargo-deb`

### Requirement: .deb declares runtime dependencies
The `.deb` package SHALL declare runtime dependencies on GTK4, WebKitGTK 6.0, and GStreamer packages.

#### Scenario: Installing .deb on Debian/Ubuntu
- **WHEN** the `.deb` is installed via `dpkg -i` or `apt install`
- **THEN** it SHALL pull in `libgtk-4-1`, `libwebkitgtk-6.0-4`, `gstreamer1.0-plugins-base`, and `gstreamer1.0-plugins-good` as dependencies

### Requirement: .deb is attached to GitHub Release
The CI workflow SHALL upload the `.deb` as a release asset on the GitHub Release.

#### Scenario: Release assets available
- **WHEN** the CI workflow completes successfully
- **THEN** the GitHub Release page SHALL have the `.deb` file available for download
