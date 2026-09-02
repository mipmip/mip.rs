## Purpose
Keep mip's behaviour covered by automated tests that run under `cargo test`, so
regressions surface in CI rather than in use.

## Requirements

### Requirement: Markdown-to-HTML conversion is tested
The system SHALL have integration tests that verify markdown input produces correct HTML output, including frontmatter rendering.

#### Scenario: Plain markdown conversion
- **WHEN** a markdown string with headings, paragraphs, and inline formatting is passed to `md_to_html_body()` with `show_frontmatter: false`
- **THEN** the returned HTML SHALL contain the correct tags (`<h1>`, `<p>`, `<em>`, `<strong>`, etc.)

#### Scenario: Frontmatter rendering enabled
- **WHEN** a markdown string with YAML frontmatter is passed to `md_to_html_body()` with `show_frontmatter: true`
- **THEN** the returned HTML SHALL contain a `<table class="frontmatter">` with key-value rows preceding the body content

#### Scenario: Frontmatter rendering disabled
- **WHEN** a markdown string with YAML frontmatter is passed to `md_to_html_body()` with `show_frontmatter: false`
- **THEN** the returned HTML SHALL NOT contain any frontmatter table

#### Scenario: Strikethrough, task lists, and tables
- **WHEN** markdown containing GFM strikethrough (`~~text~~`), task lists (`- [x]`), or tables is converted
- **THEN** the output HTML SHALL contain the corresponding elements (`<del>`, `<input type="checkbox">`, `<table>`)

### Requirement: Full HTML template assembly is tested
The system SHALL have tests that verify the full HTML build pipeline: markdown +
template → complete HTML document.

#### Scenario: Template placeholders are replaced
- **WHEN** `build_html()` is called with markdown content and a theme class
- **THEN** the returned HTML string SHALL contain the rendered markdown body and
  SHALL NOT contain the `#{BODY}` placeholder

#### Scenario: Theme class is applied
- **WHEN** `build_html()` is called with theme_class `"dark"`
- **THEN** the returned HTML SHALL include the `dark` class in the appropriate
  element

### Requirement: Config loading is tested
The system SHALL have integration tests that verify TOML config files are loaded and validated correctly.

#### Scenario: Valid config with all fields
- **WHEN** a TOML file with `theme = "dark"` and `frontmatter = true` is loaded
- **THEN** `Config::theme()` SHALL return `"dark"` and `Config::frontmatter()` SHALL return `true`

#### Scenario: Missing config file
- **WHEN** the config path points to a nonexistent file
- **THEN** `Config::load()` SHALL return defaults: theme `"system"`, frontmatter `false`

#### Scenario: Invalid theme value
- **WHEN** a TOML file with `theme = "neon"` is loaded
- **THEN** the theme SHALL fall back to `"system"` and a warning SHALL be printed to stderr

#### Scenario: Malformed TOML
- **WHEN** the config file contains invalid TOML syntax
- **THEN** `Config::load()` SHALL return defaults and print a warning to stderr

### Requirement: Port helper functions are tested
The system SHALL have unit tests for `port_is_available()`.

#### Scenario: Available port
- **WHEN** `port_is_available()` is called with a port that is not in use
- **THEN** it SHALL return `true`

#### Scenario: Occupied port
- **WHEN** `port_is_available()` is called with a port that is already bound
- **THEN** it SHALL return `false`

### Requirement: The rerender decision is tested
The system SHALL have tests that verify which filesystem events trigger a
rerender, so that a read-triggered feedback loop cannot be reintroduced.

#### Scenario: Access events are rejected
- **WHEN** `should_rerender()` is called with an `Access(Open(_))` event on the
  watched path
- **THEN** it SHALL return `false`

#### Scenario: Content and existence events are accepted
- **WHEN** `should_rerender()` is called with `Modify(Data(_))`,
  `Modify(Name(_))`, `Create(_)` or `Remove(_)` on the watched path
- **THEN** it SHALL return `true`

#### Scenario: Metadata events are rejected
- **WHEN** `should_rerender()` is called with `Modify(Metadata(_))` on the watched
  path
- **THEN** it SHALL return `false`

#### Scenario: Non-watched paths are rejected
- **WHEN** `should_rerender()` is called with a qualifying event on `doc.md.bak` or
  on `notes/doc.md` while `doc.md` is watched
- **THEN** it SHALL return `false`

### Requirement: The absence of a render loop is tested
The system SHALL have a test that reads the watched file the way rendering does and
asserts that no rerender is triggered, so that a new feedback loop fails the test
suite rather than reaching a release.

#### Scenario: Reading the watched file triggers nothing
- **WHEN** the watcher is running on a temporary directory and the watched file is
  read with `read_to_string`
- **THEN** zero rerender notifications SHALL be observed

#### Scenario: Writing the watched file triggers exactly one render
- **WHEN** the watched file is then written once
- **THEN** exactly one rerender notification SHALL be observed

### Requirement: Debouncing is tested
The system SHALL have tests that verify event coalescing.

#### Scenario: Burst within the debounce window
- **WHEN** three qualifying events arrive within 100 ms of each other
- **THEN** exactly one change notification SHALL be sent

#### Scenario: Events straddling the debounce window
- **WHEN** two qualifying events arrive more than 100 ms apart
- **THEN** two change notifications SHALL be sent

### Requirement: Theme detection fallback is tested
The system SHALL have a test that `is_system_dark()` returns a value without
aborting when the GSettings schema is unavailable.

#### Scenario: Schema unavailable
- **WHEN** `is_system_dark()` is called and the `org.gnome.desktop.interface`
  schema cannot be looked up
- **THEN** the call SHALL return a boolean rather than aborting the process

### Requirement: The in-memory page render is tested
The system SHALL have tests that verify `render_page()` produces a complete,
placeholder-free HTML document, since it is what the WebView is handed directly.

#### Scenario: Complete document
- **WHEN** `render_page()` is called with markdown content and a theme class
- **THEN** the result SHALL begin with a doctype, carry the theme class, contain
  the rendered body and the custom-CSS element, and close the document

#### Scenario: No placeholders survive
- **WHEN** `render_page()` is called with any combination of options
- **THEN** the result SHALL contain no `#{...}` placeholder

#### Scenario: No seed or bridge remnants
- **WHEN** `render_page()` is called
- **THEN** the result SHALL NOT reference a seed URL, a seed token, a polling
  script, or the removed highlight.js bundle

#### Scenario: Optional script blocks
- **WHEN** `render_page()` is called with math and mermaid enabled, then disabled
- **THEN** the KaTeX and mermaid script tags SHALL be present only when enabled

### Requirement: Server asset routes are tested
The system SHALL have integration tests that verify the warp server serves the
routes it still provides: bundled assets and document-relative files.

#### Scenario: Serving static assets from source dir
- **WHEN** a GET request is made for a file that exists in the source directory
- **THEN** the response SHALL serve that file with a 200 status

#### Scenario: Serving bundled assets
- **WHEN** a GET request is made for a bundled KaTeX or mermaid asset
- **THEN** the response SHALL serve that asset with a 200 status and the correct
  content type

#### Scenario: Unknown path
- **WHEN** a GET request is made for a path that does not exist
- **THEN** the response SHALL have a 404 status
