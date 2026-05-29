## 1. Implement media rewriting

- [x] 1.1 Add `rewrite_media_embeds` function in `markdown.rs` that takes HTML string and returns rewritten HTML
- [x] 1.2 Detect `<a href="...ext">...</a>` patterns where ext is `.webm`, `.mp4`, `.mov`, `.ogv` and rewrite to `<video src="..." controls style="max-width:100%"></video>`
- [x] 1.3 Detect `<img src="...ext"` patterns where ext is a video extension and rewrite to `<video>` tag
- [x] 1.4 Call `rewrite_media_embeds` on html_output after `push_html` in both `to_file` and `md_to_html_body`

## 2. Test example

- [x] 2.1 Add `examples/with-video.md` containing link-style and image-style video references for manual testing

## 3. Verify

- [x] 3.1 `cargo build` succeeds
- [x] 3.2 `mip README.md` renders — the two webm references on lines 16 and 18 show as video players
- [x] 3.3 Normal images (`mip.png` in README) still render as images
- [x] 3.4 `mip examples/with-video.md` shows video elements for video references
