## ADDED Requirements

### Requirement: Configurable default zoom level
The system SHALL apply a configurable default zoom level to the WebView at startup. When the `zoom` config setting is present, the WebView SHALL start at that scale factor; when absent, it SHALL start at 1.0 (100%).

#### Scenario: Zoom configured
- **WHEN** the config contains `zoom = 1.4` and a document is opened
- **THEN** the WebView SHALL start at zoom level 1.4

#### Scenario: Zoom not configured
- **WHEN** no `zoom` setting is configured
- **THEN** the WebView SHALL start at zoom level 1.0 (unchanged from prior behavior)

### Requirement: Default zoom is clamped to valid range
The configured/startup zoom level SHALL be clamped to the same bounds as the relative zoom commands (minimum 0.3, maximum 5.0). An invalid (non-numeric) value SHALL produce a warning and fall back to the default of 1.0.

#### Scenario: Value above maximum
- **WHEN** the config contains `zoom = 9.0`
- **THEN** the startup zoom SHALL be clamped to 5.0

#### Scenario: Value below minimum
- **WHEN** the config contains `zoom = 0.1`
- **THEN** the startup zoom SHALL be clamped to 0.3

#### Scenario: Non-numeric value
- **WHEN** the `zoom` value cannot be parsed as a number
- **THEN** the system SHALL print a warning and use the default zoom of 1.0

### Requirement: CLI flag overrides configured zoom
The system SHALL accept a `--zoom <factor>` CLI flag that overrides the configured `zoom` value for that run.

#### Scenario: Flag overrides config
- **WHEN** the config contains `zoom = 1.2` and `mip --zoom 2.0 file.md` is run
- **THEN** the WebView SHALL start at zoom level 2.0

#### Scenario: No flag uses config
- **WHEN** `--zoom` is not passed and `zoom = 1.2` is configured
- **THEN** the WebView SHALL start at zoom level 1.2

### Requirement: Runtime zoom setting
The system SHALL support `:set zoom <factor>` to change the live WebView zoom without restart, and `zoom` SHALL appear in `:set` tab-completion.

#### Scenario: Set zoom at runtime
- **WHEN** the user runs `:set zoom 1.5`
- **THEN** the WebView zoom level SHALL change to 1.5 immediately

#### Scenario: Zoom in completion list
- **WHEN** the user types `:set zo` and requests completion
- **THEN** `zoom` SHALL be offered as a completion
