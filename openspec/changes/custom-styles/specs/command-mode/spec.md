## ADDED Requirements

### Requirement: Runtime style switching
The system SHALL support `:set style <name>` to switch custom styles at runtime.

#### Scenario: Switch style at runtime
- **WHEN** the user executes `set style academic`
- **THEN** the system SHALL load and inject the new CSS immediately

#### Scenario: Remove custom style at runtime
- **WHEN** the user executes `set style` (empty value)
- **THEN** the system SHALL remove custom CSS and revert to default styles
