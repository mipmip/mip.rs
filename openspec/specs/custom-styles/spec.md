## Purpose
Let users override mip's default appearance with their own CSS, loaded from
`~/.config/miprs/styles/<name>/style.css`, switchable at runtime and reloaded
when the file changes.

## Requirements

### Requirement: Custom CSS loading
The system SHALL load a custom CSS file from `~/.config/miprs/styles/<name>/style.css` when the `style` setting is configured.

#### Scenario: Style configured
- **WHEN** the config contains `style = "academic"` and `~/.config/miprs/styles/academic/style.css` exists
- **THEN** the system SHALL inject the CSS after the default styles

#### Scenario: Style not found
- **WHEN** the config contains `style = "missing"` and the directory doesn't exist
- **THEN** the system SHALL print a warning and render with default styles only

#### Scenario: No style configured
- **WHEN** no `style` setting is configured
- **THEN** the system SHALL render with default styles only

### Requirement: Custom CSS overrides defaults
Custom CSS SHALL be injected after default styles so it can override via CSS specificity.

#### Scenario: Override variables
- **WHEN** the custom CSS defines `:root { --bg: #fdf6e3; }`
- **THEN** the background color SHALL use the custom value

#### Scenario: Override dark mode
- **WHEN** the custom CSS defines `.dark { --bg: #002b36; }`
- **THEN** dark mode SHALL use the custom dark background

### Requirement: Live-reload custom CSS
The system SHALL detect changes to the custom CSS file with the filesystem watcher
and reinject without restart. Detection SHALL NOT be performed by polling the
file's metadata on a timer.

#### Scenario: CSS file modified
- **WHEN** the user edits and saves the custom CSS file while mip is running
- **THEN** the preview SHALL update to reflect the new CSS within ~100ms of the
  save

#### Scenario: Style changed at runtime
- **WHEN** the user changes the active style with `:set style`
- **THEN** the system SHALL stop watching the previous CSS file and begin watching
  the newly active one

#### Scenario: No style configured
- **WHEN** no `style` setting is configured
- **THEN** the system SHALL watch no CSS file and perform no style-related work

### Requirement: Scaffold new style
The system SHALL support `--initstyle <name>` to create a new style directory with the default CSS.

#### Scenario: Create new style
- **WHEN** the user runs `mip --initstyle academic` and the style doesn't exist
- **THEN** the system SHALL create `~/.config/miprs/styles/academic/style.css` with documented default CSS and exit

#### Scenario: Style already exists
- **WHEN** the user runs `mip --initstyle academic` and the directory already exists
- **THEN** the system SHALL print an error and exit without modifying anything
