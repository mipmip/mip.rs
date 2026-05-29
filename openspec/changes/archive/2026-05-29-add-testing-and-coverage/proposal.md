## Why

mip.rs has zero tests. The codebase has grown to five modules with real logic (markdown parsing, config loading, HTML templating, server routing, script stripping) but no way to verify correctness or catch regressions. Adding tests now — while the codebase is still small — establishes the habit and catches bugs before they compound.

Bean: [mip.rs-e87h](/home/pim/cLinden/mip.rs/.beans/mip.rs-e87h--add-testing-and-coverage.md)

## What Changes

- Refactor `markdown.rs` to extract a pure "build full HTML string" function from `to_file()`, separating I/O from logic
- Refactor `server.rs` to extract the warp filter chain from `run_bro()` so routes are testable with `warp::test`
- Refactor `config.rs` so `config_path()` accepts an optional path override instead of reading env vars directly
- Add unit tests for pure functions: `strip_seed_scripts`, `pod_to_html_value`, `port_is_available`
- Add integration tests in `tests/` for markdown pipeline, config loading, and server routes
- Add `cargo-tarpaulin` as the coverage tool (local-only workflow)
- Add a coverage percentage line at the top of `README.md`, updated by a script/recipe after running tests
- Add a script or justfile recipe that runs tarpaulin and patches the README with the result

## Capabilities

### New Capabilities
- `test-suite`: Unit and integration tests covering markdown conversion, config loading, server routes, and view helpers
- `coverage-reporting`: Local coverage measurement with cargo-tarpaulin and a plain-text percentage in README.md

### Modified Capabilities

_(none — no existing spec-level requirements change)_

## Impact

- **Code**: Refactoring touches `markdown.rs`, `server.rs`, `config.rs` (extracting pure functions, no behavior change)
- **New files**: `tests/markdown_test.rs`, `tests/config_test.rs`, `tests/server_test.rs`, coverage update script
- **Dependencies**: `cargo-tarpaulin` as a dev/tool dependency (not added to Cargo.toml, installed separately)
- **README.md**: Gains a coverage percentage line at the top
