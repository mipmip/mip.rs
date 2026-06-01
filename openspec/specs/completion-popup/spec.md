## ADDED Requirements

### Requirement: Wildmenu displays matches
The system SHALL show a wildmenu label above the command bar displaying all completion matches when Tab is pressed and multiple matches exist.

#### Scenario: Multiple path matches
- **WHEN** the user types `:open R` and presses Tab and multiple files match
- **THEN** the system SHALL show a wildmenu label listing all matching filenames

#### Scenario: Single match hides wildmenu
- **WHEN** the user types `:open README.` and presses Tab and only one file matches
- **THEN** the system SHALL complete the path and NOT show the wildmenu

### Requirement: Wildmenu highlights current match
The system SHALL highlight the currently selected match in bold in the wildmenu label.

#### Scenario: Current match highlighted
- **WHEN** the wildmenu is visible showing matches
- **THEN** the currently completed match SHALL be displayed in bold

#### Scenario: Tab cycles highlight forward
- **WHEN** the user presses Tab again while the wildmenu is visible
- **THEN** the highlight SHALL move to the next match

#### Scenario: Shift+Tab cycles highlight backward
- **WHEN** the user presses Shift+Tab while the wildmenu is visible
- **THEN** the highlight SHALL move to the previous match (wrapping to last if at first)

### Requirement: Wildmenu hides on dismiss
The system SHALL hide the wildmenu when the command bar is dismissed or a command is executed.

#### Scenario: Escape hides wildmenu
- **WHEN** the user presses Escape while the wildmenu is visible
- **THEN** the wildmenu SHALL be hidden

#### Scenario: Enter hides wildmenu
- **WHEN** the user presses Enter while the wildmenu is visible
- **THEN** the wildmenu SHALL be hidden

#### Scenario: Typing hides wildmenu
- **WHEN** the user types a character while the wildmenu is visible
- **THEN** the wildmenu SHALL be hidden (new Tab press recalculates)

### Requirement: Wildmenu styling
The system SHALL style the wildmenu with the same grey background and monospace font as the command bar.

#### Scenario: Visual consistency
- **WHEN** the wildmenu is visible
- **THEN** it SHALL have grey background, monospace font, and no decorative borders
