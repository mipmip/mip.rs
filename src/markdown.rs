use std::fs;
use rand::Rng;
use rand::distr::Alphanumeric;
use rust_embed::Embed;
use gray_matter::Matter;
use gray_matter::engine::YAML;
use pulldown_cmark::{html, Options, Parser};

#[derive(Embed)]
#[folder = "asset/theme1"]
struct Asset;

/// Convert raw markdown text to HTML body (without template wrapper).
pub fn md_to_html_body(markdown_input: &str) -> String {
    let matter = Matter::<YAML>::new();
    let result = matter.parse(markdown_input);

    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TASKLISTS);
    options.insert(Options::ENABLE_TABLES);
    let parser = Parser::new_ext(&result.content, options);

    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);
    html_output
}

pub fn to_html(infile: &str, output_dir: &std::path::Path, port: u16 ){

    let markdown_input = fs::read_to_string(infile);
    if let Ok(markdown_input) = markdown_input { to_file(&markdown_input, output_dir, port) };
}

fn to_file(markdown_input: &str, output_dir: &std::path::Path, port: u16 ){
    let seed_url = format!("http://localhost:{}/.temp.seed", port);

    let matter = Matter::<YAML>::new();
    let result = matter.parse(markdown_input);

    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TASKLISTS);
    options.insert(Options::ENABLE_TABLES);
    let parser = Parser::new_ext(&result.content, options);

    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);

    let seed: String = rand::rng()
        .sample_iter(&Alphanumeric)
        .take(7)
        .map(char::from)
        .collect();

    let index_html = Asset::get("template.html").unwrap();
    let index_html_str = std::str::from_utf8(index_html.data.as_ref());
    match index_html_str {
        Ok(index_html_str) => {
            let html_complete1 = index_html_str.replace("#{BODY}", &html_output);
            let html_complete2 = html_complete1.replace("#{INITIALSEED}", &seed);
            let html_complete3 = html_complete2.replace("#{SEEDURL}", &seed_url);
            if let Err(e) = fs::write(output_dir.join(".temp.seed"), seed) {
                eprintln!("warning: could not write seed file: {}", e);
                return;
            }
            if let Err(e) = fs::write(output_dir.join(".temp.html"), html_complete3) {
                eprintln!("warning: could not write html file: {}", e);
            }
        },
        Err(_) => println!("URF this..no file")
    };
}
