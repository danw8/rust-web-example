#![feature(plugin)]
#![plugin(maud_macros)]

extern crate maud;
extern crate webplatform;

use std::sync::{Arc, Mutex};
use maud::{Markup, PreEscaped};

fn main() {
    let document = webplatform::init();
    let document = Arc::new(document);

    let loc_hash = document.location_hash_get();

    let app = document.element_query("#app").unwrap();
    let PreEscaped(markup) = member(loc_hash);
    app.html_set(&markup);

   document.on("hashchange",|_| webplatform::alert("WITNESS ME"));
    
    webplatform::spin();
}

fn member(hash: String) -> Markup{
    html!{
        h1 {
            "Hello, Emscripten!!!!"
        }
        h3 {
            "Got here by " (hash)
        }
    }
}
