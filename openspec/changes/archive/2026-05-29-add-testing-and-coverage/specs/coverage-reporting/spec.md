## ADDED Requirements

### Requirement: Coverage percentage is displayed in README
The README.md SHALL display the current test coverage as a plain-text percentage near the top of the file.

#### Scenario: Coverage line format
- **WHEN** a user reads the top of README.md
- **THEN** there SHALL be a line in the format `Coverage: <N>%` where N is an integer

### Requirement: Coverage can be measured and updated locally
There SHALL be a script or recipe that runs the test suite with coverage measurement and updates the README percentage.

#### Scenario: Running the coverage update
- **WHEN** a developer runs the coverage script
- **THEN** `cargo-tarpaulin` SHALL execute all tests, measure line coverage, and patch the coverage line in README.md with the new percentage

#### Scenario: README is only modified if coverage changes
- **WHEN** the coverage script runs and the percentage is the same as already recorded
- **THEN** README.md SHALL NOT be modified (no spurious diffs)
