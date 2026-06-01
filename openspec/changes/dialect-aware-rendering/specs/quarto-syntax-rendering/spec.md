## ADDED Requirements

### Requirement: Quarto fenced divs
The preprocessor SHALL detect Quarto fenced div blocks (`:::{.class}` ... `:::`) and replace the fence markers with styled HTML div wrappers while preserving inner content.

#### Scenario: Callout block
- **WHEN** the markdown contains `:::{.callout-note}` followed by content and closing `:::`
- **THEN** the fences SHALL be replaced with `<div class="dialect-block dialect-quarto">` with a label showing ".callout-note", and inner content SHALL be preserved as renderable markdown

#### Scenario: Multiple classes
- **WHEN** a fenced div has multiple classes like `:::{.callout-warning .collapsed}`
- **THEN** the label SHALL display all classes

### Requirement: Quarto inline attributes
The preprocessor SHALL detect Quarto inline attribute syntax (`{.class}`, `{#id}`, `{key=value}`) appended to elements and render them as subtle inline labels.

#### Scenario: Class attribute on heading
- **WHEN** a heading is followed by `{.unnumbered}`
- **THEN** the attribute SHALL be rendered as a subtle `<span class="dialect-inline dialect-quarto">` after the heading text

#### Scenario: Key-value attribute
- **WHEN** an element has `{width=80%}`
- **THEN** the attribute SHALL be rendered as a subtle inline label showing the attribute

### Requirement: Quarto YAML code block options
The preprocessor SHALL detect Quarto's `#| key: value` code block options and render them as subtle labels within the code block.

#### Scenario: Code block with options
- **WHEN** a code block contains lines starting with `#| `
- **THEN** those lines SHALL remain visible as-is within the code block (no special transformation needed — they are already valid code comments)
