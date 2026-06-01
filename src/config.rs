use serde::Deserialize;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Deserialize, Default)]
#[serde(default)]
pub struct Config {
    pub theme: Option<String>,
    pub frontmatter: Option<bool>,
    pub runcmd: Option<String>,
    pub sidetoc_width: Option<u32>,
    pub sidetoc_position: Option<String>,
    pub keybindings: Option<std::collections::HashMap<String, String>>,
    pub paragraph_numbers: Option<bool>,
    pub paragraph_numbers_start: Option<u8>,
    pub history_size: Option<u32>,
    pub math: Option<bool>,
    pub style: Option<String>,
}

impl Config {
    /// Load config from the default XDG path.
    pub fn load() -> Config {
        let path = config_path();
        Self::load_from(&path)
    }

    /// Load config from a TOML string. Returns defaults if malformed.
    pub fn load_from_str(content: &str) -> Config {
        match toml::from_str::<Config>(content) {
            Ok(config) => config,
            Err(_) => Config::default(),
        }
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
                            ..config
                        };
                    }
                }
                if let Some(ref pos) = config.sidetoc_position {
                    if !["left", "right"].contains(&pos.as_str()) {
                        eprintln!("warning: invalid sidetoc_position '{}' in config, using default", pos);
                        return Config {
                            sidetoc_position: None,
                            ..config
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

    pub fn runcmd(&self) -> Option<&str> {
        self.runcmd.as_deref()
    }

    pub fn sidetoc_width(&self) -> u32 {
        self.sidetoc_width.unwrap_or(250)
    }

    pub fn sidetoc_position(&self) -> &str {
        self.sidetoc_position.as_deref().unwrap_or("left")
    }

    pub fn paragraph_numbers(&self) -> bool {
        self.paragraph_numbers.unwrap_or(false)
    }

    pub fn paragraph_numbers_start(&self) -> u8 {
        self.paragraph_numbers_start.unwrap_or(1).clamp(1, 6)
    }

    pub fn history_size(&self) -> usize {
        self.history_size.unwrap_or(50) as usize
    }

    pub fn math(&self) -> bool {
        self.math.unwrap_or(true)
    }

    pub fn style(&self) -> Option<&str> {
        self.style.as_deref()
    }
}

/// Returns the styles directory path: `~/.config/miprs/styles/`
pub fn styles_dir() -> PathBuf {
    let config_dir = std::env::var("XDG_CONFIG_HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|_| {
            let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
            PathBuf::from(home).join(".config")
        });
    config_dir.join("miprs").join("styles")
}

/// Returns the path to a style's CSS file: `~/.config/miprs/styles/<name>/style.css`
pub fn style_css_path(name: &str) -> PathBuf {
    styles_dir().join(name).join("style.css")
}

/// Returns a documented default CSS template for `--initstyle`.
pub fn default_style_css() -> &'static str {
    r#"/* Custom style for mip
 * This file is loaded after the default styles, so you can override
 * any CSS variable or rule. The default theme variables are:
 *
 * :root (light mode):
 *   --fg: #333           (text color)
 *   --heading: #111      (heading color)
 *   --bg: #fff           (background)
 *   --link: #0969da      (link color)
 *   --h1-border: #e0e0e0
 *   --table-border: #efefef
 *   --table-odd-bg: #f3f3f3
 *   --blockquote-border: #ccc
 *   --blockquote-bg: #eee
 *   --code-bg: #f6f8fa
 *   --strong-border: #aaa
 *   --frontmatter-border: #ddd
 *   --frontmatter-th-bg: #f6f8fa
 *
 * .dark (dark mode):
 *   --fg: #d4d4d4
 *   --heading: #e0e0e0
 *   --bg: #1a1a2e
 *   --link: #58a6ff
 *   --h1-border: #333
 *   --table-border: #333
 *   --table-odd-bg: #222
 *   --blockquote-border: #444
 *   --blockquote-bg: #252525
 *   --code-bg: #2d2d3f
 *   --strong-border: #666
 *   --frontmatter-border: #444
 *   --frontmatter-th-bg: #2d2d3f
 *
 * Example: Override background and text for light mode:
 *   :root { --bg: #fdf6e3; --fg: #586e75; }
 *
 * Example: Override dark mode:
 *   .dark { --bg: #002b36; --fg: #839496; }
 *
 * Example: Change heading font:
 *   h1, h2, h3, h4, h5, h6 { font-family: Georgia, serif; }
 *
 * Example: Wider content area:
 *   .section { max-width: 900px; margin: 0 auto; }
 */
"#
}

/// Returns the documented default config template.
pub fn default_config_template() -> &'static str {
    r#"# mip configuration
# Generated by: mip --initconf

# Color theme: "system", "light", or "dark"
# "system" follows your desktop's dark/light preference
theme = "system"

# Show YAML frontmatter as a table above the content
frontmatter = false

# Command(s) to run at startup (semicolon-separated)
# Examples:
#   runcmd = "sidetoc_open"
#   runcmd = "sidetoc_open; set theme dark"
# runcmd = ""

# Side table of contents panel width in pixels
sidetoc_width = 250

# Side table of contents position: "left" or "right"
sidetoc_position = "left"

# Show hierarchical section numbers on headings (1., 1.1, 1.1.1)
paragraph_numbers = false

# Heading level to start numbering from (1 = H1, 2 = H2, etc.)
# Useful when H1 is a document title and you want numbering from H2
paragraph_numbers_start = 1

# Enable TeX math rendering via KaTeX ($..$ inline, $$...$$ display)
math = true

# Custom style name — loads CSS from ~/.config/miprs/styles/<name>/style.css
# Create a new style with: mip --initstyle <name>
# style = "academic"

# Maximum number of command bar history entries to keep
history_size = 50

# Keybindings: key_combo = "command"
# Key combos: ctrl+key, shift+key, alt+key, super+key, or plain key
# Key sequences: "key1,key2" (e.g. "g,g" = press g then g within 500ms)
# Commands can be composed with semicolons: "cmd1; cmd2"
#
# Available commands:
#   q / close              - quit mip
#   open <path> / o <path> - open a markdown file
#   print                  - open print dialog (Ctrl+P)
#   quicktoc               - toggle fullscreen TOC overlay
#   sidetoc_open           - show side TOC panel
#   sidetoc_close          - hide side TOC panel
#   sidetoc_toggle         - toggle side TOC panel
#   sidetoc_expand_width   - widen side TOC panel
#   sidetoc_shrink_width   - narrow side TOC panel
#   sidetoc_focus          - focus the side TOC panel
#   document_focus         - focus the document view
#   zoom_in                - zoom in 10% (Ctrl+=)
#   zoom_out               - zoom out 10% (Ctrl+-)
#   zoom_reset             - reset zoom to 100% (Ctrl+0)
#   scroll_down            - scroll down one step (j)
#   scroll_up              - scroll up one step (k)
#   scroll_page_down       - scroll down one page (Ctrl+F)
#   scroll_page_up         - scroll up one page (Ctrl+B)
#   scroll_half_down       - scroll down half page (Ctrl+D)
#   scroll_half_up         - scroll up half page (Ctrl+U)
#   scroll_top             - scroll to top (Home, gg)
#   scroll_bottom          - scroll to bottom (End, G)
#   scroll_next_heading    - jump to next heading (n)
#   scroll_prev_heading    - jump to previous heading (N)
#   export_html <path>     - export current document as standalone HTML file
[keybindings]
tab = "quicktoc"
"ctrl+p" = "print"
"ctrl+=" = "zoom_in"
"ctrl+-" = "zoom_out"
"ctrl+0" = "zoom_reset"
j = "scroll_down"
k = "scroll_up"
down = "scroll_down"
up = "scroll_up"
"ctrl+f" = "scroll_page_down"
"ctrl+b" = "scroll_page_up"
pagedown = "scroll_page_down"
pageup = "scroll_page_up"
"ctrl+d" = "scroll_half_down"
"ctrl+u" = "scroll_half_up"
home = "scroll_top"
end = "scroll_bottom"
"shift+g" = "scroll_bottom"
"g,g" = "scroll_top"
n = "scroll_next_heading"
"shift+n" = "scroll_prev_heading"
# "ctrl+b" = "sidetoc_toggle"
# "ctrl+y" = "open ~/todo.md"
"#
}

pub fn config_path() -> PathBuf {
    let config_dir = std::env::var("XDG_CONFIG_HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|_| {
            let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
            PathBuf::from(home).join(".config")
        });
    config_dir.join("miprs").join("config.toml")
}
