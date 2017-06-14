use maud::{Markup};
use wire;

pub fn member_home() -> Markup{
    let username = wire::http_get("getuser");
    println!("{}", username);
    html! {
        div.center.margin-md  {
            div.member-container {
                div{
                    h1 {
                        "Welcome, " (username)
                    }
                }
            }
        }

    }
}

pub fn member_list() -> Markup{
    html! {
        div.center.margin-md  {
            div.list-container {
                div{
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
        }
    }
}