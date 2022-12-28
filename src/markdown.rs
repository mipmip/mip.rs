use std::fs;
use rand::{distributions::Alphanumeric, Rng}; // 0.8
use rust_embed::RustEmbed;
use gray_matter::Matter;
use gray_matter::engine::YAML;
use pulldown_cmark::{html, Options, Parser};

#[derive(RustEmbed)]
#[folder = "asset/theme1"]
struct Asset;

pub fn to_html(path_dir: &std::path::Path, infile: &String, port: u16 ){

    let markdown_input = fs::read_to_string(infile);
    match markdown_input {
        Ok(markdown_input) => to_file(path_dir, &markdown_input, port),
        Err(_) => {}
    };
}

fn to_file(path_dir: &std::path::Path, markdown_input: &String, port: u16 ){
    let seed_url = format!("http://localhost:{}/.temp.seed", port);

    let matter = Matter::<YAML>::new();
    let result = matter.parse(&markdown_input);

    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TASKLISTS);
    options.insert(Options::ENABLE_TABLES);
    let parser = Parser::new_ext(&result.content, options);

    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);


    let seed :String = rand::thread_rng()
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
            fs::write(path_dir.join(".temp.seed"), seed).expect("Unable to write file");
            fs::write(path_dir.join(".temp.html"), html_complete3).expect("Unable to write file");
        },
        Err(_) => println!("URF this..no file")
    };
}
