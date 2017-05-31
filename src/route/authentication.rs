use maud::Markup;
use rocket::request:: Form;
use rocket::response::{Flash, Redirect};
use rocket::http::{Cookie, Cookies};
use service::user::UserService;
use rocket::request::FlashMessage;
use time;

use view;

#[get("/login")]
fn login(user_service: UserService, flash: Option<FlashMessage>) -> Markup {
	view::login::login(user_service.user, flash)
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
		return Ok(Redirect::to("/member#"));
	}
	Err(Flash::error(Redirect::to("/login"), "Invalid username or password"))
}
