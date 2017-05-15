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
extern crate r2d2;
extern crate r2d2_diesel;

mod route;
mod data;

use route::*;
use data::connection::*;


fn main() {
	let connection = match establish_connection(){
		Ok(c) => c,
		Err(e) => panic!(e)
	};
	rocket::ignite().mount("/", routes![index, files]).launch();
}
