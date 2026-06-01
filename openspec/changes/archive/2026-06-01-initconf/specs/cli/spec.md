## ADDED Requirements

### Requirement: --initconf flag
The system SHALL accept a `--initconf` flag that generates a default config file and exits.

#### Scenario: Initconf flag
- **WHEN** the user runs `mip --initconf`
- **THEN** the system SHALL generate the config file and exit (no file argument required, no preview window)
