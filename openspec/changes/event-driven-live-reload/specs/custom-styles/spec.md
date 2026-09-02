## MODIFIED Requirements

### Requirement: Live-reload custom CSS
The system SHALL detect changes to the custom CSS file with the filesystem watcher
and reinject without restart. Detection SHALL NOT be performed by polling the
file's metadata on a timer.

#### Scenario: CSS file modified
- **WHEN** the user edits and saves the custom CSS file while mip is running
- **THEN** the preview SHALL update to reflect the new CSS within ~100ms of the
  save

#### Scenario: Style changed at runtime
- **WHEN** the user changes the active style with `:set style`
- **THEN** the system SHALL stop watching the previous CSS file and begin watching
  the newly active one

#### Scenario: No style configured
- **WHEN** no `style` setting is configured
- **THEN** the system SHALL watch no CSS file and perform no style-related work
