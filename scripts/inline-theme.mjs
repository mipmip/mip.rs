#!/usr/bin/env node
// Inline a theme template's external <link>/<script> references into a single
// self-contained HTML file. Replaces the abandoned `inliner` dependency, which
// hangs on modern Node. Dependency-free, deterministic, no network.
//
// Usage: node scripts/inline-theme.mjs theme_src/theme1/template-src.html > asset/theme1/template.html
//
// Runtime placeholders (#{...}) in the template are left untouched; they are
// substituted at render time by src/markdown.rs.

import { readFileSync } from 'node:fs';
import { dirname, resolve } from 'node:path';

const src = process.argv[2];
if (!src) {
  process.stderr.write('usage: inline-theme.mjs <template-src.html>\n');
  process.exit(2);
}

const dir = dirname(src);
const read = (rel) => readFileSync(resolve(dir, rel), 'utf8');

let html = readFileSync(src, 'utf8');

// <link rel="stylesheet" href="..."> (optionally self-closed) -> inline <style>
html = html.replace(
  /<link\s+rel="stylesheet"\s+href="([^"]+)"\s*\/?>/g,
  (_m, href) => `<style>${read(href)}</style>`,
);

// <script src="..."></script> -> inline <script>...</script>
html = html.replace(
  /<script\s+src="([^"]+)"><\/script>/g,
  (_m, srcAttr) => `<script>${read(srcAttr)}</script>`,
);

process.stdout.write(html);
