use diesel::prelude::*;
use maud::Markup;
use data::database::Database;
use data::model::user::User;
use service::user::UserService;
use rocket::request::Form;
use rocket::response::Redirect;

#[get("/users")]
fn users(user_service: UserService) -> Markup {
	let users = user_service.get_users();
	html! {
		head{
			link rel="stylesheet" type="text/css" href=("files/style/index.css") /
		}
		body{
			h1 {
				"Users in the system"
			}
			h3 {
				"Add User"
			}
			form action="/adduser" method="post" accept-charset="utf-8"{
				div{
					label for="username"{ "Username" }
					input id="username" name="username" type="text" /
				}
				div {
					label for="email" { "Email" }
					input id="email" name="email" type="text" /
				}
				div {
					label for="password" { "Password" }
					input id="password" name="password" type="password" /
				}
				button id="submit" type="submit" { "Create" }
			}
			ul {
				div {
					span.user.header {
						"Username"
					}
					spane.user.header {
						"Email"
					}
				}
				@for u in users {
					li {
						span.user {
							(u.username)
						}
						span.user {
							(u.email)
						}
					}
				}
			}
		}
	}
}

#[derive(FromForm)]
struct AddUser {
    username: String,
    email: String,
    password: String,
}

#[post("/adduser", data = "<add_user>")]
fn adduser(user_service: UserService, add_user: Form<AddUser>) -> Redirect {
	let user = add_user.into_inner();
	user_service.create_user(&user.username, &user.email, &user.password).expect("Couldn't create user.");
	Redirect::to("/users")
}
