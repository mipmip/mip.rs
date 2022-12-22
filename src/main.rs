mod view;
mod server;
mod markdown;

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

fn main(){

    let args: Vec<_> = env::args().collect();

    if args.len() < 2 {
        println!("ERROR: Required arguments. \"file\"\n");
        println!("Please see the `--help`.");
        panic!("E1");
    }

    if let Some(available_port) = get_available_port() {
        markdown::to_html(&args[1]);

        let tr = tokio::runtime::Runtime::new().unwrap();
        tr.spawn(async move{
            RestBro::run_bro(available_port).await;
        });

        let _view_res = view::window(available_port);
    }
    else{
        panic!("E2");
    }

}
