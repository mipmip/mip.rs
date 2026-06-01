## ADDED Requirements

### Requirement: Print output uses light theme
The system SHALL force light theme colors in print output regardless of the current screen theme.

#### Scenario: Print from dark mode
- **WHEN** the user prints while in dark theme
- **THEN** the printed output SHALL use light background and dark text colors via `@media print` CSS

#### Scenario: Print from light mode
- **WHEN** the user prints while in light theme
- **THEN** the printed output SHALL use light colors (no change from screen)
