use std::path::PathBuf;
use std::net::TcpStream;

use gtk4::prelude::*;
use gtk4::prelude::TreeViewExt;
use gtk4::{Application, ApplicationWindow, ScrolledWindow, Paned, Orientation, Stack};
use gtk4::glib;
use webkit6::prelude::*;
use webkit6::{WebView, NavigationPolicyDecision, PolicyDecisionType};

use crate::markdown::TocEntry;

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

const COL_TITLE: u32 = 0;
const COL_ANCHOR: u32 = 1;
const COL_LEVEL: u32 = 2;

fn create_toc_store() -> gtk4::TreeStore {
    gtk4::TreeStore::new(&[
        glib::Type::STRING, // title
        glib::Type::STRING, // anchor_id
        glib::Type::U32,    // level
    ])
}

pub fn populate_toc(store: &gtk4::TreeStore, entries: &[TocEntry]) {
    store.clear();
    // Stack of (level, TreeIter) for tracking parent hierarchy
    let mut parent_stack: Vec<(u8, gtk4::TreeIter)> = Vec::new();

    for entry in entries {
        // Find the appropriate parent: pop stack until we find a level < current
        while let Some((lvl, _)) = parent_stack.last() {
            if *lvl >= entry.level {
                parent_stack.pop();
            } else {
                break;
            }
        }

        let parent_iter = parent_stack.last().map(|(_, iter)| iter);
        let iter = store.append(parent_iter);
        store.set(&iter, &[
            (COL_TITLE, &entry.title),
            (COL_ANCHOR, &entry.anchor_id),
            (COL_LEVEL, &(entry.level as u32)),
        ]);
        parent_stack.push((entry.level, iter));
    }
}

fn create_toc_view(store: &gtk4::TreeStore) -> gtk4::TreeView {
    let treeview = gtk4::TreeView::with_model(store);
    treeview.set_headers_visible(false);
    treeview.set_enable_search(false);

    let renderer = gtk4::CellRendererText::new();
    let column = gtk4::TreeViewColumn::new();
    column.pack_start(&renderer, true);
    column.add_attribute(&renderer, "text", COL_TITLE as i32);
    treeview.append_column(&column);

    treeview.set_vexpand(true);
    treeview.set_hexpand(true);

    treeview
}

fn scroll_to_anchor(webview: &WebView, anchor_id: &str) {
    let js = format!(
        "document.getElementById('{}').scrollIntoView({{behavior: 'smooth'}});",
        anchor_id.replace('\'', "\\'")
    );
    webview.evaluate_javascript(&js, None, None, None::<&gtk4::gio::Cancellable>, |_| {});
}

pub fn window(port: u16, temp_dir: PathBuf, show_frontmatter: bool, theme_mode: &str, toc_mode: &str, infile: &str) {
    let is_system_theme = theme_mode == "system";
    let toc_mode = toc_mode.to_string();
    let infile = infile.to_string();
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

        // Open external links in default browser
        let local_origin = format!("http://localhost:{}", port);
        webview.connect_decide_policy(move |_, decision, decision_type| {
            if matches!(decision_type, PolicyDecisionType::NavigationAction | PolicyDecisionType::NewWindowAction) {
                if let Some(nav_decision) = decision.downcast_ref::<NavigationPolicyDecision>() {
                    if let Some(action) = nav_decision.navigation_action() {
                        if let Some(request) = action.request() {
                            if let Some(uri) = request.uri() {
                                let uri_str = uri.as_str();
                                if !uri_str.starts_with(&local_origin) && !uri_str.starts_with("about:") {
                                    let _ = std::process::Command::new("xdg-open")
                                        .arg(uri_str)
                                        .spawn();
                                    decision.ignore();
                                    return true;
                                }
                            }
                        }
                    }
                }
            }
            false
        });

        // Load HTML directly, stripping the JS seed-polling scripts
        // since we handle reload from the Rust side.
        let initial_html = std::fs::read_to_string(&html_path).unwrap_or_default();
        let clean_html = strip_seed_scripts(&initial_html);
        let base_uri = format!("http://localhost:{}/", port);
        webview.load_html(&clean_html, Some(&base_uri));

        // Extract initial TOC
        let infile_path = infile.clone();
        let initial_toc = if let Ok(md_content) = std::fs::read_to_string(&infile_path) {
            let (_html, toc) = crate::markdown::md_to_html_body_with_toc(&md_content, show_frontmatter);
            toc
        } else {
            Vec::new()
        };

        // Build window based on toc_mode
        let toc_store = create_toc_store();
        populate_toc(&toc_store, &initial_toc);
        let treeview = create_toc_view(&toc_store);
        let toc_scrolled = ScrolledWindow::builder()
            .child(&treeview)
            .vexpand(true)
            .build();

        // Expand all tree nodes by default
        treeview.expand_all();

        let stack: Option<Stack> = if toc_mode == "zathura" {
            let s = Stack::new();
            s.add_named(&webview, Some("document"));
            s.add_named(&toc_scrolled, Some("toc"));
            s.set_visible_child_name("document");
            Some(s)
        } else {
            None
        };

        let window_ref = match toc_mode.as_str() {
            "side" => {
                let paned = Paned::new(Orientation::Horizontal);
                paned.set_start_child(Some(&toc_scrolled));
                paned.set_end_child(Some(&webview));
                paned.set_position(250);
                paned.set_shrink_start_child(false);
                paned.set_shrink_end_child(false);
                ApplicationWindow::builder()
                    .application(app)
                    .title("MiP")
                    .default_width(800)
                    .default_height(600)
                    .child(&paned)
                    .build()
            }
            "zathura" => {
                ApplicationWindow::builder()
                    .application(app)
                    .title("MiP")
                    .default_width(800)
                    .default_height(600)
                    .child(stack.as_ref().unwrap())
                    .build()
            }
            _ => {
                ApplicationWindow::builder()
                    .application(app)
                    .title("MiP")
                    .default_width(800)
                    .default_height(600)
                    .child(&webview)
                    .build()
            }
        };

        window_ref.present();

        // Connect TOC row-activated → scroll to heading
        {
            let webview_for_activate = webview.clone();
            let stack_for_activate = stack.clone();
            let toc_mode_for_activate = toc_mode.clone();
            treeview.connect_row_activated(move |tv, path, _col| {
                let Some(model) = tv.model() else { return };
                let Some(iter) = model.iter(path) else { return };
                let anchor_id: String = model.get(&iter, COL_ANCHOR as i32);
                scroll_to_anchor(&webview_for_activate, &anchor_id);
                // In zathura mode, switch back to document view
                if toc_mode_for_activate == "zathura" {
                    if let Some(ref s) = stack_for_activate {
                        s.set_visible_child_name("document");
                    }
                }
            });
        }

        // Keyboard handling for zathura mode and vim navigation
        if toc_mode == "zathura" {
            // Tab on webview → show TOC
            // Must use capture phase because WebKitGTK consumes Tab internally
            let stack_for_tab = stack.clone();
            let treeview_for_tab = treeview.clone();
            let key_controller_wv = gtk4::EventControllerKey::new();
            key_controller_wv.set_propagation_phase(gtk4::PropagationPhase::Capture);
            key_controller_wv.connect_key_pressed(move |_, keyval, _keycode, _state| {
                if keyval == gtk4::gdk::Key::Tab {
                    if let Some(ref s) = stack_for_tab {
                        if s.visible_child_name().as_deref() == Some("document") {
                            s.set_visible_child_name("toc");
                            treeview_for_tab.grab_focus();
                            return glib::Propagation::Stop;
                        }
                    }
                }
                glib::Propagation::Proceed
            });
            window_ref.add_controller(key_controller_wv);
        }

        // Key handler on TreeView: j/k navigation, Esc to close, Enter/Tab to activate
        {
            let webview_for_keys = webview.clone();
            let stack_for_keys = stack.clone();
            let toc_mode_for_keys = toc_mode.clone();
            let key_controller_tv = gtk4::EventControllerKey::new();
            let treeview_for_keys = treeview.clone();
            key_controller_tv.connect_key_pressed(move |_, keyval, _keycode, _state| {
                match keyval {
                    v if v == gtk4::gdk::Key::j => {
                        // Move cursor down
                        if let (Some(path), _) = TreeViewExt::cursor(&treeview_for_keys) {
                            let mut next = path;
                            // Try to go to first child, or next sibling, or parent's next sibling
                            if treeview_for_keys.row_expanded(&next) {
                                next.append_index(0);
                            } else {
                                next.next();
                            }
                            TreeViewExt::set_cursor(&treeview_for_keys,&next, None::<&gtk4::TreeViewColumn>, false);
                        }
                        glib::Propagation::Stop
                    }
                    v if v == gtk4::gdk::Key::k => {
                        // Move cursor up
                        if let (Some(path), _) = TreeViewExt::cursor(&treeview_for_keys) {
                            let mut prev = path;
                            if !prev.prev() {
                                if prev.up() && prev.depth() > 0 {
                                    TreeViewExt::set_cursor(&treeview_for_keys,&prev, None::<&gtk4::TreeViewColumn>, false);
                                }
                            } else {
                                TreeViewExt::set_cursor(&treeview_for_keys,&prev, None::<&gtk4::TreeViewColumn>, false);
                            }
                        }
                        glib::Propagation::Stop
                    }
                    v if v == gtk4::gdk::Key::Return => {
                        // Activate current row
                        if let (Some(path), col) = TreeViewExt::cursor(&treeview_for_keys) {
                            treeview_for_keys.row_activated(&path, col.as_ref());
                        }
                        glib::Propagation::Stop
                    }
                    v if v == gtk4::gdk::Key::Tab && toc_mode_for_keys == "zathura" => {
                        // Activate current row (same as Enter in zathura)
                        if let (Some(path), col) = TreeViewExt::cursor(&treeview_for_keys) {
                            treeview_for_keys.row_activated(&path, col.as_ref());
                        }
                        glib::Propagation::Stop
                    }
                    v if v == gtk4::gdk::Key::Escape && toc_mode_for_keys == "zathura" => {
                        // Close TOC without navigating
                        if let Some(ref s) = stack_for_keys {
                            s.set_visible_child_name("document");
                            webview_for_keys.grab_focus();
                        }
                        glib::Propagation::Stop
                    }
                    _ => glib::Propagation::Proceed,
                }
            });
            treeview.add_controller(key_controller_tv);
        }

        // Poll seed file and update page content via JS injection
        // to avoid the flicker of a full load_html() call.
        let seed_path = seed_path.clone();
        let mut last_seed = std::fs::read_to_string(&seed_path).unwrap_or_default();
        let mut last_system_dark = if is_system_theme { Some(crate::is_system_dark()) } else { None };
        let mut last_toc: Vec<TocEntry> = initial_toc;
        let mut last_html_body = String::new();

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
                        let (html_body, toc_entries) = crate::markdown::md_to_html_body_with_toc(&md_content, show_frontmatter);

                        // Only update WebView if content actually changed
                        if html_body != last_html_body {
                            let escaped = html_body
                                .replace('\\', "\\\\")
                                .replace('`', "\\`")
                                .replace("${", "\\${");
                            let js = format!(
                                "document.querySelector('.section').innerHTML = `{}`;",
                                escaped
                            );
                            webview.evaluate_javascript(&js, None, None, None::<&gtk4::gio::Cancellable>, |_| {});
                            last_html_body = html_body;
                        }

                        // Only rebuild TOC if headings changed
                        if toc_entries != last_toc {
                            populate_toc(&toc_store, &toc_entries);
                            treeview.expand_all();
                            last_toc = toc_entries;
                        }
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

    // Note: populate_toc() cannot be unit-tested here because GTK
    // TreeStore requires initialization on the main thread, which the
    // test harness doesn't guarantee. The tree-building logic is
    // verified indirectly through md_to_html_body_with_toc tests in
    // markdown.rs (TocEntry extraction with hierarchy/skipped levels)
    // and through manual verification (task group 8).
}
