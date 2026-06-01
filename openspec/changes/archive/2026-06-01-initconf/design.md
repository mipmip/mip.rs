## Context

mip has a growing config surface: theme, frontmatter, runcmd, sidetoc_width, sidetoc_position, and a [keybindings] section. Users need a way to discover and bootstrap these settings.

## Goals / Non-Goals

**Goals:**
- `mip --initconf` writes a complete, commented config template
- Safe: refuses to overwrite existing file
- Creates parent directory if needed
- Comments serve as inline documentation for all settings

**Non-Goals:**
- Interactive config wizard
- Merging with existing config (just refuses to overwrite)
- Generating config from current runtime state

## Decisions

### Hardcoded template string, not generated from struct

**Choice**: The config template is a `const &str` with handwritten comments and defaults. Not generated from the Config struct's field names.

**Rationale**: Comments, formatting, and example values are the whole point. Auto-generating from struct fields would lose the human-readable documentation. The trade-off is that new settings need to be added to both Config and the template — but that's a small maintenance cost for much better UX.

### Refuse to overwrite, suggest backup

**Choice**: If the config file already exists, print an error like: `Config file already exists at ~/.config/miprs/config.toml. Back it up first if you want to regenerate.`

**Rationale**: Overwriting would lose user customizations silently. A `--force` flag could be added later but isn't needed for v1.

### Exit immediately after writing

**Choice**: `--initconf` writes the file, prints the path, and exits with code 0. No file argument required.

**Rationale**: It's a utility flag, not a preview mode. Same pattern as `--version`.

## Risks / Trade-offs

- [Template drift] → If new settings are added to Config but not to the template, the generated file is incomplete. Mitigation: a test that checks the template contains all Config field names.
