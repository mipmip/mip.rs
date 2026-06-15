## Context

`mip` maps key combos to command strings through `KeybindingRegistry`. Defaults are registered in `KeybindingRegistry::with_defaults()` (`src/command.rs`) via `register_str(combo, command)` — e.g. `register_str("ctrl+p", "print")`. User config (`[keybindings]`) overrides defaults for the same combo. At keypress, the handler in `src/view.rs` looks up the combo and dispatches the command; the `"q" | "close"` arm calls `ctx.app.quit()`.

The combo-string parser (`parse_key_combo`) already accepts `ctrl`/`alt` modifiers and the keys `q`, `w`, `f4` (verified in `is_known_key`). So `"ctrl+q"`, `"ctrl+w"`, `"alt+f4"` are valid bindings today; they simply aren't registered.

A prior community PR (#12) added these as hardcoded checks in the keypress handler, ahead of the registry lookup. That code is not in the current branch, and being hardcoded it could not be remapped. This change takes the registry route instead.

## Goals / Non-Goals

**Goals:**
- Provide Ctrl+Q, Ctrl+W, Alt+F4 as default close shortcuts.
- Keep them overridable/disableable via config, like every other default binding.

**Non-Goals:**
- Introducing a new command (the existing `close`/`q` already quits).
- Any hardcoded key handling in `src/view.rs`.
- A "confirm before quit" prompt or unsaved-changes handling (mip is a read-only viewer).

## Decisions

**Decision: Map all three combos to the existing `close` command in `with_defaults()`.**
Three `register_str("...", "close")` lines next to the other defaults. `close` already dispatches to `app.quit()`, so no dispatch change is needed. Using `close` (not its alias `q`) is clearer in config listings.

**Decision: Rely on the registry, not hardcoded handlers.**
This is the whole point of choosing keybindings over PR #12's approach: routing through the registry means user config can override (`ctrl+w = "..."`) or remove them. Consistent with how Tab/Ctrl+P/Ctrl+0 work.

## Risks / Trade-offs

- **Alt+F4 / Ctrl+W may be intercepted by the window manager or compositor before reaching the app** → On most Linux desktops these reach the focused GTK app; where the WM grabs them first, the WM's own close action still fires, so the user still gets "close" behavior. No worse than today.
- **A user who rebinds `ctrl+w` loses the close shortcut** → Intended and consistent with all other overridable defaults; the other two (and `:close`) remain.
- **Ctrl+W is sometimes "close tab"/"delete word" elsewhere** → For a single-window viewer, "close window" is the natural meaning; acceptable.

## Migration Plan

1. Add the three `register_str(..., "close")` lines to `with_defaults()`.
2. Add the shortcuts to the config-template comment block and a unit test.
3. (Docs) CHANGELOG entry under `## Unreleased`.

Additive only; no migration. Existing custom keybindings continue to override.
