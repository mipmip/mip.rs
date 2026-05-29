## Context

mip.rs is a ~160-line Rust app across 5 modules. Several modules mix pure logic with I/O, making them hard to test without refactoring. The codebase has no tests and no coverage tooling.

## Goals / Non-Goals

**Goals:**
- Achieve realistic test coverage (target: 60-75% line coverage)
- Make pure logic testable by extracting it from I/O
- Establish a local coverage workflow that updates README.md
- Cover the highest-value code paths: markdown conversion, config loading, server routes

**Non-Goals:**
- Testing GTK4/WebKit window rendering (requires display server, low ROI)
- CI integration (local-only for now)
- 100% coverage (would require mocking GTK, not worth it)
- Changing any user-facing behavior

## Decisions

### 1. Extract pure functions from I/O wrappers

**Decision**: Split `to_file()` into `build_html()` (pure: markdown+template→String) and `to_file()` (I/O: writes to disk). Similarly, extract the warp filter chain from `run_bro()` into a `routes()` function.

**Why**: These are the highest-value test targets. Testing them currently requires temp dirs and real ports. Extracting the pure parts makes them testable with simple string assertions.

**Alternative considered**: Testing through the I/O functions with temp dirs. Rejected because it adds complexity without testing different logic.

### 2. Make config path injectable

**Decision**: `Config::load()` gains an optional path parameter (or `Config::load_from(path)`). The default `Config::load()` still reads from `XDG_CONFIG_HOME`.

**Why**: Integration tests need to point at temp config files without polluting the real config dir or relying on env var manipulation.

**Alternative considered**: Setting `XDG_CONFIG_HOME` in tests. Rejected because env vars are process-global and would cause test interference with parallel execution.

### 3. cargo-tarpaulin for coverage

**Decision**: Use `cargo-tarpaulin` installed as a standalone tool (not a Cargo.toml dependency).

**Why**: Linux-only project, tarpaulin is mature and simple. No need for the LLVM instrumentation complexity of `cargo-llvm-cov`.

**Alternative considered**: `cargo-llvm-cov` — more accurate but harder to set up and overkill for this project size.

### 4. Integration tests in `tests/` directory

**Decision**: Use `tests/` directory for integration tests, with in-module `#[cfg(test)]` for small unit tests on helpers.

**Why**: Integration tests exercise the public API of each module. Unit tests stay close to the code they test. This is idiomatic Rust.

### 5. Plain-text coverage in README

**Decision**: A single line like `Coverage: 68%` at the top of README.md, updated by a shell script that runs tarpaulin and patches the file.

**Why**: No external services, no badge APIs, no CI. Just a number that's honest and easy to update.

**Alternative considered**: Shields.io badge. Rejected — requires hosting coverage data somewhere, overkill for local workflow.

## Risks / Trade-offs

- **[Risk] Refactoring introduces bugs** → Mitigation: refactoring is purely structural (extract function), no logic changes. Existing behavior is preserved. Tests written against the refactored code serve as verification.
- **[Risk] tarpaulin coverage numbers vary between runs** → Mitigation: accept small variance; the number is indicative, not a gate.
- **[Risk] GTK/WebKit code stays untested** → Accepted trade-off: `view.rs::window()` is ~40 lines of GTK boilerplate. Manual testing is the right approach here.
