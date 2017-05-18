use diesel::prelude::*;
use maud::Markup;
use data::database::Database;
use data::model::user::User;
use rocket::request::Form;
use rocket::response::Redirect;
use rocket::http::{Cookie, Cookies};

#[get("/login")]
fn login() -> Markup {
	html! {
		head{
			link rel="stylesheet" type="text/css" href=("files/style/login.css") /
		}
		body{
			form.login-form action="/verify" method="post" accept-charset="utf-8"{
				h1.login-header {
					"Login"
				}
				div.login-body {
					div.login-field{
						label for="username"{ "Username" }
						input id="username" name="username" type="text" /
					}
					div.login-field {
						label for="password" { "Password" }
						input id="password" name="password" type="password" /
					}
					button.login-button id="submit" type="submit" { "Login" }
				}
			}
		}
	}
}

#[derive(FromForm)]
struct Credentials {
    username: String,
    password: String,
}

#[post("/verify", data = "<creds>")]
fn verify(cookies: &Cookies, creds: Form<Credentials>) -> Redirect {
	//Check if creds are valid

	// add the cookie
	cookies.add(Cookie::new("verified", creds.into_inner().username ));

	//redirect to memeber area
	Redirect::to("/")
}
