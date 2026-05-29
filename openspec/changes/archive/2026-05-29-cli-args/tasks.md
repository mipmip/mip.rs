## 1. Add dependency

- [x] 1.1 Add `argh = "0.1"` to Cargo.toml dependencies
- [x] 1.2 Run `cargo update` to resolve the new dependency

## 2. Implement CLI struct

- [x] 2.1 Create `Cli` struct with `#[derive(FromArgs)]` in main.rs
- [x] 2.2 Add `file: Option<PathBuf>` positional field
- [x] 2.3 Add `--version` bool switch
- [x] 2.4 Add `--verbose` / `-v` bool switch

## 3. Wire into main

- [x] 3.1 Replace `std::env::args()` parsing with `argh::from_env()`
- [x] 3.2 Handle `--version`: print `mip {version}` and exit 0
- [x] 3.3 Handle no file: print help and exit 0
- [x] 3.4 Remove dead `args.len() < 2` guard and unused `args` variable
- [x] 3.5 Pass file path from Cli struct to existing logic

## 4. Verify

- [x] 4.1 `cargo build` succeeds
- [x] 4.2 `mip` (no args) prints help, exits 0
- [x] 4.3 `mip --help` prints help, exits 0
- [x] 4.4 `mip --version` prints version, exits 0
- [x] 4.5 `mip README.md` opens preview normally
- [x] 4.6 `mip --verbose README.md` opens preview (verbose flag accepted)
- [x] 4.7 `mip --foo` prints error with usage hint
