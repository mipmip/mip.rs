## Requirements

### Requirement: Window title shows document identity
The system SHALL display the document title or filename in the window title bar.

#### Scenario: Frontmatter title present
- **WHEN** the markdown file has YAML frontmatter with a `title` field
- **THEN** the window title SHALL be `<title> - MiP`

#### Scenario: No frontmatter title
- **WHEN** the markdown file has no frontmatter title
- **THEN** the window title SHALL be `<filename> - MiP` (e.g. `README.md - MiP`)

#### Scenario: Title updates on live-reload
- **WHEN** the user edits the frontmatter title and saves
- **THEN** the window title SHALL update to reflect the new title

#### Scenario: Title updates on :open
- **WHEN** the user opens a different file via `:open`
- **THEN** the window title SHALL update to the new file's title or filename
