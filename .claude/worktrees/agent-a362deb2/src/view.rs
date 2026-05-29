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
pub(crate) fn strip_seed_scripts(html: &str) -> String {
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

pub fn window(port: u16, temp_dir: PathBuf, show_frontmatter: bool, theme_mode: &str) {
    let is_system_theme = theme_mode == "system";
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
        let mut last_system_dark = if is_system_theme { Some(crate::is_system_dark()) } else { None };

        glib::timeout_add_local(std::time::Duration::from_millis(500), move || {
            // Check for system theme changes
            if let Some(ref mut was_dark) = last_system_dark {
                let now_dark = crate::is_system_dark();
                if now_dark != *was_dark {
                    *was_dark = now_dark;
                    let class = if now_dark { "dark" } else { "light" };
                    let js = format!(
                        "document.documentElement.className = '{}';",
                        class
                    );
                    webview.evaluate_javascript(&js, None, None, None::<&gtk4::gio::Cancellable>, |_| {});
                }
            }

            if let Ok(current_seed) = std::fs::read_to_string(&seed_path)
                && current_seed != last_seed {
                    last_seed = current_seed;
                    if let Ok(md_content) = std::fs::read_to_string(&infile_path) {
                        let html_body = crate::markdown::md_to_html_body(&md_content, show_frontmatter);
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_strip_seed_scripts_removes_all_seed_scripts() {
        let html = r#"<html>
<head></head>
<body>
<div>content</div>
<script>var seedUrl="http://localhost:8000/.temp.seed";var initialSeed="abc1234";</script>
<script>document.addEventListener("keydown",function(e){});</script>
<script>document.getElementById("header").onclick=function(){};</script>
<script>console.log("keep me");</script>
</body>
</html>"#;

        let result = strip_seed_scripts(html);

        assert!(!result.contains("var seedUrl="));
        assert!(!result.contains("document.addEventListener(\"keydown\""));
        assert!(!result.contains("document.getElementById(\"header\")"));
        assert!(result.contains("console.log(\"keep me\")"));
        assert!(result.contains("<div>content</div>"));
    }

    #[test]
    fn test_strip_seed_scripts_preserves_non_seed_content() {
        let html = r#"<html><body><h1>Hello</h1><script>alert("safe")</script></body></html>"#;
        let result = strip_seed_scripts(html);
        assert_eq!(result, html);
    }

    #[test]
    fn test_strip_seed_scripts_handles_empty_input() {
        let result = strip_seed_scripts("");
        assert_eq!(result, "");
    }

    #[test]
    fn test_strip_seed_scripts_handles_no_scripts() {
        let html = "<html><body><p>No scripts here</p></body></html>";
        let result = strip_seed_scripts(html);
        assert_eq!(result, html);
    }
}
