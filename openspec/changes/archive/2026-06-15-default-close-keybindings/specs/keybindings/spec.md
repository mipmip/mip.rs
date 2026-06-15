## ADDED Requirements

### Requirement: Default close keybindings
The system SHALL provide default keybindings that close the application, mapped to the `close` command: `ctrl+q`, `ctrl+w`, and `alt+f4`. These SHALL be regular default keybindings, overridable by user config like any other default.

#### Scenario: Ctrl+Q closes
- **WHEN** the user presses Ctrl+Q with default keybindings
- **THEN** the system SHALL execute `close` and quit the application

#### Scenario: Ctrl+W closes
- **WHEN** the user presses Ctrl+W with default keybindings
- **THEN** the system SHALL execute `close` and quit the application

#### Scenario: Alt+F4 closes
- **WHEN** the user presses Alt+F4 with default keybindings
- **THEN** the system SHALL execute `close` and quit the application

#### Scenario: Close keybinding overridable
- **WHEN** the config contains `ctrl+w = "sidetoc_toggle"` (overriding the default `ctrl+w = "close"`)
- **THEN** pressing Ctrl+W SHALL execute `sidetoc_toggle` and SHALL NOT quit the application
