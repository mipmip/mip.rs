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

fn watch(path_dir: &std::path::Path, path_file: &String, port: u16) -> notify::Result<()> {
    let (tx, rx) = std::sync::mpsc::channel();

    let mut watcher = RecommendedWatcher::new(tx, Config::default())?;

    watcher.watch(path_dir.as_ref(), RecursiveMode::Recursive)?;

    for res in rx {
        match res {
            Ok(event) => {
                if event.paths.len() > 0 {
                    let teststr = format!("{}", event.paths[0].display());
                    if teststr.contains(path_file) {
                        markdown::to_html(&path_dir, &path_file, port);
                    }
                }
            },
            Err(e) => println!("watch error: {:?}", e),
        }
    }
    Ok(())
}

/*
fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}
*/

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

    let path_parsed0 = Path::new(s_slice);
    let path_dir_for_initial_md = path_parsed0.parent().unwrap();

    if args.len() < 2 {
        println!("ERROR: Required arguments. \"file\"\n");
        println!("Please see the `--help`.");
        process::exit(1);
    }

    if let Some(available_port) = get_available_port() {
        markdown::to_html(&path_dir_for_initial_md, &path_file, available_port);

        let tr = tokio::runtime::Runtime::new().unwrap();
        tr.spawn(async move{
            let path_parsed = Path::new(&path_file);
            let path_dir_for_watcher = path_parsed.parent().unwrap();

            if let Err(e) = watch(path_dir_for_watcher, &path_file, available_port) {
                println!("error: {:?}", e)
            }
        });

        tr.spawn(async move{
            RestBro::run_bro(s_slice2, available_port).await;
        });

        let _view_res = view::window(available_port);
    }
    else{
        panic!("E2");
    }

}
