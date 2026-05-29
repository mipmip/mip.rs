## ADDED Requirements

### Requirement: Theme option
The system SHALL accept a `--theme` option with values `system`, `light`, or `dark`.

#### Scenario: Valid theme value
- **WHEN** the user runs `mip --theme dark <file>`
- **THEN** the system SHALL use dark theme

#### Scenario: Invalid theme value
- **WHEN** the user runs `mip --theme neon <file>`
- **THEN** the system SHALL print an error and exit with non-zero code

#### Scenario: Theme option not provided
- **WHEN** the user runs `mip <file>` without `--theme`
- **THEN** the system SHALL use the config file value, or "system" if no config
