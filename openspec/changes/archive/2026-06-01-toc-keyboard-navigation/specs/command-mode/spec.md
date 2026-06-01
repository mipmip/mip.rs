## ADDED Requirements

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
