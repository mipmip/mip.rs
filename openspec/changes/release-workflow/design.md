## Context

mip.rs is a Linux-only GTK4/WebKit markdown previewer. Version is currently duplicated in `Cargo.toml` and `package.nix`. The changelog follows a `## Unreleased` / `## vX.Y.Z - DD Mon YYYY` format. The project uses jj (colocated with git) for version control. The existing GitHub Actions release workflow cross-compiles for Windows/macOS which no longer applies.

## Goals / Non-Goals

**Goals:**
- One-command interactive release: pick bump type, everything else is automated
- Single source of truth for version (`Cargo.toml`)
- Changelog gets dated automatically
- Git tag created for GitHub Release trigger
- `.deb` package built in CI and attached to the release

**Non-Goals:**
- Flatpak, AppImage, or RPM packaging (can be added later)
- Auto-publishing to crates.io
- Headless/non-interactive release (the script is developer-facing)

## Decisions

### 1. gum for interactive prompts

**Choice**: Use `charm/gum` for the release type selection (major/minor/hotfix) and confirmation prompts.

**Why**: gum is simple, beautiful, single-binary, available in nixpkgs. No need for a custom TUI.

**Alternative considered**: Plain bash `select`. Rejected — less polished, gum is already the standard for this kind of thing.

### 2. Cargo.toml as version source of truth

**Choice**: `Cargo.toml` `[package].version` is the canonical version. `package.nix` extracts it at build time using `builtins.fromTOML` or similar.

**Why**: Cargo.toml is already required by Rust. Duplicating in package.nix is error-prone. The nix build can read it.

**Alternative considered**: A standalone `VERSION` file. Rejected — adds yet another file, Cargo.toml already has the field.

### 3. Release script flow

**Choice**: `scripts/release.sh` performs these steps in order:
1. `gum choose "major" "minor" "hotfix"` → determines bump type
2. Calculate new version from current `Cargo.toml` version
3. Show diff and `gum confirm` before proceeding
4. Update `Cargo.toml` version
5. Replace `## Unreleased` in CHANGELOG.md with `## vX.Y.Z - DD Mon YYYY`
6. Add new empty `## Unreleased` section at top
7. `jj describe` with release message
8. `jj bookmark set` for the version
9. `jj git export` + `git tag vX.Y.Z`
10. Optionally `jj git push` (with confirmation)

**Why**: Each step is visible and confirmable. The jj/git dance is necessary because jj doesn't have native tags — we create the tag via git after exporting.

### 4. cargo-deb for .deb packaging

**Choice**: Add `[package.metadata.deb]` to Cargo.toml with runtime dependencies (`libgtk-4-1`, `libwebkitgtk-6.0-4`, `gstreamer1.0-plugins-base`, `gstreamer1.0-plugins-good`). CI runs `cargo deb` to produce the `.deb`.

**Why**: cargo-deb reads metadata from Cargo.toml, generates proper Debian packages with dependency declarations. Minimal config, well-maintained.

**Alternative considered**: `nfpm` (generic package builder). Rejected — cargo-deb integrates tighter with Rust projects.

### 5. GitHub Actions: Linux-only, tag-triggered

**Choice**: Replace the stale `release.yml` with a workflow that triggers on `v*` tags, installs GTK4/WebKit dev dependencies via `apt`, runs `cargo deb`, and uploads the `.deb` as a release asset.

**Why**: Simple, matches the Linux-only reality. The nix build is handled by the existing `build_nix.yml`.

## Risks / Trade-offs

- **[Risk] Ubuntu runner may not have matching GTK4/WebKit versions** → Mitigation: pin to `ubuntu-24.04` which has GTK4 and WebKitGTK 6.0 in its repos.
- **[Risk] cargo-deb may not handle the WebKit runtime dep correctly** → Mitigation: explicitly declare deps in `[package.metadata.deb]`.
- **[Risk] jj/git tag dance is fragile** → Mitigation: the script verifies each step and aborts on failure.
