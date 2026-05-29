use serde::Deserialize;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Deserialize, Default)]
#[serde(default)]
pub struct Config {
    pub theme: Option<String>,
    pub frontmatter: Option<bool>,
}

impl Config {
    /// Load config from the default XDG path.
    pub fn load() -> Config {
        let path = config_path();
        Self::load_from(&path)
    }

    /// Load config from an explicit path. Returns defaults if the file
    /// is missing or malformed.
    pub fn load_from(path: &Path) -> Config {
        let content = match fs::read_to_string(path) {
            Ok(c) => c,
            Err(_) => return Config::default(),
        };

        match toml::from_str::<Config>(&content) {
            Ok(config) => {
                if let Some(ref theme) = config.theme {
                    if !["system", "light", "dark"].contains(&theme.as_str()) {
                        eprintln!("warning: invalid theme '{}' in config, using default", theme);
                        return Config {
                            theme: None,
                            frontmatter: config.frontmatter,
                        };
                    }
                }
                config
            }
            Err(e) => {
                eprintln!("warning: invalid config file: {}", e);
                Config::default()
            }
        }
    }

    pub fn theme(&self) -> &str {
        self.theme.as_deref().unwrap_or("system")
    }

    pub fn frontmatter(&self) -> bool {
        self.frontmatter.unwrap_or(false)
    }
}

fn config_path() -> PathBuf {
    let config_dir = std::env::var("XDG_CONFIG_HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|_| {
            let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
            PathBuf::from(home).join(".config")
        });
    config_dir.join("miprs").join("config.toml")
}
