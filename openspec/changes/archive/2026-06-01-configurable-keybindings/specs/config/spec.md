## ADDED Requirements

### Requirement: Keybindings config section
The config file SHALL accept a `[keybindings]` section mapping key combo strings to command strings.

#### Scenario: Keybindings in config
- **WHEN** the config file contains:
  ```toml
  [keybindings]
  tab = "quicktoc"
  ctrl+p = "print"
  ctrl+y = "open ~/todo.md"
  ```
- **THEN** the system SHALL register these keybindings at startup

#### Scenario: Invalid key name in config
- **WHEN** the config file contains `xyzkey = "print"`
- **THEN** the system SHALL print a warning and skip that binding
