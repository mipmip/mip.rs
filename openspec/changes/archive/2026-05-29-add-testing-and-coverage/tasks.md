## 1. Refactor for testability

- [x] 1.1 Extract `build_html()` pure function from `to_file()` in `markdown.rs` — takes markdown string, template, seed URL, show_frontmatter, theme_class; returns complete HTML string
- [x] 1.2 Extract `routes()` function from `run_bro()` in `server.rs` — returns the warp Filter chain without starting the server
- [x] 1.3 Add `Config::load_from(path)` to `config.rs` — loads config from an explicit path, used by tests; existing `Config::load()` delegates to it
- [x] 1.4 Make `strip_seed_scripts` and `port_is_available` accessible for testing (pub or `pub(crate)`)

## 2. Unit tests

- [x] 2.1 Add `#[cfg(test)]` unit tests for `strip_seed_scripts()` in `view.rs` — covers script removal and content preservation
- [x] 2.2 Add `#[cfg(test)]` unit tests for `pod_to_html_value()` in `markdown.rs` — covers all Pod variants
- [x] 2.3 Add `#[cfg(test)]` unit tests for `port_is_available()` in `main.rs` — covers available and occupied ports

## 3. Integration tests

- [x] 3.1 Create `tests/markdown_test.rs` — test `md_to_html_body()` with plain markdown, frontmatter on/off, GFM extensions
- [x] 3.2 Create `tests/markdown_test.rs` — test `build_html()` verifying template placeholder replacement and theme class
- [x] 3.3 Create `tests/config_test.rs` — test `Config::load_from()` with valid config, missing file, invalid theme, malformed TOML
- [x] 3.4 Create `tests/server_test.rs` — test `routes()` with `warp::test` for .temp.html, .temp.seed, and static asset serving

## 4. Coverage tooling

- [x] 4.1 Add a coverage line (`Coverage: 0%`) to the top of README.md
- [x] 4.2 Create a shell script (`scripts/update-coverage.sh`) that runs `cargo tarpaulin`, extracts the percentage, and patches README.md if changed

## 5. Verify

- [x] 5.1 Run full test suite and confirm all tests pass
- [x] 5.2 Run coverage script and verify README is updated with actual percentage
