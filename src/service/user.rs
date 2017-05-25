use r2d2::{ GetTimeout };
use rocket::request::{Outcome, FromRequest};
use rocket::Outcome::{Success, Failure};
use rocket::http::{Cookies, Status};
use rocket::Request;
use diesel;
use diesel::prelude::*;

use data::database::*;
use data::model::user::{User, NewUser};
use data::schema::user;
use bcrypt::{DEFAULT_COST, hash, verify};
//use time;

pub struct UserService{
	pub db: Database,
	pub user: Option<User>,
}

impl<'a, 'r> FromRequest<'a, 'r> for UserService {
	type Error = GetTimeout;
	fn from_request(request: &'a Request<'r>) -> Outcome<Self, Self::Error> {
		let mut service = match DB_POOL.get() {
			Ok(db_connection) => UserService {
				db: Database(db_connection),
				user: None
			},
			Err(e) => return Failure((Status::InternalServerError, e)),
		};
		service.auth_with_cookie(request.cookies());
		Success(service)
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

	pub fn authenticate(&mut self, username: &str, password: &str) -> bool {
		let user = self.get_user(username);
		if let Some(u) = user {
			return match verify(password, &u.password){
				Ok(valid) => {
					self.user = Some(u);
					valid
				},
				Err(_) => false
			};
		}
		false
	}

	pub fn auth_with_cookie(&mut self, cookies: &Cookies) -> bool{
		let verified_cookie = match cookies.find("verified"){
			Some(c) => c,
			None => return false
		};
		self.user = self.get_user(verified_cookie.value());
		self.user.is_some()
	}
}
