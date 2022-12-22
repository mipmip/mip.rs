use std::fs;

pub fn to_html(infile: &String){

    let markdown_input = fs::read_to_string(infile);
    match markdown_input {
        Ok(markdown_input) => to_file(&markdown_input),
        Err(_) => println!("REMOVE this..no file")
    };
}

fn to_file(markdown_input: &String){
    let parser = pulldown_cmark::Parser::new(&markdown_input);

    let mut html_output = String::new();
    pulldown_cmark::html::push_html(&mut html_output, parser);
    fs::write("./.seed.html", html_output).expect("Unable to write file");
}



