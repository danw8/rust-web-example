use maud::Markup;
use super::partial::*;
use data::model::user::User;
use rocket::request::FlashMessage;

pub fn login(user: Option<User>, flash: Option<FlashMessage>) -> Markup {
	html!{
		head{
			link rel="stylesheet" type="text/css" href=("files/style/login.css") /
		}
		body{
			(navbar::navbar(&user))
			div.center {
				div.login {
					h1 {
						"Login"
					}
					@if let Some(f) = flash {
						p {
							(f.msg())
						}
					}
					form action="/verify" method="post" accept-charset="utf-8"{
						div {
							label for="username"{ "Username" }
							input id="username" name="username" type="text" /
						}
						div {
							label for="password" { "Password" }
							input id="password" name="password" type="password" /
						}
						button id="submit" type="submit" { "Login" }
						"Don't have an account?"
						a href="/account_creation" {
							"Create One!"
						}
					}
				}
			}
		}
	}
}