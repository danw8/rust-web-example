use r2d2::{ GetTimeout };
use rocket::request::{Outcome, FromRequest};
use rocket::Outcome::{Success, Failure};
use rocket::http::{Cookies, Status};
use rocket::Request;
use diesel;
use diesel::prelude::*;

use data::database::*;
use data::model::card::{Card, NewCard, CardStatus};
use data::schema::card;

pub struct CardService{
	pub db: Database,
}

impl<'a, 'r> FromRequest<'a, 'r> for CardService {
	type Error = GetTimeout;
	fn from_request(request: &'a Request<'r>) -> Outcome<Self, Self::Error> {
		let mut service = match DB_POOL.get() {
			Ok(db_connection) => CardService {
				db: Database(db_connection)
			},
			Err(e) => return Failure((Status::InternalServerError, e)),
		};
		Success(service)
	}
}

impl CardService{
	pub fn get_card(&self, id: i32) -> Option<Card> {
		match card::table.filter(card::id.eq(id))
		.limit(1).load::<Card>(self.db.connection()){
			Ok(c) => c,
			Err(e) => Vec::<Card>::new(),
		}.pop()
	}

    pub fn get_user_cards(&self, user_id: i32) -> Vec<Card> {
        match card::table.filter(card::user_id.eq(&user_id)).load::<Card>(self.db.connection()) {
            Ok(r) => r,
            Err(_) => Vec::<Card>::new()
        }
    }

	pub fn add_card(&self, user_id: i32, title: &str, description: &str, status: CardStatus) -> Result<Card, String> {
		let new_card = NewCard {
			user_id: user_id,
			title: title,
			description: description,
			status: status as i32,
		};

		match diesel::insert(&new_card).into(card::table)
		.execute(self.db.connection()) {
			Ok(_) => (),
			Err(e) => {
				println!("Saving new card to database failed: {}", e);
				return Err("Failed to add card.".to_string());
			}
		}
		use diesel::expression::sql_literal::sql;
		let new_card_id = match sql("select last_insert_id();")
		.get_result(self.db.connection()){
			Ok(id) => id,
			Err(e) => {
				println!("Retrieving new card id from database failed: {}", e);
				return Err("Failed to retrieve new card id.".to_string());
			}
		};
		if let Some(c) = self.get_card(new_card_id){
			return Ok(c);
		};
		println!("Problem with new card creation");
		Err("Problem with new card creation".to_string())
	}
}