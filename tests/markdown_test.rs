use mip::markdown::{md_to_html_body, build_html};

#[test]
fn test_md_to_html_body_headings() {
    let md = "# Heading 1\n\n## Heading 2\n\nParagraph text.";
    let html = md_to_html_body(md, false);
    assert!(html.contains("<h1>Heading 1</h1>"));
    assert!(html.contains("<h2>Heading 2</h2>"));
    assert!(html.contains("<p>Paragraph text.</p>"));
}

#[test]
fn test_md_to_html_body_inline_formatting() {
    let md = "This is **bold** and *italic* text.";
    let html = md_to_html_body(md, false);
    assert!(html.contains("<strong>bold</strong>"));
    assert!(html.contains("<em>italic</em>"));
}

#[test]
fn test_md_to_html_body_strikethrough() {
    let md = "This is ~~deleted~~ text.";
    let html = md_to_html_body(md, false);
    assert!(html.contains("<del>deleted</del>"));
}

#[test]
fn test_md_to_html_body_task_list() {
    let md = "- [x] Done\n- [ ] Todo";
    let html = md_to_html_body(md, false);
    assert!(html.contains("checked=\"\""));
    assert!(html.contains("type=\"checkbox\""));
}

#[test]
fn test_md_to_html_body_table() {
    let md = "| A | B |\n|---|---|\n| 1 | 2 |";
    let html = md_to_html_body(md, false);
    assert!(html.contains("<table>"));
    assert!(html.contains("<th>"));
    assert!(html.contains("<td>"));
}

#[test]
fn test_md_to_html_body_frontmatter_hidden() {
    let md = "---\ntitle: Hello\nauthor: Test\n---\n\n# Content";
    let html = md_to_html_body(md, false);
    assert!(!html.contains("frontmatter"));
    assert!(!html.contains("title"));
    assert!(html.contains("<h1>Content</h1>"));
}

#[test]
fn test_md_to_html_body_frontmatter_shown() {
    let md = "---\ntitle: Hello\nauthor: Test\n---\n\n# Content";
    let html = md_to_html_body(md, true);
    assert!(html.contains("<table class=\"frontmatter\">"));
    assert!(html.contains("title"));
    assert!(html.contains("Hello"));
    assert!(html.contains("<h1>Content</h1>"));
}

#[test]
fn test_build_html_replaces_placeholders() {
    let template = "<html class=\"#{THEME_CLASS}\"><body>#{BODY}<script>var seedUrl=\"#{SEEDURL}\";var initialSeed=\"#{INITIALSEED}\";</script></body></html>";
    let md = "# Hello";
    let result = build_html(md, template, "abc1234", "http://localhost:8000/.temp.seed", false, "dark");

    assert!(result.contains("<h1>Hello</h1>"));
    assert!(!result.contains("#{BODY}"));
    assert!(!result.contains("#{INITIALSEED}"));
    assert!(!result.contains("#{SEEDURL}"));
    assert!(!result.contains("#{THEME_CLASS}"));
    assert!(result.contains("abc1234"));
    assert!(result.contains("http://localhost:8000/.temp.seed"));
    assert!(result.contains("dark"));
}

#[test]
fn test_build_html_applies_theme_class() {
    let template = "<html class=\"#{THEME_CLASS}\"><body>#{BODY}</body></html>";
    let result = build_html("hello", template, "s", "u", false, "light");
    assert!(result.contains("class=\"light\""));
}

#[test]
fn test_build_html_with_frontmatter() {
    let template = "<html>#{BODY}</html>";
    let md = "---\ntitle: Test\n---\n\nBody text";
    let result = build_html(md, template, "s", "u", true, "light");
    assert!(result.contains("<table class=\"frontmatter\">"));
    assert!(result.contains("title"));
}
