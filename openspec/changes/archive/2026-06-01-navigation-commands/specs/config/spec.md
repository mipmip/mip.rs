## ADDED Requirements

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

## REMOVED Requirements

### Requirement: Toc config setting
**Reason**: Replaced by `runcmd` setting and sidetoc/quicktoc commands
**Migration**: Use `runcmd = "sidetoc_open"` or `runcmd = "quicktoc"` instead of `toc = "side"` or `toc = "zathura"`
