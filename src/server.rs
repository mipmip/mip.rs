pub struct RestBro;

impl RestBro {

    pub async fn run_bro(path_dir: &'static str, port: u16) {

        //let served_path = path_dir.clone();

        println!("{}", path_dir);
        warp::serve(warp::fs::dir(path_dir))
            .run(([127, 0, 0, 1], port))
            .await;
    }
}
