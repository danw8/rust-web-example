use maud::{Markup, PreEscaped};
use rocket::http::{Cookie, Cookies};
use service::user::UserService;
use view::partial::*;

#[get("/")]
fn index(cookies: &Cookies, mut user_service: UserService) -> Markup {
	user_service.auth_with_cookie(cookies);
	html! {
		head{
			link rel="stylesheet" type="text/css" href=("files/style/index.css") /
		}
		body{
			(navbar(user_service.user))
			h1 {
				"Hello, World"
			}
		}
	}
}

