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
- **THEN** Tab SHALL be handled by the command bar, not by TOC navigation or other handlers

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

### Requirement: Sidetoc keyboard navigation
The sidetoc TreeView SHALL support keyboard navigation when focused.

#### Scenario: Arrow up/down navigates headings
- **WHEN** the sidetoc is focused and the user presses arrow up or down
- **THEN** the cursor SHALL move to the previous or next heading

#### Scenario: Left collapses or moves to parent
- **WHEN** the sidetoc is focused and the user presses Left on an expanded row
- **THEN** the row SHALL collapse
- **WHEN** the sidetoc is focused and the user presses Left on a collapsed row
- **THEN** the cursor SHALL move to the parent row

#### Scenario: Right expands or moves to child
- **WHEN** the sidetoc is focused and the user presses Right on a collapsed row
- **THEN** the row SHALL expand
- **WHEN** the sidetoc is focused and the user presses Right on an expanded row
- **THEN** the cursor SHALL move to the first child row

#### Scenario: Enter scrolls to heading
- **WHEN** the sidetoc is focused and the user presses Enter
- **THEN** the document SHALL scroll to the selected heading

#### Scenario: Escape closes sidetoc
- **WHEN** the sidetoc is focused and the user presses Escape
- **THEN** the sidetoc SHALL close and focus SHALL return to the document

### Requirement: Quicktoc left/right navigation
The quicktoc TreeView SHALL support left/right arrow keys for collapsing and expanding subtrees.

#### Scenario: Left/right in quicktoc
- **WHEN** the quicktoc is focused and the user presses Left or Right
- **THEN** the behavior SHALL match the sidetoc (collapse/expand or navigate to parent/child)

### Requirement: Focus commands
The system SHALL support `sidetoc_focus` and `document_focus` commands.

#### Scenario: Focus sidetoc
- **WHEN** the user executes `sidetoc_focus` and the sidetoc is open
- **THEN** the sidetoc TreeView SHALL receive keyboard focus

#### Scenario: Focus document
- **WHEN** the user executes `document_focus`
- **THEN** the document WebView SHALL receive keyboard focus

### Requirement: Auto-focus on open/close
The system SHALL automatically focus the appropriate widget when opening or closing the sidetoc.

#### Scenario: Sidetoc open focuses treeview
- **WHEN** the user executes `sidetoc_open`
- **THEN** the sidetoc TreeView SHALL receive keyboard focus

#### Scenario: Sidetoc close focuses document
- **WHEN** the user executes `sidetoc_close`
- **THEN** the document WebView SHALL receive keyboard focus

### Requirement: Set command changes settings at runtime
The system SHALL support a `set` command that changes settings at runtime.

#### Scenario: Set frontmatter
- **WHEN** the user executes `set frontmatter true`
- **THEN** the system SHALL enable frontmatter display and re-render immediately

#### Scenario: Set theme
- **WHEN** the user executes `set theme dark`
- **THEN** the system SHALL switch to dark theme immediately

#### Scenario: Set paragraph numbers
- **WHEN** the user executes `set paragraph_numbers true`
- **THEN** the system SHALL enable section numbers and re-render immediately

#### Scenario: Set paragraph numbers start
- **WHEN** the user executes `set paragraph_numbers_start 2`
- **THEN** the system SHALL update the start level and re-render immediately

### Requirement: Set command validates values
The system SHALL validate values and warn on invalid input.

#### Scenario: Invalid bool value
- **WHEN** the user executes `set frontmatter banana`
- **THEN** the system SHALL print a warning and not change the setting

#### Scenario: Invalid theme value
- **WHEN** the user executes `set theme neon`
- **THEN** the system SHALL print a warning and not change the setting

#### Scenario: Out of range integer
- **WHEN** the user executes `set paragraph_numbers_start 9`
- **THEN** the system SHALL clamp the value to the valid range (1-6)

### Requirement: Set command triggers re-render
The system SHALL force an immediate re-render when a setting that affects rendering is changed.

#### Scenario: Re-render without file change
- **WHEN** the user changes `frontmatter` via `:set`
- **THEN** the preview SHALL update immediately without the file needing to change

### Requirement: Setting name completion
The system SHALL complete setting names when Tab is pressed after `:set `.

#### Scenario: Tab completes setting name
- **WHEN** the user types `:set front` and presses Tab
- **THEN** the system SHALL complete to `:set frontmatter `

#### Scenario: Tab shows all settings
- **WHEN** the user types `:set ` and presses Tab
- **THEN** the wildmenu SHALL show all available setting names
