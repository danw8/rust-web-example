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
                    "Check out your "
                    a href="#/list" {
                        "list"
                    }
                    " of cards. It's really the only thing that you can do in the member area"
                }
            }
        }

    }
}