use rust_embed::Embed;
use warp::Filter;

#[derive(Embed)]
#[folder = "asset/katex"]
struct KatexAsset;

#[derive(Embed)]
#[folder = "asset/mermaid"]
struct MermaidAsset;

fn mime_from_path(path: &str) -> &'static str {
    if path.ends_with(".js") {
        "application/javascript"
    } else if path.ends_with(".css") {
        "text/css"
    } else if path.ends_with(".woff2") {
        "font/woff2"
    } else if path.ends_with(".woff") {
        "font/woff"
    } else if path.ends_with(".ttf") {
        "font/ttf"
    } else {
        "application/octet-stream"
    }
}

pub struct RestBro;

impl RestBro {
    /// Build the warp filter chain without starting the server.
    /// This is testable with `warp::test`.
    pub fn routes(
        path_dir: String,
        temp_dir: String,
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone + 'static {
        let temp_html =
            warp::path(".temp.html").and(warp::fs::file(format!("{}/.temp.html", temp_dir)));
        let temp_seed =
            warp::path(".temp.seed").and(warp::fs::file(format!("{}/.temp.seed", temp_dir)));

        let katex = warp::path("katex").and(warp::path::tail()).and_then(
            |tail: warp::path::Tail| async move {
                let path = tail.as_str();
                match KatexAsset::get(path) {
                    Some(content) => {
                        let mime = mime_from_path(path);
                        Ok(warp::reply::with_header(
                            warp::reply::with_header(content.data.to_vec(), "content-type", mime),
                            "cache-control",
                            "public, max-age=31536000",
                        ))
                    }
                    None => Err(warp::reject::not_found()),
                }
            },
        );

        let mermaid = warp::path("mermaid").and(warp::path::tail()).and_then(
            |tail: warp::path::Tail| async move {
                let path = tail.as_str();
                match MermaidAsset::get(path) {
                    Some(content) => {
                        let mime = mime_from_path(path);
                        Ok(warp::reply::with_header(
                            warp::reply::with_header(content.data.to_vec(), "content-type", mime),
                            "cache-control",
                            "public, max-age=31536000",
                        ))
                    }
                    None => Err(warp::reject::not_found()),
                }
            },
        );

        let assets = warp::fs::dir(path_dir);

        temp_html.or(temp_seed).or(katex).or(mermaid).or(assets)
    }

    pub async fn run_bro(path_dir: String, temp_dir: String, port: u16) {
        println!("{}", path_dir);

        let routes = Self::routes(path_dir, temp_dir);

        warp::serve(routes).run(([127, 0, 0, 1], port)).await;
    }
}
