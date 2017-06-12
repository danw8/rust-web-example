use maud::{Markup};
use wire;

pub fn member_home() -> Markup{
    let username = wire::http_get("getuser");
    println!("{}", username);
    html! {
        h1 {
            "Welcome, " (username)
        }
    }
}

pub fn member_list() -> Markup{
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