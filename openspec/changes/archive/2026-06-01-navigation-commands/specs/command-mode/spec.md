## ADDED Requirements

### Requirement: Sidetoc commands
The system SHALL support commands to control the side table of contents panel.

#### Scenario: Open sidetoc
- **WHEN** the user executes `sidetoc_open`
- **THEN** the system SHALL show the sidetoc panel

#### Scenario: Close sidetoc
- **WHEN** the user executes `sidetoc_close`
- **THEN** the system SHALL hide the sidetoc panel

#### Scenario: Toggle sidetoc
- **WHEN** the user executes `sidetoc_toggle`
- **THEN** the system SHALL show the sidetoc if hidden, or hide it if visible

#### Scenario: Expand sidetoc width
- **WHEN** the user executes `sidetoc_expand_width`
- **THEN** the system SHALL increase the sidetoc panel width by a step

#### Scenario: Shrink sidetoc width
- **WHEN** the user executes `sidetoc_shrink_width`
- **THEN** the system SHALL decrease the sidetoc panel width by a step

### Requirement: Quicktoc command
The system SHALL support a command to toggle the full-screen quick table of contents view.

#### Scenario: Toggle quicktoc
- **WHEN** the user executes `quicktoc`
- **THEN** the system SHALL toggle between the document view and the TOC view (Stack switch)

### Requirement: Command composition with semicolon
The system SHALL support executing multiple commands separated by `;`.

#### Scenario: Multiple commands
- **WHEN** the user enters `sidetoc_open; set theme dark`
- **THEN** the system SHALL execute `sidetoc_open` and then `set theme dark` in sequence

#### Scenario: Semicolon in command bar
- **WHEN** the user types `:sidetoc_open; quicktoc` in the command bar
- **THEN** both commands SHALL be executed in sequence

### Requirement: No colon prefix in command strings
Command names SHALL NOT include a colon prefix. The `:` is the command bar activation key only.

#### Scenario: Internal command format
- **WHEN** a command is executed from any source (command bar, --runcmd, keybinding)
- **THEN** the command string SHALL be without colon prefix (e.g. `open foo.md`, not `:open foo.md`)
