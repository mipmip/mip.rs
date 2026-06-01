## ADDED Requirements

### Requirement: style config setting
The config file SHALL accept a `style` key with a style name string.

#### Scenario: Style in config
- **WHEN** the config contains `style = "academic"`
- **THEN** the system SHALL load CSS from `~/.config/miprs/styles/academic/style.css`
