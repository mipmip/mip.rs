## ADDED Requirements

### Requirement: System theme changes are applied live
When theme is "system", the system SHALL follow the desktop colour scheme while
running and apply changes without a restart. Detection SHALL be signal-driven via
`gio::Settings`; the system SHALL NOT poll the desktop preference on a timer and
SHALL NOT spawn a `gsettings` process repeatedly.

#### Scenario: Desktop switches to dark while running
- **WHEN** theme is "system" and the desktop colour scheme changes to dark
- **THEN** the preview SHALL switch to dark colours, and mermaid diagrams SHALL be
  re-rendered with the dark mermaid theme

#### Scenario: Desktop switches to light while running
- **WHEN** theme is "system" and the desktop colour scheme changes to light
- **THEN** the preview SHALL switch to light colours

#### Scenario: Theme is explicitly set
- **WHEN** theme is "light" or "dark"
- **THEN** desktop colour scheme changes SHALL be ignored

#### Scenario: No periodic detection
- **WHEN** the application is idle with theme "system"
- **THEN** the system SHALL perform no recurring theme check and SHALL spawn no
  subprocess

### Requirement: Missing GSettings schema degrades safely
The system SHALL not abort when the `org.gnome.desktop.interface` GSettings schema
is unavailable, as on non-GNOME desktops.

#### Scenario: Schema absent
- **WHEN** the `org.gnome.desktop.interface` schema is not installed
- **THEN** the system SHALL detect the desktop preference once at startup by other
  means and SHALL continue without live theme switching, without crashing

#### Scenario: Schema present
- **WHEN** the schema is installed
- **THEN** the system SHALL read the preference through `gio::Settings` and
  subscribe to its `changed::color-scheme` signal
