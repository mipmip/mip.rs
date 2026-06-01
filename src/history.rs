use std::fs;
use std::path::{Path, PathBuf};

pub struct CommandHistory {
    entries: Vec<String>,
    max_size: usize,
}

impl CommandHistory {
    pub fn new(max_size: usize) -> Self {
        Self {
            entries: Vec::new(),
            max_size,
        }
    }

    /// Load history from file. Returns empty history if file doesn't exist or is unreadable.
    pub fn load(path: &Path, max_size: usize) -> Self {
        let entries = match fs::read_to_string(path) {
            Ok(content) => content
                .lines()
                .filter(|l| !l.is_empty())
                .map(|l| l.to_string())
                .collect(),
            Err(_) => Vec::new(),
        };
        let mut history = Self { entries, max_size };
        history.trim();
        history
    }

    /// Save history to file. Creates parent directories if needed.
    pub fn save(&self, path: &Path) {
        if let Some(parent) = path.parent() {
            let _ = fs::create_dir_all(parent);
        }
        let content = self.entries.join("\n");
        if let Err(e) = fs::write(path, content) {
            eprintln!("warning: could not save history: {}", e);
        }
    }

    /// Push a command to history. Deduplicates (removes existing occurrence) and trims to max_size.
    pub fn push(&mut self, cmd: &str) {
        let cmd = cmd.trim();
        if cmd.is_empty() {
            return;
        }
        // Remove existing occurrence
        self.entries.retain(|e| e != cmd);
        // Append
        self.entries.push(cmd.to_string());
        self.trim();
    }

    /// Return entries matching prefix, most recent last.
    pub fn filter(&self, prefix: &str) -> Vec<&str> {
        self.entries
            .iter()
            .filter(|e| e.starts_with(prefix))
            .map(|e| e.as_str())
            .collect()
    }

    fn trim(&mut self) {
        if self.entries.len() > self.max_size {
            let excess = self.entries.len() - self.max_size;
            self.entries.drain(..excess);
        }
    }
}

/// Return the XDG-compliant history file path.
pub fn history_path() -> PathBuf {
    let state_dir = std::env::var("XDG_STATE_HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|_| {
            let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
            PathBuf::from(home).join(".local").join("state")
        });
    state_dir.join("miprs").join("history")
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    #[test]
    fn test_push_and_filter() {
        let mut h = CommandHistory::new(50);
        h.push("open foo.md");
        h.push("open bar.md");
        h.push("q");
        assert_eq!(h.filter("open"), vec!["open foo.md", "open bar.md"]);
        assert_eq!(h.filter("q"), vec!["q"]);
        assert_eq!(h.filter("z"), Vec::<&str>::new());
    }

    #[test]
    fn test_dedup_moves_to_end() {
        let mut h = CommandHistory::new(50);
        h.push("open foo.md");
        h.push("q");
        h.push("open foo.md");
        assert_eq!(h.filter(""), vec!["q", "open foo.md"]);
    }

    #[test]
    fn test_max_size_trims_oldest() {
        let mut h = CommandHistory::new(3);
        h.push("a");
        h.push("b");
        h.push("c");
        h.push("d");
        assert_eq!(h.filter(""), vec!["b", "c", "d"]);
    }

    #[test]
    fn test_empty_and_whitespace_ignored() {
        let mut h = CommandHistory::new(50);
        h.push("");
        h.push("  ");
        assert_eq!(h.filter(""), Vec::<&str>::new());
    }

    #[test]
    fn test_load_save_roundtrip() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("history");

        let mut h = CommandHistory::new(50);
        h.push("open foo.md");
        h.push("q");
        h.save(&path);

        let h2 = CommandHistory::load(&path, 50);
        assert_eq!(h2.filter(""), vec!["open foo.md", "q"]);
    }

    #[test]
    fn test_load_missing_file() {
        let h = CommandHistory::load(Path::new("/nonexistent/path/history"), 50);
        assert_eq!(h.filter(""), Vec::<&str>::new());
    }

    #[test]
    fn test_load_trims_to_max_size() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("history");
        {
            let mut f = fs::File::create(&path).unwrap();
            for i in 0..100 {
                writeln!(f, "cmd{}", i).unwrap();
            }
        }
        let h = CommandHistory::load(&path, 10);
        assert_eq!(h.filter("").len(), 10);
        assert_eq!(h.filter("").last(), Some(&"cmd99"));
    }

    #[test]
    fn test_filter_empty_prefix_returns_all() {
        let mut h = CommandHistory::new(50);
        h.push("a");
        h.push("b");
        h.push("c");
        assert_eq!(h.filter(""), vec!["a", "b", "c"]);
    }
}
