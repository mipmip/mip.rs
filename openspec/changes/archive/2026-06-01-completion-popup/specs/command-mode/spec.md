## ADDED Requirements

### Requirement: Command name completion
The system SHALL complete partial command names when Tab is pressed and the text contains no space (no argument yet).

#### Scenario: Unique command prefix
- **WHEN** the user types `:op` and presses Tab
- **THEN** the system SHALL complete to `:open ` (with trailing space)

#### Scenario: Ambiguous command prefix
- **WHEN** the user types `:c` and presses Tab
- **THEN** the system SHALL complete to `:close ` (unique match for `c` prefix)

#### Scenario: Multiple command matches
- **WHEN** the user types `:o` and presses Tab
- **THEN** the system SHALL show wildmenu with `o` and `open`, completing to the first match
