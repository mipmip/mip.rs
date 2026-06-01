## ADDED Requirements

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
