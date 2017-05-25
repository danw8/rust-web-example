use maud::{Markup};
use service::user::UserService;
use view;

#[get("/")]
fn index(user_service: UserService) -> Markup {
	view::index::index(user_service.user)
}

