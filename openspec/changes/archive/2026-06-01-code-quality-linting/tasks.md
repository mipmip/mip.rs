## 1. Fix existing issues

- [x] 1.1 Run `cargo fmt` to fix formatting drift
- [x] 1.2 Run `cargo clippy --fix` to fix the collapsible_if warning

## 2. Makefile targets

- [x] 2.1 Add `make lint` target: `cargo fmt --check && cargo clippy -- -D warnings`
- [x] 2.2 Add `make check` target: `make lint && make test`

## 3. Verify

- [x] 3.1 `make check` passes cleanly
