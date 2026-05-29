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

fn expand_tilde(path: &str) -> String {
    if path.starts_with('~') {
        let home = std::env::var("HOME").unwrap_or_default();
        format!("{}{}", home, &path[1..])
    } else {
        path.to_string()
    }
}

fn execute_command(cmd: &str, arg: &str, app: &Application) {
    match cmd {
        "q" | "close" => {
            app.quit();
        }
        "open" | "o" => {
            if !arg.is_empty() {
                let path = expand_tilde(arg);
                let path = std::path::Path::new(&path);
                if path.exists() {
                    // For now, open in a new mip process
                    let _ = std::process::Command::new(std::env::current_exe().unwrap())
                        .arg(path)
                        .spawn();
                    app.quit();
                }
            }
        }
        _ => {} // Unknown command — silently ignore
    }
}

fn complete_path(
    current_arg: &str,
    cmd_prefix: &str,
    entry: &gtk4::Entry,
    matches: &std::rc::Rc<std::cell::RefCell<Vec<String>>>,
    index: &std::rc::Rc<std::cell::Cell<usize>>,
    prefix: &std::rc::Rc<std::cell::RefCell<String>>,
) {
    let expanded = expand_tilde(current_arg);

    // If matches are empty or prefix changed, rebuild match list
    if matches.borrow().is_empty() || *prefix.borrow() != current_arg {
        *prefix.borrow_mut() = current_arg.to_string();
        index.set(0);

        let (dir, file_prefix) = if expanded.ends_with('/') || expanded.is_empty() {
            (expanded.as_str().to_string(), "".to_string())
        } else {
            let p = std::path::Path::new(&expanded);
            let dir = p.parent().map_or(".", |d| if d.as_os_str().is_empty() { "." } else { d.to_str().unwrap_or(".") }).to_string();
            let file = p.file_name().map_or("", |f| f.to_str().unwrap_or("")).to_string();
            (dir, file)
        };

        let mut found = Vec::new();
        if let Ok(entries) = std::fs::read_dir(&dir) {
            for entry_result in entries.flatten() {
                let name = entry_result.file_name().to_string_lossy().to_string();
                if name.starts_with(&file_prefix) {
                    let full = if dir == "." {
                        name.clone()
                    } else {
                        format!("{}/{}", dir.trim_end_matches('/'), name)
                    };
                    // Add trailing / for directories
                    let full = if entry_result.path().is_dir() {
                        format!("{}/", full)
                    } else {
                        full
                    };
                    found.push(full);
                }
            }
        }
        found.sort();
        *matches.borrow_mut() = found;
    } else {
        // Cycle to next match
        let next = (index.get() + 1) % matches.borrow().len().max(1);
        index.set(next);
    }

    let matches_ref = matches.borrow();
    if let Some(completion) = matches_ref.get(index.get()) {
        // Convert back to use ~ if original used ~
        let display_path = if current_arg.starts_with('~') {
            let home = std::env::var("HOME").unwrap_or_default();
            if completion.starts_with(&home) {
                completion.replacen(&home, "~", 1)
            } else {
                completion.clone()
            }
        } else {
            completion.clone()
        };
        let new_text = format!("{}{}", cmd_prefix, display_path);
        entry.set_text(&new_text);
        entry.set_position(new_text.len() as i32);
    }
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

        // Command bar (hidden by default)
        let cmd_entry = gtk4::Entry::new();
        cmd_entry.set_visible(false);
        cmd_entry.add_css_class("command-bar");
        cmd_entry.set_has_frame(false);

        // Apply styling: monospace, grey background, no borders, no focus ring
        let css_provider = gtk4::CssProvider::new();
        css_provider.load_from_data(
            "entry.command-bar { font-family: monospace; padding: 4px 8px; background: #e8e8e8; border: none; border-radius: 0; outline: none; box-shadow: none; } \
             entry.command-bar:focus { outline: none; box-shadow: none; border: none; }"
        );
        gtk4::style_context_add_provider_for_display(
            &gtk4::gdk::Display::default().unwrap(),
            &css_provider,
            gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );

        // Wrap window content + command bar in a vertical box
        // First, take the existing child out and put it in the box
        let content_widget = window_ref.child().unwrap();
        window_ref.set_child(None::<&gtk4::Widget>);
        let outer_box = gtk4::Box::new(gtk4::Orientation::Vertical, 0);
        content_widget.set_vexpand(true);
        outer_box.append(&content_widget);
        outer_box.append(&cmd_entry);
        window_ref.set_child(Some(&outer_box));

        window_ref.present();

        // Command bar: re-grab focus if it loses it while visible (modal behavior)
        {
            let cmd_entry_for_focus = cmd_entry.clone();
            let focus_controller = gtk4::EventControllerFocus::new();
            focus_controller.connect_leave(move |_| {
                if gtk4::prelude::WidgetExt::is_visible(&cmd_entry_for_focus) {
                    // Re-grab focus on next idle tick
                    let entry = cmd_entry_for_focus.clone();
                    glib::idle_add_local_once(move || {
                        if gtk4::prelude::WidgetExt::is_visible(&entry) {
                            entry.grab_focus();
                        }
                    });
                }
            });
            cmd_entry.add_controller(focus_controller);
        }

        // Command bar: `:` on window shows it (capture phase so it fires before children)
        {
            let cmd_entry_for_colon = cmd_entry.clone();
            let key_controller_cmd = gtk4::EventControllerKey::new();
            key_controller_cmd.set_propagation_phase(gtk4::PropagationPhase::Capture);
            key_controller_cmd.connect_key_pressed(move |_, keyval, _keycode, _state| {
                // Only activate if the command bar isn't already visible
                if keyval == gtk4::gdk::Key::colon && !gtk4::prelude::WidgetExt::is_visible(&cmd_entry_for_colon) {
                    cmd_entry_for_colon.set_text(":");
                    cmd_entry_for_colon.set_visible(true);
                    cmd_entry_for_colon.grab_focus();
                    // Set cursor after colon on next idle tick (after GTK's select-all-on-focus)
                    let entry = cmd_entry_for_colon.clone();
                    glib::idle_add_local_once(move || {
                        entry.select_region(1, 1); // deselect all, cursor at pos 1
                        entry.set_position(1);
                    });
                    return glib::Propagation::Stop;
                }
                glib::Propagation::Proceed
            });
            window_ref.add_controller(key_controller_cmd);
        }

        // Command bar: Enter executes (use activate signal — more reliable than key handler)
        {
            let cmd_entry_for_activate = cmd_entry.clone();
            let webview_for_activate = webview.clone();
            let app_for_activate = app.clone();
            cmd_entry.connect_activate(move |entry| {
                let text = entry.text().to_string();
                cmd_entry_for_activate.set_text("");
                cmd_entry_for_activate.set_visible(false);
                webview_for_activate.grab_focus();
                // Parse and execute command (strip leading :)
                let text = text.strip_prefix(':').unwrap_or(&text);
                let mut parts = text.splitn(2, char::is_whitespace);
                let cmd = parts.next().unwrap_or("").trim();
                let arg = parts.next().unwrap_or("").trim();
                execute_command(cmd, arg, &app_for_activate);
            });
        }

        // Command bar: Escape dismisses, Tab completes (capture phase to intercept before other handlers)
        {
            let cmd_entry_for_keys = cmd_entry.clone();
            let webview_for_cmd = webview.clone();
            let key_controller_entry = gtk4::EventControllerKey::new();
            key_controller_entry.set_propagation_phase(gtk4::PropagationPhase::Capture);
            let tab_matches: std::rc::Rc<std::cell::RefCell<Vec<String>>> = std::rc::Rc::new(std::cell::RefCell::new(Vec::new()));
            let tab_index: std::rc::Rc<std::cell::Cell<usize>> = std::rc::Rc::new(std::cell::Cell::new(0));
            let tab_prefix: std::rc::Rc<std::cell::RefCell<String>> = std::rc::Rc::new(std::cell::RefCell::new(String::new()));

            let tab_matches_clone = tab_matches.clone();
            let tab_index_clone = tab_index.clone();
            let tab_prefix_clone = tab_prefix.clone();

            key_controller_entry.connect_key_pressed(move |_, keyval, _keycode, _state| {
                match keyval {
                    v if v == gtk4::gdk::Key::Escape => {
                        cmd_entry_for_keys.set_text("");
                        cmd_entry_for_keys.set_visible(false);
                        webview_for_cmd.grab_focus();
                        tab_matches_clone.borrow_mut().clear();
                        tab_index_clone.set(0);
                        glib::Propagation::Stop
                    }
                    v if v == gtk4::gdk::Key::BackSpace => {
                        let text = cmd_entry_for_keys.text().to_string();
                        if text == ":" {
                            // Backspace on just ":" closes command bar (like vim)
                            cmd_entry_for_keys.set_text("");
                            cmd_entry_for_keys.set_visible(false);
                            webview_for_cmd.grab_focus();
                            tab_matches_clone.borrow_mut().clear();
                            tab_index_clone.set(0);
                            glib::Propagation::Stop
                        } else if text.len() > 1 && cmd_entry_for_keys.position() <= 1 {
                            // Don't allow deleting the colon when there's text after it
                            glib::Propagation::Stop
                        } else {
                            tab_matches_clone.borrow_mut().clear();
                            tab_index_clone.set(0);
                            glib::Propagation::Proceed
                        }
                    }
                    v if v == gtk4::gdk::Key::Tab => {
                        let text = cmd_entry_for_keys.text().to_string();
                        let text_stripped = text.strip_prefix(':').unwrap_or(&text);
                        // Only complete for open/o commands
                        if let Some(path_arg) = text_stripped.strip_prefix("open ").or_else(|| text_stripped.strip_prefix("o ")) {
                            let cmd_prefix = if text_stripped.starts_with("open ") { ":open " } else { ":o " };
                            complete_path(path_arg, cmd_prefix, &cmd_entry_for_keys, &tab_matches_clone, &tab_index_clone, &tab_prefix_clone);
                        }
                        glib::Propagation::Stop
                    }
                    v if v == gtk4::gdk::Key::Home || (v == gtk4::gdk::Key::Left && cmd_entry_for_keys.position() <= 1) => {
                        // Don't allow cursor before the colon
                        glib::Propagation::Stop
                    }
                    _ => {
                        // Any non-Tab key resets tab completion state
                        tab_matches_clone.borrow_mut().clear();
                        tab_index_clone.set(0);
                        glib::Propagation::Proceed
                    }
                }
            });
            cmd_entry.add_controller(key_controller_entry);
        }

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
            let cmd_entry_for_zathura = cmd_entry.clone();
            let key_controller_wv = gtk4::EventControllerKey::new();
            key_controller_wv.set_propagation_phase(gtk4::PropagationPhase::Capture);
            key_controller_wv.connect_key_pressed(move |_, keyval, _keycode, _state| {
                // Don't intercept when command bar is open
                if gtk4::prelude::WidgetExt::is_visible(&cmd_entry_for_zathura) {
                    return glib::Propagation::Proceed;
                }
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
