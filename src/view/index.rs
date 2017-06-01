use data::model::user::User;
use maud::Markup;
use super::partial::*;

pub fn index(user: Option<User>) -> Markup {
	html! {
		head{
			link rel="stylesheet" type="text/css" href=("/files/style/index.css") /
		}
		body{
			(navbar(&user))
			div.container {
				div.wrapper {
					h1.one-hundred.margin-md {
						"Welcome, Rustacean!"
					}
					h3.one-hundred.margin-md {
						"This is an example web application written entirely in the Rust programming language. "
						"Entirely in Rust might be a stretch but it is the goal. "
						"Some non Rust things are CSS for styling, TOML for configuration and SQL for migrations. "
						"Check out the application by making an account or logging in if you already made one. "
						"This is open source and available on " 
						a href="https://github.com/danw8/rust-web-example" {
							"github"
						}
						"."
					}
					div.center.margin-md {
						div.account_choice {
							div.border_right {
								h2 {
									"Want to try it?"
								}
								a href="/account_creation" { "Create an account" }
							}
							div {
								h2 {
									"Have an account?"
								}
								@if user.is_some() {
									a href="/member#/" { "Go to the member area" }
								}
								@else{
									a href="/login" { "Login" }
								}
							}						
						}
					}
				}
			}
		}
	}
}