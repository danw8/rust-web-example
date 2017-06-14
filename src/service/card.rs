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
    pub fn get_user_cards(&self, user_id: i32) -> Vec<Card> {
        match card::table.filter(card::user_id.eq(&user_id)).load::<Card>(self.db.connection()) {
            Ok(r) => r,
            Err(_) => Vec::<Card>::new()
        }
    }
}