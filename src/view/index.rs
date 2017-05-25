use data::model::user::User;
use maud::Markup;
use super::partial::*;

pub fn index(user: Option<User>) -> Markup {
	html! {
		head{
			link rel="stylesheet" type="text/css" href=("files/style/index.css") /
		}
		body{
			(navbar(&user))
			h1 {
				"Hello, World"
			}
		}
	}
}