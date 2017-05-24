use maud::Markup;
use data::model::user::User;

pub fn navbar(user: Option<User>) -> Markup{
    html!{
        header {
            h1 {
                a href="/" {
                    "Rust Web Example"
                }
            }
            ul {
                @if user.is_some() {
                    li {
                        "Hi " (user.unwrap().username) "."
                    }
                    li {
                        a href="/logout" {
                            "Logout"
                        }
                    }
                } @else {
                    li {
                        a href="/login" {
                            "Login"
                        }
                    }
                }
            }
        }
    }
}