## ADDED Requirements

### Requirement: Keybindings execute commands
The system SHALL execute the command string mapped to a key combo when that key combo is pressed.

#### Scenario: Default keybinding
- **WHEN** the user presses Tab (with default keybindings)
- **THEN** the system SHALL execute `quicktoc`

#### Scenario: Custom keybinding
- **WHEN** the config contains `ctrl+y = "open ~/todo.md"` and the user presses Ctrl+Y
- **THEN** the system SHALL execute `open ~/todo.md`

#### Scenario: Composed command keybinding
- **WHEN** the config contains `ctrl+shift+t = "sidetoc_open; set theme dark"` and the user presses Ctrl+Shift+T
- **THEN** the system SHALL execute both commands in sequence

### Requirement: Config overrides defaults
User-defined keybindings SHALL override default keybindings for the same key combo.

#### Scenario: Override default
- **WHEN** the config contains `tab = "sidetoc_toggle"` (overriding default `tab = "quicktoc"`)
- **THEN** pressing Tab SHALL execute `sidetoc_toggle`

### Requirement: Keybindings inactive during command bar
The system SHALL NOT process keybindings while the command bar is visible.

#### Scenario: Key pressed in command bar
- **WHEN** the command bar is visible and the user presses a bound key
- **THEN** the keystroke SHALL go to the command bar entry, not trigger the keybinding

### Requirement: Colon always activates command bar
The `:` key SHALL always activate the command bar and cannot be rebound.

#### Scenario: Colon is not rebindable
- **WHEN** the config contains `colon = "print"`
- **THEN** the system SHALL ignore this binding and `:` SHALL still open the command bar

### Requirement: Key combo string format
Key combos SHALL be specified as modifier+key strings separated by `+`.

#### Scenario: Modifier key combo
- **WHEN** the config contains `ctrl+p = "print"`
- **THEN** pressing Ctrl+P SHALL execute `print`

#### Scenario: Plain key
- **WHEN** the config contains `tab = "quicktoc"`
- **THEN** pressing Tab SHALL execute `quicktoc`

### Requirement: Key sequence support
The system SHALL support comma-separated key sequences (e.g. `"g,g"`, `"ctrl+g,g"`).

#### Scenario: Sequence syntax
- **WHEN** the config contains `"g,g" = "scroll_top"`
- **THEN** pressing `g` followed by `g` within 500ms SHALL execute `scroll_top`

#### Scenario: Sequence timeout
- **WHEN** the first key of a sequence is pressed and 500ms elapses without a second key
- **THEN** the pending state SHALL be silently discarded on next keypress

#### Scenario: Non-matching second key
- **WHEN** a non-matching key is pressed during pending state
- **THEN** the pending state SHALL be discarded and the new key processed normally

#### Scenario: Sequence conflicts with single binding
- **WHEN** a key is both a single-combo binding AND the first key of a sequence
- **THEN** the sequence SHALL take priority (the single binding becomes unreachable)

### Requirement: Pending state management
Pending key state SHALL be cleared when the command bar opens or focus moves to a TreeView.
