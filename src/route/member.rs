use std::path::PathBuf;
use maud::Markup;
use rocket::response::{Flash, Redirect};
use service::user::UserService;
use view;

#[get("/member#")]
fn member(user_service: UserService) -> Result<Markup, Flash<Redirect>> {
	if user_service.user.is_none() {
		return Err(Flash::error(Redirect::to("/login"), "You are not authorized in the member area."));
	}
	Ok(view::member::member(user_service.user))
}

// #[get("/member/<path..>")]
// fn member_path(user_service: UserService, path: PathBuf) -> Result<Markup, Flash<Redirect>> {
// 	if user_service.user.is_none() {
// 		return Err(Flash::error(Redirect::to("/login"), "You are not authorized in the member area."));
// 	}
// 	Ok(view::member::member(user_service.user))
// }
