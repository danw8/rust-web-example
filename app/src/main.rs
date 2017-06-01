#![feature(plugin)]
#![plugin(maud_macros)]

extern crate maud;
extern crate libc;
#[macro_use] extern crate webplatform;

//use std::sync::{Arc, Mutex};
use std::rc::Rc;
use std::str;
use std::ffi::{CString, CStr};
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

trait Interop {
    fn as_int(self, _:&mut Vec<CString>) -> libc::c_int;
}

impl<'a> Interop for &'a str {
    fn as_int(self, arena:&mut Vec<CString>) -> libc::c_int {
        let c = CString::new(self).unwrap();
        let ret = c.as_ptr() as libc::c_int;
        arena.push(c);
        return ret;
    }
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
    let username = http_get("getuser");
    println!("{}", username);
    html! {
        h1 {
            "Welcome, " (username)
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

fn http_get(url: &str) -> String{
    let a = js!{ (url) b"\
            var xmlHttp = new XMLHttpRequest();\
            xmlHttp.open( 'GET', UTF8ToString($0), false );\
            xmlHttp.send( null );\
            return allocate(intArrayFromString(xmlHttp.responseText), 'i8', ALLOC_STACK);\
        \0" };
    println!("{:?}", a);
    unsafe {
        str::from_utf8(CStr::from_ptr(a as *const libc::c_char).to_bytes()).unwrap().to_owned()
    }
}
