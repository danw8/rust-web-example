use maud::Markup;
use service::user::UserService;
use rocket::request::Form;
use rocket::request::FlashMessage;
use rocket::response::{Flash, Redirect};
use view;

#[get("/account_creation")]
fn account_creation(user_service: UserService, flash: Option<FlashMessage>) -> Markup {
	view::account_creation::account_creation(user_service.user, flash)
}

#[derive(FromForm)]
struct AddUser {
    username: String,
    email: String,
    password: String,
	confirm: String,
}

#[post("/adduser", data = "<add_user>")]
fn adduser(user_service: UserService, add_user: Form<AddUser>) -> Result<Redirect, Flash<Redirect>> {
	let user = add_user.into_inner();
	if user.password != user.confirm {
		return Err(Flash::error(Redirect::to("/account_creation"), "Passwords did not match"));
	}
	match user_service.create_user(&user.username, &user.email, &user.password){
		Ok(_) => return Ok(Redirect::to("/login")),
		Err(e) => Err(Flash::error(Redirect::to("/account_creation"), e))
	}
}
