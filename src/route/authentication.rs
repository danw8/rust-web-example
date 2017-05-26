use maud::Markup;
use rocket::request:: Form;
use rocket::response::{Flash, Redirect};
use rocket::http::{Cookie, Cookies};
use service::user::UserService;
use rocket::request::FlashMessage;
use time;

use view::partial::*;

#[get("/login")]
fn login(user_service: UserService, flash: Option<FlashMessage>) -> Markup {
	html! {
		head{
			link rel="stylesheet" type="text/css" href=("files/style/login.css") /
		}
		body{
			(navbar(&user_service.user))
			div.login{
				form.login-form action="/verify" method="post" accept-charset="utf-8"{
					h1.login-header {
						"Login"
					}
					div.login-body {
						@if let Some(f) = flash {
							p.error {
									(f.msg())
							}
						}
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
}

#[get("/logout")]
pub fn logout(cookies: &Cookies) -> Redirect {
	cookies.remove("verified");
	Redirect::to("/")
}

#[derive(FromForm)]
struct Credentials {
    username: String,
    password: String,
}

#[post("/verify", data = "<creds>")]
fn verify(cookies: &Cookies, mut user_service: UserService, creds: Form<Credentials>) -> Result<Redirect, Flash<Redirect>> {
	let creds = creds.into_inner();
	let verified = user_service.authenticate(&creds.username, &creds.password);

	if verified {
		let expire_time = time::now() + time::Duration::minutes(30);
		let cookie = Cookie::build("verified", creds.username)
			.domain("localhost")
			.path("/")
			//.secure(true)
			.http_only(true)
			.expires(expire_time)
			.finish();
		cookies.add(cookie);
		return Ok(Redirect::to("/member"));
	}
	Err(Flash::error(Redirect::to("/login"), "Invalid username or password"))
}
