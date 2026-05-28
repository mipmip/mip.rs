mod view;
mod server;
mod markdown;

use notify::{RecommendedWatcher, RecursiveMode, Watcher, Config};
use std::fs;
use std::path::{Path, PathBuf};
use std::process;
use std::env;
use std::net::TcpListener;
use crate::server::RestBro;

fn get_available_port() -> Option<u16> {
    (8000..9000)
        .find(|port| port_is_available(*port))
}

fn port_is_available(port: u16) -> bool {
    TcpListener::bind(("127.0.0.1", port)).is_ok()
}

fn watch(path_dir: &std::path::Path, path_file: &str, temp_dir: &std::path::Path, port: u16) -> notify::Result<()> {
    let (tx, rx) = std::sync::mpsc::channel();

    let mut watcher = RecommendedWatcher::new(tx, Config::default())?;

    watcher.watch(path_dir.as_ref(), RecursiveMode::Recursive)?;

    for res in rx {
        match res {
            Ok(event) => {
                if !event.paths.is_empty() {
                    let teststr = format!("{}", event.paths[0].display());
                    if teststr.contains(path_file) {
                        markdown::to_html(path_file, temp_dir, port);
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
    let path_file0 = std::env::args()
        .nth(1)
        .expect("Argument 1 needs to be a path");

    let args: Vec<_> = env::args().collect();

    let path_file = String::from(&path_file0);

    let s_slice = string_to_static_str(path_file0);

    let path_parsed1 = Path::new(s_slice);
    let path_dir_for_server = path_parsed1.parent().unwrap();

    let s_slice2 = string_to_static_str(path_dir_for_server.to_str().unwrap().to_string());

    if args.len() < 2 {
        println!("ERROR: Required arguments. \"file\"\n");
        println!("Please see the `--help`.");
        process::exit(1);
    }

    let temp_dir: PathBuf = env::temp_dir().join(format!("mip-{}", process::id()));
    fs::create_dir_all(&temp_dir).expect("Unable to create temp directory");
    let temp_dir_str = string_to_static_str(temp_dir.to_str().unwrap().to_string());
    let temp_dir_for_watcher = temp_dir.clone();

    if let Some(available_port) = get_available_port() {
        markdown::to_html(&path_file, &temp_dir, available_port);

        // Run tokio runtime in a separate thread so it doesn't compete
        // with the GTK4 main loop for the main thread.
        std::thread::spawn(move || {
            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(async move {
                let watcher_handle = tokio::spawn(async move {
                    let path_parsed = Path::new(&path_file);
                    let path_dir_for_watcher = path_parsed.parent().unwrap();

                    if let Err(e) = watch(path_dir_for_watcher, &path_file, &temp_dir_for_watcher, available_port) {
                        println!("error: {:?}", e)
                    }
                });

                let server_handle = tokio::spawn(async move {
                    RestBro::run_bro(s_slice2, temp_dir_str, available_port).await;
                });

                let _ = tokio::join!(watcher_handle, server_handle);
            });
        });

        view::window(available_port, temp_dir);
    }
    else{
        panic!("E2");
    }
}
