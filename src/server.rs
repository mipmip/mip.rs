pub struct RestBro;

impl RestBro {

    pub async fn run_bro(port: u16) {

        warp::serve(warp::fs::dir("."))
            .run(([127, 0, 0, 1], port))
            .await;
    }
}
