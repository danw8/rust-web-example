use diesel::mysql::MysqlConnection;
use r2d2::{ Pool, Config };
use r2d2_diesel_mysql::ConnectionManager;
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

static CONFIG_PATH: &'static str = "config/data.toml";

pub fn get_db_url() -> Result<String, String>  {
	let config = load_data_config()?;
	Ok(format!("mysql://{}:{}@{}/{}",
		config.username, config.password, config.host, config.schema))
}

/// An r2d2 conneciton pool for use in routes.
pub fn create_db_pool() -> Result<Pool<ConnectionManager<MysqlConnection>>, String> {
	let database_url = get_db_url()?;

	let pool_config = Config::default();
	let manager = ConnectionManager::<MysqlConnection>::new(database_url.clone());
	match Pool::new(pool_config, manager) {
		Ok(p) => Ok(p),
		Err(_) => Err(format!("Error creating connection pool with connection string '{}'", database_url))
	}
}

/// A single connection to a mysql database
// pub fn establish_connection() -> Result<MysqlConnection, String> {
// 	// Must be in <mysql://[[user]]:[[password]]@host[:port][/database]> form.
// 	let database_url = get_db_url()?;
// 	match MysqlConnection::establish(&database_url) {
// 		Ok(c) => Ok(c),
// 		Err(_) => Err(format!("Error connecting to {}", database_url))
// 	}
// }

/// Loading a config from
fn load_data_config() -> Result<SqlConfig, String> {
	let file = match File::open(CONFIG_PATH) {
		Ok(f) => f,
		Err(_) => return Err(format!("Couldn't open config file: {}", CONFIG_PATH))
	};

	let mut buf_reader = BufReader::new(file);
	let mut contents = String::new();
	match buf_reader.read_to_string(&mut contents) {
		Ok(_) => {},
		Err(_) => return Err(format!("Couldn't read the config file: {}", CONFIG_PATH))
	};

	let config : SqlConfig = match toml::from_str(&contents){
		Ok(c) => c,
		Err(_) => return Err(format!("Couldn't deserialize config file: {}", CONFIG_PATH))
	};
	Ok(config)
}


