## ADDED Requirements

### Requirement: Set command changes settings at runtime
The system SHALL support a `set` command that changes settings at runtime.

#### Scenario: Set frontmatter
- **WHEN** the user executes `set frontmatter true`
- **THEN** the system SHALL enable frontmatter display and re-render immediately

#### Scenario: Set theme
- **WHEN** the user executes `set theme dark`
- **THEN** the system SHALL switch to dark theme immediately

#### Scenario: Set paragraph numbers
- **WHEN** the user executes `set paragraph_numbers true`
- **THEN** the system SHALL enable section numbers and re-render immediately

#### Scenario: Set paragraph numbers start
- **WHEN** the user executes `set paragraph_numbers_start 2`
- **THEN** the system SHALL update the start level and re-render immediately

### Requirement: Set command validates values
The system SHALL validate values and warn on invalid input.

#### Scenario: Invalid bool value
- **WHEN** the user executes `set frontmatter banana`
- **THEN** the system SHALL print a warning and not change the setting

#### Scenario: Invalid theme value
- **WHEN** the user executes `set theme neon`
- **THEN** the system SHALL print a warning and not change the setting

#### Scenario: Out of range integer
- **WHEN** the user executes `set paragraph_numbers_start 9`
- **THEN** the system SHALL clamp the value to the valid range (1-6)

### Requirement: Set command triggers re-render
The system SHALL force an immediate re-render when a setting that affects rendering is changed.

#### Scenario: Re-render without file change
- **WHEN** the user changes `frontmatter` via `:set`
- **THEN** the preview SHALL update immediately without the file needing to change

### Requirement: Setting name completion
The system SHALL complete setting names when Tab is pressed after `:set `.

#### Scenario: Tab completes setting name
- **WHEN** the user types `:set front` and presses Tab
- **THEN** the system SHALL complete to `:set frontmatter `

#### Scenario: Tab shows all settings
- **WHEN** the user types `:set ` and presses Tab
- **THEN** the wildmenu SHALL show all available setting names
