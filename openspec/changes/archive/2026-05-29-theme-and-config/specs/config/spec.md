## ADDED Requirements

### Requirement: Config file loading
The system SHALL load configuration from `~/.config/miprs/config.toml` if the file exists, using `$XDG_CONFIG_HOME/miprs/config.toml` when `XDG_CONFIG_HOME` is set.

#### Scenario: Config file exists
- **WHEN** `~/.config/miprs/config.toml` exists with valid TOML
- **THEN** the system SHALL apply the configured values as defaults

#### Scenario: Config file missing
- **WHEN** the config file does not exist
- **THEN** the system SHALL use built-in defaults without error

#### Scenario: Invalid config value
- **WHEN** the config file contains an invalid value for a known key
- **THEN** the system SHALL print a warning and use the default for that field

### Requirement: CLI flags override config
CLI flags SHALL take precedence over config file values.

#### Scenario: CLI overrides config theme
- **WHEN** the config file sets `theme = "dark"` and the user passes `--theme light`
- **THEN** the system SHALL use light theme

#### Scenario: CLI overrides config frontmatter
- **WHEN** the config file sets `frontmatter = true` and the user does not pass `--frontmatter`
- **THEN** the system SHALL show frontmatter (config value applies)

### Requirement: Config supports theme setting
The config file SHALL accept a `theme` key with values `"system"`, `"light"`, or `"dark"`.

#### Scenario: Theme in config
- **WHEN** the config file contains `theme = "dark"`
- **THEN** the system SHALL use dark theme unless overridden by CLI

### Requirement: Config supports frontmatter setting
The config file SHALL accept a `frontmatter` key with boolean value.

#### Scenario: Frontmatter in config
- **WHEN** the config file contains `frontmatter = true`
- **THEN** the system SHALL show frontmatter unless overridden by CLI
