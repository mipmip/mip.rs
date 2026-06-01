## ADDED Requirements

### Requirement: paragraph_numbers config setting
The config file SHALL accept a `paragraph_numbers` key with boolean value.

#### Scenario: Enable paragraph numbers
- **WHEN** the config file contains `paragraph_numbers = true`
- **THEN** the system SHALL show section numbers on headings

### Requirement: paragraph_numbers_start config setting
The config file SHALL accept a `paragraph_numbers_start` key with integer value (1-6).

#### Scenario: Custom start level
- **WHEN** the config file contains `paragraph_numbers_start = 2`
- **THEN** numbering SHALL start from H2 headings
