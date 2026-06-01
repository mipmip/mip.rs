## 1. Config template

- [x] 1.1 Add `pub fn default_config_template() -> &'static str` to config.rs returning the documented template
- [x] 1.2 Make `config_path()` public in config.rs
- [x] 1.3 Template includes all current settings with comments: theme, frontmatter, runcmd, sidetoc_width, sidetoc_position, [keybindings]

## 2. CLI flag and handler

- [x] 2.1 Add `--initconf` switch to Cli struct in main.rs
- [x] 2.2 Handle `--initconf` early in main (before file argument check): create dir, check existence, write file, print path, exit 0
- [x] 2.3 Print error and exit 1 if config file already exists

## 3. Tests

- [x] 3.1 Test that `default_config_template()` contains all known setting names
- [x] 3.2 Test that the template is valid TOML (parses without error)
- [x] 3.3 Test that the template parses into a valid Config struct

## 4. Verify

- [x] 4.1 `cargo build` succeeds
- [x] 4.2 `cargo test` passes
- [x] 4.3 `mip --initconf` creates the file with documented defaults
- [x] 4.4 `mip --initconf` again prints error (file exists)
- [x] 4.5 `mip --help` shows `--initconf` option
