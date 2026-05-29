use std::io::Write;
use mip::config::Config;

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
    let cfg = Config::load_from(std::path::Path::new("/tmp/nonexistent-miprs-config-xyz.toml"));
    assert_eq!(cfg.theme(), "system");
    assert!(!cfg.frontmatter());
}

#[test]
fn test_load_from_invalid_theme_falls_back() {
    let file = write_temp_config("theme = \"neon\"\nfrontmatter = true\n");
    let cfg = Config::load_from(file.path());
    // Invalid theme falls back to default ("system")
    assert_eq!(cfg.theme(), "system");
    // But frontmatter is preserved
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
fn test_load_from_toc_side() {
    let file = write_temp_config("toc = \"side\"\n");
    let cfg = Config::load_from(file.path());
    assert_eq!(cfg.toc(), "side");
}

#[test]
fn test_load_from_toc_zathura() {
    let file = write_temp_config("toc = \"zathura\"\n");
    let cfg = Config::load_from(file.path());
    assert_eq!(cfg.toc(), "zathura");
}

#[test]
fn test_load_from_toc_off() {
    let file = write_temp_config("toc = \"off\"\n");
    let cfg = Config::load_from(file.path());
    assert_eq!(cfg.toc(), "off");
}

#[test]
fn test_load_from_toc_invalid_falls_back() {
    let file = write_temp_config("toc = \"floating\"\n");
    let cfg = Config::load_from(file.path());
    assert_eq!(cfg.toc(), "off");
}

#[test]
fn test_load_from_toc_missing_defaults_to_off() {
    let file = write_temp_config("theme = \"dark\"\n");
    let cfg = Config::load_from(file.path());
    assert_eq!(cfg.toc(), "off");
}
