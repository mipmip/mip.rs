use web_view::*;

fn main() {
    let html_content = "<html><body><h1>Hello, MIP!</h1></body></html>";

    web_view::builder()
        .title("My Project")
        .content(Content::Html(html_content))
        .size(320, 480)
        .resizable(true)
        .debug(true)
        .user_data(())
        .invoke_handler(|_webview, _arg| Ok(()))
        .run()
        .unwrap();
}
