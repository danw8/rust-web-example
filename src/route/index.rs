use diesel::prelude::*;
use maud::Markup;
use data::database::Database;
use data::model::user::User;



#[get("/")]
fn index() -> Markup {
	html! {
		head{
			link rel="stylesheet" type="text/css" href=("files/style/index.css") /
		}
		body{
			h1 {
				"Hello, World"
			}
		}
	}
}

#[get("/users")]
fn users(db: Database) -> Markup {
	use data::schema::user::dsl::*;
	let users = user.load::<User>(db.connection()).expect("Error loading users");
	html! {
		head{
			link rel="stylesheet" type="text/css" href=("files/style/index.css") /
		}
		body{
			h1 {
				"Users in the system"
			}
			ul {
				@for u in users {
					li {
						(u.username) (u.email)
					}
				}
			}
		}
	}
}