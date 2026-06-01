## ADDED Requirements

### Requirement: Dialect block visual styling
Dialect block elements SHALL be styled with a subtle appearance that is clearly distinguishable from regular content but not visually distracting.

#### Scenario: Block element appearance
- **WHEN** a `dialect-block` div is rendered
- **THEN** it SHALL have a light gray background, a left border accent, rounded corners, and monospace font for the label

#### Scenario: Inline element appearance
- **WHEN** a `dialect-inline` span is rendered
- **THEN** it SHALL have a subtle gray background, rounded corners, monospace font, and slightly reduced font size

### Requirement: Dialect label display
Each dialect block or inline element SHALL display a label identifying the construct type.

#### Scenario: Hugo shortcode label
- **WHEN** a Hugo shortcode block is rendered
- **THEN** the label SHALL display the shortcode name (e.g., "figure", "notice warning") in a muted color

#### Scenario: Quarto div label
- **WHEN** a Quarto fenced div is rendered
- **THEN** the label SHALL display the class/attribute string (e.g., ".callout-note") in a muted color

### Requirement: Dialect-specific accent colors
Hugo and Quarto blocks SHALL use different subtle accent colors to visually distinguish dialects.

#### Scenario: Hugo accent
- **WHEN** a `.dialect-hugo` element is rendered
- **THEN** it SHALL use a warm accent color (e.g., orange-tinted left border)

#### Scenario: Quarto accent
- **WHEN** a `.dialect-quarto` element is rendered
- **THEN** it SHALL use a cool accent color (e.g., blue-tinted left border)
