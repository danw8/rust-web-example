use maud::{Markup, PreEscaped};
use rocket::http::{Cookie, Cookies};
use service::user::UserService;
use view;

#[get("/")]
fn index(cookies: &Cookies, mut user_service: UserService) -> Markup {
	user_service.auth_with_cookie(cookies);
	view::index::index(user_service.user)
}

