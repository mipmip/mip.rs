## keybindings (modified)

Extend the keybinding system to support key sequences.

### Requirements

#### Sequence syntax
- MUST support comma-separated key sequences in binding strings: `"g,g"`, `"z,z"`, `"ctrl+g,g"`
- Each element of a sequence MUST be a valid KeyCombo (key name + optional modifiers)
- Single-key bindings (no comma) MUST continue to work unchanged
- MUST validate all keys in a sequence — reject the binding if any key is unknown

#### Sequence execution
- When the first key of a sequence is pressed, MUST consume the event and enter pending state
- When the second key is pressed within 500ms, MUST execute the bound command
- When 500ms elapses without a second key, the pending state MUST be silently discarded on next keypress
- When a non-matching key is pressed during pending state, MUST discard pending state and process the new key normally
- MUST NOT use timers or async — timeout is checked on next keypress via elapsed time comparison

#### Conflict resolution
- If a key is both a single-combo binding AND the first key of a sequence, the sequence MUST take priority (the single binding becomes unreachable)
- SHOULD warn at registration time if a single binding conflicts with a sequence prefix

#### Configuration
- Key sequences MUST be configurable in `[keybindings]` section of config.toml using comma syntax
- MUST be documented in the `--initconf` template with examples
- Default sequence bindings MUST be overridable by user config

#### Pending state management
- Pending key state MUST be cleared when the command bar opens
- Pending key state MUST be cleared when focus moves to a TreeView
