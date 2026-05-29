## ADDED Requirements

### Requirement: Colon activates command bar
The system SHALL show a command bar at the bottom of the window when the user presses `:`.

#### Scenario: Colon pressed
- **WHEN** the user presses `:` while the command bar is hidden
- **THEN** the system SHALL show the command bar with `:` prefilled and the cursor at the end

### Requirement: Escape dismisses command bar
The system SHALL hide the command bar without executing when the user presses Escape.

#### Scenario: Escape pressed in command bar
- **WHEN** the user presses Escape while the command bar is visible
- **THEN** the system SHALL hide the command bar, clear its text, and return focus to the WebView

### Requirement: Enter executes command
The system SHALL execute the entered command and hide the command bar when the user presses Enter.

#### Scenario: Enter pressed with valid command
- **WHEN** the user types `:q` and presses Enter
- **THEN** the system SHALL execute the quit command and close the application

#### Scenario: Enter pressed with unknown command
- **WHEN** the user types `:foobar` and presses Enter
- **THEN** the system SHALL hide the command bar (unknown commands are silently ignored)

### Requirement: Quit command
The system SHALL quit the application when `:q` or `:close` is entered.

#### Scenario: Quit with :q
- **WHEN** the user enters `:q`
- **THEN** the system SHALL close the application

#### Scenario: Quit with :close
- **WHEN** the user enters `:close`
- **THEN** the system SHALL close the application

### Requirement: Open command
The system SHALL open a different markdown file when `:open <path>` or `:o <path>` is entered.

#### Scenario: Open existing file
- **WHEN** the user enters `:open ~/docs/README.md` and the file exists
- **THEN** the system SHALL render the new file in the preview

### Requirement: Tab completion for file paths
The system SHALL complete file paths when Tab is pressed in the command bar during an `:open` command.

#### Scenario: Tab completes partial path
- **WHEN** the user has typed `:open ~/doc` and presses Tab
- **THEN** the system SHALL complete to `:open ~/docs/` if `~/docs/` is the only match

#### Scenario: Tab cycles through matches
- **WHEN** multiple files match the prefix and the user presses Tab repeatedly
- **THEN** the system SHALL cycle through the matching entries
