## Why

There's no release process — `make release` is a WIP stub, version is duplicated across `Cargo.toml` and `package.nix`, the changelog `## Unreleased` section has to be manually dated, and the existing GitHub Actions release workflow is stale (still tries to cross-compile for Windows/macOS which were dropped in v0.3.0). Releasing a new version requires too many manual, error-prone steps.

Bean: [mip.rs-8uxm](/home/pim/cLinden/mip.rs/.beans/mip.rs-8uxm--release-workflow.md)

## What Changes

- Add an interactive `scripts/release.sh` using `gum` for major/minor/hotfix selection
- Make `Cargo.toml` the single source of truth for version; `package.nix` reads from it
- Release script bumps version, stamps changelog date, creates jj bookmark + git tag
- Add `gum` to the nix flake dev shell
- Replace the stale `release.yml` GitHub Action with a Linux-only workflow that builds `.deb` via `cargo-deb` and nix, attaching artifacts to the GitHub Release
- Replace the `make release` stub with a call to the release script

## Capabilities

### New Capabilities
- `release-script`: Interactive gum-based release script that bumps version, updates changelog, tags the release
- `deb-packaging`: `.deb` package generation via `cargo-deb` with proper GTK4/WebKit runtime dependencies

### Modified Capabilities

_(none — no existing spec-level requirements change)_

## Impact

- **New files**: `scripts/release.sh`, updated `.github/workflows/release.yml`
- **Modified files**: `Cargo.toml` (add `[package.metadata.deb]`), `package.nix` (read version from Cargo.toml), `flake.nix` (add `gum` to dev shell), `Makefile` (replace `release` stub)
- **Dependencies**: `gum` (dev shell only), `cargo-deb` (CI only)
- **Removed**: stale cross-platform release targets (Windows, macOS) from CI
