## Requirements

### Requirement: Scroll commands
The system SHALL support vim-style document scrolling commands.

#### Scenario: Line scroll
- `scroll_down` / `scroll_up` — scroll by one step (~60px)

#### Scenario: Page scroll
- `scroll_page_down` / `scroll_page_up` — scroll by one viewport height

#### Scenario: Half-page scroll
- `scroll_half_down` / `scroll_half_up` — scroll by half viewport height

#### Scenario: Document bounds
- `scroll_top` — scroll to top of document
- `scroll_bottom` — scroll to bottom of document

All scroll commands MUST use instant scrolling (no animation), matching vim behavior.

### Requirement: Heading navigation
The system SHALL support jumping between headings in the document.

#### Scenario: Next heading
- **WHEN** the user triggers `scroll_next_heading`
- **THEN** the document SHALL scroll to the next `h1`-`h6[id]` element below the current viewport position

#### Scenario: Previous heading
- **WHEN** the user triggers `scroll_prev_heading`
- **THEN** the document SHALL scroll to the previous `h1`-`h6[id]` element above the current viewport position

#### Scenario: No more headings
- **WHEN** there is no next/prev heading
- **THEN** the system SHALL do nothing (no wrap-around)

### Requirement: Default keybindings
The system SHALL register these default vim-style keybindings:
- `j` / `down` → `scroll_down`
- `k` / `up` → `scroll_up`
- `ctrl+f` / `pagedown` → `scroll_page_down`
- `ctrl+b` / `pageup` → `scroll_page_up`
- `ctrl+d` → `scroll_half_down`
- `ctrl+u` → `scroll_half_up`
- `home` → `scroll_top`
- `end` / `shift+g` → `scroll_bottom`
- `g,g` → `scroll_top` (key sequence)
- `n` → `scroll_next_heading`
- `shift+n` → `scroll_prev_heading`

### Requirement: Focus context
Scroll commands SHALL only fire in the correct focus context.

#### Scenario: TreeView focused
- **WHEN** a TreeView (sidetoc or quicktoc) is focused
- **THEN** `j`/`k` SHALL NOT be intercepted by scroll commands

#### Scenario: Command bar open
- **WHEN** the command bar is open
- **THEN** no scroll keybindings SHALL fire
- **AND** pending key state SHALL be cleared
