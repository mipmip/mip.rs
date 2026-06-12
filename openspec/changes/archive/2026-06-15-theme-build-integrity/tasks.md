## 1. Replace the broken inliner build tool (do first)

- [x] 1.1 Add `scripts/inline-theme.mjs` — a dependency-free Node script that reads `template-src.html`, inlines `<link rel="stylesheet" href="style.css">` as `<style>…</style>` and `<script src="bridge.js"></script>` as `<script>…</script>`, leaves `#{...}` placeholders untouched, and writes the result to stdout
- [x] 1.2 Update the `compthemes` target in the `Makefile` to invoke `node scripts/inline-theme.mjs theme_src/theme1/template-src.html > asset/theme1/template.html` (drop `yarn run inliner` and the `tail`/`head` banner trimming)
- [x] 1.3 Remove the `inliner` dependency from `package.json`
- [x] 1.4 Verify `make compthemes` runs to completion on the dev shell's Node (no hang) and produces a non-empty artifact containing inlined CSS, inlined bridge.js, and all `#{...}` placeholders

## 2. Back-port — restore source into theme_src

- [x] 2.1 Back-port the drifted styling into `theme_src/theme1/style.css` (CSS custom properties on `:root`, `@media(prefers-color-scheme:dark)`, `.dark`/`.light` classes, `@media print` palette, `table.frontmatter` styling, `.section-number` styling)
- [x] 2.2 Back-port the drifted markup into `theme_src/theme1/template-src.html` (`class="#{THEME_CLASS}"` on `<html>`, the `<style id="custom-css">#{CUSTOM_CSS}</style>` block)
- [x] 2.3 Run `make compthemes` to regenerate `asset/theme1/template.html` from the updated source, and re-commit it as the new canonical baseline
- [x] 2.4 Confirm the regenerated artifact contains all runtime placeholders (`#{BODY}`, `#{SEEDURL}`, `#{INITIALSEED}`, `#{THEME_CLASS}`, `#{CUSTOM_CSS}`) and all back-ported styling (CSS variables, dark mode, print palette, frontmatter, section numbers)

## 3. Drift detection check

- [x] 3.1 Add a `check-themes` target to the `Makefile` that regenerates the template with the new generator into a temp path (NOT into `asset/`)
- [x] 3.2 Make the target `diff` the temp output against the committed `asset/theme1/template.html` and exit non-zero on mismatch
- [x] 3.3 On mismatch, print the offending path and a one-line hint ("edit `theme_src/`, run `make compthemes`; never edit `asset/theme1/template.html`")
- [x] 3.4 Verify the target exits 0 when in sync and non-zero after a deliberate test edit, and that running it does not modify the committed artifact

## 4. Wire into build / CI

- [x] 4.1 Add `check-themes` to the `make check` target (alongside `lint` and `test`)
- [x] 4.2 Ensure CI runs `check-themes` (or `make check`); the new generator needs only Node (already in the dev shell), no extra provisioning

## 5. Documentation

- [x] 5.1 Add a "generated file — do not edit; edit `theme_src/` instead" note in `theme_src/` (README or banner) and reference it near the artifact
- [x] 5.2 Add a short line to CLAUDE.md stating `asset/theme1/template.html` is generated from `theme_src/` and must not be hand-edited

## 6. Verification

- [x] 6.1 Run `make check` end-to-end and confirm it passes with the synced artifact
- [x] 6.2 Confirm the app still renders correctly (dark mode, frontmatter table, section numbers, custom CSS injection) after the tool replacement + back-port + regeneration
