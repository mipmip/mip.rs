use warp::Filter;

pub struct RestBro;

impl RestBro {
    /// Build the warp filter chain without starting the server.
    /// This is testable with `warp::test`.
    pub fn routes(
        path_dir: String,
        temp_dir: String,
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone + 'static {
        let temp_html = warp::path(".temp.html")
            .and(warp::fs::file(format!("{}/.temp.html", temp_dir)));
        let temp_seed = warp::path(".temp.seed")
            .and(warp::fs::file(format!("{}/.temp.seed", temp_dir)));
        let assets = warp::fs::dir(path_dir);

        temp_html.or(temp_seed).or(assets)
    }

    pub async fn run_bro(path_dir: &'static str, temp_dir: &'static str, port: u16) {

        println!("{}", path_dir);

        let routes = Self::routes(path_dir.to_string(), temp_dir.to_string());

        warp::serve(routes)
            .run(([127, 0, 0, 1], port))
            .await;
    }
}
