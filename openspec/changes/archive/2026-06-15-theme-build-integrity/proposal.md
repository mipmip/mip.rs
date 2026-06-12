## Why

`asset/theme1/template.html` is a build artifact generated from `theme_src/` by the `inliner` (Makefile `compthemes` target), yet it is committed to git and therefore editable. A large amount of theming work — CSS custom properties, dark mode, print palette, frontmatter table styling, section numbers, the `#{THEME_CLASS}` html attribute, and the `#{CUSTOM_CSS}` style block — was hand-edited directly into the generated artifact and **never back-ported to `theme_src/`**. Because the binary embeds the generated file, the app works today; but the next `make compthemes`/`make build` regenerates from the stale source and **silently deletes all of it** (verified: the generated file shrinks ~2 KB and reverts to a pre-dark-mode state). Nothing guards against this, so the regression is one routine build away.

Worse, the pipeline is now **completely broken**: the `inliner` dependency (v1.13.1, last released ~2018, depends on the deprecated `request` package and `update-notifier`) **hangs forever on Node 24** — the version the dev shell's `pkgs.nodejs` now resolves to. It hangs even on trivial input, so `make compthemes` cannot run at all, and its `>` redirect truncates `asset/theme1/template.html` to 0 bytes before the (never-arriving) output, destroying the artifact. The drifted artifact survives only because nobody can successfully regenerate it. Restoring the source-of-truth invariant therefore requires first replacing the build tool.

## What Changes

- Replace the abandoned `inliner` dependency with a small, dependency-light generator script (plain Node, no third-party deps) that inlines `style.css` and `bridge.js` into `template-src.html` and preserves the runtime `#{...}` placeholders. Deterministic, no network, works on current Node.
- Back-port the already-drifted styling/markup from `asset/theme1/template.html` into `theme_src/theme1/` so the source and the generated artifact are back in sync.
- Establish `theme_src/` as the single source of truth for theme assets, and `asset/theme1/template.html` as a generated, never-hand-edited artifact.
- Re-commit the freshly generated artifact as the new canonical baseline, and require it to be byte-reproducible from `theme_src/` via the new generator, so drift is detectable.
- Add a regenerate-and-diff guard (a `make` check usable in CI / pre-commit) that fails when the committed artifact does not match a fresh regeneration from `theme_src/`.
- Document the prohibition on editing the generated artifact directly (where styling changes must go instead).

This change replaces the build tool, then back-ports, then adds the guard, in that order, so the guard lands green: with a working generator and the drifted styling restored to `theme_src/`, the committed artifact is whatever a fresh regeneration produces, and the check passes by construction.

Because the old `inliner` minified its output and is now unrunnable, the new generator defines a *new* canonical generation; the committed `asset/theme1/template.html` is replaced with the new generator's output rather than reverse-engineered to match the old minified bytes. The rendered result is equivalent (same inlined CSS/JS, same placeholders); only incidental whitespace/minification differs.

## Capabilities

### New Capabilities
- `theme-build-pipeline`: Defines the theme build pipeline's integrity contract — `theme_src/` as source of truth, the generated artifact's reproducibility, the prohibition on direct edits, and the drift-detection check.

### Modified Capabilities
<!-- None. The behavioral `theming` spec already correctly requires the drifted
     features (CSS variables, dark mode, print theme, frontmatter dark colors,
     html theme class); this change adds a separate build-integrity capability
     rather than altering those behavioral requirements. -->

## Impact

- **Build tool**: the `inliner` devDependency is removed from `package.json`; a new committed generator script (e.g. `scripts/inline-theme.mjs`) replaces it. The `compthemes` target invokes the new script instead of `yarn run inliner`.
- **Theme source**: `theme_src/theme1/{template-src.html, style.css}` gains the drifted styling/markup back-ported from the generated artifact.
- **Generated artifact**: `asset/theme1/template.html` is re-committed as the new generator's output (functionally equivalent; incidental whitespace differs).
- **Build**: `Makefile` gains a check target that regenerates the template into a temp file and diffs it against the committed `asset/theme1/template.html`, failing on mismatch. Wired into `make check` / CI.
- **Docs**: A short note (CLAUDE.md and/or `theme_src/` README) stating the artifact is generated and must not be hand-edited.
- **Source files referenced**: pipeline `Makefile` (`compthemes`), source `theme_src/theme1/{template-src.html, style.css, bridge.js}`, generated `asset/theme1/template.html`, embed `src/markdown.rs` (`#[folder = "asset/theme1"]`, placeholder substitution).
