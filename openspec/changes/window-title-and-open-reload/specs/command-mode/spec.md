## MODIFIED Requirements

### Requirement: Open command
The system SHALL reload a different markdown file in-place when `open <path>` or `o <path>` is entered, instead of spawning a new process.

#### Scenario: Open existing file
- **WHEN** the user enters `open ~/docs/README.md` and the file exists
- **THEN** the system SHALL render the new file in the current window, preserving runtime settings

#### Scenario: Open file in different directory
- **WHEN** the user opens a file in a different directory than the current file
- **THEN** the system SHALL update the file watcher and server to the new directory, and images SHALL resolve correctly

#### Scenario: Open preserves settings
- **WHEN** the user has changed settings via `:set` (e.g. theme, frontmatter) and then runs `:open`
- **THEN** the runtime settings SHALL be preserved for the new document
