use maud::Markup;
use rocket::response::{Flash, Redirect};
use rocket_contrib::{JSON, Value};
use service::user::UserService;
use service::card::CardService;
use data::model::card::Card;
use view;
use serde_json;

#[get("/member#")]
fn member(user_service: UserService) -> Result<Markup, Flash<Redirect>> {
	if user_service.user.is_none() {
		return Err(Flash::error(Redirect::to("/login"), "You are not authorized in the member area."));
	}
	Ok(view::member::member(user_service.user))
}

#[get("/getuser")]
fn getuser(user_service: UserService) -> String {
	if let Some(u) = user_service.user {
		return u.username;
	}
	String::new()
}

#[get("/usercards")]
fn get_user_cards(user_service: UserService, card_service: CardService) -> JSON<Value> {
	if let Some(u) = user_service.user {
		return JSON(json!(card_service.get_user_cards(u.id)));
	}
	return JSON(json!(Vec::<Card>::new()));
}
