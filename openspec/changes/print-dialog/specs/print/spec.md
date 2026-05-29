## ADDED Requirements

### Requirement: Ctrl+P opens print dialog
The system SHALL open the GTK print dialog when the user presses Ctrl+P in the preview window.

#### Scenario: Ctrl+P pressed
- **WHEN** the user presses Ctrl+P in the preview window
- **THEN** the system SHALL open the GTK print dialog via WebKitGTK PrintOperation

### Requirement: Print dialog supports PDF export
The system SHALL allow the user to export to PDF via the "Print to File" option in the GTK print dialog.

#### Scenario: Print to PDF
- **WHEN** the user selects "Print to File" in the print dialog and chooses PDF format
- **THEN** the system SHALL generate a PDF file at the chosen location

### Requirement: Print dialog supports physical printing
The system SHALL allow the user to print to a physical printer via the GTK print dialog.

#### Scenario: Print to printer
- **WHEN** the user selects a printer in the print dialog and clicks Print
- **THEN** the system SHALL send the rendered content to the selected printer
