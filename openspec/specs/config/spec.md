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

### Requirement: Sidetoc width setting
The config file SHALL accept a `sidetoc_width` key with an integer value (pixels).

#### Scenario: Custom sidetoc width
- **WHEN** the config file contains `sidetoc_width = 300`
- **THEN** the sidetoc panel SHALL open with 300px width

### Requirement: Sidetoc position setting
The config file SHALL accept a `sidetoc_position` key with values `"left"` or `"right"`.

#### Scenario: Sidetoc on right
- **WHEN** the config file contains `sidetoc_position = "right"`
- **THEN** the sidetoc panel SHALL appear on the right side of the document

### Requirement: Runcmd setting
The config file SHALL accept a `runcmd` key with a command string to execute at startup.

#### Scenario: Startup commands via config
- **WHEN** the config file contains `runcmd = "sidetoc_open"`
- **THEN** the system SHALL execute `sidetoc_open` at startup

### Requirement: Keybindings config section
The config file SHALL accept a `[keybindings]` section mapping key combo strings to command strings.

#### Scenario: Keybindings in config
- **WHEN** the config file contains a `[keybindings]` section with key combo = command pairs
- **THEN** the system SHALL register these keybindings at startup

#### Scenario: Invalid key name in config
- **WHEN** the config file contains an invalid key name in `[keybindings]`
- **THEN** the system SHALL print a warning and skip that binding

### Requirement: paragraph_numbers config setting
The config file SHALL accept a `paragraph_numbers` key with boolean value.

#### Scenario: Enable paragraph numbers
- **WHEN** the config file contains `paragraph_numbers = true`
- **THEN** the system SHALL show section numbers on headings

### Requirement: paragraph_numbers_start config setting
The config file SHALL accept a `paragraph_numbers_start` key with integer value (1-6).

#### Scenario: Custom start level
- **WHEN** the config file contains `paragraph_numbers_start = 2`
- **THEN** numbering SHALL start from H2 headings
