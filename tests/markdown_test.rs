use mip::markdown::{md_to_html_body, md_to_html_body_with_toc, build_html};

#[test]
fn test_md_to_html_body_headings() {
    let md = "# Heading 1\n\n## Heading 2\n\nParagraph text.";
    let html = md_to_html_body(md, false);
    assert!(html.contains("<h1 id=\"heading-1\">Heading 1</h1>"));
    assert!(html.contains("<h2 id=\"heading-2\">Heading 2</h2>"));
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
    assert!(html.contains("<h1 id=\"content\">Content</h1>"));
}

#[test]
fn test_md_to_html_body_frontmatter_shown() {
    let md = "---\ntitle: Hello\nauthor: Test\n---\n\n# Content";
    let html = md_to_html_body(md, true);
    assert!(html.contains("<table class=\"frontmatter\">"));
    assert!(html.contains("title"));
    assert!(html.contains("Hello"));
    assert!(html.contains("<h1 id=\"content\">Content</h1>"));
}

#[test]
fn test_build_html_replaces_placeholders() {
    let template = "<html class=\"#{THEME_CLASS}\"><body>#{BODY}<script>var seedUrl=\"#{SEEDURL}\";var initialSeed=\"#{INITIALSEED}\";</script></body></html>";
    let md = "# Hello";
    let result = build_html(md, template, "abc1234", "http://localhost:8000/.temp.seed", false, "dark");

    assert!(result.contains("<h1 id=\"hello\">Hello</h1>"));
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

#[test]
fn test_md_to_html_body_with_toc_realistic() {
    let md = r#"# Introduction

Some text here.

## Getting Started

### Prerequisites

Install Rust.

### Installation

Run cargo install.

## Usage

### Basic Usage

Just run it.

### Advanced Usage

- [x] Feature A
- [ ] Feature B

## FAQ

| Q | A |
|---|---|
| Why? | Because |
"#;
    let (html, toc) = md_to_html_body_with_toc(md, false);

    assert_eq!(toc.len(), 8);
    assert_eq!(toc[0].title, "Introduction");
    assert_eq!(toc[0].level, 1);
    assert_eq!(toc[1].title, "Getting Started");
    assert_eq!(toc[1].level, 2);
    assert_eq!(toc[2].title, "Prerequisites");
    assert_eq!(toc[2].level, 3);
    assert_eq!(toc[3].title, "Installation");
    assert_eq!(toc[3].level, 3);
    assert_eq!(toc[4].title, "Usage");
    assert_eq!(toc[4].level, 2);
    assert_eq!(toc[7].title, "FAQ");

    // All headings have id attributes
    assert!(html.contains("<h1 id=\"introduction\">"));
    assert!(html.contains("<h2 id=\"getting-started\">"));
    assert!(html.contains("<h3 id=\"prerequisites\">"));

    // GFM extensions still work
    assert!(html.contains("checked=\"\""));
    assert!(html.contains("<table>"));
}

#[test]
fn test_md_to_html_body_with_toc_with_frontmatter() {
    let md = "---\ntitle: My Doc\n---\n\n# Real Heading\n\n## Sub Heading";
    let (html, toc) = md_to_html_body_with_toc(md, true);

    assert_eq!(toc.len(), 2);
    assert_eq!(toc[0].title, "Real Heading");
    assert_eq!(toc[1].title, "Sub Heading");
    assert!(html.contains("<table class=\"frontmatter\">"));
    assert!(html.contains("<h1 id=\"real-heading\">"));
}

#[test]
fn test_md_to_html_body_with_toc_code_in_heading() {
    let md = "## The `main` function";
    let (_html, toc) = md_to_html_body_with_toc(md, false);

    assert_eq!(toc.len(), 1);
    assert_eq!(toc[0].title, "The main function");
    assert_eq!(toc[0].anchor_id, "the-main-function");
}
