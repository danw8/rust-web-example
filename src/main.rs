#![feature(plugin)]
#![plugin(maud_macros)]
#![plugin(rocket_codegen)]

extern crate maud;
extern crate rocket;
extern crate toml;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_codegen;
#[macro_use]
extern crate serde_derive;

mod route;
mod data;

use route::*;
use data::connection::*;


fn main() {
	let connection = establish_connection();
	rocket::ignite().mount("/", routes![index, files]).launch();
}