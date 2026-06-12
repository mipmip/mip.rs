## Context

The theme build pipeline is: `theme_src/theme1/{template-src.html, style.css, bridge.js}` → `yarn run inliner` (Makefile `compthemes` target) → `asset/theme1/template.html`. The Rust binary embeds the *generated* file via `rust-embed` (`src/markdown.rs`, `#[folder = "asset/theme1"]`) and substitutes runtime placeholders (`#{BODY}`, `#{SEEDURL}`, `#{INITIALSEED}`, `#{THEME_CLASS}`, `#{CUSTOM_CSS}`) at render time.

The generated artifact is committed to git. Because it is committed and editable, theming work (CSS variables, dark mode, `@media print`, `.frontmatter` table, `.section-number`, the `#{THEME_CLASS}` html attribute, the `#{CUSTOM_CSS}` block) was written directly into it and never reflected back into `theme_src/`. The app works only because the binary embeds the hand-edited artifact; any `make compthemes` would regenerate from the stale source and discard the work. There is currently no mechanism that notices this divergence.

The build tool itself is broken: `inliner` 1.13.1 (last released ~2018, depends on the deprecated `request` package plus `update-notifier`) hangs indefinitely on Node 24 — what the dev shell's unpinned `pkgs.nodejs` now resolves to. It hangs even on trivial input, so `make compthemes` cannot run, and the Makefile's `>` redirect truncates the artifact to 0 bytes on the way to a hang. A minimal dependency-free Node script that inlines the two referenced files was verified to run on Node 24 and produce equivalent output. So the pipeline must be repaired (tool replaced) before the source-of-truth invariant can hold.

## Goals / Non-Goals

**Goals:**
- Restore the drifted styling/markup into `theme_src/` so source and the generated artifact are back in sync (done first, so the guard lands green).
- Make `theme_src/` the authoritative source and the generated artifact verifiably derived from it.
- Detect drift automatically so a routine build can never silently regress theming.
- Keep the check cheap, deterministic, and non-mutating so it runs in `make check` and CI.

**Non-Goals:**
- Changing the runtime theming behavior or the placeholder substitution in `src/markdown.rs`. The back-port restores existing appearance to source; it does not alter rendered output.
- Minifying the generated artifact. The old tool minified; the replacement does not, and we accept the resulting (functionally equivalent) output as the new baseline.
- Reproducing the historical minified bytes of `asset/theme1/template.html`.
- Removing the generated artifact from git or switching to build-time generation inside the Rust build (considered below, deferred).

## Decisions

**Decision: Replace `inliner` with a small committed generator script (plain Node, zero third-party deps).**
`inliner` is abandonware that no longer runs on current Node, and 1.13.1 is its latest release — there is no upgrade. Its job here is narrow and fully known: read `template-src.html`, replace `<link rel="stylesheet" href="style.css">` with `<style>…</style>`, replace `<script src="bridge.js"></script>` with `<script>…</script>`, and leave the `#{...}` placeholders untouched. A ~30-line `scripts/inline-theme.mjs` using only `node:fs`/`node:path` does this deterministically with no network. The `compthemes` target invokes `node scripts/inline-theme.mjs theme_src/theme1/template-src.html > asset/theme1/template.html`.
- *Alternative — pin an old Node just for the theme build:* keeps a fragile abandoned dep and a second toolchain; rejected.
- *Alternative — minify the output (match old inliner behavior):* unnecessary. The artifact is embedded, not served over a network; readability of the generated file is fine and a non-minified file diffs more legibly in the guard. We do not minify.

**Decision: Re-commit the generator's output as the new canonical baseline (do not reverse-engineer the old minified bytes).**
The old artifact was minified by a tool we are deleting; reproducing its exact bytes has no value once `theme_src/` is authoritative. The new generator's output becomes the committed `asset/theme1/template.html`. The byte-exact guard then protects *this* baseline going forward. Equivalence is verified functionally (same inlined CSS/JS, same placeholders, app renders correctly), not by byte-matching the historical artifact.

**Decision: Add a `check-themes` make target that regenerates and diffs, rather than removing the committed artifact.**
The artifact stays committed (so `cargo build` / `rust-embed` needs no Node toolchain), and a check guarantees it stays in sync. Implementation: regenerate the template into a temp file using the exact `compthemes` command, then `diff` against `asset/theme1/template.html`; non-zero exit on mismatch with a message naming the file. Wire it into `make check` (alongside `lint test`) and CI.
- *Alternative — stop committing the artifact and generate it during the build:* cleanest source-of-truth story, but forces a Node/yarn dependency into every Rust build (including `cargo install` and packaging) and complicates `rust-embed`. Rejected for now; revisit if the toolchain is already guaranteed.
- *Alternative — git pre-commit hook only:* not enforced in CI and easy to bypass; use CI/`make check` as the real gate and optionally add a hook as convenience.

**Decision: Diff must be byte-exact.**
The new generator is deterministic (string replacement, no minifier, no network), so byte-exact comparison is the simplest reliable invariant and makes "in sync" unambiguous.

**Decision: The check regenerates into a temp path and never overwrites the committed file.**
Avoids the failure mode where running the check itself mutates the working tree (which is exactly how the original drift got laundered into a commit). The existing `compthemes` target writes directly to `asset/`; the check target must use a temp destination instead.

**Decision: Document the "generated — do not edit" rule next to the artifact and in contributor docs.**
A banner/README in `theme_src/` (and a line in CLAUDE.md) so the prohibition is discoverable at the point of temptation, not only in the spec.

## Risks / Trade-offs

- **The check would fail on the current tree (drift exists today)** → Sequencing handles this: the back-port (this change's first task group) restores `theme_src/` before `check-themes` is wired in, so the check is green when introduced. Order matters — wire the check only after the back-port verifies.
- **`inliner` version differences produce a different byte output across environments** → Pin the `inliner`/Node version (it is already run through the project's yarn/nix toolchain); if drift appears only across environments, treat it as a tooling-pin bug, not a reason to relax the diff.
- **Contributors run the check and see only "files differ" without guidance** → The target should print the offending path and a one-line hint ("run `make compthemes`; edit `theme_src/`, never `asset/`").
- **CI lacks the Node/yarn toolchain** → The check needs the same toolchain as `compthemes`; ensure CI provisions it (the dev shell already does).

## Migration Plan

1. Back-port drifted styling/markup from `asset/theme1/template.html` into `theme_src/theme1/`; verify regeneration reproduces the committed artifact byte-for-byte.
2. Add `check-themes` make target (regenerate to temp + diff, non-mutating).
3. Wire `check-themes` into `make check` and CI.
4. Add the "generated — do not edit" note to `theme_src/` and CLAUDE.md.

Rollback: the check is additive and non-mutating; removing the target reverts to current behavior with no artifact changes.

## Open Questions

- Should the artifact eventually be removed from git in favor of build-time generation, once the Node toolchain is guaranteed in all build paths (packaging, `cargo install`)? Deferred.
- Should a pre-commit hook be added in addition to the CI/`make check` gate, or is the gate sufficient?
