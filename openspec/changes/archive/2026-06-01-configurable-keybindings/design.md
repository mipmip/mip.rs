## Context

The navigation-commands change removed hardcoded Tab for quicktoc and added runtime commands. Ctrl+P for print is still hardcoded. The command system supports any command string with `;` composition. Keybindings are the last piece — mapping keys to commands.

Depends on: navigation-commands (for the command infrastructure and `;` composition).

## Goals / Non-Goals

**Goals:**
- `[keybindings]` config section mapping key combos to command strings
- Default keybindings (e.g. `tab = "quicktoc"`, `ctrl+p = "print"`)
- Config overrides defaults
- Any command string works as a value (including `;` composed)
- Key combo parsing: modifiers (ctrl, shift, alt, super) + key name

**Non-Goals:**
- Runtime keybinding modification (`:bind` command) — config only for now
- Key sequences / chords (only single key combos)
- Per-mode keybindings (e.g. different bindings in TOC vs document)

## Decisions

### Keybinding registry as HashMap<KeyCombo, String>

**Choice**: A `KeyCombo` struct holding `(keyval: u32, modifiers: ModifierType)`. Parse config strings like `ctrl+p` into KeyCombo. Store in a HashMap mapping to command strings.

```rust
struct KeyCombo {
    keyval: gtk4::gdk::Key,
    modifiers: gtk4::gdk::ModifierType,
}
```

**Rationale**: Fast lookup in the key handler. The key handler checks if the pressed key+modifiers match any registered combo.

### Key combo string format

```
"ctrl+p"          → Ctrl + P
"tab"             → Tab
"ctrl+shift+t"    → Ctrl + Shift + T
"j"               → J (no modifier)
"shift+tab"       → Shift + Tab
```

Parse by splitting on `+`, last part is the key name, rest are modifiers.

### Default keybindings

```rust
defaults = {
    "tab": "quicktoc",
    "ctrl+p": "print",
    "j": "toc_down",   // only active in TOC context
    "k": "toc_up",     // only active in TOC context
}
```

Config overrides: if user sets `tab = "sidetoc_toggle"`, it replaces the default `tab = "quicktoc"`.

### `:` stays special

The `:` command bar activation is NOT a keybinding — it's always active and handled separately. It can't be rebound.

### Key handler flow

```
key pressed
  → if command bar visible: handle in entry (existing logic)
  → else: look up (keyval, modifiers) in keybinding registry
    → if found: execute_commands(command_string)
    → if not found: Propagation::Proceed
```

## Risks / Trade-offs

- [Key name mapping] → Need to map strings like "tab", "escape", "a"–"z", "f1"–"f12" to GDK key constants. Mitigation: support common keys, warn on unknown.
- [Modifier conflicts] → User might bind something that conflicts with GTK internals. Mitigation: document known conflicts.
