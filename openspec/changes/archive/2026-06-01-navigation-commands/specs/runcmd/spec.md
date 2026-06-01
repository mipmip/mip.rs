## ADDED Requirements

### Requirement: --runcmd CLI option
The system SHALL accept a `--runcmd` CLI option that executes a command string at startup.

#### Scenario: Single startup command
- **WHEN** the user runs `mip --runcmd sidetoc_open README.md`
- **THEN** the system SHALL execute `sidetoc_open` after the window is created

#### Scenario: Composed startup commands
- **WHEN** the user runs `mip --runcmd "sidetoc_open; set theme dark" README.md`
- **THEN** the system SHALL execute both commands in sequence at startup

#### Scenario: Runcmd replaces --toc
- **WHEN** the user runs `mip --toc side README.md`
- **THEN** the system SHALL print an error suggesting `--runcmd sidetoc_open` instead

### Requirement: Config runcmd overridden by CLI
CLI `--runcmd` SHALL take precedence over config file `runcmd`.

#### Scenario: CLI overrides config runcmd
- **WHEN** the config has `runcmd = "sidetoc_open"` and the user passes `--runcmd quicktoc`
- **THEN** the system SHALL execute `quicktoc` (CLI wins)
