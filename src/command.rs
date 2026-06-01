//! Pure command-mode logic: parsing, matching, tilde expansion, wildmenu markup.
//! All functions here are free of GTK dependencies for testability.

const COMMANDS: &[&str] = &[
    "close",
    "document_focus",
    "export_html",
    "o",
    "open",
    "print",
    "q",
    "quicktoc",
    "scroll_bottom",
    "scroll_down",
    "scroll_half_down",
    "scroll_half_up",
    "scroll_next_heading",
    "scroll_page_down",
    "scroll_page_up",
    "scroll_prev_heading",
    "scroll_top",
    "scroll_up",
    "set",
    "sidetoc_close",
    "sidetoc_expand_width",
    "sidetoc_focus",
    "sidetoc_open",
    "sidetoc_shrink_width",
    "sidetoc_toggle",
    "zoom_in",
    "zoom_out",
    "zoom_reset",
];

const SETTINGS: &[&str] = &[
    "frontmatter",
    "paragraph_numbers",
    "paragraph_numbers_start",
    "style",
    "theme",
];

/// Match a prefix against the known settings list. Returns sorted matches.
pub fn match_settings(prefix: &str) -> Vec<String> {
    SETTINGS
        .iter()
        .filter(|s| s.starts_with(prefix))
        .map(|s| s.to_string())
        .collect()
}

pub fn expand_tilde(path: &str) -> String {
    if let Some(stripped) = path.strip_prefix('~') {
        let home = std::env::var("HOME").unwrap_or_default();
        format!("{}{}", home, stripped)
    } else {
        path.to_string()
    }
}

/// Strip leading `:` and split into (command, argument).
pub fn parse_command(text: &str) -> (&str, &str) {
    let text = text.strip_prefix(':').unwrap_or(text);
    let mut parts = text.splitn(2, char::is_whitespace);
    let cmd = parts.next().unwrap_or("").trim();
    let arg = parts.next().unwrap_or("").trim();
    (cmd, arg)
}

/// Match a prefix against the known command list. Returns sorted matches.
pub fn match_commands(prefix: &str) -> Vec<String> {
    let mut matches: Vec<String> = COMMANDS
        .iter()
        .filter(|cmd| cmd.starts_with(prefix) && **cmd != prefix)
        .map(|cmd| cmd.to_string())
        .collect();
    // Also include exact match if it exists
    if COMMANDS.contains(&prefix) {
        matches.insert(0, prefix.to_string());
    }
    matches.sort();
    matches.dedup();
    matches
}

const MARKDOWN_EXTENSIONS: &[&str] = &[".md", ".markdown", ".mkd", ".qmd"];

fn is_markdown_file(name: &str) -> bool {
    let lower = name.to_lowercase();
    MARKDOWN_EXTENSIONS.iter().any(|ext| lower.ends_with(ext))
}

/// List filesystem entries matching a path fragment. Returns sorted full paths.
/// Directories get a trailing `/`. Only markdown files are included.
pub fn match_paths(path_fragment: &str) -> Vec<String> {
    let expanded = expand_tilde(path_fragment);

    let (dir, file_prefix) = if expanded.ends_with('/') || expanded.is_empty() {
        (expanded.as_str().to_string(), "".to_string())
    } else {
        let p = std::path::Path::new(&expanded);
        let dir = p
            .parent()
            .map_or(".", |d| {
                if d.as_os_str().is_empty() {
                    "."
                } else {
                    d.to_str().unwrap_or(".")
                }
            })
            .to_string();
        let file = p
            .file_name()
            .map_or("", |f| f.to_str().unwrap_or(""))
            .to_string();
        (dir, file)
    };

    let mut found = Vec::new();
    if let Ok(entries) = std::fs::read_dir(&dir) {
        for entry_result in entries.flatten() {
            let name = entry_result.file_name().to_string_lossy().to_string();
            let is_dir = entry_result.path().is_dir();
            if name.starts_with(&file_prefix) && (is_dir || is_markdown_file(&name)) {
                let full = if dir == "." {
                    name.clone()
                } else {
                    format!("{}/{}", dir.trim_end_matches('/'), name)
                };
                let full = if is_dir { format!("{}/", full) } else { full };
                found.push(full);
            }
        }
    }
    found.sort();
    found
}

/// Convert a path back to use `~` if the original used tilde.
pub fn unexpand_tilde(path: &str, original_used_tilde: bool) -> String {
    if original_used_tilde {
        let home = std::env::var("HOME").unwrap_or_default();
        if path.starts_with(&home) {
            path.replacen(&home, "~", 1)
        } else {
            path.to_string()
        }
    } else {
        path.to_string()
    }
}

/// Extract just the filename from a full path for display in the wildmenu.
pub fn display_name(path: &str) -> &str {
    // For paths ending in /, show the dir name with /
    if let Some(trimmed) = path.strip_suffix('/') {
        let name_start = trimmed.rfind('/').map_or(0, |i| i + 1);
        &path[name_start..]
    } else {
        path.rsplit('/').next().unwrap_or(path)
    }
}

/// Generate Pango markup for the wildmenu, bolding the match at `current_index`.
/// Uses a sliding window of `max_visible` items centered around the current selection.
pub fn wildmenu_markup(matches: &[String], current_index: usize, max_visible: usize) -> String {
    if matches.is_empty() {
        return String::new();
    }

    let total = matches.len();

    // Compute sliding window: keep current_index visible, centered when possible
    let (window_start, window_end) = if total <= max_visible {
        (0, total)
    } else {
        let half = max_visible / 2;
        let start = if current_index <= half {
            0
        } else if current_index + half >= total {
            total - max_visible
        } else {
            current_index - half
        };
        (start, (start + max_visible).min(total))
    };

    let parts: Vec<String> = matches[window_start..window_end]
        .iter()
        .enumerate()
        .map(|(i, m)| {
            let name = display_name(m);
            let escaped = name
                .replace('&', "&amp;")
                .replace('<', "&lt;")
                .replace('>', "&gt;");
            if i + window_start == current_index {
                format!("<b>{}</b>", escaped)
            } else {
                escaped
            }
        })
        .collect();

    let mut result = String::new();
    if window_start > 0 {
        result.push_str(&format!("(+{})   ", window_start));
    }
    result.push_str(&parts.join("   "));
    if window_end < total {
        result.push_str(&format!("   (+{} more)", total - window_end));
    }
    result
}

// --- Keybinding support ---

/// A key combination: a key name + modifier flags.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct KeyCombo {
    pub key: String, // lowercase key name, e.g. "p", "tab", "f1"
    pub ctrl: bool,
    pub shift: bool,
    pub alt: bool,
    pub super_: bool,
}

/// Parse a key combo string like "ctrl+p", "tab", "ctrl+shift+t" into a KeyCombo.
/// Returns None if the key name is not recognized.
pub fn parse_key_combo(s: &str) -> Option<KeyCombo> {
    let parts: Vec<&str> = s.split('+').map(|p| p.trim()).collect();
    if parts.is_empty() {
        return None;
    }

    let key_name = normalize_key_name(&parts.last().unwrap().to_lowercase());
    if !is_known_key(&key_name) {
        return None;
    }

    let mut combo = KeyCombo {
        key: key_name,
        ctrl: false,
        shift: false,
        alt: false,
        super_: false,
    };

    for &part in &parts[..parts.len() - 1] {
        match part.to_lowercase().as_str() {
            "ctrl" | "control" => combo.ctrl = true,
            "shift" => combo.shift = true,
            "alt" => combo.alt = true,
            "super" => combo.super_ = true,
            _ => return None, // unknown modifier
        }
    }

    Some(combo)
}

/// Normalize symbol characters to their key names.
fn normalize_key_name(name: &str) -> String {
    match name {
        "-" => "minus".to_string(),
        "+" => "plus".to_string(),
        "=" => "equal".to_string(),
        "[" => "bracketleft".to_string(),
        "]" => "bracketright".to_string(),
        ";" => "semicolon".to_string(),
        "'" => "apostrophe".to_string(),
        "," => "comma".to_string(),
        "." => "period".to_string(),
        "/" => "slash".to_string(),
        "\\" => "backslash".to_string(),
        "`" => "grave".to_string(),
        _ => name.to_string(),
    }
}

fn is_known_key(name: &str) -> bool {
    matches!(
        name,
        "a" | "b"
            | "c"
            | "d"
            | "e"
            | "f"
            | "g"
            | "h"
            | "i"
            | "j"
            | "k"
            | "l"
            | "m"
            | "n"
            | "o"
            | "p"
            | "q"
            | "r"
            | "s"
            | "t"
            | "u"
            | "v"
            | "w"
            | "x"
            | "y"
            | "z"
            | "0"
            | "1"
            | "2"
            | "3"
            | "4"
            | "5"
            | "6"
            | "7"
            | "8"
            | "9"
            | "tab"
            | "escape"
            | "return"
            | "enter"
            | "space"
            | "backspace"
            | "delete"
            | "up"
            | "down"
            | "left"
            | "right"
            | "home"
            | "end"
            | "pageup"
            | "pagedown"
            | "f1"
            | "f2"
            | "f3"
            | "f4"
            | "f5"
            | "f6"
            | "f7"
            | "f8"
            | "f9"
            | "f10"
            | "f11"
            | "f12"
            | "minus"
            | "plus"
            | "equal"
            | "bracketleft"
            | "bracketright"
            | "semicolon"
            | "apostrophe"
            | "comma"
            | "period"
            | "slash"
            | "backslash"
            | "grave"
    )
}

/// Convert a GDK keyval to the key name string used in KeyCombo.
pub fn keyval_to_name(keyval: u32) -> Option<String> {
    // Map GDK key constants to our key names
    // We use the numeric values to avoid depending on gtk4::gdk here
    let name = match keyval {
        0xff09 => "tab",       // GDK_KEY_Tab
        0xfe20 => "tab",       // GDK_KEY_ISO_Left_Tab (shift+tab)
        0xff1b => "escape",    // GDK_KEY_Escape
        0xff0d => "return",    // GDK_KEY_Return
        0x020 => "space",      // GDK_KEY_space
        0xff08 => "backspace", // GDK_KEY_BackSpace
        0xffff => "delete",    // GDK_KEY_Delete
        0xff52 => "up",        // GDK_KEY_Up
        0xff54 => "down",      // GDK_KEY_Down
        0xff51 => "left",      // GDK_KEY_Left
        0xff53 => "right",     // GDK_KEY_Right
        0xff50 => "home",      // GDK_KEY_Home
        0xff57 => "end",       // GDK_KEY_End
        0xff55 => "pageup",    // GDK_KEY_Page_Up
        0xff56 => "pagedown",  // GDK_KEY_Page_Down
        0xffbe..=0xffc9 => {
            // GDK_KEY_F1..F12
            let n = keyval - 0xffbe + 1;
            return Some(format!("f{}", n));
        }
        0x061..=0x07a => {
            // GDK_KEY_a..z
            return Some(String::from(char::from(keyval as u8)));
        }
        0x041..=0x05a => {
            // GDK_KEY_A..Z (uppercase, map to lowercase)
            return Some(String::from(char::from((keyval as u8) + 32)));
        }
        0x001..=0x01a => {
            // Control characters (Ctrl+A=0x001 .. Ctrl+Z=0x01a)
            return Some(String::from(char::from(keyval as u8 + 0x060))); // map to a..z
        }
        0x030..=0x039 => {
            // GDK_KEY_0..9
            return Some(String::from(char::from(keyval as u8)));
        }
        0x02d => "minus",
        0x02b => "plus",
        0x03d => "equal",
        0x05b => "bracketleft",
        0x05d => "bracketright",
        0x03b => "semicolon",
        0x027 => "apostrophe",
        0x02c => "comma",
        0x02e => "period",
        0x02f => "slash",
        0x05c => "backslash",
        0x060 => "grave",
        _ => return None,
    };
    Some(name.to_string())
}

/// Result of looking up a key in the registry.
#[derive(Debug, Clone, PartialEq)]
pub enum LookupResult<'a> {
    /// A single-combo binding: execute immediately.
    Command(&'a str),
    /// The key is the first key of one or more sequences: enter pending state.
    SequencePrefix,
    /// No binding found.
    None,
}

/// What a key maps to internally.
#[derive(Debug, Clone)]
enum BindingAction {
    Command(String),
    SequencePrefix(std::collections::HashMap<KeyCombo, String>),
}

/// A keybinding registry mapping key combos to command strings,
/// with support for two-key sequences like "g,g".
pub struct KeybindingRegistry {
    bindings: std::collections::HashMap<KeyCombo, BindingAction>,
}

/// Parse a binding string that may be a single combo ("ctrl+p") or a
/// comma-separated sequence ("g,g", "ctrl+g,g"). Returns None if any
/// key in the string is invalid.
pub fn parse_binding_str(s: &str) -> Option<Vec<KeyCombo>> {
    let parts: Vec<&str> = s.split(',').collect();
    let mut combos = Vec::with_capacity(parts.len());
    for part in parts {
        combos.push(parse_key_combo(part.trim())?);
    }
    if combos.is_empty() {
        None
    } else {
        Some(combos)
    }
}

impl Default for KeybindingRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl KeybindingRegistry {
    pub fn new() -> Self {
        Self {
            bindings: std::collections::HashMap::new(),
        }
    }

    /// Create a registry with default keybindings.
    pub fn with_defaults() -> Self {
        let mut registry = Self::new();
        registry.register_str("tab", "quicktoc");
        registry.register_str("ctrl+p", "print");
        registry.register_str("ctrl+=", "zoom_in");
        registry.register_str("ctrl+-", "zoom_out");
        registry.register_str("ctrl+0", "zoom_reset");
        // Vim-style scroll navigation
        registry.register_str("j", "scroll_down");
        registry.register_str("k", "scroll_up");
        registry.register_str("down", "scroll_down");
        registry.register_str("up", "scroll_up");
        registry.register_str("ctrl+f", "scroll_page_down");
        registry.register_str("ctrl+b", "scroll_page_up");
        registry.register_str("pagedown", "scroll_page_down");
        registry.register_str("pageup", "scroll_page_up");
        registry.register_str("ctrl+d", "scroll_half_down");
        registry.register_str("ctrl+u", "scroll_half_up");
        registry.register_str("home", "scroll_top");
        registry.register_str("end", "scroll_bottom");
        registry.register_str("shift+g", "scroll_bottom");
        registry.register_str("g,g", "scroll_top");
        registry.register_str("n", "scroll_next_heading");
        registry.register_str("shift+n", "scroll_prev_heading");
        registry
    }

    /// Register a keybinding from string key combo (single or sequence).
    pub fn register_str(&mut self, combo_str: &str, command: &str) {
        let Some(combos) = parse_binding_str(combo_str) else {
            eprintln!("warning: invalid keybinding '{}', skipping", combo_str);
            return;
        };

        if combos.len() == 1 {
            // Single combo binding
            let combo = combos.into_iter().next().unwrap();
            // If there's already a SequencePrefix here, the single binding
            // becomes unreachable — warn but still store as sequence prefix
            // takes priority at lookup time. Actually per spec: sequence prefix
            // takes priority, so don't overwrite a SequencePrefix with a Command.
            match self.bindings.get(&combo) {
                Some(BindingAction::SequencePrefix(_)) => {
                    eprintln!(
                        "warning: keybinding '{}' conflicts with a sequence prefix, single binding will be unreachable",
                        combo_str
                    );
                }
                _ => {
                    self.bindings
                        .insert(combo, BindingAction::Command(command.to_string()));
                }
            }
        } else if combos.len() == 2 {
            // Two-key sequence
            let first = combos[0].clone();
            let second = combos[1].clone();

            // Replace any existing single Command on the first key with a SequencePrefix
            let action = self
                .bindings
                .entry(first)
                .or_insert_with(|| BindingAction::SequencePrefix(std::collections::HashMap::new()));

            match action {
                BindingAction::Command(_) => {
                    // Upgrade to sequence prefix, old single binding becomes unreachable
                    let mut map = std::collections::HashMap::new();
                    map.insert(second, command.to_string());
                    *action = BindingAction::SequencePrefix(map);
                }
                BindingAction::SequencePrefix(map) => {
                    map.insert(second, command.to_string());
                }
            }
        } else {
            eprintln!(
                "warning: sequences longer than 2 keys not supported: '{}'",
                combo_str
            );
        }
    }

    /// Look up what a key combo maps to.
    pub fn lookup(
        &self,
        keyval: u32,
        ctrl: bool,
        shift: bool,
        alt: bool,
        super_: bool,
    ) -> LookupResult<'_> {
        let Some(key) = keyval_to_name(keyval) else {
            return LookupResult::None;
        };
        let combo = KeyCombo {
            key,
            ctrl,
            shift,
            alt,
            super_,
        };
        match self.bindings.get(&combo) {
            Some(BindingAction::Command(cmd)) => LookupResult::Command(cmd),
            Some(BindingAction::SequencePrefix(_)) => LookupResult::SequencePrefix,
            None => LookupResult::None,
        }
    }

    /// Look up the second key of a sequence. Returns the command if the
    /// (first, second) pair is a registered sequence.
    #[allow(clippy::too_many_arguments)]
    pub fn lookup_sequence(
        &self,
        first_keyval: u32,
        first_ctrl: bool,
        first_shift: bool,
        first_alt: bool,
        first_super: bool,
        second_keyval: u32,
        second_ctrl: bool,
        second_shift: bool,
        second_alt: bool,
        second_super: bool,
    ) -> Option<&str> {
        let first_key = keyval_to_name(first_keyval)?;
        let first = KeyCombo {
            key: first_key,
            ctrl: first_ctrl,
            shift: first_shift,
            alt: first_alt,
            super_: first_super,
        };
        let second_key = keyval_to_name(second_keyval)?;
        let second = KeyCombo {
            key: second_key,
            ctrl: second_ctrl,
            shift: second_shift,
            alt: second_alt,
            super_: second_super,
        };

        if let Some(BindingAction::SequencePrefix(map)) = self.bindings.get(&first) {
            map.get(&second).map(|s| s.as_str())
        } else {
            None
        }
    }

    /// Register all keybindings from a config HashMap, overriding defaults.
    pub fn register_from_config(
        &mut self,
        config_bindings: &std::collections::HashMap<String, String>,
    ) {
        for (combo_str, command) in config_bindings {
            self.register_str(combo_str, command);
        }
    }
}

/// Split a command string on `;` into individual commands.
/// Each part is trimmed. Empty parts are skipped.
pub fn split_commands(text: &str) -> Vec<String> {
    text.split(';')
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    // expand_tilde tests
    #[test]
    fn test_expand_tilde_with_tilde() {
        let result = expand_tilde("~/foo");
        let home = std::env::var("HOME").unwrap();
        assert_eq!(result, format!("{}/foo", home));
    }

    #[test]
    fn test_expand_tilde_without_tilde() {
        assert_eq!(expand_tilde("/usr/bin"), "/usr/bin");
        assert_eq!(expand_tilde("relative/path"), "relative/path");
    }

    #[test]
    fn test_expand_tilde_just_tilde() {
        let result = expand_tilde("~");
        let home = std::env::var("HOME").unwrap();
        assert_eq!(result, home);
    }

    // parse_command tests
    #[test]
    fn test_parse_command_simple() {
        assert_eq!(parse_command(":q"), ("q", ""));
    }

    #[test]
    fn test_parse_command_with_arg() {
        assert_eq!(parse_command(":open foo.md"), ("open", "foo.md"));
    }

    #[test]
    fn test_parse_command_no_colon() {
        assert_eq!(parse_command("q"), ("q", ""));
    }

    #[test]
    fn test_parse_command_extra_whitespace() {
        assert_eq!(parse_command(":open   foo.md"), ("open", "foo.md"));
    }

    #[test]
    fn test_parse_command_empty() {
        assert_eq!(parse_command(":"), ("", ""));
    }

    #[test]
    fn test_parse_command_arg_with_spaces() {
        assert_eq!(
            parse_command(":open path with spaces.md"),
            ("open", "path with spaces.md")
        );
    }

    // match_commands tests
    #[test]
    fn test_match_commands_unique_prefix() {
        let matches = match_commands("op");
        assert_eq!(matches, vec!["open"]);
    }

    #[test]
    fn test_match_commands_ambiguous() {
        let mut matches = match_commands("o");
        matches.sort();
        assert!(matches.contains(&"o".to_string()));
        assert!(matches.contains(&"open".to_string()));
    }

    #[test]
    fn test_match_commands_no_match() {
        let matches = match_commands("xyz");
        assert!(matches.is_empty());
    }

    #[test]
    fn test_match_commands_exact() {
        let matches = match_commands("q");
        // "q" is exact match, but "quicktoc" also starts with "q"
        assert!(matches.contains(&"q".to_string()));
        assert!(matches.contains(&"quicktoc".to_string()));
    }

    #[test]
    fn test_match_commands_cl() {
        let matches = match_commands("cl");
        assert_eq!(matches, vec!["close"]);
    }

    // match_paths tests
    #[test]
    fn test_match_paths_existing_dir() {
        let matches = match_paths("examples/");
        assert!(!matches.is_empty());
        // Should contain markdown files
        assert!(matches.iter().any(|m| m.ends_with(".md")));
    }

    #[test]
    fn test_match_paths_partial_filename() {
        let matches = match_paths("README");
        assert!(matches.iter().any(|m| m.contains("README.md")));
    }

    #[test]
    fn test_match_paths_excludes_non_markdown() {
        let matches = match_paths("src/");
        // src/ has .rs files but no .md files, only subdirectories should appear
        assert!(
            matches
                .iter()
                .all(|m| m.ends_with('/') || is_markdown_file(m))
        );
    }

    #[test]
    fn test_match_paths_nonexistent() {
        let matches = match_paths("/nonexistent_dir_xyz/");
        assert!(matches.is_empty());
    }

    #[test]
    fn test_match_paths_directories_have_trailing_slash() {
        let matches = match_paths("src");
        // "src/" should be in matches since src is a directory
        assert!(matches.iter().any(|m| m == "src/"));
    }

    // wildmenu_markup tests
    #[test]
    fn test_wildmenu_markup_single() {
        let matches = vec!["README.md".to_string()];
        let result = wildmenu_markup(&matches, 0, 10);
        assert_eq!(result, "<b>README.md</b>");
    }

    #[test]
    fn test_wildmenu_markup_multiple() {
        let matches = vec!["README.md".to_string(), "src/".to_string()];
        let result = wildmenu_markup(&matches, 0, 10);
        assert_eq!(result, "<b>README.md</b>   src/");
    }

    #[test]
    fn test_wildmenu_markup_second_selected() {
        let matches = vec!["README.md".to_string(), "src/".to_string()];
        let result = wildmenu_markup(&matches, 1, 10);
        assert_eq!(result, "README.md   <b>src/</b>");
    }

    #[test]
    fn test_wildmenu_markup_truncation_start() {
        let matches: Vec<String> = (0..15).map(|i| format!("file{}.txt", i)).collect();
        let result = wildmenu_markup(&matches, 0, 10);
        assert!(result.contains("(+5 more)"));
        assert!(result.contains("<b>file0.txt</b>"));
        assert!(!result.starts_with("(+")); // no left indicator at start
    }

    #[test]
    fn test_wildmenu_markup_sliding_window_middle() {
        let matches: Vec<String> = (0..15).map(|i| format!("file{}.txt", i)).collect();
        let result = wildmenu_markup(&matches, 10, 10);
        // Should show (+N) on left and (+N more) on right
        assert!(result.starts_with("(+"));
        assert!(result.contains("<b>file10.txt</b>"));
    }

    #[test]
    fn test_wildmenu_markup_sliding_window_end() {
        let matches: Vec<String> = (0..15).map(|i| format!("file{}.txt", i)).collect();
        let result = wildmenu_markup(&matches, 14, 10);
        assert!(result.starts_with("(+5)"));
        assert!(result.contains("<b>file14.txt</b>"));
        assert!(!result.contains("more)")); // no right indicator at end
    }

    #[test]
    fn test_wildmenu_markup_empty() {
        let result = wildmenu_markup(&[], 0, 10);
        assert_eq!(result, "");
    }

    #[test]
    fn test_wildmenu_markup_escapes_pango() {
        let matches = vec!["file<1>.txt".to_string()];
        let result = wildmenu_markup(&matches, 0, 10);
        assert!(result.contains("&lt;"));
        assert!(result.contains("&gt;"));
    }

    // display_name tests
    #[test]
    fn test_display_name_file() {
        assert_eq!(display_name("src/main.rs"), "main.rs");
    }

    #[test]
    fn test_display_name_dir() {
        assert_eq!(display_name("src/"), "src/");
    }

    #[test]
    fn test_display_name_no_slash() {
        assert_eq!(display_name("README.md"), "README.md");
    }

    // unexpand_tilde tests
    #[test]
    fn test_unexpand_tilde_yes() {
        let home = std::env::var("HOME").unwrap();
        let path = format!("{}/docs", home);
        assert_eq!(unexpand_tilde(&path, true), "~/docs");
    }

    #[test]
    fn test_unexpand_tilde_no() {
        let home = std::env::var("HOME").unwrap();
        let path = format!("{}/docs", home);
        assert_eq!(unexpand_tilde(&path, false), path);
    }

    // split_commands tests
    #[test]
    fn test_split_commands_single() {
        assert_eq!(split_commands("sidetoc_open"), vec!["sidetoc_open"]);
    }

    #[test]
    fn test_split_commands_multiple() {
        assert_eq!(
            split_commands("sidetoc_open; set theme dark"),
            vec!["sidetoc_open", "set theme dark"]
        );
    }

    #[test]
    fn test_split_commands_whitespace() {
        assert_eq!(
            split_commands("  sidetoc_open ;  quicktoc  "),
            vec!["sidetoc_open", "quicktoc"]
        );
    }

    #[test]
    fn test_split_commands_empty_parts() {
        assert_eq!(
            split_commands("sidetoc_open;;quicktoc"),
            vec!["sidetoc_open", "quicktoc"]
        );
    }

    #[test]
    fn test_split_commands_empty() {
        let result: Vec<String> = split_commands("");
        assert!(result.is_empty());
    }

    // parse_key_combo tests
    #[test]
    fn test_parse_key_combo_simple_key() {
        let combo = parse_key_combo("tab").unwrap();
        assert_eq!(combo.key, "tab");
        assert!(!combo.ctrl && !combo.shift && !combo.alt && !combo.super_);
    }

    #[test]
    fn test_parse_key_combo_with_ctrl() {
        let combo = parse_key_combo("ctrl+p").unwrap();
        assert_eq!(combo.key, "p");
        assert!(combo.ctrl);
        assert!(!combo.shift);
    }

    #[test]
    fn test_parse_key_combo_ctrl_shift() {
        let combo = parse_key_combo("ctrl+shift+t").unwrap();
        assert_eq!(combo.key, "t");
        assert!(combo.ctrl);
        assert!(combo.shift);
    }

    #[test]
    fn test_parse_key_combo_case_insensitive() {
        let combo = parse_key_combo("Ctrl+P").unwrap();
        assert_eq!(combo.key, "p");
        assert!(combo.ctrl);
    }

    #[test]
    fn test_parse_key_combo_unknown_key() {
        assert!(parse_key_combo("xyzkey").is_none());
    }

    #[test]
    fn test_parse_key_combo_unknown_modifier() {
        assert!(parse_key_combo("hyper+p").is_none());
    }

    #[test]
    fn test_parse_key_combo_f_keys() {
        let combo = parse_key_combo("f1").unwrap();
        assert_eq!(combo.key, "f1");
    }

    // keyval_to_name tests
    #[test]
    fn test_keyval_to_name_tab() {
        assert_eq!(keyval_to_name(0xff09), Some("tab".to_string()));
    }

    #[test]
    fn test_keyval_to_name_letter() {
        assert_eq!(keyval_to_name(0x061), Some("a".to_string()));
    }

    #[test]
    fn test_keyval_to_name_uppercase() {
        assert_eq!(keyval_to_name(0x041), Some("a".to_string()));
    }

    #[test]
    fn test_keyval_to_name_f1() {
        assert_eq!(keyval_to_name(0xffbe), Some("f1".to_string()));
    }

    #[test]
    fn test_keyval_to_name_unknown() {
        assert_eq!(keyval_to_name(0xfffff), None);
    }

    // KeybindingRegistry tests
    #[test]
    fn test_registry_defaults() {
        let reg = KeybindingRegistry::with_defaults();
        // Tab -> quicktoc
        assert_eq!(
            reg.lookup(0xff09, false, false, false, false),
            LookupResult::Command("quicktoc")
        );
        // Ctrl+P -> print
        assert_eq!(
            reg.lookup(0x070, true, false, false, false),
            LookupResult::Command("print")
        );
        // j -> scroll_down
        assert_eq!(
            reg.lookup(0x06a, false, false, false, false),
            LookupResult::Command("scroll_down")
        );
        // k -> scroll_up
        assert_eq!(
            reg.lookup(0x06b, false, false, false, false),
            LookupResult::Command("scroll_up")
        );
        // g -> sequence prefix (for g,g)
        assert_eq!(
            reg.lookup(0x067, false, false, false, false),
            LookupResult::SequencePrefix
        );
        // g,g -> scroll_top
        assert_eq!(
            reg.lookup_sequence(
                0x067, false, false, false, false, 0x067, false, false, false, false
            ),
            Some("scroll_top")
        );
        // shift+g -> scroll_bottom
        assert_eq!(
            reg.lookup(0x067, false, true, false, false),
            LookupResult::Command("scroll_bottom")
        );
    }

    #[test]
    fn test_registry_lookup_miss() {
        let reg = KeybindingRegistry::with_defaults();
        assert_eq!(
            reg.lookup(0x061, false, false, false, false),
            LookupResult::None
        ); // 'a' not bound
    }

    #[test]
    fn test_registry_override() {
        let mut reg = KeybindingRegistry::with_defaults();
        reg.register_str("tab", "sidetoc_toggle");
        assert_eq!(
            reg.lookup(0xff09, false, false, false, false),
            LookupResult::Command("sidetoc_toggle")
        );
    }

    #[test]
    fn test_registry_custom_binding() {
        let mut reg = KeybindingRegistry::new();
        reg.register_str("ctrl+y", "open ~/todo.md");
        assert_eq!(
            reg.lookup(0x079, true, false, false, false),
            LookupResult::Command("open ~/todo.md")
        );
    }

    #[test]
    fn test_registry_from_config() {
        let mut reg = KeybindingRegistry::with_defaults();
        let mut config_bindings = std::collections::HashMap::new();
        config_bindings.insert("tab".to_string(), "sidetoc_toggle".to_string());
        config_bindings.insert("ctrl+b".to_string(), "quicktoc".to_string());
        reg.register_from_config(&config_bindings);
        assert_eq!(
            reg.lookup(0xff09, false, false, false, false),
            LookupResult::Command("sidetoc_toggle")
        );
        assert_eq!(
            reg.lookup(0x062, true, false, false, false),
            LookupResult::Command("quicktoc")
        );
    }

    // parse_binding_str tests
    #[test]
    fn test_parse_binding_str_single() {
        let combos = parse_binding_str("ctrl+p").unwrap();
        assert_eq!(combos.len(), 1);
        assert_eq!(combos[0].key, "p");
        assert!(combos[0].ctrl);
    }

    #[test]
    fn test_parse_binding_str_sequence() {
        let combos = parse_binding_str("g,g").unwrap();
        assert_eq!(combos.len(), 2);
        assert_eq!(combos[0].key, "g");
        assert_eq!(combos[1].key, "g");
        assert!(!combos[0].ctrl && !combos[1].ctrl);
    }

    #[test]
    fn test_parse_binding_str_sequence_with_modifier() {
        let combos = parse_binding_str("ctrl+g,g").unwrap();
        assert_eq!(combos.len(), 2);
        assert_eq!(combos[0].key, "g");
        assert!(combos[0].ctrl);
        assert_eq!(combos[1].key, "g");
        assert!(!combos[1].ctrl);
    }

    #[test]
    fn test_parse_binding_str_invalid() {
        assert!(parse_binding_str("xyzkey").is_none());
        assert!(parse_binding_str("g,xyzkey").is_none());
    }

    // Sequence registry tests
    #[test]
    fn test_registry_sequence_register_and_lookup() {
        let mut reg = KeybindingRegistry::new();
        reg.register_str("g,g", "scroll_top");
        // g is a sequence prefix
        assert_eq!(
            reg.lookup(0x067, false, false, false, false),
            LookupResult::SequencePrefix
        );
        // g,g -> scroll_top
        assert_eq!(
            reg.lookup_sequence(
                0x067, false, false, false, false, 0x067, false, false, false, false
            ),
            Some("scroll_top")
        );
        // g,k -> None (not registered)
        assert_eq!(
            reg.lookup_sequence(
                0x067, false, false, false, false, 0x06b, false, false, false, false
            ),
            None
        );
    }

    #[test]
    fn test_registry_sequence_overrides_single() {
        let mut reg = KeybindingRegistry::new();
        reg.register_str("g", "some_command");
        // Now register a sequence starting with g — it should upgrade to SequencePrefix
        reg.register_str("g,g", "scroll_top");
        // g is now a sequence prefix, not a direct command
        assert_eq!(
            reg.lookup(0x067, false, false, false, false),
            LookupResult::SequencePrefix
        );
    }

    #[test]
    fn test_registry_mixed_single_and_sequence() {
        let mut reg = KeybindingRegistry::new();
        reg.register_str("j", "scroll_down");
        reg.register_str("g,g", "scroll_top");
        // j is still a direct command
        assert_eq!(
            reg.lookup(0x06a, false, false, false, false),
            LookupResult::Command("scroll_down")
        );
        // g is a sequence prefix
        assert_eq!(
            reg.lookup(0x067, false, false, false, false),
            LookupResult::SequencePrefix
        );
    }

    #[test]
    fn test_registry_config_with_sequence() {
        let mut reg = KeybindingRegistry::new();
        let mut config = std::collections::HashMap::new();
        config.insert("g,g".to_string(), "scroll_top".to_string());
        config.insert("z,z".to_string(), "center_screen".to_string());
        reg.register_from_config(&config);
        assert_eq!(
            reg.lookup(0x067, false, false, false, false),
            LookupResult::SequencePrefix
        );
        assert_eq!(
            reg.lookup(0x07a, false, false, false, false),
            LookupResult::SequencePrefix
        );
        assert_eq!(
            reg.lookup_sequence(
                0x07a, false, false, false, false, 0x07a, false, false, false, false
            ),
            Some("center_screen")
        );
    }

    // match_settings tests
    #[test]
    fn test_match_settings_prefix() {
        let matches = match_settings("front");
        assert_eq!(matches, vec!["frontmatter"]);
    }

    #[test]
    fn test_match_settings_paragraph() {
        let mut matches = match_settings("paragraph");
        matches.sort();
        assert_eq!(
            matches,
            vec!["paragraph_numbers", "paragraph_numbers_start"]
        );
    }

    #[test]
    fn test_match_settings_no_match() {
        let matches = match_settings("xyz");
        assert!(matches.is_empty());
    }

    #[test]
    fn test_match_settings_empty_prefix() {
        let matches = match_settings("");
        assert_eq!(matches.len(), 5);
    }

    #[test]
    fn test_export_html_in_command_list() {
        let matches = match_commands("export");
        assert_eq!(matches, vec!["export_html"]);
    }
}
