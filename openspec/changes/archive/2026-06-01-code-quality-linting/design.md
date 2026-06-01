## Context

Clippy and rustfmt are in the nix dev shell. No config files needed — defaults are fine.

## Goals / Non-Goals

**Goals:** Lint as part of the workflow, fix existing issues.
**Non-Goals:** Custom lint rules, CI integration (can add later).

## Decisions

### Separate targets: lint, test, check

**Choice**: `make lint` = fmt + clippy, `make test` = cargo test, `make check` = lint + test.

**Why**: Flexible — sometimes you want tests without formatting enforcement.
