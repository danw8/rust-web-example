use maud::Markup;
use super::partial::*;
use data::model::user::User;
use rocket::request::FlashMessage;

pub fn account_creation(user: Option<User>, flash: Option<FlashMessage>) -> Markup {
	html!{
		head{
			link rel="stylesheet" type="text/css" href=("files/style/account_creation.css") /
		}
		body{
			(navbar::navbar(&user))
			div.center {
				div.create {
					h1 {
						"Create an account"
					}
					@if let Some(f) = flash {
						p {
							(f.msg())
						}
					}
					form action="/adduser" method="post" accept-charset="utf-8"{
						div{
							label for="username"{ "Username" }
							input id="username" name="username" type="text" /
						}
						div {
							label for="email" { "Email" }
							input id="email" name="email" type="text" /
						}
						div {
							label for="password" { "Password" }
							input id="password" name="password" type="password" /
						}
						div {
							label for="confirm" { "Confirm Password" }
							input id="confirm" name="confirm" type="password" /
						}
						button id="submit" type="submit" { "Create" }
					}
				}
			}
		}
	}
}