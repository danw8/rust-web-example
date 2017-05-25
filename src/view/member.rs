use data::model::user::User;
use maud::Markup;
use super::partial::*;

pub fn member(user: Option<User>) -> Markup {
	html! {
		head{
			link rel="stylesheet" type="text/css" href=("files/style/member.css") /
		}
		body{
			(navbar(&user))
			h1 {
				"Welcome, " (user.unwrap().username)
			}
		}
	}
}