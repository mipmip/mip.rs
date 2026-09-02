## Purpose
Render the document in light or dark colours, either pinned explicitly or
followed from the desktop's own colour-scheme preference.

## Requirements

### Requirement: System theme mode
The system SHALL respect the user's OS color scheme preference when theme is set to "system".

#### Scenario: OS prefers dark
- **WHEN** theme is "system" and the OS is set to dark mode
- **THEN** the rendered HTML SHALL use dark colors via `@media (prefers-color-scheme: dark)`

#### Scenario: OS prefers light
- **WHEN** theme is "system" and the OS is set to light mode
- **THEN** the rendered HTML SHALL use light colors

### Requirement: Explicit dark theme
The system SHALL render with dark colors when theme is set to "dark", regardless of OS preference.

#### Scenario: Dark theme forced
- **WHEN** theme is "dark"
- **THEN** the `<html>` tag SHALL have class "dark" and all elements SHALL use dark color variables

### Requirement: Explicit light theme
The system SHALL render with light colors when theme is set to "light", regardless of OS preference.

#### Scenario: Light theme forced
- **WHEN** theme is "light"
- **THEN** the `<html>` tag SHALL have class "light" and all elements SHALL use light color variables

### Requirement: Dark mode for frontmatter table
The frontmatter table SHALL use appropriate dark mode colors when dark theme is active.

#### Scenario: Frontmatter table in dark mode
- **WHEN** dark theme is active and frontmatter display is enabled
- **THEN** the `.frontmatter` table SHALL use dark background, light text, and dark border colors

### Requirement: CSS variable-based theming
All theme colors SHALL be defined as CSS custom properties (variables) on `:root`.

#### Scenario: Color variables defined
- **WHEN** the template is rendered
- **THEN** all color values in the CSS SHALL reference CSS variables (e.g., `var(--bg)`, `var(--fg)`)

### Requirement: Print output uses light theme
The system SHALL force light theme colors in print output regardless of the current screen theme.

#### Scenario: Print from dark mode
- **WHEN** the user prints while in dark theme
- **THEN** the printed output SHALL use light background and dark text colors via `@media print` CSS

#### Scenario: Print from light mode
- **WHEN** the user prints while in light theme
- **THEN** the printed output SHALL use light colors (no change from screen)

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
