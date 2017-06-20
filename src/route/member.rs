use maud::Markup;
use rocket::response::{Flash, Redirect};
use rocket_contrib::{JSON, Value};
use service::user::UserService;
use service::card::CardService;
use data::model::card::{Card, NewCard, CardStatus};
use view;

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

#[get("/deletecard/<id>")]
fn deletecard(user_service: UserService, card_service: CardService, id: i32) -> String {
	if let Some(user) = user_service.user {
		let can_delete = card_service.user_owns_card(id, user.id);
		if can_delete {
			if card_service.delete_card(id) {
				return "success".to_string();
			}
		}
	}
	"failure".to_string()
}

#[post("/updatecard", format = "application/json", data = "<card>")]
fn updatecard(user_service: UserService, card_service: CardService, card: JSON<Card>) -> String{
	if let Some(user) = user_service.user {
		let card = card.0;
		let can_update = card_service.user_owns_card(card.id, user.id);
		if can_update {
			let _ = match card_service.update_card(card) {
				Ok(c) => c,
				Err(e) => return e,
			};
			return "success".to_string();
		}
	}
	"failure".to_string()
}

#[post("/addcard", format = "application/json", data = "<card>")]
fn addcard(user_service: UserService, card_service: CardService, card: JSON<NewCard>) -> JSON<Value> {
	if let Some(user) = user_service.user {
		let status = card.0.get_status();
		let new_card = match card_service.add_card(user.id, card.0.title, card.0.description, status){
			Ok(c) => c,
			Err(e) => {
				return JSON(json!(Failure { message: "Couldn't add card".to_string()} ))
			},
		};
		return JSON(json!(new_card));
	}
	return JSON(json!(Failure { message: "Couldn't add card".to_string()} ))
}

#[derive(Deserialize, Serialize,)]
struct Failure {
	message: String
}
