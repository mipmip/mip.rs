## ADDED Requirements

### Requirement: Links to video files render as video elements
The system SHALL rewrite `<a>` tags linking to video files into `<video>` elements with playback controls.

#### Scenario: Link to local webm file
- **WHEN** the markdown contains `[text](video.webm)`
- **THEN** the rendered HTML SHALL contain `<video src="video.webm" controls style="max-width:100%"></video>` instead of a link

#### Scenario: Link to remote mp4 file
- **WHEN** the markdown contains `[text](https://example.com/video.mp4)`
- **THEN** the rendered HTML SHALL contain `<video src="https://example.com/video.mp4" controls style="max-width:100%"></video>`

### Requirement: Image syntax with video files renders as video elements
The system SHALL rewrite `<img>` tags pointing to video files into `<video>` elements with playback controls.

#### Scenario: Image syntax with video extension
- **WHEN** the markdown contains `![alt](demo.webm)`
- **THEN** the rendered HTML SHALL contain `<video src="demo.webm" controls style="max-width:100%"></video>` instead of an image tag

### Requirement: Supported video extensions
The system SHALL recognize `.webm`, `.mp4`, `.mov`, and `.ogv` as video file extensions.

#### Scenario: Non-video extension not rewritten
- **WHEN** the markdown contains `[text](file.png)` or `![](image.webp)`
- **THEN** the rendered HTML SHALL keep the original `<a>` or `<img>` tag unchanged

### Requirement: Regular links and images unaffected
The system SHALL not modify links or images that do not point to video files.

#### Scenario: Normal image preserved
- **WHEN** the markdown contains `![](screenshot.png)`
- **THEN** the rendered HTML SHALL contain `<img src="screenshot.png"` unchanged
