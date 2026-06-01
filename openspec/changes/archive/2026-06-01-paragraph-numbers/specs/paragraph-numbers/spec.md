## ADDED Requirements

### Requirement: Hierarchical section numbers in preview
The system SHALL prepend hierarchical section numbers to heading text in the rendered preview when `paragraph_numbers` is enabled.

#### Scenario: Numbers on headings
- **WHEN** `paragraph_numbers = true` and the document has H1, H2, H3 headings
- **THEN** the preview SHALL show "1 Heading", "1.1 Subheading", "1.1.1 Sub-subheading"

#### Scenario: Numbers disabled by default
- **WHEN** no `paragraph_numbers` config is set
- **THEN** headings SHALL render without section numbers

### Requirement: Configurable start level
The system SHALL support a `paragraph_numbers_start` setting that controls which heading level begins the numbering.

#### Scenario: Start from H2
- **WHEN** `paragraph_numbers_start = 2` and the document has H1 and H2 headings
- **THEN** H1 headings SHALL have no number and H2 headings SHALL start at "1."

#### Scenario: Start from H1 (default)
- **WHEN** `paragraph_numbers_start = 1` (or not set) and the document has H1 headings
- **THEN** H1 headings SHALL start at "1."

### Requirement: Numbers in TOC views
The system SHALL show the same section numbers in the sidetoc and quicktoc TreeView displays.

#### Scenario: TOC entries with numbers
- **WHEN** `paragraph_numbers = true`
- **THEN** TOC entries SHALL display with section number prefixes matching the preview

### Requirement: Section number styling
Section numbers in the preview SHALL be wrapped in a `<span class="section-number">` element for CSS styling.

#### Scenario: Styled numbers
- **WHEN** a heading has a section number
- **THEN** the number SHALL be in a `<span class="section-number">` before the heading text
