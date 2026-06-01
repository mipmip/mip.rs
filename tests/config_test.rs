use mip::config::Config;
use std::io::Write;

fn write_temp_config(content: &str) -> tempfile::NamedTempFile {
    let mut file = tempfile::NamedTempFile::new().unwrap();
    file.write_all(content.as_bytes()).unwrap();
    file.flush().unwrap();
    file
}

#[test]
fn test_load_from_valid_config_all_fields() {
    let file = write_temp_config("theme = \"dark\"\nfrontmatter = true\n");
    let cfg = Config::load_from(file.path());
    assert_eq!(cfg.theme(), "dark");
    assert!(cfg.frontmatter());
}

#[test]
fn test_load_from_valid_config_light_theme() {
    let file = write_temp_config("theme = \"light\"\n");
    let cfg = Config::load_from(file.path());
    assert_eq!(cfg.theme(), "light");
    assert!(!cfg.frontmatter());
}

#[test]
fn test_load_from_missing_file() {
    let cfg = Config::load_from(std::path::Path::new(
        "/tmp/nonexistent-miprs-config-xyz.toml",
    ));
    assert_eq!(cfg.theme(), "system");
    assert!(!cfg.frontmatter());
}

#[test]
fn test_load_from_invalid_theme_falls_back() {
    let file = write_temp_config("theme = \"neon\"\nfrontmatter = true\n");
    let cfg = Config::load_from(file.path());
    assert_eq!(cfg.theme(), "system");
    assert!(cfg.frontmatter());
}

#[test]
fn test_load_from_malformed_toml() {
    let file = write_temp_config("this is not [valid toml {{{{");
    let cfg = Config::load_from(file.path());
    assert_eq!(cfg.theme(), "system");
    assert!(!cfg.frontmatter());
}

#[test]
fn test_load_from_empty_file() {
    let file = write_temp_config("");
    let cfg = Config::load_from(file.path());
    assert_eq!(cfg.theme(), "system");
    assert!(!cfg.frontmatter());
}

#[test]
fn test_load_from_system_theme() {
    let file = write_temp_config("theme = \"system\"\n");
    let cfg = Config::load_from(file.path());
    assert_eq!(cfg.theme(), "system");
}

#[test]
fn test_load_from_runcmd() {
    let file = write_temp_config("runcmd = \"sidetoc_open\"\n");
    let cfg = Config::load_from(file.path());
    assert_eq!(cfg.runcmd(), Some("sidetoc_open"));
}

#[test]
fn test_load_from_runcmd_missing() {
    let file = write_temp_config("theme = \"dark\"\n");
    let cfg = Config::load_from(file.path());
    assert_eq!(cfg.runcmd(), None);
}

#[test]
fn test_load_from_sidetoc_width() {
    let file = write_temp_config("sidetoc_width = 300\n");
    let cfg = Config::load_from(file.path());
    assert_eq!(cfg.sidetoc_width(), 300);
}

#[test]
fn test_load_from_sidetoc_width_default() {
    let file = write_temp_config("");
    let cfg = Config::load_from(file.path());
    assert_eq!(cfg.sidetoc_width(), 250);
}

#[test]
fn test_load_from_sidetoc_position_right() {
    let file = write_temp_config("sidetoc_position = \"right\"\n");
    let cfg = Config::load_from(file.path());
    assert_eq!(cfg.sidetoc_position(), "right");
}

#[test]
fn test_load_from_sidetoc_position_invalid() {
    let file = write_temp_config("sidetoc_position = \"top\"\n");
    let cfg = Config::load_from(file.path());
    assert_eq!(cfg.sidetoc_position(), "left");
}

#[test]
fn test_load_from_sidetoc_position_default() {
    let file = write_temp_config("");
    let cfg = Config::load_from(file.path());
    assert_eq!(cfg.sidetoc_position(), "left");
}

// initconf template tests

#[test]
fn test_default_config_template_contains_all_settings() {
    let template = mip::config::default_config_template();
    // All Config struct field names should appear in the template
    assert!(template.contains("theme"));
    assert!(template.contains("frontmatter"));
    assert!(template.contains("runcmd"));
    assert!(template.contains("sidetoc_width"));
    assert!(template.contains("sidetoc_position"));
    assert!(template.contains("[keybindings]"));
}

#[test]
fn test_default_config_template_is_valid_toml() {
    let template = mip::config::default_config_template();
    let result: Result<toml::Value, _> = toml::from_str(template);
    assert!(
        result.is_ok(),
        "Template is not valid TOML: {:?}",
        result.err()
    );
}

#[test]
fn test_default_config_template_parses_to_config() {
    let template = mip::config::default_config_template();
    let cfg = Config::load_from_str(template);
    assert_eq!(cfg.theme(), "system");
    assert!(!cfg.frontmatter());
    assert_eq!(cfg.sidetoc_width(), 250);
    assert_eq!(cfg.sidetoc_position(), "left");
}

#[test]
fn test_load_from_math_true() {
    let file = write_temp_config("math = true\n");
    let cfg = Config::load_from(file.path());
    assert!(cfg.math());
}

#[test]
fn test_load_from_math_false() {
    let file = write_temp_config("math = false\n");
    let cfg = Config::load_from(file.path());
    assert!(!cfg.math());
}

#[test]
fn test_load_from_math_missing_defaults_to_true() {
    let file = write_temp_config("theme = \"dark\"\n");
    let cfg = Config::load_from(file.path());
    assert!(cfg.math());
}

#[test]
fn test_load_from_mermaid_true() {
    let file = write_temp_config("mermaid = true\n");
    let cfg = Config::load_from(file.path());
    assert!(cfg.mermaid());
}

#[test]
fn test_load_from_mermaid_false() {
    let file = write_temp_config("mermaid = false\n");
    let cfg = Config::load_from(file.path());
    assert!(!cfg.mermaid());
}

#[test]
fn test_load_from_mermaid_missing_defaults_to_true() {
    let file = write_temp_config("theme = \"dark\"\n");
    let cfg = Config::load_from(file.path());
    assert!(cfg.mermaid());
}
