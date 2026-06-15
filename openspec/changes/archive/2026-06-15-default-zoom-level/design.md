## Context

`mip` renders documents in a WebKitGTK `WebView` created with `WebView::new()` in `src/view.rs` (~line 795). The WebView starts at zoom 1.0 and is only ever rescaled by the existing `zoom_in`/`zoom_out`/`zoom_reset` commands (`src/view.rs:502-512`), which clamp to 0.3–5.0. There is no startup/default zoom, so on HiDPI/Wayland-scaled displays users must re-zoom every launch.

Config follows a consistent pattern in `src/config.rs`: each setting is an `Option<T>` field on `Config` (`#[serde(default)]`) with an accessor that supplies the default (e.g. `theme()` → `"system"`, `sidetoc_width()` → `250`). CLI override flags live in `src/main.rs` and take precedence over config. Runtime `:set <name> <value>` is dispatched in `src/view.rs` with the settable names listed in `SETTINGS` (`src/command.rs`) for completion.

## Goals / Non-Goals

**Goals:**
- A persisted default zoom applied at startup, overridable per-run via CLI and adjustable live via `:set`.
- Reuse the existing zoom bounds (0.3–5.0) and the established config/CLI/`:set` patterns exactly.

**Non-Goals:**
- Persisting the *current* zoom back to config when the user presses `Ctrl+=`/`Ctrl+-` (one-directional: config → startup only).
- Auto-detecting display DPI / compensating for compositor scaling. The user sets the factor explicitly.
- Changing the existing relative zoom commands.

## Decisions

**Decision: `zoom: Option<f64>` on `Config` with a `zoom()` accessor that defaults to 1.0 and clamps to 0.3–5.0.**
Mirrors `sidetoc_width()` / `paragraph_numbers_start()` (which already clamps). Keeping the clamp in the accessor means every consumer (startup, `:set`) gets a valid value without duplicating bounds. `f64` matches `webkit6` `set_zoom_level`.

**Decision: `--zoom <f64>` CLI flag overrides config when present.**
Consistent with existing override flags. Resolution order: `--zoom` (if passed) → config `zoom` → 1.0. Apply the resolved value once, right after `WebView::new()`.

**Decision: `:set zoom <factor>` parses a float, clamps, and calls `set_zoom_level` directly.**
The `:set` arm reuses the same clamp bounds; invalid input prints a warning (matching the existing `theme`/`paragraph_numbers_start` arms that warn on bad values) and leaves the current zoom unchanged. Add `"zoom"` to `SETTINGS` for completion.

**Decision: Default stays 1.0.**
Behavior is unchanged for everyone unless they opt in. A non-1.0 global default would surprise users on non-HiDPI displays where 1.0 is already correct.

## Risks / Trade-offs

- **A user sets an extreme value and the document becomes unreadable** → The 0.3–5.0 clamp bounds it; `zoom_reset` (Ctrl+0) still returns to 1.0 at runtime.
- **`:set zoom` changes live zoom but does not write config** → Intended (consistent with other `:set` commands being session-only); documented implicitly by the config being the persistence mechanism.
- **Float formatting in the config template** → Document as `zoom = 1.0` (a plain float); TOML parses it into `Option<f64>` via serde.

## Migration Plan

1. Add `zoom` field + `zoom()` accessor (with clamp) to `src/config.rs`; add config-template line and unit tests.
2. Add `--zoom` flag to `src/main.rs`; thread the resolved value to the view setup.
3. Apply `set_zoom_level(resolved)` after `WebView::new()` in `src/view.rs`.
4. Add `"zoom"` to `SETTINGS` and a `"zoom"` arm to the `:set` dispatch.

No data migration; the setting is additive and defaults to current behavior.
