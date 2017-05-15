use super::connection::*;
use diesel::mysql::MysqlConnection;
use r2d2::{ Pool, PooledConnection, GetTimeout };
use r2d2_diesel_mysql::ConnectionManager;
use rocket::request::{Outcome, FromRequest};
use rocket::Outcome::{Success, Failure};
use rocket::http::Status;
use rocket::Request;

lazy_static! {
	pub static ref DB_POOL: Pool<ConnectionManager<MysqlConnection>> = create_db_pool().unwrap();
}

pub struct Database(PooledConnection<ConnectionManager<MysqlConnection>>);

impl Database {
	pub fn connection(&self) -> &MysqlConnection {
		&*self.0
	}
}

impl<'a, 'r> FromRequest<'a, 'r> for Database {
	type Error = GetTimeout;
	fn from_request(_: &'a Request<'r>) -> Outcome<Self, Self::Error> {
		match DB_POOL.get() {
			Ok(db_connection) => Success(Database(db_connection)),
			Err(e) => Failure((Status::InternalServerError, e)),
		}
	}
}