use diesel::prelude::*;
use maud::Markup;
use data::database::Database;
use data::model::user::User;
use service::user::UserService;



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

