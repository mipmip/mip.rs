## ADDED Requirements

### Requirement: Generate default config file
The system SHALL write a documented config template to the default config path when `--initconf` is passed.

#### Scenario: No existing config
- **WHEN** the user runs `mip --initconf` and no config file exists
- **THEN** the system SHALL create `~/.config/miprs/config.toml` with all settings documented in comments, print the path, and exit with code 0

#### Scenario: Config file already exists
- **WHEN** the user runs `mip --initconf` and the config file already exists
- **THEN** the system SHALL print an error message and exit with non-zero code without modifying the existing file

#### Scenario: Parent directory does not exist
- **WHEN** the user runs `mip --initconf` and `~/.config/miprs/` does not exist
- **THEN** the system SHALL create the directory before writing the file

### Requirement: Config template documents all settings
The generated config file SHALL include commented documentation for every available setting.

#### Scenario: All settings present
- **WHEN** the config file is generated
- **THEN** it SHALL contain entries for theme, frontmatter, runcmd, sidetoc_width, sidetoc_position, and a [keybindings] section with default bindings
