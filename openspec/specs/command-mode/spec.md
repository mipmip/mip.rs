## ADDED Requirements

### Requirement: Colon activates command bar
The system SHALL show a command bar at the bottom of the window when the user presses `:`.

#### Scenario: Colon pressed
- **WHEN** the user presses `:` while the command bar is hidden
- **THEN** the system SHALL show the command bar with an empty text field and focus it

#### Scenario: Colon ignored when command bar visible
- **WHEN** the user presses `:` while the command bar is already visible
- **THEN** the system SHALL type `:` into the entry normally

### Requirement: Command bar is modal
The system SHALL capture all keyboard input while the command bar is visible. The only ways to return to the preview are Escape (dismiss) or Enter (execute).

#### Scenario: Typing while command bar open
- **WHEN** the command bar is visible and the user types
- **THEN** all keystrokes SHALL go to the command bar, not the WebView or TOC

### Requirement: Escape dismisses command bar
The system SHALL hide the command bar without executing when the user presses Escape.

#### Scenario: Escape pressed in command bar
- **WHEN** the user presses Escape while the command bar is visible
- **THEN** the system SHALL hide the command bar, clear its text, and return focus to the WebView

### Requirement: Enter executes command
The system SHALL execute the entered command and hide the command bar when the user presses Enter.

#### Scenario: Enter pressed with valid command
- **WHEN** the user types `q` and presses Enter
- **THEN** the system SHALL execute the quit command and close the application

#### Scenario: Enter pressed with unknown command
- **WHEN** the user types `foobar` and presses Enter
- **THEN** the system SHALL hide the command bar (unknown commands are silently ignored)

### Requirement: Command text has no colon prefix
The system SHALL NOT include `:` in the command bar text. Commands are typed without a prefix.

#### Scenario: User types command
- **WHEN** the command bar opens after pressing `:`
- **THEN** the entry text SHALL be empty and the user types the command directly (e.g. `q`, `open file.md`)

### Requirement: Quit command
The system SHALL quit the application when `q` or `close` is entered.

#### Scenario: Quit with q
- **WHEN** the user enters `q`
- **THEN** the system SHALL close the application

### Requirement: Open command
The system SHALL open a different markdown file when `open <path>` or `o <path>` is entered.

#### Scenario: Open existing file
- **WHEN** the user enters `open ~/docs/README.md` and the file exists
- **THEN** the system SHALL open the file in a new mip instance and close the current one

### Requirement: Tab completion for file paths
The system SHALL complete file paths when Tab is pressed in the command bar during an `open` command.

#### Scenario: Tab completes partial path
- **WHEN** the user has typed `open ~/doc` and presses Tab
- **THEN** the system SHALL complete to `open ~/docs/` if `~/docs/` is the only match

#### Scenario: Tab cycles through matches
- **WHEN** multiple files match the prefix and the user presses Tab repeatedly
- **THEN** the system SHALL cycle through the matching entries

#### Scenario: Tab takes priority over other bindings
- **WHEN** the command bar is visible and the user presses Tab
- **THEN** Tab SHALL be handled by the command bar, not by zathura TOC navigation or other handlers

### Requirement: Command bar styling
The system SHALL style the command bar with grey background, no borders, no rounded corners, no focus ring, and monospace font.

#### Scenario: Visual appearance
- **WHEN** the command bar is visible
- **THEN** it SHALL have a flat grey background, monospace font, and no decorative borders or focus indicators

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

### Requirement: No colon prefix in command strings
Command names SHALL NOT include a colon prefix. The `:` is the command bar activation key only.

#### Scenario: Internal command format
- **WHEN** a command is executed from any source (command bar, --runcmd, keybinding)
- **THEN** the command string SHALL be without colon prefix (e.g. `open foo.md`, not `:open foo.md`)
