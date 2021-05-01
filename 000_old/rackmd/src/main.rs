extern crate markdown;
extern crate web_view;
use web_view::*;

fn main() {
    let html: String =
        markdown::to_html("[__I am markdown__](https://www.google.com/webhp?hl=zh-TW)");
    let size = (800, 600);
    let resizable = true;
    let debug = true;
    let init_cb = |_webview| {};
    let frontend_cb = |_webview: &mut _, _arg: &_, _userdata: &mut _| {};
    let userdata = ();
    run(
        "Minimal webview example",
        Content::Html(html),
        Some(size),
        resizable,
        debug,
        init_cb,
        frontend_cb,
        userdata
       );
}
