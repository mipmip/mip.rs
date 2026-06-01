use gray_matter::Matter;
use gray_matter::engine::YAML;
use gray_matter::value::pod::Pod;
use pulldown_cmark::{Event, HeadingLevel, Options, Parser, Tag, TagEnd, html};
use rand::Rng;
use rand::distr::Alphanumeric;
use rust_embed::Embed;
use std::fs;

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
        Pod::Array(items) => items
            .iter()
            .map(pod_to_html_value)
            .collect::<Vec<_>>()
            .join(", "),
        Pod::Hash(map) => map
            .iter()
            .map(|(k, v)| format!("{}: {}", k, pod_to_html_value(v)))
            .collect::<Vec<_>>()
            .join(", "),
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
/// When `paragraph_numbers` is true, injects section numbers into headings and TOC titles.
pub fn md_to_html_body_with_toc(
    markdown_input: &str,
    show_frontmatter: bool,
    paragraph_numbers: bool,
    paragraph_numbers_start: u8,
    math: bool,
) -> (String, Vec<TocEntry>, Option<String>) {
    let matter = Matter::<YAML>::new();
    let result = matter.parse(markdown_input);

    // Extract title from frontmatter if present
    let frontmatter_title = result.data.as_ref().and_then(|data| {
        if let Pod::Hash(map) = data {
            map.get("title").and_then(|v| {
                if let Pod::String(s) = v {
                    Some(s.clone())
                } else {
                    None
                }
            })
        } else {
            None
        }
    });

    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TASKLISTS);
    options.insert(Options::ENABLE_TABLES);
    if math {
        options.insert(Options::ENABLE_MATH);
    }
    let parser = Parser::new_ext(&result.content, options);

    let mut html_output = String::new();

    if show_frontmatter && let Some(ref data) = result.data {
        html_output.push_str(&frontmatter_to_html(data));
    }

    let (events, mut toc) = extract_headings_and_inject_ids(parser);
    html::push_html(&mut html_output, events.into_iter());
    let mut html_output = rewrite_media_embeds(&html_output);

    if paragraph_numbers {
        let numbers = compute_section_numbers(&toc, paragraph_numbers_start);
        html_output = inject_section_numbers(&html_output, &toc, &numbers);
        // Prepend numbers to TOC titles
        for (entry, number) in toc.iter_mut().zip(numbers.iter()) {
            if !number.is_empty() {
                entry.title = format!("{} {}", number, entry.title);
            }
        }
    }

    (html_output, toc, frontmatter_title)
}

/// Compute hierarchical section numbers for TOC entries.
/// Headings below `start_level` get empty strings.
/// Returns a parallel vec of number strings ("1.", "1.1", "1.1.2", etc.).
pub fn compute_section_numbers(entries: &[TocEntry], start_level: u8) -> Vec<String> {
    let mut counters = [0u32; 6]; // h1..h6
    let mut numbers = Vec::with_capacity(entries.len());

    for entry in entries {
        if entry.level < start_level {
            numbers.push(String::new());
            continue;
        }

        let depth = (entry.level - start_level) as usize;
        if depth >= 6 {
            numbers.push(String::new());
            continue;
        }

        counters[depth] += 1;
        // Reset all deeper counters
        for counter in counters.iter_mut().skip(depth + 1) {
            *counter = 0;
        }

        // Build "1.2.3" from counters[0..=depth]
        let num: String = counters[..=depth]
            .iter()
            .map(|c| c.to_string())
            .collect::<Vec<_>>()
            .join(".");
        numbers.push(num);
    }

    numbers
}

/// Inject section numbers into HTML heading tags.
/// Finds `<h{n} id="...">` and prepends `<span class="section-number">N.N</span> `.
fn inject_section_numbers(html: &str, entries: &[TocEntry], numbers: &[String]) -> String {
    let mut result = html.to_string();

    // Process in reverse order so earlier offsets aren't invalidated
    for (entry, number) in entries.iter().zip(numbers.iter()).rev() {
        if number.is_empty() {
            continue;
        }
        // Find the heading tag with this anchor id
        let id_pattern = format!("id=\"{}\"", entry.anchor_id);
        if let Some(id_pos) = result.find(&id_pattern) {
            // Find the closing > of the opening tag
            if let Some(tag_end) = result[id_pos..].find('>') {
                let insert_pos = id_pos + tag_end + 1;
                let number_html = format!("<span class=\"section-number\">{}</span> ", number);
                result.insert_str(insert_pos, &number_html);
            }
        }
    }

    result
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
    while let Some(a_start_rel) = result[search_from..].find("<a href=\"") {
        let a_start = search_from + a_start_rel;
        let href_start = a_start + 9; // after <a href="
        let Some(href_end) = result[href_start..].find('"') else {
            break;
        };
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
    while let Some(img_start_rel) = result[search_from..].find("<img src=\"") {
        let img_start = search_from + img_start_rel;
        let src_start = img_start + 10; // after <img src="
        let Some(src_end) = result[src_start..].find('"') else {
            break;
        };
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
    let (html, _toc, _title) =
        md_to_html_body_with_toc(markdown_input, show_frontmatter, false, 1, false);
    html
}

/// Build the complete HTML document from markdown content.
/// This is a pure function: no I/O, no randomness.
/// Takes markdown string, template string, seed, seed URL,
/// show_frontmatter flag, and theme class; returns complete HTML string.
const MATH_SCRIPTS: &str = r#"<link rel="stylesheet" href="/katex/katex.min.css">
    <script src="/katex/katex.min.js"></script>
    <script>
      function renderMath(){document.querySelectorAll('.math').forEach(function(el){var math=el.textContent;var displayMode=el.classList.contains('math-display');try{katex.render(math,el,{displayMode:displayMode,throwOnError:false});}catch(e){el.textContent=math;}});}
      document.addEventListener('DOMContentLoaded',renderMath);
    </script>"#;

pub fn build_html(
    markdown_input: &str,
    template: &str,
    seed: &str,
    seed_url: &str,
    show_frontmatter: bool,
    theme_class: &str,
    math: bool,
) -> String {
    let html_body = md_to_html_body(markdown_input, show_frontmatter);
    let mut result = template
        .replace("#{BODY}", &html_body)
        .replace("#{INITIALSEED}", seed)
        .replace("#{SEEDURL}", seed_url)
        .replace("#{THEME_CLASS}", theme_class);
    if math {
        result = result.replace("</head>", &format!("{}\n</head>", MATH_SCRIPTS));
    }
    result
}

pub fn to_html(
    infile: &str,
    output_dir: &std::path::Path,
    port: u16,
    show_frontmatter: bool,
    theme_class: &str,
    math: bool,
) {
    let markdown_input = fs::read_to_string(infile);
    if let Ok(markdown_input) = markdown_input {
        to_file(
            &markdown_input,
            output_dir,
            port,
            show_frontmatter,
            theme_class,
            math,
        )
    };
}

fn to_file(
    markdown_input: &str,
    output_dir: &std::path::Path,
    port: u16,
    show_frontmatter: bool,
    theme_class: &str,
    math: bool,
) {
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
            let html_complete = build_html(
                markdown_input,
                template,
                &seed,
                &seed_url,
                show_frontmatter,
                theme_class,
                math,
            );
            if let Err(e) = fs::write(output_dir.join(".temp.seed"), seed) {
                if e.kind() != std::io::ErrorKind::NotFound {
                    eprintln!("warning: could not write seed file: {}", e);
                }
                return;
            }
            if let Err(e) = fs::write(output_dir.join(".temp.html"), html_complete)
                && e.kind() != std::io::ErrorKind::NotFound
            {
                eprintln!("warning: could not write html file: {}", e);
            }
        }
        Err(_) => println!("URF this..no file"),
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
        let pod = Pod::Array(vec![Pod::String("a".to_string()), Pod::Integer(1)]);
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
        let (html, toc, _title) = md_to_html_body_with_toc(md, false, false, 1, false);

        assert_eq!(toc.len(), 3);
        assert_eq!(
            toc[0],
            TocEntry {
                level: 1,
                title: "Title".into(),
                anchor_id: "title".into()
            }
        );
        assert_eq!(
            toc[1],
            TocEntry {
                level: 2,
                title: "Section".into(),
                anchor_id: "section".into()
            }
        );
        assert_eq!(
            toc[2],
            TocEntry {
                level: 3,
                title: "Sub".into(),
                anchor_id: "sub".into()
            }
        );

        assert!(html.contains("<h1 id=\"title\">Title</h1>"));
        assert!(html.contains("<h2 id=\"section\">Section</h2>"));
        assert!(html.contains("<h3 id=\"sub\">Sub</h3>"));
    }

    #[test]
    fn test_md_to_html_body_with_toc_skipped_levels() {
        let md = "# Top\n\n### Skipped h2\n\n## Back to h2";
        let (_html, toc, _title) = md_to_html_body_with_toc(md, false, false, 1, false);

        assert_eq!(toc.len(), 3);
        assert_eq!(toc[0].level, 1);
        assert_eq!(toc[1].level, 3);
        assert_eq!(toc[2].level, 2);
    }

    #[test]
    fn test_md_to_html_body_with_toc_no_headings() {
        let md = "Just a paragraph.\n\nAnother one.";
        let (_html, toc, _title) = md_to_html_body_with_toc(md, false, false, 1, false);
        assert!(toc.is_empty());
    }

    #[test]
    fn test_md_to_html_body_with_toc_duplicate_headings() {
        let md = "# Intro\n\n## Intro\n\n### Intro";
        let (_html, toc, _title) = md_to_html_body_with_toc(md, false, false, 1, false);

        assert_eq!(toc[0].anchor_id, "intro");
        assert_eq!(toc[1].anchor_id, "intro-1");
        assert_eq!(toc[2].anchor_id, "intro-2");
    }

    #[test]
    fn test_md_to_html_body_with_toc_frontmatter_excluded() {
        let md = "---\ntitle: Test\n---\n\n# Real Heading";
        let (_html, toc, _title) = md_to_html_body_with_toc(md, true, false, 1, false);

        assert_eq!(toc.len(), 1);
        assert_eq!(toc[0].title, "Real Heading");
    }

    // compute_section_numbers tests
    #[test]
    fn test_section_numbers_basic_hierarchy() {
        let entries = vec![
            TocEntry {
                level: 1,
                title: "A".into(),
                anchor_id: "a".into(),
            },
            TocEntry {
                level: 2,
                title: "B".into(),
                anchor_id: "b".into(),
            },
            TocEntry {
                level: 2,
                title: "C".into(),
                anchor_id: "c".into(),
            },
            TocEntry {
                level: 1,
                title: "D".into(),
                anchor_id: "d".into(),
            },
            TocEntry {
                level: 2,
                title: "E".into(),
                anchor_id: "e".into(),
            },
            TocEntry {
                level: 3,
                title: "F".into(),
                anchor_id: "f".into(),
            },
        ];
        let nums = compute_section_numbers(&entries, 1);
        assert_eq!(nums, vec!["1", "1.1", "1.2", "2", "2.1", "2.1.1"]);
    }

    #[test]
    fn test_section_numbers_start_level_2() {
        let entries = vec![
            TocEntry {
                level: 1,
                title: "Title".into(),
                anchor_id: "t".into(),
            },
            TocEntry {
                level: 2,
                title: "A".into(),
                anchor_id: "a".into(),
            },
            TocEntry {
                level: 3,
                title: "B".into(),
                anchor_id: "b".into(),
            },
            TocEntry {
                level: 2,
                title: "C".into(),
                anchor_id: "c".into(),
            },
        ];
        let nums = compute_section_numbers(&entries, 2);
        assert_eq!(nums, vec!["", "1", "1.1", "2"]);
    }

    #[test]
    fn test_section_numbers_skipped_levels() {
        let entries = vec![
            TocEntry {
                level: 1,
                title: "A".into(),
                anchor_id: "a".into(),
            },
            TocEntry {
                level: 3,
                title: "B".into(),
                anchor_id: "b".into(),
            },
        ];
        let nums = compute_section_numbers(&entries, 1);
        assert_eq!(nums, vec!["1", "1.0.1"]);
    }

    #[test]
    fn test_section_numbers_single() {
        let entries = vec![TocEntry {
            level: 2,
            title: "A".into(),
            anchor_id: "a".into(),
        }];
        let nums = compute_section_numbers(&entries, 2);
        assert_eq!(nums, vec!["1"]);
    }

    #[test]
    fn test_section_numbers_empty() {
        let nums = compute_section_numbers(&[], 1);
        assert!(nums.is_empty());
    }

    #[test]
    fn test_section_numbers_with_toc_integration() {
        let md = "# Title\n\n## First\n\n### Sub\n\n## Second";
        let (_html, toc, _title) = md_to_html_body_with_toc(md, false, true, 2, false);
        // H1 has no number, H2 starts at 1
        assert_eq!(toc[0].title, "Title"); // no number
        assert_eq!(toc[1].title, "1 First");
        assert_eq!(toc[2].title, "1.1 Sub");
        assert_eq!(toc[3].title, "2 Second");
    }

    #[test]
    fn test_section_numbers_in_html() {
        let md = "# Heading One\n\n## Heading Two";
        let (html, _toc, _title) = md_to_html_body_with_toc(md, false, true, 1, false);
        assert!(html.contains("<span class=\"section-number\">1</span> Heading One"));
        assert!(html.contains("<span class=\"section-number\">1.1</span> Heading Two"));
    }

    #[test]
    fn test_frontmatter_title_present() {
        let md = "---\ntitle: My Document\n---\n\n# Heading";
        let (_html, _toc, title) = md_to_html_body_with_toc(md, false, false, 1, false);
        assert_eq!(title, Some("My Document".to_string()));
    }

    #[test]
    fn test_frontmatter_title_missing() {
        let md = "---\nauthor: John\n---\n\n# Heading";
        let (_html, _toc, title) = md_to_html_body_with_toc(md, false, false, 1, false);
        assert_eq!(title, None);
    }

    #[test]
    fn test_no_frontmatter() {
        let md = "# Just a heading\n\nSome content.";
        let (_html, _toc, title) = md_to_html_body_with_toc(md, false, false, 1, false);
        assert_eq!(title, None);
    }

    // Math rendering tests

    #[test]
    fn test_math_inline_produces_span() {
        let md = "The formula $x^2 + y^2 = z^2$ is well known.";
        let (html, _toc, _title) = md_to_html_body_with_toc(md, false, false, 1, true);
        assert!(html.contains("<span class=\"math math-inline\">"));
        assert!(html.contains("x^2 + y^2 = z^2"));
    }

    #[test]
    fn test_math_display_produces_span() {
        let md = "$$\\int_0^\\infty e^{-x} dx = 1$$";
        let (html, _toc, _title) = md_to_html_body_with_toc(md, false, false, 1, true);
        assert!(html.contains("<span class=\"math math-display\">"));
    }

    #[test]
    fn test_math_disabled_no_spans() {
        let md = "The formula $x^2$ and $$y^2$$ should not be math.";
        let (html, _toc, _title) = md_to_html_body_with_toc(md, false, false, 1, false);
        assert!(!html.contains("math-inline"));
        assert!(!html.contains("math-display"));
    }

    #[test]
    fn test_math_in_code_block_not_rendered() {
        let md = "```\n$not math$\n$$also not math$$\n```";
        let (html, _toc, _title) = md_to_html_body_with_toc(md, false, false, 1, true);
        assert!(!html.contains("math-inline"));
        assert!(!html.contains("math-display"));
    }

    #[test]
    fn test_math_in_inline_code_not_rendered() {
        let md = "Use `$variable` in your code.";
        let (html, _toc, _title) = md_to_html_body_with_toc(md, false, false, 1, true);
        assert!(!html.contains("math-inline"));
    }

    #[test]
    fn test_math_in_heading_toc_plain() {
        let md = "# The $E = mc^2$ equation";
        let (_html, toc, _title) = md_to_html_body_with_toc(md, false, false, 1, true);
        assert_eq!(toc.len(), 1);
        // TOC title should have plain text only (no raw TeX)
        assert_eq!(toc[0].title, "The  equation");
    }
}
