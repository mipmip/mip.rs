## ADDED Requirements

### Requirement: Application icon in taskbar
The system SHALL display the mip icon in the desktop taskbar/dock when running.

#### Scenario: Icon visible in taskbar
- **WHEN** mip is running
- **THEN** the taskbar/dock SHALL show the mip SVG icon instead of a generic GTK icon

### Requirement: Desktop file for app launchers
The system SHALL include a `.desktop` file for Linux desktop integration.

#### Scenario: Visible in app launcher
- **WHEN** mip is installed via Nix
- **THEN** it SHALL appear in desktop app launchers (GNOME, KDE) with the mip icon

#### Scenario: Open markdown files
- **WHEN** the user right-clicks a `.md` file and selects "Open with"
- **THEN** mip SHALL appear as an option

### Requirement: Clean icon directory
The icon directory SHALL contain only the SVG source file.

#### Scenario: No PNG files
- **WHEN** the repository is checked out
- **THEN** `icons/` SHALL contain only `mip-icon.svg` (no PNG files)
