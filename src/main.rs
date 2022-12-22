mod view;
mod server;
mod markdown;

use notify::{RecommendedWatcher, RecursiveMode, Watcher, Config};
use std::path::Path;
use std::env;
use std::net::TcpListener;
use crate::server::RestBro;

fn get_available_port() -> Option<u16> {
    (8000..9000)
        .find(|port| port_is_available(*port))
}

fn port_is_available(port: u16) -> bool {
    match TcpListener::bind(("127.0.0.1", port)) {
        Ok(_) => true,
        Err(_) => false,
    }
}

fn watch<P: AsRef<Path>>(path: P, filepath: &String, port: u16) -> notify::Result<()> {
    let (tx, rx) = std::sync::mpsc::channel();

    let mut watcher = RecommendedWatcher::new(tx, Config::default())?;

    watcher.watch(path.as_ref(), RecursiveMode::Recursive)?;

    for res in rx {
        match res {
            Ok(event) => {
                if event.paths.len() > 0 {
                    let teststr = format!("{}", event.paths[0].display());
                    if teststr.contains(filepath) {
                        markdown::to_html(&filepath, port);
                    }
                }
            },
            Err(e) => println!("watch error: {:?}", e),
        }
    }

    Ok(())
}

fn main() {
    let path = std::env::args()
        .nth(1)
        .expect("Argument 1 needs to be a path");

    let args: Vec<_> = env::args().collect();

    if args.len() < 2 {
        println!("ERROR: Required arguments. \"file\"\n");
        println!("Please see the `--help`.");
        panic!("E1");
    }

    if let Some(available_port) = get_available_port() {
        markdown::to_html(&path, available_port);

        let tr = tokio::runtime::Runtime::new().unwrap();
        tr.spawn(async move{
            let path_parsed = Path::new(&path);
            let parent = path_parsed.parent().unwrap();

            if let Err(e) = watch(parent, &path, available_port) {
                println!("error: {:?}", e)
            }
        });

        tr.spawn(async move{
            RestBro::run_bro(available_port).await;
        });

        let _view_res = view::window(available_port);
    }
    else{
        panic!("E2");
    }

}
