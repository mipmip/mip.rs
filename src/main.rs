mod view;
mod server;
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

    if let Some(available_port) = get_available_port() {
        let tr = tokio::runtime::Runtime::new().unwrap();
        tr.spawn(async move{
            RestBro::run_bro(available_port).await;
        });
        //println!("Everything working good!");

        let _view_res = view::window(available_port);
    }
    else{
        println!("Could not find free port");
    }
}
