## Why

Running `mip` without arguments panics with `.expect()` instead of showing usage. There are no CLI flags (`--help`, `--version`, `--verbose`). As mip grows, we need proper argument parsing.

Beans: mip.rs-spp0, mip.rs-uloj

## What Changes

- Add `argh` crate for derive-based CLI argument parsing
- Replace raw `std::env::args()` parsing in `main.rs` with an `argh`-derived `Cli` struct
- Add `--help` (auto-generated from doc comments), `--version`, and `--verbose` flags
- Running with no arguments prints help and exits cleanly (no panic)
- Remove dead code: the unreachable `args.len() < 2` guard

## Capabilities

### New Capabilities
- `cli`: Command-line interface argument parsing, help text, version output, and verbose mode

### Modified Capabilities

## Impact

- `Cargo.toml`: add `argh` dependency
- `src/main.rs`: rewrite argument handling, wire verbose flag through to enable debug output later
