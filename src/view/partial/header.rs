use maud::Markup;
use data::model::user::User;

pub fn navbar(user: Option<User>) -> Markup{
    html!{
        h1 {
            "Rust Web Example"
        }
        ul {
            @if user.is_some() {
                li {
                    (user.unwrap().username)
                }
                li {
                    a href="/logout" {
                        "logout"
                    }
                }
            } @else {
                li {
                    a href="/login" {
                        "login"
                    }
                }
            }
        }
    }
}