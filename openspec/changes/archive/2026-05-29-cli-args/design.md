## Context

mip currently parses arguments manually with `std::env::args().nth(1).expect(...)`. This panics on no arguments, and there's an unreachable `args.len() < 2` guard after the panic. No flags exist.

## Goals / Non-Goals

**Goals:**
- Clean CLI with `--help`, `--version`, `--verbose` flags
- No-args invocation prints help and exits 0
- Foundation for future flags (`--port`, `--theme`, etc.)

**Non-Goals:**
- Implementing verbose logging infrastructure (flag is wired but output comes later)
- Subcommands

## Decisions

### Use `argh` for argument parsing

**Choice**: `argh` over `pico-args` or hand-rolled parsing.

**Rationale**: argh is actively maintained (2026), generates `--help` from doc comments (stays in sync), and adding future flags is one struct field. The syn 2.0 compile cost is already paid by rust-embed. pico-args is unmaintained since 2022 and requires hand-written help text.

**Alternatives considered**:
- `pico-args`: Zero deps but stalled since 2022, manual help text drifts
- `clap`: Too heavy for this project's needs
- Hand-rolled: Works for 3 flags but doesn't scale

### Make file argument optional in the struct

The `file` field is `Option<PathBuf>`. When `None`, print help and exit 0. This avoids argh's default error on missing positional args and matches the bean requirement (uloj) for friendly no-args behavior.

### Version handled manually

argh has no built-in `--version`. Add a `--version` bool switch, print `mip {version}` from `env!("CARGO_PKG_VERSION")`, exit 0.

## Risks / Trade-offs

- [argh is Google/Fuchsia-maintained] → Wide usage, but if abandoned, the derive macro is simple enough to vendor or replace with clap later.
- [Optional file means argh won't auto-error on missing file] → We handle the None case explicitly with help output, which is the desired UX anyway.
