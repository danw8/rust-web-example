use webplatform::{HtmlNode, Document};
use std::rc::Rc;

use component::member_home;
use component::member_list;

pub fn route(mut location: String, app: HtmlNode, document: Rc<Document>){
    if location.ends_with('/'){
        location.pop();
    }
    println!("{}", location);
    let path = if location.len() < 2 {
        vec!["".to_string()]
    } else {
        location[2..].split("/").filter(|x| x.len() > 0).map(|x| x.to_string()).collect::<Vec<_>>()
    };

    match &*path[0] {
        "list" => member_list::member_list(app, document),
        _ => member_home::member_home(app),
    };
}