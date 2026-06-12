# theme_src — source of truth for the bundled theme

This directory is the **single source of truth** for mip's bundled theme.

```
theme_src/theme1/template-src.html   HTML template (references style.css, bridge.js)
theme_src/theme1/style.css           theme CSS (colors via CSS variables, dark mode, print)
theme_src/theme1/bridge.js           in-page JavaScript
        │  make compthemes  (node scripts/inline-theme.mjs)
        ▼
asset/theme1/template.html           GENERATED — embedded into the binary by rust-embed
```

## Do not edit `asset/theme1/template.html`

`asset/theme1/template.html` is a **generated build artifact**. Any change you make
there will be silently overwritten the next time `make compthemes` runs. Edit the
files in `theme_src/` instead, then regenerate:

```sh
make compthemes
```

`make check` (and CI) run `make check-themes`, which regenerates the template and
fails if the committed `asset/theme1/template.html` does not match — so drift is
caught automatically.

The `#{...}` tokens (`#{BODY}`, `#{SEEDURL}`, `#{INITIALSEED}`, `#{THEME_CLASS}`,
`#{CUSTOM_CSS}`) are runtime placeholders substituted by `src/markdown.rs`; leave
them intact.
