use webplatform::{HtmlNode};
pub mod view;

pub fn member_home(app: HtmlNode) {
    app.html_set(&view::member_home().into_string());
}