use warp::Filter;

pub struct RestBro;

impl RestBro {

    pub async fn run_bro(path_dir: &'static str, temp_dir: &'static str, port: u16) {

        println!("{}", path_dir);

        let temp_html = warp::path(".temp.html")
            .and(warp::fs::file(format!("{}/.temp.html", temp_dir)));
        let temp_seed = warp::path(".temp.seed")
            .and(warp::fs::file(format!("{}/.temp.seed", temp_dir)));
        let assets = warp::fs::dir(path_dir);

        warp::serve(temp_html.or(temp_seed).or(assets))
            .run(([127, 0, 0, 1], port))
            .await;
    }
}
