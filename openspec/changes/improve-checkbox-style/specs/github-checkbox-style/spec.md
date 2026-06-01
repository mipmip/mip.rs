## ADDED Requirements

### Requirement: Checkbox visual styling
Task list checkboxes SHALL be styled with `appearance: none` and custom CSS to replace browser defaults. Checkboxes SHALL be 16×16px with 3px border-radius.

#### Scenario: Unchecked checkbox appearance
- **WHEN** a markdown task list contains an unchecked item (`- [ ] text`)
- **THEN** the checkbox SHALL render with a 2px solid gray border (`#d0d7de`), white background, and rounded corners

#### Scenario: Checked checkbox appearance
- **WHEN** a markdown task list contains a checked item (`- [x] text`)
- **THEN** the checkbox SHALL render with a blue background (`#0969da`), a white checkmark, blue border, and rounded corners

### Requirement: Task list bullet removal
Task list items containing a checkbox SHALL NOT display a list bullet marker.

#### Scenario: No bullets on task list items
- **WHEN** a list item contains a checkbox input element
- **THEN** the list item SHALL have `list-style: none` and negative margin-left to align with regular list content

### Requirement: Checkbox vertical alignment
Checkboxes SHALL be vertically centered relative to the text on the same line.

#### Scenario: Checkbox aligns with text
- **WHEN** a checkbox is rendered inline with text
- **THEN** the checkbox SHALL use `vertical-align: middle` with a small negative top margin for optical alignment
