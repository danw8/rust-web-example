use data::model::user::User;
use maud::{Markup, PreEscaped};
use super::partial::*;

pub fn member(user: Option<User>) -> Markup {
	html! {
		head{
			link rel="stylesheet" type="text/css" href=("/files/style/member.css") /
			
		}
		body{
			(navbar(&user))
			div id="app" {}

			script src="/files/script/app/app.js" {}
		}
	}
}