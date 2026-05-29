## ADDED Requirements

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
The system SHALL have tests that verify the full HTML build pipeline: markdown + template → complete HTML document.

#### Scenario: Template placeholders are replaced
- **WHEN** `build_html()` is called with markdown content, a seed URL, and a theme class
- **THEN** the returned HTML string SHALL contain the rendered markdown body, SHALL NOT contain the `#{BODY}` placeholder, and SHALL contain a valid seed value

#### Scenario: Theme class is applied
- **WHEN** `build_html()` is called with theme_class `"dark"`
- **THEN** the returned HTML SHALL include the `dark` class in the appropriate element

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

### Requirement: Server routes are tested
The system SHALL have integration tests that verify the warp server routes serve correct content.

#### Scenario: Serving temp HTML file
- **WHEN** a GET request is made to `/.temp.html`
- **THEN** the response SHALL contain the contents of the temp HTML file with a 200 status

#### Scenario: Serving temp seed file
- **WHEN** a GET request is made to `/.temp.seed`
- **THEN** the response SHALL contain the contents of the temp seed file with a 200 status

#### Scenario: Serving static assets from source dir
- **WHEN** a GET request is made for a file that exists in the source directory
- **THEN** the response SHALL serve that file with a 200 status

### Requirement: View helper functions are tested
The system SHALL have unit tests for pure helper functions in the view module.

#### Scenario: Strip seed scripts removes polling scripts
- **WHEN** `strip_seed_scripts()` is called with HTML containing the seedUrl variable script, the keydown polling script, and the header link script
- **THEN** all three script blocks SHALL be removed from the output

#### Scenario: Strip seed scripts preserves other content
- **WHEN** `strip_seed_scripts()` is called with HTML containing non-seed scripts and body content
- **THEN** the non-seed content SHALL remain unchanged

### Requirement: Port helper functions are tested
The system SHALL have unit tests for `port_is_available()`.

#### Scenario: Available port
- **WHEN** `port_is_available()` is called with a port that is not in use
- **THEN** it SHALL return `true`

#### Scenario: Occupied port
- **WHEN** `port_is_available()` is called with a port that is already bound
- **THEN** it SHALL return `false`
