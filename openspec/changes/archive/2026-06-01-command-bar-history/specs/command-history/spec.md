## ADDED Requirements

### Requirement: Up arrow cycles history backward
The command bar SHALL show the previous history entry when the user presses ↑.

#### Scenario: First ↑ press shows most recent entry
- **WHEN** the command bar is open and the user presses ↑
- **THEN** the entry text SHALL be replaced with the most recent history entry

#### Scenario: Multiple ↑ presses cycle backward
- **WHEN** the user presses ↑ repeatedly
- **THEN** the entry text SHALL cycle through history from most recent to oldest

#### Scenario: ↑ at oldest entry stays at oldest
- **WHEN** the user is at the oldest history entry and presses ↑
- **THEN** the entry text SHALL remain at the oldest entry

### Requirement: Down arrow cycles history forward
The command bar SHALL show the next (more recent) history entry when the user presses ↓.

#### Scenario: ↓ past newest restores original input
- **WHEN** the user has cycled into history and presses ↓ past the most recent entry
- **THEN** the entry text SHALL be restored to what the user originally typed

### Requirement: Prefix filtering narrows history
When the user has typed a prefix before pressing ↑, only matching history entries SHALL be shown.

#### Scenario: Typed prefix filters results
- **WHEN** the user has typed `:op` and presses ↑
- **THEN** only history entries starting with `op` SHALL be cycled through

#### Scenario: Empty prefix shows all history
- **WHEN** the user has typed only `:` and presses ↑
- **THEN** all history entries SHALL be available

### Requirement: Executed commands are saved to history
The command bar SHALL append executed commands to the history.

#### Scenario: Command added on Enter
- **WHEN** the user types a command and presses Enter
- **THEN** the command (without leading `:`) SHALL be appended to history

#### Scenario: Dismissed commands are not saved
- **WHEN** the user presses Escape to dismiss the command bar
- **THEN** nothing SHALL be added to history

### Requirement: History is deduplicated
The history SHALL contain at most one occurrence of each command, keeping the most recent.

#### Scenario: Repeated command moves to end
- **WHEN** the user executes `open foo.md` which already exists in history
- **THEN** the old occurrence SHALL be removed and a new one appended at the end

### Requirement: History is persistent
The history SHALL be saved to disk and loaded on startup.

#### Scenario: History survives restart
- **WHEN** the user executes commands, closes mip, and reopens it
- **THEN** the previous history SHALL be available via ↑

### Requirement: History size is configurable
The maximum number of history entries SHALL be configurable via the `history_size` config option, defaulting to 50.

#### Scenario: History exceeds max size
- **WHEN** the history has 50 entries and a new command is executed
- **THEN** the oldest entry SHALL be removed

### Requirement: Typing resets browse state
When browsing history, typing a character SHALL exit history browse mode.

#### Scenario: Character input after ↑
- **WHEN** the user has pressed ↑ to browse history and then types a character
- **THEN** the browse state SHALL be reset and the typed character SHALL be appended normally
