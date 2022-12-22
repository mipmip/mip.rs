use warp::Filter;

pub struct RestBro;

impl RestBro {

    pub async fn run_bro(port: u16) {


        let markdown = warp::get()
            .and(warp::path(".temp.html"))
            .and(warp::fs::file("./.temp.html"));

        let seed =  warp::get()
            .and(warp::path(".temp.seed"))
            .and(warp::fs::file("./.temp.seed"));

        let routes = markdown.or(seed);
        warp::serve(routes)
            .run(([127, 0, 0, 1], port))
            .await;

    }

}
