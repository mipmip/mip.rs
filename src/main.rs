use argh::FromArgs;
use notify::{RecommendedWatcher, RecursiveMode, Watcher};
use std::fs;
use std::path::{Path, PathBuf};
use std::process;
use std::env;
use std::net::TcpListener;
use mip::server::RestBro;

/// mip - Markdown In Preview
#[derive(FromArgs)]
struct Cli {
    /// path to the markdown file
    #[argh(positional)]
    file: Option<PathBuf>,

    /// print version
    #[argh(switch)]
    version: bool,

    /// enable verbose output
    #[allow(dead_code)]
    #[argh(switch, short = 'v')]
    verbose: bool,

    /// show frontmatter as a table
    #[argh(switch)]
    frontmatter: bool,

    /// color theme: system, light, or dark
    #[argh(option)]
    theme: Option<String>,

    /// run command(s) at startup (e.g. "sidetoc_open")
    #[argh(option)]
    runcmd: Option<String>,

    /// generate default config file at ~/.config/miprs/config.toml
    #[argh(switch)]
    initconf: bool,
}

fn get_available_port() -> Option<u16> {
    (8000..9000)
        .find(|port| port_is_available(*port))
}

pub(crate) fn port_is_available(port: u16) -> bool {
    TcpListener::bind(("127.0.0.1", port)).is_ok()
}

fn watch(path_dir: &std::path::Path, path_file: &str, temp_dir: &std::path::Path, port: u16, show_frontmatter: bool, theme_class: &str) -> notify::Result<()> {
    let (tx, rx) = std::sync::mpsc::channel();

    let mut watcher = RecommendedWatcher::new(tx, notify::Config::default())?;

    watcher.watch(path_dir.as_ref(), RecursiveMode::Recursive)?;

    for res in rx {
        match res {
            Ok(event) => {
                if !event.paths.is_empty() {
                    let teststr = format!("{}", event.paths[0].display());
                    if teststr.contains(path_file) {
                        mip::markdown::to_html(path_file, temp_dir, port, show_frontmatter, theme_class);
                    }
                }
            },
            Err(e) => println!("watch error: {:?}", e),
        }
    }
    Ok(())
}

fn string_to_static_str(s: String) -> &'static str {
    Box::leak(s.into_boxed_str())
}

fn main() {
    let cli: Cli = argh::from_env();

    if cli.version {
        println!("mip {}", env!("CARGO_PKG_VERSION"));
        process::exit(0);
    }

    if cli.initconf {
        let path = mip::config::config_path();
        if path.exists() {
            eprintln!("Config file already exists at {}", path.display());
            eprintln!("Back it up first if you want to regenerate.");
            process::exit(1);
        }
        if let Some(parent) = path.parent() {
            if let Err(e) = fs::create_dir_all(parent) {
                eprintln!("error: could not create directory {}: {}", parent.display(), e);
                process::exit(1);
            }
        }
        match fs::write(&path, mip::config::default_config_template()) {
            Ok(_) => {
                println!("Config file created at {}", path.display());
                process::exit(0);
            }
            Err(e) => {
                eprintln!("error: could not write config file: {}", e);
                process::exit(1);
            }
        }
    }

    let path_file0 = match cli.file {
        Some(p) => p.to_str().unwrap().to_string(),
        None => {
            // Trigger argh's help output by parsing --help
            match Cli::from_args(&["mip"], &["--help"]) {
                Ok(_) => unreachable!(),
                Err(early_exit) => {
                    println!("{}", early_exit.output);
                    process::exit(0);
                }
            }
        }
    };

    // Load config and merge with CLI flags
    let cfg = mip::config::Config::load();

    let theme = if let Some(ref t) = cli.theme {
        if !["system", "light", "dark"].contains(&t.as_str()) {
            eprintln!("error: invalid theme '{}'. Must be system, light, or dark.", t);
            process::exit(1);
        }
        t.as_str()
    } else {
        cfg.theme()
    };

    let theme_class = match theme {
        "light" => "light",
        "dark" => "dark",
        _ => {
            // Detect system dark mode preference
            if mip::is_system_dark() { "dark" } else { "light" }
        }
    };

    // CLI --frontmatter overrides config (flag presence means true)
    let show_frontmatter = if cli.frontmatter { true } else { cfg.frontmatter() };

    // Resolve runcmd: CLI overrides config
    let runcmd = cli.runcmd.as_deref().or_else(|| cfg.runcmd());

    let path_file = String::from(&path_file0);

    let s_slice = string_to_static_str(path_file0);

    let path_parsed1 = Path::new(s_slice);
    let path_dir_for_server = path_parsed1.parent().unwrap();

    let s_slice2 = string_to_static_str(path_dir_for_server.to_str().unwrap().to_string());

    let temp_dir: PathBuf = env::temp_dir().join(format!("mip-{}", process::id()));
    fs::create_dir_all(&temp_dir).expect("Unable to create temp directory");
    let temp_dir_str = string_to_static_str(temp_dir.to_str().unwrap().to_string());
    let temp_dir_for_watcher = temp_dir.clone();
    let theme_class_string = theme_class.to_string();
    let path_file_for_view = path_file.clone();
    let runcmd_string = runcmd.map(|s| s.to_string());
    let sidetoc_width = cfg.sidetoc_width();
    let sidetoc_position = cfg.sidetoc_position().to_string();
    let paragraph_numbers = cfg.paragraph_numbers();
    let paragraph_numbers_start = cfg.paragraph_numbers_start();

    // Build keybinding registry: defaults + config overrides
    let mut keybinding_registry = mip::command::KeybindingRegistry::with_defaults();
    if let Some(ref kb) = cfg.keybindings {
        keybinding_registry.register_from_config(kb);
    }

    if let Some(available_port) = get_available_port() {
        mip::markdown::to_html(&path_file, &temp_dir, available_port, show_frontmatter, &theme_class_string);

        // Run tokio runtime in a separate thread so it doesn't compete
        // with the GTK4 main loop for the main thread.
        std::thread::spawn(move || {
            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(async move {
                let watcher_handle = tokio::spawn(async move {
                    let path_parsed = Path::new(&path_file);
                    let path_dir_for_watcher = path_parsed.parent().unwrap();

                    if let Err(e) = watch(path_dir_for_watcher, &path_file, &temp_dir_for_watcher, available_port, show_frontmatter, &theme_class_string) {
                        println!("error: {:?}", e)
                    }
                });

                let server_handle = tokio::spawn(async move {
                    RestBro::run_bro(s_slice2, temp_dir_str, available_port).await;
                });

                let _ = tokio::join!(watcher_handle, server_handle);
            });
        });

        mip::view::window(available_port, temp_dir, show_frontmatter, theme, &path_file_for_view, runcmd_string.as_deref(), sidetoc_width, &sidetoc_position, keybinding_registry, paragraph_numbers, paragraph_numbers_start, cfg.history_size());
    }
    else{
        panic!("E2");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_port_is_available_on_free_port() {
        // Use a high port that's very unlikely to be in use
        assert!(port_is_available(19876));
    }

    #[test]
    fn test_port_is_available_on_occupied_port() {
        // Bind a port, then check it's not available
        let listener = TcpListener::bind(("127.0.0.1", 0)).unwrap();
        let port = listener.local_addr().unwrap().port();
        assert!(!port_is_available(port));
        drop(listener);
    }

    #[test]
    fn test_get_available_port_returns_some() {
        assert!(get_available_port().is_some());
    }
}
