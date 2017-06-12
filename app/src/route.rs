use webplatform::{HtmlNode};
use maud::{PreEscaped};

use view::*;

pub fn route(mut location: String, app: HtmlNode){
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