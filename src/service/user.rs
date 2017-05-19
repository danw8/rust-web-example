use r2d2::{ GetTimeout };
use rocket::request::{Outcome, FromRequest};
use rocket::Outcome::{Success, Failure};
use rocket::http::Status;
use rocket::Request;
use diesel;
use diesel::prelude::*;

use data::database::*;
use data::model::user::{User, NewUser};
use data::schema::user;
use bcrypt::{DEFAULT_COST, hash, verify};

pub struct UserService{
	pub db: Database,
}

impl<'a, 'r> FromRequest<'a, 'r> for UserService {
	type Error = GetTimeout;
	fn from_request(_: &'a Request<'r>) -> Outcome<Self, Self::Error> {
		match DB_POOL.get() {
			Ok(db_connection) => Success(UserService {
				db: Database(db_connection),
			}),
			Err(e) => Failure((Status::InternalServerError, e)),
		}
	}
}

impl UserService{
	pub fn get_users(&self) -> Vec<User> {
		user::table.load::<User>(self.db.connection()).expect("Error loading users")
	}

	pub fn get_user(&self, uname: &str) -> Option<User> {
		user::table.filter(user::username.eq(uname)).limit(1).load::<User>(self.db.connection()).expect("Error loading user").pop()
	}

	pub fn create_user<'a>(&self, username: &'a str, email: &'a str, password: &'a str) -> Option<User> {
		let password_hash = hash(password, DEFAULT_COST).expect("Could not hash password");

		let new_user = NewUser {
			username: username,
			email: email,
			password: password_hash.as_str(),
		};

		diesel::insert(&new_user).into(user::table)
        		.execute(self.db.connection())
        		.expect("Error saving new post");

        	self.get_user(username)
	}

	pub fn authenticate(&self, username: &str, password: &str) -> bool {
		let user = self.get_user(username);
		if user.is_some() {
			let user = user.unwrap();
			return match verify(password, &user.password){
				Ok(valid) => valid,
				Err(_) => false
			};
		}
		false
	}
}
