#![feature(plugin, custom_derive, custom_attribute)]
#![plugin(maud_macros)]
#![plugin(rocket_codegen)]


#[macro_use] extern crate diesel;
#[macro_use] extern crate diesel_codegen;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate lazy_static;
extern crate maud;
extern crate rocket;
extern crate toml;
extern crate r2d2;
extern crate r2d2_diesel_mysql;
extern crate dotenv;
extern crate bcrypt;
extern crate time;
extern crate serde_json;

mod route;
mod data;
mod service;
mod view;

use route::*;


fn main() {
	rocket::ignite().mount("/", routes![
		index, account_creation, files, login,
		verify, adduser, member, logout, getuser,
		get_user_cards, updatecard, deletecard,
		addcard]).launch();
}
