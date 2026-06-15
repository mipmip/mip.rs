use argh::FromArgs;
use mip::server::RestBro;
use notify::{RecommendedWatcher, RecursiveMode, Watcher};
use std::env;
use std::fs;
use std::net::TcpListener;
use std::path::{Path, PathBuf};
use std::process;

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

    /// disable math rendering
    #[argh(switch)]
    no_math: bool,

    /// create a new custom style directory with default CSS
    #[argh(option)]
    initstyle: Option<String>,

    /// disable mermaid diagram rendering
    #[argh(switch)]
    no_mermaid: bool,

    /// startup zoom level (e.g. 1.4); overrides config, clamped to 0.3-5.0
    #[argh(option)]
    zoom: Option<f64>,
}

fn get_available_port() -> Option<u16> {
    (8000..9000).find(|port| port_is_available(*port))
}

pub(crate) fn port_is_available(port: u16) -> bool {
    TcpListener::bind(("127.0.0.1", port)).is_ok()
}

#[allow(clippy::too_many_arguments)]
fn watch(
    path_dir: PathBuf,
    path_file: String,
    temp_dir: PathBuf,
    port: u16,
    show_frontmatter: bool,
    theme_class: String,
    new_file_rx: std::sync::mpsc::Receiver<PathBuf>,
    math: bool,
    custom_css: String,
    mermaid: bool,
) -> notify::Result<()> {
    let (tx, rx) = std::sync::mpsc::channel();

    let mut watcher = RecommendedWatcher::new(tx, notify::Config::default())?;
    let mut current_dir = path_dir;
    let mut current_file = path_file;

    watcher.watch(&current_dir, RecursiveMode::Recursive)?;

    loop {
        // Check for new file paths from :open
        while let Ok(new_path) = new_file_rx.try_recv() {
            let new_file = new_path.to_string_lossy().to_string();
            if let Some(new_dir) = new_path.parent() {
                let new_dir = new_dir.to_path_buf();
                if new_dir != current_dir {
                    let _ = watcher.unwatch(&current_dir);
                    let _ = watcher.watch(&new_dir, RecursiveMode::Recursive);
                    current_dir = new_dir;
                }
            }
            current_file = new_file;
        }

        // Check for file events with a timeout
        match rx.recv_timeout(std::time::Duration::from_millis(200)) {
            Ok(Ok(event)) => {
                if !event.paths.is_empty() {
                    let teststr = format!("{}", event.paths[0].display());
                    if teststr.contains(&current_file) {
                        mip::markdown::to_html(
                            &current_file,
                            &temp_dir,
                            port,
                            show_frontmatter,
                            &theme_class,
                            math,
                            mermaid,
                            &custom_css,
                        );
                    }
                }
            }
            Ok(Err(e)) => println!("watch error: {:?}", e),
            Err(std::sync::mpsc::RecvTimeoutError::Timeout) => {}
            Err(std::sync::mpsc::RecvTimeoutError::Disconnected) => break,
        }
    }
    Ok(())
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
        if let Some(parent) = path.parent()
            && let Err(e) = fs::create_dir_all(parent)
        {
            eprintln!(
                "error: could not create directory {}: {}",
                parent.display(),
                e
            );
            process::exit(1);
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

    if let Some(ref style_name) = cli.initstyle {
        let style_dir = mip::config::styles_dir().join(style_name);
        if style_dir.exists() {
            eprintln!("Style directory already exists at {}", style_dir.display());
            eprintln!("Remove it first if you want to recreate.");
            process::exit(1);
        }
        if let Err(e) = fs::create_dir_all(&style_dir) {
            eprintln!(
                "error: could not create directory {}: {}",
                style_dir.display(),
                e
            );
            process::exit(1);
        }
        let css_path = style_dir.join("style.css");
        match fs::write(&css_path, mip::config::default_style_css()) {
            Ok(_) => {
                println!("Style created at {}", css_path.display());
                println!("Add this to your config to use it:");
                println!("  style = \"{}\"", style_name);
                process::exit(0);
            }
            Err(e) => {
                eprintln!("error: could not write style file: {}", e);
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
            eprintln!(
                "error: invalid theme '{}'. Must be system, light, or dark.",
                t
            );
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
            if mip::is_system_dark() {
                "dark"
            } else {
                "light"
            }
        }
    };

    // CLI --frontmatter overrides config (flag presence means true)
    let show_frontmatter = if cli.frontmatter {
        true
    } else {
        cfg.frontmatter()
    };

    // Resolve runcmd: CLI overrides config
    let runcmd = cli.runcmd.as_deref().or_else(|| cfg.runcmd());

    // CLI --no-math overrides config (flag presence means false)
    let math = if cli.no_math { false } else { cfg.math() };

    // Load custom CSS from style setting
    let style_name = cfg.style().map(|s| s.to_string());
    let custom_css = if let Some(ref name) = style_name {
        let css_path = mip::config::style_css_path(name);
        match fs::read_to_string(&css_path) {
            Ok(css) => css,
            Err(_) => {
                eprintln!(
                    "warning: style '{}' not found at {}",
                    name,
                    css_path.display()
                );
                String::new()
            }
        }
    } else {
        String::new()
    };

    // CLI --no-mermaid overrides config (flag presence means false)
    let mermaid = if cli.no_mermaid { false } else { cfg.mermaid() };

    // CLI --zoom overrides config; clamp to the same bounds as zoom_in/zoom_out
    let zoom = cli.zoom.unwrap_or_else(|| cfg.zoom()).clamp(0.3, 5.0);

    let path_file = path_file0;

    let path_parsed = Path::new(&path_file);
    let path_dir_for_server = {
        let parent = path_parsed.parent().unwrap();
        if parent.as_os_str().is_empty() {
            PathBuf::from(".")
        } else {
            parent.to_path_buf()
        }
    };
    // Canonicalize so the symlink target is absolute
    let path_dir_for_server = fs::canonicalize(&path_dir_for_server).unwrap_or(path_dir_for_server);

    let temp_dir: PathBuf = env::temp_dir().join(format!("mip-{}", process::id()));
    fs::create_dir_all(&temp_dir).expect("Unable to create temp directory");

    // Create docroot symlink pointing to the document's parent directory
    let docroot = temp_dir.join("docroot");
    if let Err(e) = std::os::unix::fs::symlink(&path_dir_for_server, &docroot) {
        eprintln!("warning: could not create docroot symlink: {}", e);
    }

    let temp_dir_for_watcher = temp_dir.clone();
    let temp_dir_str = temp_dir.to_str().unwrap().to_string();
    let theme_class_string = theme_class.to_string();
    let path_file_for_view = path_file.clone();
    let path_file_for_watcher = path_file.clone();
    let path_dir_for_watcher = path_dir_for_server.clone();
    let theme_class_for_watcher = theme_class_string.clone();
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
        mip::markdown::to_html(
            &path_file,
            &temp_dir,
            available_port,
            show_frontmatter,
            &theme_class_string,
            math,
            mermaid,
            &custom_css,
        );

        // Channel for :open to send new file paths to the watcher
        let (new_file_tx, new_file_rx) = std::sync::mpsc::channel::<PathBuf>();

        // Serve from the docroot symlink so :open can update the target
        let server_dir = docroot.to_str().unwrap().to_string();

        // Run tokio runtime in a separate thread so it doesn't compete
        // with the GTK4 main loop for the main thread.
        std::thread::spawn(move || {
            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(async move {
                let watcher_handle = tokio::spawn(async move {
                    if let Err(e) = watch(
                        path_dir_for_watcher,
                        path_file_for_watcher,
                        temp_dir_for_watcher,
                        available_port,
                        show_frontmatter,
                        theme_class_for_watcher,
                        new_file_rx,
                        math,
                        custom_css,
                        mermaid,
                    ) {
                        println!("error: {:?}", e)
                    }
                });

                let server_handle = tokio::spawn(async move {
                    RestBro::run_bro(server_dir, temp_dir_str, available_port).await;
                });

                let _ = tokio::join!(watcher_handle, server_handle);
            });
        });

        mip::view::window(
            available_port,
            temp_dir,
            show_frontmatter,
            theme,
            &path_file_for_view,
            runcmd_string.as_deref(),
            sidetoc_width,
            &sidetoc_position,
            keybinding_registry,
            paragraph_numbers,
            paragraph_numbers_start,
            cfg.history_size(),
            new_file_tx,
            math,
            mermaid,
            style_name.as_deref(),
            zoom,
        );
    } else {
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
