use diesel::prelude::*;
use diesel::mysql::MysqlConnection;
use toml;

use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

#[derive(Debug, Deserialize)]
struct SqlConfig {
	username: String,
	password: String,
	host: String,
	schema: String
}


pub fn establish_connection() -> MysqlConnection {

	let config = load_data_config();
	let database_url = format!("mysql://{}:{}@{}/{}", config.username, config.password, config.host, config.schema);
	// Must be in <mysql://[[user]]:[[password]]@host[:port][/database]> form.
	//let database_url = "mysql://root:7lx3ytm1@localhost/rust_web_example";
	MysqlConnection::establish(&database_url)
		.expect(&format!("Error connecting to {}", database_url))
}

fn load_data_config() -> SqlConfig{
	let file = File::open("config/data.toml").expect("Couldn't open config/data.toml");
	let mut buf_reader = BufReader::new(file);
	let mut contents = String::new();
	buf_reader.read_to_string(&mut contents).expect("Couldn't read the file config/data.toml");

	let config : SqlConfig = toml::from_str(&contents).expect("Couldn't deserialize config/data.toml");
	return config;
}