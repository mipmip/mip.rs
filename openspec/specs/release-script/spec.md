## Requirements

### Requirement: Interactive release type selection
The release script SHALL prompt the user to select major, minor, or hotfix using `gum choose`.

#### Scenario: User selects minor release
- **WHEN** the user runs the release script and selects "minor"
- **THEN** the version SHALL be bumped from e.g. `0.3.0` to `0.4.0`

#### Scenario: User selects hotfix release
- **WHEN** the user runs the release script and selects "hotfix"
- **THEN** the version SHALL be bumped from e.g. `0.3.0` to `0.3.1`

#### Scenario: User selects major release
- **WHEN** the user runs the release script and selects "major"
- **THEN** the version SHALL be bumped from e.g. `0.3.0` to `1.0.0`

### Requirement: Version is bumped in Cargo.toml
The release script SHALL update the `version` field in `Cargo.toml` as the single source of truth.

#### Scenario: Cargo.toml version updated
- **WHEN** a release is performed
- **THEN** `Cargo.toml` SHALL contain the new version string

### Requirement: Changelog is stamped with version and date
The release script SHALL replace `## Unreleased` with the new version and current date, and add a fresh `## Unreleased` section above it.

#### Scenario: Changelog updated on release
- **WHEN** a minor release to v0.4.0 is performed on 2026-06-15
- **THEN** CHANGELOG.md SHALL contain `## v0.4.0 - 15 Jun 2026` where `## Unreleased` was, with a new empty `## Unreleased` section above

### Requirement: Git tag is created
The release script SHALL create a git tag matching the version (e.g. `v0.4.0`) via jj git export + git tag.

#### Scenario: Tag created after release
- **WHEN** a release is performed for v0.4.0
- **THEN** a git tag `v0.4.0` SHALL exist pointing at the release commit

### Requirement: Confirmation before destructive actions
The release script SHALL show the planned changes and require confirmation before modifying files or creating tags.

#### Scenario: User cancels
- **WHEN** the user declines the confirmation prompt
- **THEN** no files SHALL be modified and no tags SHALL be created

### Requirement: jj bookmark is set
The release script SHALL set a jj bookmark for the release version.

#### Scenario: Bookmark created
- **WHEN** a release is performed for v0.4.0
- **THEN** a jj bookmark `v0.4.0` SHALL point at the release commit
