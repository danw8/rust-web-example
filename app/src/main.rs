#![feature(plugin)]
#![plugin(maud_macros)]

extern crate maud;
extern crate webplatform;

//use std::sync::{Arc, Mutex};
use std::rc::Rc;
use maud::{Markup, PreEscaped};
use webplatform::{Event, Document};

macro_rules! enclose {
    ( ($( $x:ident ),*) $y:expr ) => {
        {
            $(let $x = $x.clone();)*
            $y
        }
    };
}

fn main(){
    let document = Rc::new(webplatform::init());

    let loc_hash = document.location_hash_get();

    let app = document.element_query("#app").unwrap();
    let PreEscaped(markup) = member(loc_hash);
    app.html_set(&markup);

    document.on("hashchange", enclose!{ 
        (document) move |_:Event| {
            //route(document);
            let h = document.location_hash_get();
            let a = document.element_query("#app").unwrap();
            let PreEscaped(markup) = member(h);
            a.html_set(&markup);
        }
    });
    
    webplatform::spin();
}

// fn route(document: Rc<Document>){
//     let loc_hash = document.location_hash_get();
//     let app = document.element_query("#app").unwrap();
//     let PreEscaped(markup) = member(loc_hash);
//     app.html_set(&markup);
// }

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
