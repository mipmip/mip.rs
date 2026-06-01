## ADDED Requirements

### Requirement: --initstyle flag
The system SHALL accept `--initstyle <name>` to scaffold a new custom style and exit.

#### Scenario: Initstyle flag
- **WHEN** the user runs `mip --initstyle mytheme`
- **THEN** the system SHALL create the style directory and CSS file, print the path, and exit
