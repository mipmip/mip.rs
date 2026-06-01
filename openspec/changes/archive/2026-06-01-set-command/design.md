## Context

Settings are currently passed as plain values into closures at startup — `show_frontmatter: bool`, `paragraph_numbers: bool`, etc. These are captured by value and cannot be changed at runtime. The `CommandContext` is already an `Rc` shared across closures, making it the natural place for mutable runtime settings.

## Goals / Non-Goals

**Goals:**
- `:set theme dark`, `:set frontmatter true`, etc. work at runtime
- Changed settings take effect immediately (force re-render)
- Tab completion for setting names
- Clean separation between static config (sidetoc_position, runcmd) and runtime-mutable settings

**Non-Goals:**
- `:set` for settings that require layout rebuild (sidetoc_position)
- `:set` for keybindings (too complex for this change)
- Persisting runtime changes back to config file

## Decisions

### RuntimeSettings struct in CommandContext

**Choice**: Add a `RuntimeSettings` struct to `CommandContext` using `Cell<bool>`, `Cell<u8>`, and `RefCell<String>` for mutable fields.

```rust
struct RuntimeSettings {
    frontmatter: Cell<bool>,
    paragraph_numbers: Cell<bool>,
    paragraph_numbers_start: Cell<u8>,
    theme: RefCell<String>,  // "system", "light", "dark"
    force_render: Cell<bool>,
}
```

**Rationale**: `Cell` for Copy types, `RefCell` for String. All inside the existing `Rc<CommandContext>`, accessible from all closures.

### Force re-render flag

**Choice**: `RuntimeSettings::force_render` is a `Cell<bool>`. The `set` command sets it to `true`. The poll loop checks it alongside the seed change — if `force_render` is true, re-render even if the seed hasn't changed, then reset the flag.

**Rationale**: Minimal change to the poll loop. No new channels or signals needed.

### Theme change handling

**Choice**: When `:set theme <value>` is called, update `RuntimeSettings::theme` AND inject the class change via JS (same as the system theme detection does). The force_render flag handles the content re-render.

**Rationale**: Theme needs both CSS class change (immediate visual) and content re-render (for template-level theme_class).

### Setting name completion

**Choice**: Add `SETTINGS` list in command.rs: `["frontmatter", "paragraph_numbers", "paragraph_numbers_start", "theme"]`. The Tab handler detects `:set ` prefix and completes against this list. Same wildmenu as command names.

### Value validation

**Choice**: Validate in the `set` command handler. Invalid values print a warning to stderr but don't crash. Unknown setting names are silently ignored (consistent with unknown commands).

```
:set frontmatter true     → valid (bool)
:set frontmatter banana   → warning, no change
:set theme dark           → valid
:set theme neon           → warning, no change
:set paragraph_numbers_start 3  → valid (1-6)
:set paragraph_numbers_start 9  → warning, clamped to 6
```

## Risks / Trade-offs

- [Refactor scope] → Changing from captured variables to RuntimeSettings touches the poll loop and render calls. Mitigation: RuntimeSettings reads have the same interface as the old variables.
- [Theme re-render] → Changing theme at runtime needs both JS class injection and content re-render. Two separate mechanisms but both already exist.
