### Requirement: Zoom in increases page scale
The system SHALL increase the WebView zoom level by 10% when the `zoom_in` command is executed.

#### Scenario: Zoom in from default
- **WHEN** the user presses Ctrl+= (or runs `:zoom_in`)
- **THEN** the zoom level SHALL increase from 1.0 to 1.1

#### Scenario: Zoom in clamped at maximum
- **WHEN** the zoom level is at 5.0 and `zoom_in` is executed
- **THEN** the zoom level SHALL remain at 5.0

### Requirement: Zoom out decreases page scale
The system SHALL decrease the WebView zoom level by 10% when the `zoom_out` command is executed.

#### Scenario: Zoom out from default
- **WHEN** the user presses Ctrl+- (or runs `:zoom_out`)
- **THEN** the zoom level SHALL decrease from 1.0 to 0.9

#### Scenario: Zoom out clamped at minimum
- **WHEN** the zoom level is at 0.3 and `zoom_out` is executed
- **THEN** the zoom level SHALL remain at 0.3

### Requirement: Zoom reset restores default scale
The system SHALL reset the WebView zoom level to 1.0 when the `zoom_reset` command is executed.

#### Scenario: Reset after zooming in
- **WHEN** the user has zoomed to 1.5 and presses Ctrl+0 (or runs `:zoom_reset`)
- **THEN** the zoom level SHALL return to 1.0
