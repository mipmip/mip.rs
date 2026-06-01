use std::path::PathBuf;
use std::net::TcpStream;

use gtk4::prelude::*;
use gtk4::prelude::TreeViewExt;
use gtk4::glib::translate::IntoGlib;
use gtk4::{Application, ApplicationWindow, ScrolledWindow, Paned, Orientation, Stack};
use gtk4::glib;
use webkit6::prelude::*;
use webkit6::{WebView, NavigationPolicyDecision, PolicyDecisionType, PrintOperation};

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

/// Post-process captured DOM HTML for standalone export.
/// Strips all script tags, localhost link tags, and the header div.
/// Ensures DOCTYPE is present at the top.
fn post_process_export(html: &str) -> String {
    let mut result = String::with_capacity(html.len() + 20);

    // Ensure DOCTYPE is present
    if !html.trim_start().starts_with("<!DOCTYPE") && !html.trim_start().starts_with("<!doctype") {
        result.push_str("<!DOCTYPE html>\n");
    }

    // Process the HTML character by character to strip unwanted elements
    let bytes = html.as_bytes();
    let len = bytes.len();
    let mut i = 0;
    while i < len {
        // Check for tags we want to strip
        if bytes[i] == b'<' {
            // Strip <script...>...</script>
            if html[i..].starts_with("<script") {
                if let Some(end) = html[i..].find("</script>") {
                    i += end + 9; // skip past </script>
                    continue;
                }
            }
            // Strip <link ...> referencing localhost
            if html[i..].starts_with("<link ") || html[i..].starts_with("<link\n") || html[i..].starts_with("<link\t") {
                // Find the end of this tag
                if let Some(end) = html[i..].find('>') {
                    let tag = &html[i..i + end + 1];
                    if tag.contains("localhost") || tag.contains("127.0.0.1") {
                        i += end + 1;
                        continue;
                    }
                }
            }
            // Strip <div id="header">...</div>
            if html[i..].starts_with("<div id=\"header\"") {
                if let Some(end) = html[i..].find("</div>") {
                    i += end + 6; // skip past </div>
                    continue;
                }
            }
        }
        result.push(html.as_bytes()[i] as char);
        i += 1;
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

const SIDETOC_WIDTH_STEP: i32 = 50;

pub struct RuntimeSettings {
    pub frontmatter: std::cell::Cell<bool>,
    pub paragraph_numbers: std::cell::Cell<bool>,
    pub paragraph_numbers_start: std::cell::Cell<u8>,
    pub theme: std::cell::RefCell<String>,
    pub force_render: std::cell::Cell<bool>,
    pub infile: std::cell::RefCell<String>,
    pub filename: std::cell::RefCell<String>,
    pub math: std::cell::Cell<bool>,
}

struct CommandContext {
    app: Application,
    window: ApplicationWindow,
    paned: Paned,
    stack: Stack,
    webview: WebView,
    sidetoc_treeview: gtk4::TreeView,
    sidetoc_right: bool,
    sidetoc_width: i32,
    sidetoc_open: std::cell::Cell<bool>,
    settings: RuntimeSettings,
    watcher_tx: std::sync::mpsc::Sender<PathBuf>,
    temp_dir: PathBuf,
    port: u16,
}

fn execute_command(cmd: &str, arg: &str, ctx: &CommandContext) {
    match cmd {
        "q" | "close" => {
            ctx.app.quit();
        }
        "open" | "o" => {
            if !arg.is_empty() {
                let path = crate::command::expand_tilde(arg);
                let path_ref = std::path::Path::new(&path);
                if path_ref.exists() {
                    let new_infile = path_ref.canonicalize()
                        .unwrap_or_else(|_| path_ref.to_path_buf())
                        .to_string_lossy()
                        .to_string();
                    let new_filename = path_ref.file_name()
                        .map(|f| f.to_string_lossy().to_string())
                        .unwrap_or_else(|| "MiP".to_string());

                    // Update infile and filename in settings
                    *ctx.settings.infile.borrow_mut() = new_infile.clone();
                    *ctx.settings.filename.borrow_mut() = new_filename.clone();

                    // Update docroot symlink for server
                    if let Some(parent) = path_ref.canonicalize().ok().and_then(|p| p.parent().map(|p| p.to_path_buf())) {
                        let docroot = ctx.temp_dir.join("docroot");
                        let _ = std::fs::remove_file(&docroot);
                        let _ = std::os::unix::fs::symlink(&parent, &docroot);
                    }

                    // Send new path to watcher thread
                    let _ = ctx.watcher_tx.send(PathBuf::from(&new_infile));

                    // Re-render immediately
                    let sf = ctx.settings.frontmatter.get();
                    let theme_str = ctx.settings.theme.borrow().clone();
                    let theme_class = match theme_str.as_str() {
                        "light" => "light",
                        "dark" => "dark",
                        _ => if crate::is_system_dark() { "dark" } else { "light" },
                    };
                    crate::markdown::to_html(&new_infile, &ctx.temp_dir, ctx.port, sf, theme_class, ctx.settings.math.get());

                    // Update window title
                    let window_title = format!("{} - MiP", new_filename);
                    ctx.window.set_title(Some(&window_title));

                    // Force re-render in poll loop
                    ctx.settings.force_render.set(true);
                }
            }
        }
        "sidetoc_open" => {
            if ctx.sidetoc_right {
                let total = ctx.window.width();
                ctx.paned.set_position(total - ctx.sidetoc_width);
            } else {
                ctx.paned.set_position(ctx.sidetoc_width);
            }
            ctx.sidetoc_treeview.grab_focus();
            ctx.sidetoc_open.set(true);
        }
        "document_focus" => {
            ctx.webview.grab_focus();
        }
        "sidetoc_focus" => {
            if ctx.sidetoc_open.get() {
                ctx.sidetoc_treeview.grab_focus();
            }
        }
        "sidetoc_close" => {
            if ctx.sidetoc_right {
                ctx.paned.set_position(ctx.window.width());
            } else {
                ctx.paned.set_position(0);
            }
            ctx.webview.grab_focus();
            ctx.sidetoc_open.set(false);
        }
        "sidetoc_toggle" => {
            if ctx.sidetoc_open.get() {
                execute_command("sidetoc_close", "", ctx);
            } else {
                execute_command("sidetoc_open", "", ctx);
            }
        }
        "sidetoc_expand_width" => {
            let pos = ctx.paned.position();
            if ctx.sidetoc_right {
                ctx.paned.set_position((pos - SIDETOC_WIDTH_STEP).max(0));
            } else {
                ctx.paned.set_position(pos + SIDETOC_WIDTH_STEP);
            }
        }
        "sidetoc_shrink_width" => {
            let pos = ctx.paned.position();
            if ctx.sidetoc_right {
                ctx.paned.set_position(pos + SIDETOC_WIDTH_STEP);
            } else {
                ctx.paned.set_position((pos - SIDETOC_WIDTH_STEP).max(0));
            }
        }
        "export_html" => {
            if arg.is_empty() {
                eprintln!("warning: export_html requires a file path");
                return;
            }
            let path = crate::command::expand_tilde(arg);
            let path_clone = path.clone();
            ctx.webview.evaluate_javascript(
                "document.documentElement.outerHTML",
                None, None, None::<&gtk4::gio::Cancellable>,
                move |result| {
                    match result {
                        Ok(value) => {
                            let html = value.to_string();
                            if html.is_empty() {
                                eprintln!("warning: export_html got empty DOM result, not writing file");
                                return;
                            }
                            let processed = post_process_export(&html);
                            let out = std::path::Path::new(&path_clone);
                            if let Some(parent) = out.parent() {
                                if let Err(e) = std::fs::create_dir_all(parent) {
                                    eprintln!("warning: export_html could not create directories: {}", e);
                                    return;
                                }
                            }
                            if let Err(e) = std::fs::write(out, processed) {
                                eprintln!("warning: export_html failed to write file: {}", e);
                            }
                        }
                        Err(e) => {
                            eprintln!("warning: export_html JS evaluation error: {}", e);
                        }
                    }
                },
            );
        }
        "print" => {
            let print_op = PrintOperation::new(&ctx.webview);
            print_op.run_dialog(Some(&ctx.window));
        }
        "set" => {
            let mut parts = arg.splitn(2, char::is_whitespace);
            let setting = parts.next().unwrap_or("").trim();
            let value = parts.next().unwrap_or("").trim();
            match setting {
                "frontmatter" => {
                    match value {
                        "true" | "1" | "on" => ctx.settings.frontmatter.set(true),
                        "false" | "0" | "off" => ctx.settings.frontmatter.set(false),
                        _ => { eprintln!("warning: invalid value '{}' for frontmatter (expected true/false)", value); return; }
                    }
                    ctx.settings.force_render.set(true);
                }
                "paragraph_numbers" => {
                    match value {
                        "true" | "1" | "on" => ctx.settings.paragraph_numbers.set(true),
                        "false" | "0" | "off" => ctx.settings.paragraph_numbers.set(false),
                        _ => { eprintln!("warning: invalid value '{}' for paragraph_numbers (expected true/false)", value); return; }
                    }
                    ctx.settings.force_render.set(true);
                }
                "paragraph_numbers_start" => {
                    if let Ok(n) = value.parse::<u8>() {
                        ctx.settings.paragraph_numbers_start.set(n.clamp(1, 6));
                        ctx.settings.force_render.set(true);
                    } else {
                        eprintln!("warning: invalid value '{}' for paragraph_numbers_start (expected 1-6)", value);
                    }
                }
                "theme" => {
                    if ["system", "light", "dark"].contains(&value) {
                        *ctx.settings.theme.borrow_mut() = value.to_string();
                        // Inject CSS class change immediately
                        let class = match value {
                            "light" => "light",
                            "dark" => "dark",
                            _ => if crate::is_system_dark() { "dark" } else { "light" },
                        };
                        let js = format!("document.documentElement.className = '{}';", class);
                        ctx.webview.evaluate_javascript(&js, None, None, None::<&gtk4::gio::Cancellable>, |_| {});
                        ctx.settings.force_render.set(true);
                    } else {
                        eprintln!("warning: invalid value '{}' for theme (expected system/light/dark)", value);
                    }
                }
                _ => {}
            }
        }
        "quicktoc" => {
            if ctx.stack.visible_child_name().as_deref() == Some("document") {
                ctx.stack.set_visible_child_name("toc");
            } else {
                ctx.stack.set_visible_child_name("document");
                ctx.webview.grab_focus();
            }
        }
        "zoom_in" => {
            let level = (ctx.webview.zoom_level() + 0.1).min(5.0);
            ctx.webview.set_zoom_level(level);
        }
        "zoom_out" => {
            let level = (ctx.webview.zoom_level() - 0.1).max(0.3);
            ctx.webview.set_zoom_level(level);
        }
        "zoom_reset" => {
            ctx.webview.set_zoom_level(1.0);
        }
        "scroll_down" => {
            ctx.webview.evaluate_javascript("window.scrollBy(0, 60)", None, None, None::<&gtk4::gio::Cancellable>, |_| {});
        }
        "scroll_up" => {
            ctx.webview.evaluate_javascript("window.scrollBy(0, -60)", None, None, None::<&gtk4::gio::Cancellable>, |_| {});
        }
        "scroll_page_down" => {
            ctx.webview.evaluate_javascript("window.scrollBy(0, window.innerHeight)", None, None, None::<&gtk4::gio::Cancellable>, |_| {});
        }
        "scroll_page_up" => {
            ctx.webview.evaluate_javascript("window.scrollBy(0, -window.innerHeight)", None, None, None::<&gtk4::gio::Cancellable>, |_| {});
        }
        "scroll_half_down" => {
            ctx.webview.evaluate_javascript("window.scrollBy(0, window.innerHeight/2)", None, None, None::<&gtk4::gio::Cancellable>, |_| {});
        }
        "scroll_half_up" => {
            ctx.webview.evaluate_javascript("window.scrollBy(0, -window.innerHeight/2)", None, None, None::<&gtk4::gio::Cancellable>, |_| {});
        }
        "scroll_top" => {
            ctx.webview.evaluate_javascript("window.scrollTo(0, 0)", None, None, None::<&gtk4::gio::Cancellable>, |_| {});
        }
        "scroll_bottom" => {
            ctx.webview.evaluate_javascript("window.scrollTo(0, document.body.scrollHeight)", None, None, None::<&gtk4::gio::Cancellable>, |_| {});
        }
        "scroll_next_heading" => {
            ctx.webview.evaluate_javascript(
                "(function(){var h=document.querySelectorAll('h1[id],h2[id],h3[id],h4[id],h5[id],h6[id]');var y=window.scrollY+10;for(var i=0;i<h.length;i++){if(h[i].offsetTop>y){h[i].scrollIntoView({behavior:'instant'});return;}}})()",
                None, None, None::<&gtk4::gio::Cancellable>, |_| {},
            );
        }
        "scroll_prev_heading" => {
            ctx.webview.evaluate_javascript(
                "(function(){var h=document.querySelectorAll('h1[id],h2[id],h3[id],h4[id],h5[id],h6[id]');var y=window.scrollY-10;for(var i=h.length-1;i>=0;i--){if(h[i].offsetTop<y){h[i].scrollIntoView({behavior:'instant'});return;}}})()",
                None, None, None::<&gtk4::gio::Cancellable>, |_| {},
            );
        }
        _ => {}
    }
}

fn execute_commands(text: &str, ctx: &CommandContext) {
    for part in crate::command::split_commands(text) {
        let (cmd, arg) = crate::command::parse_command(&part);
        execute_command(cmd, arg, ctx);
    }
}

/// Handle Tab/Shift+Tab completion for both command names and paths.
/// Updates entry text, match state, wildmenu label, and index.
fn handle_tab_completion(
    entry: &gtk4::Entry,
    wildmenu: &gtk4::Label,
    matches: &std::rc::Rc<std::cell::RefCell<Vec<String>>>,
    index: &std::rc::Rc<std::cell::Cell<usize>>,
    prefix: &std::rc::Rc<std::cell::RefCell<String>>,
    reverse: bool,
) {
    let text = entry.text().to_string();
    let text_stripped = text.strip_prefix(':').unwrap_or(&text);

    // Determine if we're completing a command name or a path
    if !text_stripped.contains(' ') {
        // Command name completion
        let current_prefix = text_stripped;
        if matches.borrow().is_empty() {
            *prefix.borrow_mut() = current_prefix.to_string();
            let found = crate::command::match_commands(current_prefix);
            *matches.borrow_mut() = found;
            index.set(0);
        } else {
            cycle_index(index, matches.borrow().len(), reverse);
        }

        let matches_ref = matches.borrow();
        if let Some(completion) = matches_ref.get(index.get()) {
            if matches_ref.len() == 1 {
                // Single match: complete with trailing space, hide wildmenu
                let new_text = format!(":{} ", completion);
                entry.set_text(&new_text);
                entry.set_position(new_text.len() as i32);
                wildmenu.set_visible(false);
            } else {
                // Multiple matches: complete to current, show wildmenu
                let new_text = format!(":{}", completion);
                entry.set_text(&new_text);
                entry.set_position(new_text.len() as i32);
                wildmenu.set_markup(&crate::command::wildmenu_markup(&matches_ref, index.get(), 10));
                wildmenu.set_visible(true);
            }
        }
    } else if let Some(setting_prefix) = text_stripped.strip_prefix("set ") {
        // Setting name completion for :set
        // Only complete the setting name (no space in the setting prefix yet)
        let setting_prefix = setting_prefix.trim();
        if !setting_prefix.contains(' ') {
            if matches.borrow().is_empty() {
                *prefix.borrow_mut() = setting_prefix.to_string();
                let found = crate::command::match_settings(setting_prefix);
                *matches.borrow_mut() = found;
                index.set(0);
            } else {
                cycle_index(index, matches.borrow().len(), reverse);
            }

            let matches_ref = matches.borrow();
            if let Some(completion) = matches_ref.get(index.get()) {
                if matches_ref.len() == 1 {
                    let new_text = format!(":set {} ", completion);
                    entry.set_text(&new_text);
                    entry.set_position(new_text.len() as i32);
                    wildmenu.set_visible(false);
                } else {
                    let new_text = format!(":set {}", completion);
                    entry.set_text(&new_text);
                    entry.set_position(new_text.len() as i32);
                    wildmenu.set_markup(&crate::command::wildmenu_markup(&matches_ref, index.get(), 10));
                    wildmenu.set_visible(true);
                }
            }
        }
    } else {
        // Path completion for open/o commands
        if let Some(path_arg) = text_stripped.strip_prefix("open ").or_else(|| text_stripped.strip_prefix("o ")) {
            let cmd_prefix = if text_stripped.starts_with("open ") { ":open " } else { ":o " };
            let used_tilde = path_arg.starts_with('~');

            if matches.borrow().is_empty() {
                // First Tab: build match list from what the user typed
                *prefix.borrow_mut() = path_arg.to_string();
                let found = crate::command::match_paths(path_arg);
                *matches.borrow_mut() = found;
                index.set(0);
            } else {
                // Subsequent Tabs: cycle through existing matches
                cycle_index(index, matches.borrow().len(), reverse);
            }

            let matches_ref = matches.borrow();
            if let Some(completion) = matches_ref.get(index.get()) {
                let display_path = crate::command::unexpand_tilde(completion, used_tilde);
                let new_text = format!("{}{}", cmd_prefix, display_path);
                entry.set_text(&new_text);
                entry.set_position(new_text.len() as i32);

                if matches_ref.len() > 1 {
                    wildmenu.set_markup(&crate::command::wildmenu_markup(&matches_ref, index.get(), 10));
                    wildmenu.set_visible(true);
                } else {
                    wildmenu.set_visible(false);
                }
            }
        }
    }
}

fn cycle_index(index: &std::rc::Rc<std::cell::Cell<usize>>, len: usize, reverse: bool) {
    if len == 0 { return; }
    if reverse {
        index.set(if index.get() == 0 { len - 1 } else { index.get() - 1 });
    } else {
        index.set((index.get() + 1) % len);
    }
}

pub fn window(port: u16, temp_dir: PathBuf, show_frontmatter: bool, theme_mode: &str, infile: &str, runcmd: Option<&str>, sidetoc_width: u32, sidetoc_position: &str, keybinding_registry: crate::command::KeybindingRegistry, paragraph_numbers: bool, paragraph_numbers_start: u8, history_size: usize, watcher_tx: std::sync::mpsc::Sender<PathBuf>, math: bool) {
    let theme_mode = theme_mode.to_string();
    let infile = infile.to_string();
    let runcmd = runcmd.map(|s| s.to_string());
    let keybinding_registry = std::rc::Rc::new(keybinding_registry);
    let sidetoc_width = sidetoc_width as i32;
    let sidetoc_right = sidetoc_position == "right";
    let history_path = crate::history::history_path();
    let history = std::rc::Rc::new(std::cell::RefCell::new(
        crate::history::CommandHistory::load(&history_path, history_size),
    ));
    let _ = gtk4::init();
    gtk4::Window::set_default_icon_name("mip");
    let app = Application::builder()
        .application_id("org.mipmip.mip")
        .build();

    let html_path = temp_dir.join(".temp.html");
    let seed_path = temp_dir.join(".temp.seed");
    let history_for_shutdown = history.clone();
    let history_path_for_shutdown = history_path.clone();
    let temp_dir_cleanup = temp_dir.clone();

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
        let (initial_toc, initial_title) = if let Ok(md_content) = std::fs::read_to_string(&infile_path) {
            let (_html, toc, title) = crate::markdown::md_to_html_body_with_toc(&md_content, show_frontmatter, paragraph_numbers, paragraph_numbers_start, math);
            (toc, title)
        } else {
            (Vec::new(), None)
        };

        // Build TOC widgets (always created, hidden by default)
        let toc_store = create_toc_store();
        populate_toc(&toc_store, &initial_toc);
        let treeview = create_toc_view(&toc_store);
        let toc_scrolled = ScrolledWindow::builder()
            .child(&treeview)
            .vexpand(true)
            .build();
        treeview.expand_all();

        // Quicktoc: Stack toggles between document and TOC
        let stack = Stack::new();
        stack.add_named(&webview, Some("document"));
        // Quicktoc uses a separate ScrolledWindow wrapping the same treeview model
        let quicktoc_treeview = create_toc_view(&toc_store);
        quicktoc_treeview.expand_all();
        let quicktoc_scrolled = ScrolledWindow::builder()
            .child(&quicktoc_treeview)
            .vexpand(true)
            .build();
        stack.add_named(&quicktoc_scrolled, Some("toc"));
        stack.set_visible_child_name("document");

        // Sidetoc: Paned with TOC on one side, Stack on the other
        let paned = Paned::new(Orientation::Horizontal);
        if sidetoc_right {
            paned.set_start_child(Some(&stack));
            paned.set_end_child(Some(&toc_scrolled));
        } else {
            paned.set_start_child(Some(&toc_scrolled));
            paned.set_end_child(Some(&stack));
        }
        // Allow shrinking to zero so we can collapse the sidetoc
        paned.set_shrink_start_child(true);
        paned.set_shrink_end_child(true);
        // Prevent paned from stealing arrow/page keys for divider adjustment
        paned.set_focusable(false);
        // Start with sidetoc collapsed (position 0 for left, max for right)
        if sidetoc_right {
            // Will be corrected after window is shown
            paned.set_position(10000);
        } else {
            paned.set_position(0);
        }
        // Track whether sidetoc is open

        // Compute initial window title
        let filename = std::path::Path::new(&infile_path)
            .file_name()
            .map(|f| f.to_string_lossy().to_string())
            .unwrap_or_else(|| "MiP".to_string());
        let window_title = if let Some(ref title) = initial_title {
            format!("{} - MiP", title)
        } else {
            format!("{} - MiP", filename)
        };

        let window_ref = ApplicationWindow::builder()
            .application(app)
            .title(&window_title)
            .default_width(800)
            .default_height(600)
            .child(&paned)
            .build();

        // Command context for executing commands
        let cmd_ctx = std::rc::Rc::new(CommandContext {
            app: app.clone(),
            window: window_ref.clone(),
            paned: paned.clone(),
            stack: stack.clone(),
            webview: webview.clone(),
            sidetoc_treeview: treeview.clone(),
            sidetoc_right,
            sidetoc_width,
            sidetoc_open: std::cell::Cell::new(false),
            settings: RuntimeSettings {
                frontmatter: std::cell::Cell::new(show_frontmatter),
                paragraph_numbers: std::cell::Cell::new(paragraph_numbers),
                paragraph_numbers_start: std::cell::Cell::new(paragraph_numbers_start),
                theme: std::cell::RefCell::new(theme_mode.to_string()),
                force_render: std::cell::Cell::new(false),
                infile: std::cell::RefCell::new(infile_path.clone()),
                filename: std::cell::RefCell::new(filename),
                math: std::cell::Cell::new(math),
            },
            watcher_tx: watcher_tx.clone(),
            temp_dir: temp_dir.clone(),
            port,
        });

        // Command bar (hidden by default)
        let cmd_entry = gtk4::Entry::new();
        cmd_entry.set_visible(false);
        cmd_entry.add_css_class("command-bar");
        cmd_entry.set_has_frame(false);

        // Apply styling: monospace, grey background, no borders, no focus ring
        let css_provider = gtk4::CssProvider::new();
        css_provider.load_from_data(
            "entry.command-bar { font-family: monospace; padding: 4px 8px; background: #e8e8e8; border: none; border-radius: 0; outline: none; box-shadow: none; } \
             entry.command-bar:focus { outline: none; box-shadow: none; border: none; } \
             label.wildmenu { font-family: monospace; padding: 4px 8px; background: #e8e8e8; border: none; border-radius: 0; }"
        );
        gtk4::style_context_add_provider_for_display(
            &gtk4::gdk::Display::default().unwrap(),
            &css_provider,
            gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );

        // Wildmenu label (hidden by default, shows completion matches)
        let wildmenu_label = gtk4::Label::new(None);
        wildmenu_label.set_visible(false);
        wildmenu_label.set_use_markup(true);
        wildmenu_label.set_xalign(0.0);
        wildmenu_label.add_css_class("wildmenu");

        // Wrap window content + wildmenu + command bar in a vertical box
        let content_widget = window_ref.child().unwrap();
        window_ref.set_child(None::<&gtk4::Widget>);
        let outer_box = gtk4::Box::new(gtk4::Orientation::Vertical, 0);
        content_widget.set_vexpand(true);
        outer_box.append(&content_widget);
        outer_box.append(&wildmenu_label);
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

        // Window key handler: `:` opens command bar, other keys check keybinding registry
        // Includes pending-key state for two-key sequences (e.g. g,g)
        {
            let cmd_entry_for_keys = cmd_entry.clone();
            let registry = keybinding_registry.clone();
            let ctx_for_keys = cmd_ctx.clone();
            let pending_key: std::rc::Rc<std::cell::Cell<Option<(u32, bool, bool, bool, bool, std::time::Instant)>>> =
                std::rc::Rc::new(std::cell::Cell::new(None));
            let pending_for_cmd = pending_key.clone();
            let key_controller_cmd = gtk4::EventControllerKey::new();
            key_controller_cmd.set_propagation_phase(gtk4::PropagationPhase::Capture);
            key_controller_cmd.connect_key_pressed(move |_, keyval, _keycode, state| {
                // Skip when command bar is visible (all keys go to entry)
                if gtk4::prelude::WidgetExt::is_visible(&cmd_entry_for_keys) {
                    pending_key.set(None);
                    return glib::Propagation::Proceed;
                }

                // Skip when a TreeView is focused (let TreeView j/k handlers work)
                if let Some(focus_widget) = gtk4::prelude::GtkWindowExt::focus(&ctx_for_keys.window) {
                    if focus_widget.type_().name() == "GtkTreeView" {
                        pending_key.set(None);
                        return glib::Propagation::Proceed;
                    }
                }

                // `:` always opens command bar (not rebindable)
                if keyval == gtk4::gdk::Key::colon {
                    pending_key.set(None);
                    cmd_entry_for_keys.set_text(":");
                    cmd_entry_for_keys.set_visible(true);
                    cmd_entry_for_keys.grab_focus();
                    let entry = cmd_entry_for_keys.clone();
                    glib::idle_add_local_once(move || {
                        entry.select_region(1, 1);
                        entry.set_position(1);
                    });
                    return glib::Propagation::Stop;
                }

                let ctrl = state.contains(gtk4::gdk::ModifierType::CONTROL_MASK);
                let shift = state.contains(gtk4::gdk::ModifierType::SHIFT_MASK);
                let alt = state.contains(gtk4::gdk::ModifierType::ALT_MASK);
                let super_ = state.contains(gtk4::gdk::ModifierType::SUPER_MASK);
                let kv = keyval.into_glib();

                // Check pending key state for sequences
                if let Some((prev_kv, prev_ctrl, prev_shift, prev_alt, prev_super, instant)) = pending_key.get() {
                    pending_key.set(None);
                    if instant.elapsed() < std::time::Duration::from_millis(500) {
                        if let Some(command) = registry.lookup_sequence(
                            prev_kv, prev_ctrl, prev_shift, prev_alt, prev_super,
                            kv, ctrl, shift, alt, super_,
                        ) {
                            let command = command.to_string();
                            execute_commands(&command, &ctx_for_keys);
                            return glib::Propagation::Stop;
                        }
                    }
                    // Sequence didn't match or timed out — fall through to process current key fresh
                }

                // Look up keybinding
                match registry.lookup(kv, ctrl, shift, alt, super_) {
                    crate::command::LookupResult::Command(command) => {
                        let command = command.to_string();
                        execute_commands(&command, &ctx_for_keys);
                        glib::Propagation::Stop
                    }
                    crate::command::LookupResult::SequencePrefix => {
                        pending_key.set(Some((kv, ctrl, shift, alt, super_, std::time::Instant::now())));
                        glib::Propagation::Stop
                    }
                    crate::command::LookupResult::None => {
                        glib::Propagation::Proceed
                    }
                }
            });
            window_ref.add_controller(key_controller_cmd);

            // Clear pending key when command bar becomes visible
            {
                let pending_for_visibility = pending_for_cmd;
                let cmd_entry_watch = cmd_entry.clone();
                cmd_entry_watch.connect_notify_local(Some("visible"), move |entry, _| {
                    if gtk4::prelude::WidgetExt::is_visible(entry) {
                        pending_for_visibility.set(None);
                    }
                });
            }
        }

        // Command bar: Enter executes (use activate signal — more reliable than key handler)
        {
            let cmd_entry_for_activate = cmd_entry.clone();
            let webview_for_activate = webview.clone();
            let wildmenu_for_activate = wildmenu_label.clone();
            let ctx = cmd_ctx.clone();
            let history_for_activate = history.clone();
            cmd_entry.connect_activate(move |entry| {
                let text = entry.text().to_string();
                cmd_entry_for_activate.set_text("");
                cmd_entry_for_activate.set_visible(false);
                wildmenu_for_activate.set_visible(false);
                webview_for_activate.grab_focus();
                // Strip leading : and execute (supports ; composition)
                let text = text.strip_prefix(':').unwrap_or(&text);
                if !text.is_empty() {
                    history_for_activate.borrow_mut().push(text);
                }
                execute_commands(text, &ctx);
            });
        }

        // Command bar: Escape dismisses, Tab/Shift+Tab completes (capture phase)
        {
            let cmd_entry_for_keys = cmd_entry.clone();
            let webview_for_cmd = webview.clone();
            let wildmenu_for_keys = wildmenu_label.clone();
            let key_controller_entry = gtk4::EventControllerKey::new();
            key_controller_entry.set_propagation_phase(gtk4::PropagationPhase::Capture);
            let tab_matches: std::rc::Rc<std::cell::RefCell<Vec<String>>> = std::rc::Rc::new(std::cell::RefCell::new(Vec::new()));
            let tab_index: std::rc::Rc<std::cell::Cell<usize>> = std::rc::Rc::new(std::cell::Cell::new(0));
            let tab_prefix: std::rc::Rc<std::cell::RefCell<String>> = std::rc::Rc::new(std::cell::RefCell::new(String::new()));

            let tab_matches_clone = tab_matches.clone();
            let tab_index_clone = tab_index.clone();
            let tab_prefix_clone = tab_prefix.clone();

            let hist_index: std::rc::Rc<std::cell::Cell<Option<usize>>> = std::rc::Rc::new(std::cell::Cell::new(None));
            let hist_matches: std::rc::Rc<std::cell::RefCell<Vec<String>>> = std::rc::Rc::new(std::cell::RefCell::new(Vec::new()));
            let hist_saved: std::rc::Rc<std::cell::RefCell<String>> = std::rc::Rc::new(std::cell::RefCell::new(String::new()));
            let hist_index_clone = hist_index.clone();
            let hist_matches_clone = hist_matches.clone();
            let hist_saved_clone = hist_saved.clone();
            let history_for_keys = history.clone();

            key_controller_entry.connect_key_pressed(move |_, keyval, _keycode, state| {
                let shift = state.contains(gtk4::gdk::ModifierType::SHIFT_MASK);
                match keyval {
                    v if v == gtk4::gdk::Key::Escape => {
                        cmd_entry_for_keys.set_text("");
                        cmd_entry_for_keys.set_visible(false);
                        wildmenu_for_keys.set_visible(false);
                        webview_for_cmd.grab_focus();
                        tab_matches_clone.borrow_mut().clear();
                        tab_index_clone.set(0);
                        glib::Propagation::Stop
                    }
                    v if v == gtk4::gdk::Key::BackSpace => {
                        let text = cmd_entry_for_keys.text().to_string();
                        if text == ":" {
                            cmd_entry_for_keys.set_text("");
                            cmd_entry_for_keys.set_visible(false);
                            wildmenu_for_keys.set_visible(false);
                            webview_for_cmd.grab_focus();
                            tab_matches_clone.borrow_mut().clear();
                            tab_index_clone.set(0);
                            glib::Propagation::Stop
                        } else if text.len() > 1 && cmd_entry_for_keys.position() <= 1 {
                            glib::Propagation::Stop
                        } else {
                            tab_matches_clone.borrow_mut().clear();
                            tab_index_clone.set(0);
                            wildmenu_for_keys.set_visible(false);
                            glib::Propagation::Proceed
                        }
                    }
                    v if v == gtk4::gdk::Key::Tab || v == gtk4::gdk::Key::ISO_Left_Tab => {
                        let reverse = shift || v == gtk4::gdk::Key::ISO_Left_Tab;
                        handle_tab_completion(
                            &cmd_entry_for_keys,
                            &wildmenu_for_keys,
                            &tab_matches_clone,
                            &tab_index_clone,
                            &tab_prefix_clone,
                            reverse,
                        );
                        glib::Propagation::Stop
                    }
                    v if v == gtk4::gdk::Key::Up => {
                        // Build filtered matches on first ↑ press
                        if hist_index_clone.get().is_none() {
                            let text = cmd_entry_for_keys.text().to_string();
                            let prefix = text.strip_prefix(':').unwrap_or(&text);
                            *hist_saved_clone.borrow_mut() = prefix.to_string();
                            let matches: Vec<String> = history_for_keys.borrow()
                                .filter(prefix)
                                .into_iter()
                                .map(|s| s.to_string())
                                .collect();
                            *hist_matches_clone.borrow_mut() = matches;
                            let len = hist_matches_clone.borrow().len();
                            if len > 0 {
                                hist_index_clone.set(Some(len - 1));
                            }
                        } else if let Some(idx) = hist_index_clone.get() {
                            if idx > 0 {
                                hist_index_clone.set(Some(idx - 1));
                            }
                        }
                        // Show entry at current index
                        if let Some(idx) = hist_index_clone.get() {
                            let matches = hist_matches_clone.borrow();
                            if let Some(entry) = matches.get(idx) {
                                let new_text = format!(":{}", entry);
                                cmd_entry_for_keys.set_text(&new_text);
                                cmd_entry_for_keys.set_position(new_text.len() as i32);
                            }
                        }
                        glib::Propagation::Stop
                    }
                    v if v == gtk4::gdk::Key::Down => {
                        if let Some(idx) = hist_index_clone.get() {
                            let len = hist_matches_clone.borrow().len();
                            if idx + 1 < len {
                                hist_index_clone.set(Some(idx + 1));
                                let matches = hist_matches_clone.borrow();
                                if let Some(entry) = matches.get(idx + 1) {
                                    let new_text = format!(":{}", entry);
                                    cmd_entry_for_keys.set_text(&new_text);
                                    cmd_entry_for_keys.set_position(new_text.len() as i32);
                                }
                            } else {
                                // Past newest: restore saved input
                                hist_index_clone.set(None);
                                hist_matches_clone.borrow_mut().clear();
                                let saved = hist_saved_clone.borrow().clone();
                                let new_text = format!(":{}", saved);
                                cmd_entry_for_keys.set_text(&new_text);
                                cmd_entry_for_keys.set_position(new_text.len() as i32);
                            }
                        }
                        glib::Propagation::Stop
                    }
                    v if v == gtk4::gdk::Key::Home || (v == gtk4::gdk::Key::Left && cmd_entry_for_keys.position() <= 1) => {
                        glib::Propagation::Stop
                    }
                    _ => {
                        // Don't reset on modifier-only keys (Shift, Ctrl, Alt, Super)
                        let is_modifier = matches!(keyval,
                            v if v == gtk4::gdk::Key::Shift_L
                              || v == gtk4::gdk::Key::Shift_R
                              || v == gtk4::gdk::Key::Control_L
                              || v == gtk4::gdk::Key::Control_R
                              || v == gtk4::gdk::Key::Alt_L
                              || v == gtk4::gdk::Key::Alt_R
                              || v == gtk4::gdk::Key::Super_L
                              || v == gtk4::gdk::Key::Super_R
                        );
                        if !is_modifier {
                            tab_matches_clone.borrow_mut().clear();
                            tab_index_clone.set(0);
                            wildmenu_for_keys.set_visible(false);
                            hist_index_clone.set(None);
                            hist_matches_clone.borrow_mut().clear();
                        }
                        glib::Propagation::Proceed
                    }
                }
            });
            cmd_entry.add_controller(key_controller_entry);
        }

        // Connect TOC row-activated → scroll to heading (for sidetoc treeview)
        {
            let webview_for_activate = webview.clone();
            treeview.connect_row_activated(move |tv, path, _col| {
                let Some(model) = tv.model() else { return };
                let Some(iter) = model.iter(path) else { return };
                let anchor_id: String = model.get(&iter, COL_ANCHOR as i32);
                scroll_to_anchor(&webview_for_activate, &anchor_id);
            });
        }

        // Sidetoc treeview key handler: Escape closes, left/right collapse/expand, Enter scrolls
        {
            let ctx_for_keys = cmd_ctx.clone();
            let treeview_for_keys = treeview.clone();
            let key_controller_sidetoc = gtk4::EventControllerKey::new();
            key_controller_sidetoc.connect_key_pressed(move |_, keyval, _keycode, _state| {
                match keyval {
                    v if v == gtk4::gdk::Key::Escape => {
                        execute_command("sidetoc_close", "", &ctx_for_keys);
                        glib::Propagation::Stop
                    }
                    v if v == gtk4::gdk::Key::Left => {
                        // Collapse current row
                        if let (Some(path), _) = TreeViewExt::cursor(&treeview_for_keys) {
                            if treeview_for_keys.row_expanded(&path) {
                                treeview_for_keys.collapse_row(&path);
                            } else {
                                // Move to parent if already collapsed
                                let mut parent = path.clone();
                                if parent.up() && parent.depth() > 0 {
                                    TreeViewExt::set_cursor(&treeview_for_keys, &parent, None::<&gtk4::TreeViewColumn>, false);
                                }
                            }
                        }
                        glib::Propagation::Stop
                    }
                    v if v == gtk4::gdk::Key::Right => {
                        // Expand current row
                        if let (Some(path), _) = TreeViewExt::cursor(&treeview_for_keys) {
                            if !treeview_for_keys.row_expanded(&path) {
                                treeview_for_keys.expand_row(&path, false);
                            } else {
                                // Move to first child if already expanded
                                let mut child = path.clone();
                                child.append_index(0);
                                TreeViewExt::set_cursor(&treeview_for_keys, &child, None::<&gtk4::TreeViewColumn>, false);
                            }
                        }
                        glib::Propagation::Stop
                    }
                    v if v == gtk4::gdk::Key::l => {
                        // Focus the webview (vim-style: l = move right to document)
                        ctx_for_keys.webview.grab_focus();
                        glib::Propagation::Stop
                    }
                    _ => glib::Propagation::Proceed,
                }
            });
            treeview.add_controller(key_controller_sidetoc);
        }

        // Connect quicktoc row-activated → scroll + switch back to document
        {
            let webview_for_qtoc = webview.clone();
            let stack_for_qtoc = stack.clone();
            quicktoc_treeview.connect_row_activated(move |tv, path, _col| {
                let Some(model) = tv.model() else { return };
                let Some(iter) = model.iter(path) else { return };
                let anchor_id: String = model.get(&iter, COL_ANCHOR as i32);
                scroll_to_anchor(&webview_for_qtoc, &anchor_id);
                stack_for_qtoc.set_visible_child_name("document");
            });
        }

        // Key handler on quicktoc TreeView: j/k navigation, Esc to close, Enter to activate
        {
            let webview_for_keys = webview.clone();
            let stack_for_keys = stack.clone();
            let key_controller_tv = gtk4::EventControllerKey::new();
            let treeview_for_keys = quicktoc_treeview.clone();
            key_controller_tv.connect_key_pressed(move |_, keyval, _keycode, _state| {
                match keyval {
                    v if v == gtk4::gdk::Key::j => {
                        if let (Some(path), _) = TreeViewExt::cursor(&treeview_for_keys) {
                            let mut next = path;
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
                        if let (Some(path), col) = TreeViewExt::cursor(&treeview_for_keys) {
                            treeview_for_keys.row_activated(&path, col.as_ref());
                        }
                        glib::Propagation::Stop
                    }
                    v if v == gtk4::gdk::Key::Left => {
                        if let (Some(path), _) = TreeViewExt::cursor(&treeview_for_keys) {
                            if treeview_for_keys.row_expanded(&path) {
                                treeview_for_keys.collapse_row(&path);
                            } else {
                                let mut parent = path.clone();
                                if parent.up() && parent.depth() > 0 {
                                    TreeViewExt::set_cursor(&treeview_for_keys, &parent, None::<&gtk4::TreeViewColumn>, false);
                                }
                            }
                        }
                        glib::Propagation::Stop
                    }
                    v if v == gtk4::gdk::Key::Right => {
                        if let (Some(path), _) = TreeViewExt::cursor(&treeview_for_keys) {
                            if !treeview_for_keys.row_expanded(&path) {
                                treeview_for_keys.expand_row(&path, false);
                            } else {
                                let mut child = path.clone();
                                child.append_index(0);
                                TreeViewExt::set_cursor(&treeview_for_keys, &child, None::<&gtk4::TreeViewColumn>, false);
                            }
                        }
                        glib::Propagation::Stop
                    }
                    v if v == gtk4::gdk::Key::Escape => {
                        stack_for_keys.set_visible_child_name("document");
                        webview_for_keys.grab_focus();
                        glib::Propagation::Stop
                    }
                    _ => glib::Propagation::Proceed,
                }
            });
            quicktoc_treeview.add_controller(key_controller_tv);
        }

        // Execute runcmd at startup (after all widgets are built)
        if let Some(ref runcmd_text) = runcmd {
            execute_commands(runcmd_text, &cmd_ctx);
        }

        // Poll seed file and update page content via JS injection
        // to avoid the flicker of a full load_html() call.
        let seed_path = seed_path.clone();
        let mut last_seed = std::fs::read_to_string(&seed_path).unwrap_or_default();
        let mut last_system_dark: Option<bool> = Some(crate::is_system_dark());
        let mut last_toc: Vec<TocEntry> = initial_toc;
        let mut last_html_body = String::new();
        let ctx_for_poll = cmd_ctx.clone();

        glib::timeout_add_local(std::time::Duration::from_millis(500), move || {
            // Check for system theme changes (only when theme is "system")
            if let Some(ref mut was_dark) = last_system_dark {
                let current_theme = ctx_for_poll.settings.theme.borrow().clone();
                if current_theme == "system" {
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
            }

            // Check for force render (from :set command)
            let force = ctx_for_poll.settings.force_render.get();
            if force {
                ctx_for_poll.settings.force_render.set(false);
            }

            let seed_changed = if let Ok(current_seed) = std::fs::read_to_string(&seed_path) {
                if current_seed != last_seed {
                    last_seed = current_seed;
                    true
                } else {
                    false
                }
            } else {
                false
            };

            if seed_changed || force {
                    let current_infile = ctx_for_poll.settings.infile.borrow().clone();
                    if let Ok(md_content) = std::fs::read_to_string(&current_infile) {
                        let sf = ctx_for_poll.settings.frontmatter.get();
                        let pn = ctx_for_poll.settings.paragraph_numbers.get();
                        let pns = ctx_for_poll.settings.paragraph_numbers_start.get();
                        let (html_body, toc_entries, doc_title) = crate::markdown::md_to_html_body_with_toc(&md_content, sf, pn, pns, ctx_for_poll.settings.math.get());

                        // Update window title
                        let current_filename = ctx_for_poll.settings.filename.borrow().clone();
                        let title = if let Some(ref t) = doc_title {
                            format!("{} - MiP", t)
                        } else {
                            format!("{} - MiP", current_filename)
                        };
                        ctx_for_poll.window.set_title(Some(&title));

                        // Only update WebView if content actually changed
                        if html_body != last_html_body {
                            let escaped = html_body
                                .replace('\\', "\\\\")
                                .replace('`', "\\`")
                                .replace("${", "\\${");
                            let js = format!(
                                "document.querySelector('.section').innerHTML = `{}`;if(typeof renderMath==='function')renderMath();",
                                escaped
                            );
                            webview.evaluate_javascript(&js, None, None, None::<&gtk4::gio::Cancellable>, |_| {});
                            last_html_body = html_body;
                        }

                        // Only rebuild TOC if headings changed
                        if toc_entries != last_toc {
                            populate_toc(&toc_store, &toc_entries);
                            treeview.expand_all();
                            quicktoc_treeview.expand_all();
                            last_toc = toc_entries;
                        }
                    }
            }
            glib::ControlFlow::Continue
        });
    });

    app.connect_shutdown(move |_| {
        history_for_shutdown.borrow().save(&history_path_for_shutdown);
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

    // --- post_process_export tests ---

    #[test]
    fn test_post_process_strips_scripts() {
        let html = r#"<!DOCTYPE html><html><head><script>var x=1;</script></head><body><p>Hello</p><script src="bridge.js"></script></body></html>"#;
        let result = post_process_export(html);
        assert!(!result.contains("<script"));
        assert!(!result.contains("</script>"));
        assert!(result.contains("<p>Hello</p>"));
    }

    #[test]
    fn test_post_process_strips_localhost_links() {
        let html = r#"<!DOCTYPE html><html><head><link rel="stylesheet" href="http://localhost:8000/katex/katex.min.css"><style>body{}</style></head><body></body></html>"#;
        let result = post_process_export(html);
        assert!(!result.contains("localhost"));
        assert!(result.contains("<style>body{}</style>"));
    }

    #[test]
    fn test_post_process_strips_header_div() {
        let html = r#"<!DOCTYPE html><html><body><div id="header"><a href="http://localhost:8000/">file.md</a></div><div id="content">Hello</div></body></html>"#;
        let result = post_process_export(html);
        assert!(!result.contains(r#"id="header""#));
        assert!(result.contains(r#"id="content"#));
    }

    #[test]
    fn test_post_process_preserves_content() {
        let html = r#"<!DOCTYPE html><html><head><style>.katex{color:red}</style></head><body><h1 id="title">Title</h1><p>Content here</p></body></html>"#;
        let result = post_process_export(html);
        assert!(result.contains("<h1 id=\"title\">Title</h1>"));
        assert!(result.contains("<p>Content here</p>"));
        assert!(result.contains(".katex{color:red}"));
    }

    #[test]
    fn test_post_process_ensures_doctype() {
        let html = r#"<html><body><p>No doctype</p></body></html>"#;
        let result = post_process_export(html);
        assert!(result.starts_with("<!DOCTYPE html>"));
        assert!(result.contains("<p>No doctype</p>"));
    }

    #[test]
    fn test_post_process_preserves_existing_doctype() {
        let html = r#"<!DOCTYPE html><html><body><p>Has doctype</p></body></html>"#;
        let result = post_process_export(html);
        // Should not have double DOCTYPE
        assert_eq!(result.matches("<!DOCTYPE").count(), 1);
    }

    #[test]
    fn test_post_process_passthrough_no_scripts_links() {
        let html = r#"<!DOCTYPE html><html><head><style>h1{}</style></head><body><p>Clean</p></body></html>"#;
        let result = post_process_export(html);
        assert_eq!(result, html);
    }

    #[test]
    fn test_post_process_preserves_inline_svgs() {
        let html = r#"<!DOCTYPE html><html><body><svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 100 100"><rect width="100" height="100" fill="red"/></svg></body></html>"#;
        let result = post_process_export(html);
        assert!(result.contains("<svg"));
        assert!(result.contains("<rect"));
        assert!(result.contains("</svg>"));
    }

    #[test]
    fn test_post_process_preserves_katex_math_spans() {
        let html = r#"<!DOCTYPE html><html><body><span class="katex"><span class="katex-mathml"><math xmlns="http://www.w3.org/1998/Math/MathML"><mi>x</mi></math></span><span class="katex-html"><span class="base"><span class="mord mathnormal">x</span></span></span></span></body></html>"#;
        let result = post_process_export(html);
        assert!(result.contains("class=\"katex\""));
        assert!(result.contains("katex-mathml"));
        assert!(result.contains("katex-html"));
        assert!(result.contains("<mi>x</mi>"));
    }
}
