use mip::server::RestBro;

#[tokio::test]
async fn test_route_temp_html() {
    let dir = tempfile::tempdir().unwrap();
    let temp_dir = dir.path().to_str().unwrap().to_string();
    std::fs::write(dir.path().join(".temp.html"), "<html>hello</html>").unwrap();
    std::fs::write(dir.path().join(".temp.seed"), "abc1234").unwrap();

    let routes = RestBro::routes(temp_dir.clone(), temp_dir);

    let resp = warp::test::request()
        .path("/.temp.html")
        .reply(&routes)
        .await;

    assert_eq!(resp.status(), 200);
    let body = String::from_utf8_lossy(resp.body());
    assert!(body.contains("<html>hello</html>"));
}

#[tokio::test]
async fn test_route_temp_seed() {
    let dir = tempfile::tempdir().unwrap();
    let temp_dir = dir.path().to_str().unwrap().to_string();
    std::fs::write(dir.path().join(".temp.html"), "<html></html>").unwrap();
    std::fs::write(dir.path().join(".temp.seed"), "seed123").unwrap();

    let routes = RestBro::routes(temp_dir.clone(), temp_dir);

    let resp = warp::test::request()
        .path("/.temp.seed")
        .reply(&routes)
        .await;

    assert_eq!(resp.status(), 200);
    let body = String::from_utf8_lossy(resp.body());
    assert!(body.contains("seed123"));
}

#[tokio::test]
async fn test_route_static_asset() {
    let dir = tempfile::tempdir().unwrap();
    let temp_dir = dir.path().to_str().unwrap().to_string();

    // Create required temp files
    std::fs::write(dir.path().join(".temp.html"), "").unwrap();
    std::fs::write(dir.path().join(".temp.seed"), "").unwrap();

    // Create a static asset
    std::fs::write(dir.path().join("image.png"), "fakepng").unwrap();

    let routes = RestBro::routes(temp_dir.clone(), temp_dir);

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
    let temp_dir = dir.path().to_str().unwrap().to_string();
    std::fs::write(dir.path().join(".temp.html"), "").unwrap();
    std::fs::write(dir.path().join(".temp.seed"), "").unwrap();

    let routes = RestBro::routes(temp_dir.clone(), temp_dir);

    let resp = warp::test::request()
        .path("/nonexistent.txt")
        .reply(&routes)
        .await;

    assert_eq!(resp.status(), 404);
}

#[tokio::test]
async fn test_route_katex_js() {
    let dir = tempfile::tempdir().unwrap();
    let temp_dir = dir.path().to_str().unwrap().to_string();
    std::fs::write(dir.path().join(".temp.html"), "").unwrap();
    std::fs::write(dir.path().join(".temp.seed"), "").unwrap();

    let routes = RestBro::routes(temp_dir.clone(), temp_dir);

    let resp = warp::test::request()
        .path("/katex/katex.min.js")
        .reply(&routes)
        .await;

    assert_eq!(resp.status(), 200);
    assert!(resp.headers().get("content-type").unwrap().to_str().unwrap().contains("javascript"));
    let body = String::from_utf8_lossy(resp.body());
    assert!(body.len() > 1000); // katex.min.js is ~270KB
}

#[tokio::test]
async fn test_route_katex_css() {
    let dir = tempfile::tempdir().unwrap();
    let temp_dir = dir.path().to_str().unwrap().to_string();
    std::fs::write(dir.path().join(".temp.html"), "").unwrap();
    std::fs::write(dir.path().join(".temp.seed"), "").unwrap();

    let routes = RestBro::routes(temp_dir.clone(), temp_dir);

    let resp = warp::test::request()
        .path("/katex/katex.min.css")
        .reply(&routes)
        .await;

    assert_eq!(resp.status(), 200);
    assert!(resp.headers().get("content-type").unwrap().to_str().unwrap().contains("css"));
}

#[tokio::test]
async fn test_route_katex_font() {
    let dir = tempfile::tempdir().unwrap();
    let temp_dir = dir.path().to_str().unwrap().to_string();
    std::fs::write(dir.path().join(".temp.html"), "").unwrap();
    std::fs::write(dir.path().join(".temp.seed"), "").unwrap();

    let routes = RestBro::routes(temp_dir.clone(), temp_dir);

    let resp = warp::test::request()
        .path("/katex/fonts/KaTeX_Main-Regular.woff2")
        .reply(&routes)
        .await;

    assert_eq!(resp.status(), 200);
    assert!(resp.headers().get("content-type").unwrap().to_str().unwrap().contains("woff2"));
}
