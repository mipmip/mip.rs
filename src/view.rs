use std::path::PathBuf;
use std::net::TcpStream;

use gtk4::prelude::*;
use gtk4::{Application, ApplicationWindow};
use gtk4::glib;
use webkit6::prelude::*;
use webkit6::WebView;

/// Remove the seed-polling JS from the generated HTML.
/// Script 0: sets seedUrl/initialSeed vars
/// Script 1: keydown handler + XHR polling + location.reload()
/// Script 2: header link using seedUrl
/// We strip scripts 0 and 1 since reload is handled from Rust.
fn strip_seed_scripts(html: &str) -> String {
    let mut result = html.to_string();
    // Remove the seedUrl variable script
    if let Some(start) = result.find("<script>var seedUrl=")
        && let Some(end) = result[start..].find("</script>") {
            result = format!("{}{}", &result[..start], &result[start + end + 9..]);
        }
    // Remove the polling/reload script
    if let Some(start) = result.find("<script>document.addEventListener(\"keydown\"")
        && let Some(end) = result[start..].find("</script>") {
            result = format!("{}{}", &result[..start], &result[start + end + 9..]);
        }
    // Remove the header link script that references seedUrl
    if let Some(start) = result.find("<script>document.getElementById(\"header\")")
        && let Some(end) = result[start..].find("</script>") {
            result = format!("{}{}", &result[..start], &result[start + end + 9..]);
        }
    result
}

fn wait_for_server(port: u16) {
    for _ in 0..50 {
        if TcpStream::connect(("127.0.0.1", port)).is_ok() {
            return;
        }
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
    eprintln!("warning: server not ready after 5s");
}

pub fn window(port: u16, temp_dir: PathBuf) {
    let app = Application::builder()
        .application_id("org.mipmip.mip")
        .build();

    let html_path = temp_dir.join(".temp.html");
    let seed_path = temp_dir.join(".temp.seed");

    app.connect_activate(move |app| {
        wait_for_server(port);

        let webview = WebView::new();
        webview.set_vexpand(true);
        webview.set_hexpand(true);

        // Load HTML directly, stripping the JS seed-polling scripts
        // since we handle reload from the Rust side.
        let initial_html = std::fs::read_to_string(&html_path).unwrap_or_default();
        let clean_html = strip_seed_scripts(&initial_html);
        let base_uri = format!("http://localhost:{}/", port);
        webview.load_html(&clean_html, Some(&base_uri));

        let window = ApplicationWindow::builder()
            .application(app)
            .title("MiP")
            .default_width(800)
            .default_height(600)
            .child(&webview)
            .build();

        window.present();

        // Poll seed file and update page content via JS injection
        // to avoid the flicker of a full load_html() call.
        let seed_path = seed_path.clone();
        let infile_path = std::env::args().nth(1).unwrap();
        let mut last_seed = std::fs::read_to_string(&seed_path).unwrap_or_default();

        glib::timeout_add_local(std::time::Duration::from_millis(500), move || {
            if let Ok(current_seed) = std::fs::read_to_string(&seed_path)
                && current_seed != last_seed {
                    last_seed = current_seed;
                    if let Ok(md_content) = std::fs::read_to_string(&infile_path) {
                        let html_body = crate::markdown::md_to_html_body(&md_content);
                        let escaped = html_body
                            .replace('\\', "\\\\")
                            .replace('`', "\\`")
                            .replace("${", "\\${");
                        let js = format!(
                            "document.querySelector('.section').innerHTML = `{}`;",
                            escaped
                        );
                        webview.evaluate_javascript(&js, None, None, None::<&gtk4::gio::Cancellable>, |_| {});
                    }
                }
            glib::ControlFlow::Continue
        });
    });

    let temp_dir_cleanup = temp_dir.clone();
    app.connect_shutdown(move |_| {
        let _ = std::fs::remove_dir_all(&temp_dir_cleanup);
    });

    app.run_with_args::<String>(&[]);
}
