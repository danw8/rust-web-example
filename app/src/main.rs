#![feature(plugin)]
#![plugin(maud_macros)]

extern crate maud;
extern crate libc;
#[macro_use] extern crate webplatform;
#[macro_use] extern crate serde_derive;
extern crate serde;
extern crate serde_json;

mod wire;
mod route;
mod view;


//use std::sync::{Arc, Mutex};
use std::rc::Rc;

use webplatform::{Event};


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

    document.on("hashchange", enclose!{ 
        (document) move |_:Event| {
            //route(document);
            let l = document.location_hash_get();
            let a = document.element_query("#app").unwrap();
            route::route(l, a);
        }
    });
    let location = document.location_hash_get();
    let app = document.element_query("#app").unwrap();
    route::route(location, app);

    webplatform::spin();
}