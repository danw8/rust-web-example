use maud::Markup;
use rocket::request:: Form;
use rocket::response::{Flash, Redirect};
use rocket::http::{Cookie, Cookies};
use service::user::UserService;
use rocket::request::FlashMessage;

#[get("/login")]
fn login(flash: Option<FlashMessage>) -> Markup {
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
					@if flash.is_some() {
						p.error {
								(flash.unwrap().msg())
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

#[derive(FromForm)]
struct Credentials {
    username: String,
    password: String,
}

#[post("/verify", data = "<creds>")]
fn verify(cookies: &Cookies, user_service: UserService, creds: Form<Credentials>) -> Result<Redirect, Flash<Redirect>> {
	let creds = creds.into_inner();
	let verified = user_service.authenticate(&creds.username, &creds.password);

	if verified {
		cookies.add(Cookie::new("verified", creds.username ));
		//redirect to memeber area
		return Ok(Redirect::to("/member"));
	}
	Err(Flash::error(Redirect::to("/login"), "Invalid username or password"))
}
