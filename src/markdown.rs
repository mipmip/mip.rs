use std::fs;
use rand::Rng;
use rand::distr::Alphanumeric;
use rust_embed::Embed;
use gray_matter::Matter;
use gray_matter::engine::YAML;
use gray_matter::value::pod::Pod;
use pulldown_cmark::{html, Event, Options, Parser, Tag, TagEnd, HeadingLevel};

#[derive(Embed)]
#[folder = "asset/theme1"]
struct Asset;

pub(crate) fn pod_to_html_value(pod: &Pod) -> String {
    match pod {
        Pod::String(s) => s.clone(),
        Pod::Integer(i) => i.to_string(),
        Pod::Float(f) => f.to_string(),
        Pod::Boolean(b) => b.to_string(),
        Pod::Null => String::new(),
        Pod::Array(items) => {
            items.iter()
                .map(|item| pod_to_html_value(item))
                .collect::<Vec<_>>()
                .join(", ")
        }
        Pod::Hash(map) => {
            map.iter()
                .map(|(k, v)| format!("{}: {}", k, pod_to_html_value(v)))
                .collect::<Vec<_>>()
                .join(", ")
        }
    }
}

fn frontmatter_to_html(data: &Pod) -> String {
    if let Pod::Hash(map) = data {
        let mut html = String::from("<table class=\"frontmatter\"><tbody>\n");
        for (key, value) in map {
            html.push_str(&format!(
                "<tr><th>{}</th><td>{}</td></tr>\n",
                key,
                pod_to_html_value(value)
            ));
        }
        html.push_str("</tbody></table>\n");
        html
    } else {
        String::new()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct TocEntry {
    pub level: u8,
    pub title: String,
    pub anchor_id: String,
}

pub fn slugify(text: &str) -> String {
    let slug: String = text
        .to_lowercase()
        .chars()
        .map(|c| if c.is_alphanumeric() { c } else { '-' })
        .collect();
    // Collapse consecutive dashes and trim
    let mut result = String::new();
    let mut prev_dash = false;
    for c in slug.chars() {
        if c == '-' {
            if !prev_dash && !result.is_empty() {
                result.push('-');
            }
            prev_dash = true;
        } else {
            prev_dash = false;
            result.push(c);
        }
    }
    if result.ends_with('-') {
        result.pop();
    }
    result
}

fn slugify_unique(text: &str, existing: &[String]) -> String {
    let base = slugify(text);
    if base.is_empty() {
        let id = format!("heading-{}", existing.len());
        return id;
    }
    if !existing.contains(&base) {
        return base;
    }
    let mut n = 1;
    loop {
        let candidate = format!("{}-{}", base, n);
        if !existing.contains(&candidate) {
            return candidate;
        }
        n += 1;
    }
}

fn heading_level_to_u8(level: HeadingLevel) -> u8 {
    match level {
        HeadingLevel::H1 => 1,
        HeadingLevel::H2 => 2,
        HeadingLevel::H3 => 3,
        HeadingLevel::H4 => 4,
        HeadingLevel::H5 => 5,
        HeadingLevel::H6 => 6,
    }
}

fn extract_headings_and_inject_ids<'a>(
    events: impl Iterator<Item = Event<'a>>,
) -> (Vec<Event<'a>>, Vec<TocEntry>) {
    let mut toc = Vec::new();
    let mut used_ids: Vec<String> = Vec::new();
    let mut output_events: Vec<Event<'a>> = Vec::new();
    let mut in_heading = false;
    let mut heading_level: u8 = 0;
    let mut heading_text = String::new();

    for event in events {
        match &event {
            Event::Start(Tag::Heading { level, .. }) => {
                in_heading = true;
                heading_level = heading_level_to_u8(*level);
                heading_text.clear();
                // Don't push the start event yet — we'll replace it
                output_events.push(event);
                continue;
            }
            Event::End(TagEnd::Heading(level)) => {
                in_heading = false;
                let anchor_id = slugify_unique(&heading_text, &used_ids);
                used_ids.push(anchor_id.clone());
                toc.push(TocEntry {
                    level: heading_level,
                    title: heading_text.clone(),
                    anchor_id: anchor_id.clone(),
                });
                // Inject the id attribute by replacing the Start event
                // Find the last Start(Heading) in output_events and replace it
                let lvl = *level;
                for ev in output_events.iter_mut().rev() {
                    if matches!(ev, Event::Start(Tag::Heading { .. })) {
                        *ev = Event::Start(Tag::Heading {
                            level: lvl,
                            id: Some(anchor_id.clone().into()),
                            classes: vec![],
                            attrs: vec![],
                        });
                        break;
                    }
                }
                output_events.push(event);
                continue;
            }
            Event::Text(text) if in_heading => {
                heading_text.push_str(text);
            }
            Event::Code(code) if in_heading => {
                heading_text.push_str(code);
            }
            _ => {}
        }
        output_events.push(event);
    }

    (output_events, toc)
}

/// Convert raw markdown text to HTML body with TOC entries.
pub fn md_to_html_body_with_toc(markdown_input: &str, show_frontmatter: bool) -> (String, Vec<TocEntry>) {
    let matter = Matter::<YAML>::new();
    let result = matter.parse(markdown_input);

    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TASKLISTS);
    options.insert(Options::ENABLE_TABLES);
    let parser = Parser::new_ext(&result.content, options);

    let mut html_output = String::new();

    if show_frontmatter {
        if let Some(ref data) = result.data {
            html_output.push_str(&frontmatter_to_html(data));
        }
    }

    let (events, toc) = extract_headings_and_inject_ids(parser);
    html::push_html(&mut html_output, events.into_iter());
    (rewrite_media_embeds(&html_output), toc)
}

const VIDEO_EXTENSIONS: &[&str] = &[".webm", ".mp4", ".mov", ".ogv"];

fn is_video_url(url: &str) -> bool {
    let lower = url.to_lowercase();
    // Strip query string and fragment for extension check
    let path = lower.split('?').next().unwrap_or(&lower);
    let path = path.split('#').next().unwrap_or(path);
    VIDEO_EXTENSIONS.iter().any(|ext| path.ends_with(ext))
}

fn rewrite_media_embeds(html: &str) -> String {
    let mut result = html.to_string();

    // Rewrite <a href="...video_ext">...</a> → <video>
    // pulldown-cmark produces: <a href="URL">text</a>
    let mut search_from = 0;
    loop {
        let Some(a_start) = result[search_from..].find("<a href=\"") else { break };
        let a_start = search_from + a_start;
        let href_start = a_start + 9; // after <a href="
        let Some(href_end) = result[href_start..].find('"') else { break };
        let href_end = href_start + href_end;
        let url = &result[href_start..href_end];

        if is_video_url(url) {
            // Find the closing </a>
            let Some(a_close) = result[href_end..].find("</a>") else {
                search_from = href_end;
                continue;
            };
            let a_close_end = href_end + a_close + 4; // end of </a>
            let video_tag = format!(
                "<video src=\"{}\" controls style=\"max-width:100%\"></video>",
                url
            );
            result.replace_range(a_start..a_close_end, &video_tag);
            search_from = a_start + video_tag.len();
        } else {
            search_from = href_end;
        }
    }

    // Rewrite <img src="...video_ext" .../> → <video>
    // pulldown-cmark produces: <img src="URL" alt="text" />
    search_from = 0;
    loop {
        let Some(img_start) = result[search_from..].find("<img src=\"") else { break };
        let img_start = search_from + img_start;
        let src_start = img_start + 10; // after <img src="
        let Some(src_end) = result[src_start..].find('"') else { break };
        let src_end = src_start + src_end;
        let url = &result[src_start..src_end];

        if is_video_url(url) {
            // Find the closing > of the img tag
            let Some(img_close) = result[src_end..].find('>') else {
                search_from = src_end;
                continue;
            };
            let img_close_end = src_end + img_close + 1;
            let video_tag = format!(
                "<video src=\"{}\" controls style=\"max-width:100%\"></video>",
                url
            );
            result.replace_range(img_start..img_close_end, &video_tag);
            search_from = img_start + video_tag.len();
        } else {
            search_from = src_end;
        }
    }

    result
}

/// Convert raw markdown text to HTML body (without template wrapper).
pub fn md_to_html_body(markdown_input: &str, show_frontmatter: bool) -> String {
    let (html, _toc) = md_to_html_body_with_toc(markdown_input, show_frontmatter);
    html
}

/// Build the complete HTML document from markdown content.
/// This is a pure function: no I/O, no randomness.
/// Takes markdown string, template string, seed, seed URL,
/// show_frontmatter flag, and theme class; returns complete HTML string.
pub fn build_html(
    markdown_input: &str,
    template: &str,
    seed: &str,
    seed_url: &str,
    show_frontmatter: bool,
    theme_class: &str,
) -> String {
    let html_body = md_to_html_body(markdown_input, show_frontmatter);
    template
        .replace("#{BODY}", &html_body)
        .replace("#{INITIALSEED}", seed)
        .replace("#{SEEDURL}", seed_url)
        .replace("#{THEME_CLASS}", theme_class)
}

pub fn to_html(infile: &str, output_dir: &std::path::Path, port: u16, show_frontmatter: bool, theme_class: &str){

    let markdown_input = fs::read_to_string(infile);
    if let Ok(markdown_input) = markdown_input { to_file(&markdown_input, output_dir, port, show_frontmatter, theme_class) };
}

fn to_file(markdown_input: &str, output_dir: &std::path::Path, port: u16, show_frontmatter: bool, theme_class: &str){
    let seed_url = format!("http://localhost:{}/.temp.seed", port);

    let seed: String = rand::rng()
        .sample_iter(&Alphanumeric)
        .take(7)
        .map(char::from)
        .collect();

    let index_html = Asset::get("template.html").unwrap();
    let index_html_str = std::str::from_utf8(index_html.data.as_ref());
    match index_html_str {
        Ok(template) => {
            let html_complete = build_html(markdown_input, template, &seed, &seed_url, show_frontmatter, theme_class);
            if let Err(e) = fs::write(output_dir.join(".temp.seed"), seed) {
                eprintln!("warning: could not write seed file: {}", e);
                return;
            }
            if let Err(e) = fs::write(output_dir.join(".temp.html"), html_complete) {
                eprintln!("warning: could not write html file: {}", e);
            }
        },
        Err(_) => println!("URF this..no file")
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pod_to_html_value_string() {
        let pod = Pod::String("hello".to_string());
        assert_eq!(pod_to_html_value(&pod), "hello");
    }

    #[test]
    fn test_pod_to_html_value_integer() {
        let pod = Pod::Integer(42);
        assert_eq!(pod_to_html_value(&pod), "42");
    }

    #[test]
    fn test_pod_to_html_value_float() {
        let pod = Pod::Float(3.14);
        assert_eq!(pod_to_html_value(&pod), "3.14");
    }

    #[test]
    fn test_pod_to_html_value_boolean() {
        assert_eq!(pod_to_html_value(&Pod::Boolean(true)), "true");
        assert_eq!(pod_to_html_value(&Pod::Boolean(false)), "false");
    }

    #[test]
    fn test_pod_to_html_value_null() {
        assert_eq!(pod_to_html_value(&Pod::Null), "");
    }

    #[test]
    fn test_pod_to_html_value_array() {
        let pod = Pod::Array(vec![
            Pod::String("a".to_string()),
            Pod::Integer(1),
        ]);
        assert_eq!(pod_to_html_value(&pod), "a, 1");
    }

    #[test]
    fn test_pod_to_html_value_hash() {
        let mut map = std::collections::HashMap::new();
        map.insert("key".to_string(), Pod::String("val".to_string()));
        let pod = Pod::Hash(map);
        assert_eq!(pod_to_html_value(&pod), "key: val");
    }

    #[test]
    fn test_slugify_basic() {
        assert_eq!(slugify("Hello World"), "hello-world");
    }

    #[test]
    fn test_slugify_special_chars() {
        assert_eq!(slugify("Setup & Dependencies"), "setup-dependencies");
    }

    #[test]
    fn test_slugify_unicode() {
        assert_eq!(slugify("Über cool"), "über-cool");
    }

    #[test]
    fn test_slugify_consecutive_dashes() {
        assert_eq!(slugify("a - - b"), "a-b");
    }

    #[test]
    fn test_slugify_leading_trailing() {
        assert_eq!(slugify("  hello  "), "hello");
    }

    #[test]
    fn test_slugify_empty() {
        assert_eq!(slugify(""), "");
    }

    #[test]
    fn test_slugify_numbers() {
        assert_eq!(slugify("Step 1: Do it"), "step-1-do-it");
    }

    #[test]
    fn test_slugify_unique_duplicates() {
        let existing = vec!["intro".to_string()];
        assert_eq!(slugify_unique("Intro", &existing), "intro-1");

        let existing2 = vec!["intro".to_string(), "intro-1".to_string()];
        assert_eq!(slugify_unique("Intro", &existing2), "intro-2");
    }

    #[test]
    fn test_slugify_unique_empty_text() {
        let existing = vec![];
        assert_eq!(slugify_unique("", &existing), "heading-0");
    }

    #[test]
    fn test_md_to_html_body_with_toc_basic() {
        let md = "# Title\n\n## Section\n\nText\n\n### Sub";
        let (html, toc) = md_to_html_body_with_toc(md, false);

        assert_eq!(toc.len(), 3);
        assert_eq!(toc[0], TocEntry { level: 1, title: "Title".into(), anchor_id: "title".into() });
        assert_eq!(toc[1], TocEntry { level: 2, title: "Section".into(), anchor_id: "section".into() });
        assert_eq!(toc[2], TocEntry { level: 3, title: "Sub".into(), anchor_id: "sub".into() });

        assert!(html.contains("<h1 id=\"title\">Title</h1>"));
        assert!(html.contains("<h2 id=\"section\">Section</h2>"));
        assert!(html.contains("<h3 id=\"sub\">Sub</h3>"));
    }

    #[test]
    fn test_md_to_html_body_with_toc_skipped_levels() {
        let md = "# Top\n\n### Skipped h2\n\n## Back to h2";
        let (_html, toc) = md_to_html_body_with_toc(md, false);

        assert_eq!(toc.len(), 3);
        assert_eq!(toc[0].level, 1);
        assert_eq!(toc[1].level, 3);
        assert_eq!(toc[2].level, 2);
    }

    #[test]
    fn test_md_to_html_body_with_toc_no_headings() {
        let md = "Just a paragraph.\n\nAnother one.";
        let (_html, toc) = md_to_html_body_with_toc(md, false);
        assert!(toc.is_empty());
    }

    #[test]
    fn test_md_to_html_body_with_toc_duplicate_headings() {
        let md = "# Intro\n\n## Intro\n\n### Intro";
        let (_html, toc) = md_to_html_body_with_toc(md, false);

        assert_eq!(toc[0].anchor_id, "intro");
        assert_eq!(toc[1].anchor_id, "intro-1");
        assert_eq!(toc[2].anchor_id, "intro-2");
    }

    #[test]
    fn test_md_to_html_body_with_toc_frontmatter_excluded() {
        let md = "---\ntitle: Test\n---\n\n# Real Heading";
        let (_html, toc) = md_to_html_body_with_toc(md, true);

        assert_eq!(toc.len(), 1);
        assert_eq!(toc[0].title, "Real Heading");
    }
}
