#![feature(plugin)]
#![plugin(maud_macros)]

extern crate maud;
extern crate webplatform;

use std::sync::{Arc, Mutex};
use maud::{Markup, PreEscaped};

fn main() {
    let document = webplatform::init();
    let document = Arc::new(document);

    let app = document.element_query("#app").unwrap();
    let PreEscaped(markup) = member();
    app.html_set(&markup);
    
    webplatform::spin();
}

fn member() -> Markup{
    html!{
        "Hello, Emscripten!!!!"
    }
}
