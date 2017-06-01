#![feature(plugin)]
#![plugin(maud_macros)]

extern crate maud;
extern crate webplatform;

//use std::sync::{Arc, Mutex};
use std::rc::Rc;
use maud::{Markup, PreEscaped};
use webplatform::{Event, Document, HtmlNode};


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
            route(l, a);
        }
    });
    let mut location = document.location_hash_get();
    let app = document.element_query("#app").unwrap();
    route(location, app);

    webplatform::spin();
}

fn route(mut location: String, app: HtmlNode){
    if location.ends_with('/'){
        location.pop();
    }
    println!("{}", location);
    let path = if location.len() < 2 {
        vec!["".to_string()]
    } else {
        location[2..].split("/").filter(|x| x.len() > 0).map(|x| x.to_string()).collect::<Vec<_>>()
    };

    let PreEscaped(markup) = match &*path[0] {
        "list" => member_list(),
        _ => member_home(),
    };
    //let PreEscaped(markup) = member(location);
    app.html_set(&markup);
}

fn member_home() -> Markup{
    html! {
        h1 {
            "Welcome, Member!"
        }
    }
}

fn member_list() -> Markup{
    html! {
        h1 {
            "Your List"
        }
        ul {
            li { "item 1"}
            li { "item 2"}
            li { "item 3"}
        }
    }
}
