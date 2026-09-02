use mip::server::RestBro;

#[tokio::test]
async fn test_route_static_asset() {
    let dir = tempfile::tempdir().unwrap();
    let serve_dir = dir.path().to_str().unwrap().to_string();

    // Create a static asset
    std::fs::write(dir.path().join("image.png"), "fakepng").unwrap();

    let routes = RestBro::routes(serve_dir);

    let resp = warp::test::request()
        .path("/image.png")
        .reply(&routes)
        .await;

    assert_eq!(resp.status(), 200);
    let body = String::from_utf8_lossy(resp.body());
    assert!(body.contains("fakepng"));
}

#[tokio::test]
async fn test_route_missing_file_404() {
    let dir = tempfile::tempdir().unwrap();
    let serve_dir = dir.path().to_str().unwrap().to_string();

    let routes = RestBro::routes(serve_dir);

    let resp = warp::test::request()
        .path("/nonexistent.txt")
        .reply(&routes)
        .await;

    assert_eq!(resp.status(), 404);
}

#[tokio::test]
async fn test_route_katex_js() {
    let dir = tempfile::tempdir().unwrap();
    let serve_dir = dir.path().to_str().unwrap().to_string();

    let routes = RestBro::routes(serve_dir);

    let resp = warp::test::request()
        .path("/katex/katex.min.js")
        .reply(&routes)
        .await;

    assert_eq!(resp.status(), 200);
    assert!(
        resp.headers()
            .get("content-type")
            .unwrap()
            .to_str()
            .unwrap()
            .contains("javascript")
    );
    let body = String::from_utf8_lossy(resp.body());
    assert!(body.len() > 1000); // katex.min.js is ~270KB
}

#[tokio::test]
async fn test_route_katex_css() {
    let dir = tempfile::tempdir().unwrap();
    let serve_dir = dir.path().to_str().unwrap().to_string();

    let routes = RestBro::routes(serve_dir);

    let resp = warp::test::request()
        .path("/katex/katex.min.css")
        .reply(&routes)
        .await;

    assert_eq!(resp.status(), 200);
    assert!(
        resp.headers()
            .get("content-type")
            .unwrap()
            .to_str()
            .unwrap()
            .contains("css")
    );
}

#[tokio::test]
async fn test_route_katex_font() {
    let dir = tempfile::tempdir().unwrap();
    let serve_dir = dir.path().to_str().unwrap().to_string();

    let routes = RestBro::routes(serve_dir);

    let resp = warp::test::request()
        .path("/katex/fonts/KaTeX_Main-Regular.woff2")
        .reply(&routes)
        .await;

    assert_eq!(resp.status(), 200);
    assert!(
        resp.headers()
            .get("content-type")
            .unwrap()
            .to_str()
            .unwrap()
            .contains("woff2")
    );
}

#[tokio::test]
async fn test_route_mermaid_js() {
    let dir = tempfile::tempdir().unwrap();
    let serve_dir = dir.path().to_str().unwrap().to_string();

    let routes = RestBro::routes(serve_dir);

    let resp = warp::test::request()
        .path("/mermaid/mermaid.min.js")
        .reply(&routes)
        .await;

    assert_eq!(resp.status(), 200);
    assert!(
        resp.headers()
            .get("content-type")
            .unwrap()
            .to_str()
            .unwrap()
            .contains("javascript")
    );
    let body = String::from_utf8_lossy(resp.body());
    assert!(body.len() > 1000); // mermaid.min.js is ~3MB
}
