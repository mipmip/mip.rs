use warp::Filter;

pub struct RestBro;

impl RestBro {

    pub async fn run_bro(port: u16) {

        let routes = warp::any()
            .map(|| "Hello, World!");

        warp::serve(routes)
            .run(([127, 0, 0, 1], port))
            .await;

    }

}
