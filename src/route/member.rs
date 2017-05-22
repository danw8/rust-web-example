use maud::Markup;
use rocket::http::{Cookie, Cookies};
use rocket::response::{Flash, Redirect};
use service::user::UserService;

#[get("/member")]
fn member(cookies: &Cookies, mut user_service: UserService) -> Result<Markup, Flash<Redirect>> {
	if !user_service.auth_with_cookie(cookies) {
		return Err(Flash::error(Redirect::to("/login"), "You are not authorized in the member area."));
	}
	let markup = html! {
		head{
			link rel="stylesheet" type="text/css" href=("files/style/index.css") /
		}
		body{
			h1 {
				"Welcome, " (user_service.user.unwrap().username)
			}
		}
	};
	return Ok(markup)
}
