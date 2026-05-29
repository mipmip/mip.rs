## ADDED Requirements

### Requirement: Frontmatter rendered as table when enabled
The system SHALL render YAML frontmatter as a styled HTML key-value table prepended before the markdown body when the frontmatter display flag is enabled.

#### Scenario: Frontmatter flag enabled with frontmatter present
- **WHEN** the user runs `mip --frontmatter <file>` and the file contains YAML frontmatter
- **THEN** the system SHALL render the frontmatter as an HTML table above the markdown content

#### Scenario: Frontmatter flag enabled with no frontmatter
- **WHEN** the user runs `mip --frontmatter <file>` and the file has no frontmatter
- **THEN** the system SHALL render the markdown content normally with no table

#### Scenario: Frontmatter flag not provided
- **WHEN** the user runs `mip <file>` without `--frontmatter`
- **THEN** the system SHALL strip frontmatter and render only the markdown content (current behavior)

### Requirement: Array values displayed comma-separated
The system SHALL render frontmatter array values as comma-separated inline text.

#### Scenario: Array value in frontmatter
- **WHEN** a frontmatter key has an array value like `tags: [rust, gtk4]`
- **THEN** the table cell SHALL display `rust, gtk4`

### Requirement: Nested values displayed as YAML string
The system SHALL render nested object values as inline YAML text.

#### Scenario: Nested object in frontmatter
- **WHEN** a frontmatter key has a nested object value
- **THEN** the table cell SHALL display the value as a YAML-formatted string
