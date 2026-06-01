## ADDED Requirements

### Requirement: / opens search bar
The system SHALL open a search bar when the user presses `/` outside the command bar.

#### Scenario: / pressed with document focused
- **WHEN** the user presses `/` while the document is focused
- **THEN** a search bar SHALL appear with `/` prefix in document search mode

#### Scenario: / pressed with TOC focused
- **WHEN** the user presses `/` while the sidetoc or quicktoc TreeView is focused
- **THEN** a search bar SHALL appear with `/` prefix in TOC filter mode

### Requirement: Document search highlights matches live
In document search mode, the system SHALL highlight all matching text in the WebView as the user types.

#### Scenario: Typing a search term
- **WHEN** the user types "markdown" in the search bar in document mode
- **THEN** all occurrences of "markdown" SHALL be highlighted in the WebView (case-insensitive)

#### Scenario: Match count displayed
- **WHEN** matches are found
- **THEN** the match count SHALL be displayed (e.g., "3/17")

### Requirement: Enter closes search bar and positions at first match
In document search mode, pressing Enter SHALL close the search bar and leave the WebView scrolled to the first match.

#### Scenario: Enter after typing search term
- **WHEN** the user types a search term and presses Enter
- **THEN** the search bar SHALL close and the first match SHALL remain highlighted and visible

### Requirement: n/N navigate matches
After a document search, the system SHALL navigate to the next/previous match with `n`/`N`.

#### Scenario: n pressed after search
- **WHEN** the user presses `n` after a search has been performed
- **THEN** the WebView SHALL scroll to the next match

#### Scenario: N pressed after search
- **WHEN** the user presses `N` (Shift+n) after a search has been performed
- **THEN** the WebView SHALL scroll to the previous match

#### Scenario: Search wraps around
- **WHEN** the user presses `n` at the last match
- **THEN** the search SHALL wrap to the first match

### Requirement: TOC filter narrows headings live
In TOC filter mode, the system SHALL filter TOC entries live as the user types, hiding non-matching headings.

#### Scenario: Typing a filter term
- **WHEN** the user types "install" in the search bar in TOC mode
- **THEN** only TOC entries containing "install" (case-insensitive) SHALL be visible

#### Scenario: Empty filter shows all entries
- **WHEN** the search bar is cleared
- **THEN** all TOC entries SHALL be visible

### Requirement: Escape clears search
Pressing Escape SHALL dismiss the search bar and clear all search state.

#### Scenario: Escape during document search
- **WHEN** the user presses Escape while in document search mode
- **THEN** the search bar SHALL close and all highlights SHALL be cleared

#### Scenario: Escape during TOC filter
- **WHEN** the user presses Escape while in TOC filter mode
- **THEN** the search bar SHALL close and the full unfiltered TOC SHALL be restored
