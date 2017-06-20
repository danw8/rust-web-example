use data::model::user::User;
use maud::{Markup};
use super::partial::*;

pub fn member(user: Option<User>) -> Markup {
	html! {
		head {
			link rel="stylesheet" type="text/css" href=("/files/style/member.css") /
			link rel="stylesheet" type="text/css" href="https://maxcdn.bootstrapcdn.com/font-awesome/4.7.0/css/font-awesome.min.css" /
		}
		body{
			(navbar(&user))
			div id="app" {}

			script src="/files/script/app/app.js" {}
		}
	}
}