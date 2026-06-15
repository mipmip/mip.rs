## 1. Version source of truth

- [x] 1.1 Update `package.nix` to read version from `Cargo.toml` instead of hardcoding it
- [x] 1.2 Verify `nix build` still works with the dynamic version

## 2. Release script

- [x] 2.1 Add `gum` to `flake.nix` dev shell `buildInputs`
- [x] 2.2 Create `scripts/release.sh` with gum-based interactive flow: choose bump type, calculate new version, confirm, bump Cargo.toml, stamp changelog, jj describe + bookmark, git tag
- [x] 2.3 Update `Makefile` `release` target to call `scripts/release.sh`

## 3. Debian packaging

- [x] 3.1 Add `[package.metadata.deb]` section to `Cargo.toml` with description, dependencies, section, and asset declarations
- [x] 3.2 Verify `cargo deb` produces a valid `.deb` locally (inside nix develop)

## 4. GitHub Actions

- [x] 4.1 Replace `.github/workflows/release.yml` with Linux-only workflow: trigger on `v*` tags, install GTK4/WebKit dev deps, run `cargo deb`, upload `.deb` to release
- [x] 4.2 Remove stale Windows/macOS cross-compilation targets

## 5. Verify

- [x] 5.1 Dry-run the release script (without pushing) to verify all steps work
- [x] 5.2 `nix build` succeeds with dynamic version
