use maud::Markup;

#[get("/member")]
fn member() -> Markup {
	html! {
		head{
			link rel="stylesheet" type="text/css" href=("files/style/index.css") /
		}
		body{
			h1 {
				"Welcome, Member"
			}
		}
	}
}
