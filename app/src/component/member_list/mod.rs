use webplatform::{HtmlNode, Document, Event};
use serde_json;
use std::rc::Rc;
use wire;
pub mod view;

#[derive(Deserialize, Serialize, Clone)]
pub struct Card {
	pub id: i32,
	pub user_id: i32,
	pub title: String,
	pub description: String,
    status: i32,
}

macro_rules! enclose {
    ( ($( $x:ident ),*) $y:expr ) => {
        {
            $(let $x = $x.clone();)*
            $y
        }
    };
}

pub fn member_list(app: HtmlNode, document: Rc<Document>){
	let cards : Vec<Card> = serde_json::from_str(&wire::http_get("usercards")).unwrap();
	app.html_set(&view::member_list().into_string());
	let cardlist = document.element_query(".list-cards").unwrap();
	for card in cards {
		cardlist.html_append(&view::card(&card).into_string());
		let selector = document.element_query(".update-and-remove-me").unwrap();
		selector.class_remove("update-and-remove-me");
		selector.on("change", enclose!{ 
			(card) move |e:Event| {
				if let Some(status_selector) = e.target {
					let new_card_status = status_selector.prop_get_str("value");
					let mut card = card.clone();
					card.status = 0;
					if new_card_status == "Complete" {
						card.status = 1;
					}
					let card_json = serde_json::to_string(&card).unwrap();
					let _ = wire::http_post("updatecard", &card_json);
				}
			}
    	});
	}
}

