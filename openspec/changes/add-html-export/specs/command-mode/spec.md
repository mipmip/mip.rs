## command-mode (modified)

Changes to the existing command mode capability.

### Requirements

#### New command
- MUST add `export_html` to the command list for tab completion
- `export_html` MUST accept a file path argument
- Path argument MUST support tilde expansion (consistent with `open` command)
- Path argument SHOULD support tab-completion of filesystem paths (reuse existing path completion from `open`)
