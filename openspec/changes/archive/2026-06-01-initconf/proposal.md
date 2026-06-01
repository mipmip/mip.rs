## Why

There's no easy way to create a config file or discover what settings are available. Users have to read code or specs to know what options exist. `mip --initconf` generates a documented config file with all settings and their defaults, serving as both bootstrapping and documentation.

Bean: mip.rs-phqb

## What Changes

- Add `--initconf` CLI flag that writes a documented config template to `~/.config/miprs/config.toml`
- The generated file includes all settings with comments explaining each option
- Refuses to overwrite an existing config file (prints error suggesting backup)
- Creates the parent directory (`~/.config/miprs/`) if it doesn't exist
- Exits after writing (does not open a preview window)

## Capabilities

### New Capabilities
- `initconf`: CLI flag to generate a documented default config file

### Modified Capabilities
- `cli`: Adding `--initconf` switch

## Impact

- `src/main.rs`: add `--initconf` flag, handle before window creation
- `src/config.rs`: add `default_config_template()` function returning the documented template string, add `config_path()` as public
