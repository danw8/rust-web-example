use maud::Markup;
use data::model::user::User;

pub fn navbar(user: &Option<User>) -> Markup{
    html!{
        header {
            h1 {
                a href="/" {
                    "Rust Web Example"
                }
            }
            ul {
                @if let &Some(ref u) = user {
                    li {
                        "Hi " (u.username) "."
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