use maud::Markup;

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