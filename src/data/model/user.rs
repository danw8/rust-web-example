use super::super::schema::user;

#[derive(Queryable, Deserialize, Serialize)]
pub struct User {
	pub id: i32,
	pub username: String,
	pub email: String,
	pub password: String,
}

#[derive(Insertable)]
#[table_name="user"]
pub struct NewUser<'a> {
    pub username: &'a str,
    pub email: &'a str,
    pub password: &'a str,
}
