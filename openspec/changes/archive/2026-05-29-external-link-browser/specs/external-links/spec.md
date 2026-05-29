## ADDED Requirements

### Requirement: External links open in default browser
The system SHALL open external URLs in the user's default browser when clicked in the preview.

#### Scenario: Click external HTTPS link
- **WHEN** the user clicks a link to `https://github.com/...` in the preview
- **THEN** the system SHALL open the URL with `xdg-open` and NOT navigate the WebView

#### Scenario: Click mailto link
- **WHEN** the user clicks a `mailto:` link in the preview
- **THEN** the system SHALL open it with `xdg-open`

### Requirement: Internal navigation allowed
The system SHALL allow navigation to internal localhost URLs used by the preview server.

#### Scenario: Initial page load
- **WHEN** the WebView loads the initial HTML from `http://localhost:{port}/`
- **THEN** the system SHALL allow the navigation

#### Scenario: Local asset request
- **WHEN** the WebView requests an image or resource from `http://localhost:{port}/...`
- **THEN** the system SHALL allow the request

### Requirement: WebView stays on preview
The system SHALL prevent the WebView from navigating away from the preview content.

#### Scenario: External link does not navigate WebView
- **WHEN** the user clicks an external link
- **THEN** the WebView SHALL remain showing the current markdown preview
