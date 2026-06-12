## Requirements

### Requirement: theme_src is the single source of truth
The theme sources under `theme_src/theme1/` (`template-src.html`, `style.css`, `bridge.js`) SHALL be the single source of truth for the bundled theme. `asset/theme1/template.html` SHALL be treated as a generated build artifact produced from those sources by the theme generator (`make compthemes`).

#### Scenario: Styling change made in source
- **WHEN** a theme styling or markup change is required
- **THEN** the change SHALL be made in `theme_src/theme1/` and the generated artifact SHALL be regenerated from it

#### Scenario: Generated artifact is not edited directly
- **WHEN** a contributor (human or AI) needs to change theme appearance or template markup
- **THEN** they SHALL NOT hand-edit `asset/theme1/template.html`, because the next regeneration would silently overwrite such edits

### Requirement: Theme generator has no unmaintained or network dependencies
The theme generator SHALL inline `style.css` and `bridge.js` into `template-src.html` using only maintained, dependency-light tooling, and SHALL NOT require network access or perform remote fetches. It SHALL run on the Node version provided by the project's dev shell.

#### Scenario: Generator runs on current Node offline
- **WHEN** `make compthemes` is run in the dev shell with no network access
- **THEN** it SHALL complete and write `asset/theme1/template.html` without hanging

#### Scenario: No abandoned inliner dependency
- **WHEN** the project dependencies are inspected
- **THEN** the abandoned `inliner` package SHALL NOT be a dependency of the theme build

### Requirement: Generated artifact is reproducible from source
`asset/theme1/template.html` SHALL be byte-for-byte reproducible by running the documented theme generator over `theme_src/theme1/template-src.html`. The committed artifact SHALL match the output of a fresh regeneration.

#### Scenario: Committed artifact matches regeneration
- **WHEN** the template is regenerated from the current `theme_src/` into a fresh file
- **THEN** that output SHALL be identical to the committed `asset/theme1/template.html`

#### Scenario: Runtime placeholders are preserved
- **WHEN** the artifact is regenerated
- **THEN** the runtime placeholders `#{BODY}`, `#{SEEDURL}`, `#{INITIALSEED}`, `#{THEME_CLASS}`, and `#{CUSTOM_CSS}` SHALL be present in the output unchanged, so `src/markdown.rs` substitution continues to work

### Requirement: Drift detection check
The build SHALL provide a check that regenerates the template from `theme_src/` and fails when the result differs from the committed `asset/theme1/template.html`. This check SHALL be runnable in CI and as part of `make check`.

#### Scenario: Artifact is in sync
- **WHEN** the committed artifact matches a fresh regeneration from `theme_src/`
- **THEN** the check SHALL pass with a zero exit code

#### Scenario: Artifact has drifted
- **WHEN** the committed artifact differs from a fresh regeneration from `theme_src/` (e.g. it was hand-edited, or `theme_src/` changed without regenerating)
- **THEN** the check SHALL fail with a non-zero exit code and report which file is out of sync

#### Scenario: Check does not mutate the committed artifact
- **WHEN** the drift detection check runs
- **THEN** it SHALL regenerate into a temporary location and SHALL NOT modify the committed `asset/theme1/template.html`
