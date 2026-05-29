## ADDED Requirements

### Requirement: Help output on no arguments
The system SHALL print help text and exit with code 0 when invoked with no arguments.

#### Scenario: No arguments provided
- **WHEN** the user runs `mip` with no arguments
- **THEN** the system SHALL print usage information and exit with code 0

### Requirement: Help flag
The system SHALL print help text and exit with code 0 when `--help` or `-h` is passed.

#### Scenario: Help flag
- **WHEN** the user runs `mip --help`
- **THEN** the system SHALL print usage information including available options and exit with code 0

### Requirement: Version flag
The system SHALL print the version and exit with code 0 when `--version` is passed.

#### Scenario: Version output
- **WHEN** the user runs `mip --version`
- **THEN** the system SHALL print `mip <version>` where version matches `Cargo.toml` and exit with code 0

### Requirement: Verbose flag
The system SHALL accept a `--verbose` or `-v` flag that enables debug output.

#### Scenario: Verbose flag accepted
- **WHEN** the user runs `mip --verbose <file>`
- **THEN** the system SHALL start normally with verbose mode enabled

#### Scenario: Verbose flag without file
- **WHEN** the user runs `mip --verbose` with no file argument
- **THEN** the system SHALL print help text and exit with code 0

### Requirement: File positional argument
The system SHALL accept a file path as a positional argument.

#### Scenario: Valid file path
- **WHEN** the user runs `mip README.md`
- **THEN** the system SHALL open the file for preview

#### Scenario: Unknown flags rejected
- **WHEN** the user passes an unrecognized flag like `--foo`
- **THEN** the system SHALL print an error message with usage hint and exit with a non-zero code
