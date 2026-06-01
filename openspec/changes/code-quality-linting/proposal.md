## Why

Clippy and rustfmt are available but not integrated into the workflow. There's 1 clippy warning and formatting drift across files. The bean asks for linting as part of "full testing".

Bean: [mip.rs-z2g7](/home/pim/cLinden/mip.rs/.beans/mip.rs-z2g7--code-quality-and-linting.md)

## What Changes

- Fix existing clippy warning and formatting drift
- Add `make lint` (fmt check + clippy with -D warnings)
- Add `make check` (lint + test — the "full testing" target)

## Capabilities

### New Capabilities
_(none — tooling only)_

### Modified Capabilities
_(none)_

## Impact

- `Makefile`: add `lint` and `check` targets
- All `.rs` files: `cargo fmt` applied
- `src/main.rs`: fix 1 clippy warning
