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

#[derive(Deserialize, Serialize, Clone)]
pub struct NewCard<'a> {
    pub user_id: i32,
    pub title: &'a str,
    pub description: &'a str,
    pub status: i32,
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
		let id = format!("list-card-{}", card.id);
		cardlist.html_append(&view::card(&card, &id).into_string());
		let selector = document.element_query(".new-status").unwrap();
		selector.class_remove("new-status");
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

		let button = document.element_query(".new-button").unwrap();
		button.class_remove("new-button");
		button.on("click", enclose!{
			(card, document, id) move |_:Event| {
				let url = format!("deletecard/{}", card.id);
				let _ = wire::http_get(&url);
				let id = format!("#{}", id);
				let card_element = document.element_query(&id).unwrap();
				card_element.remove_self();
			}
		});
	}

	let add_card = document.element_query("#add-card-button").unwrap();
	add_card.on("click", enclose!{
		(document) move |_:Event| {
			let add_title = document.element_query("#add-card-title").unwrap();
			let add_description = document.element_query("#add-card-description").unwrap();
			let title_value = add_title.prop_get_str("value");
			add_title.prop_set_str("value", "");
			let description_value = add_description.prop_get_str("value");
			add_description.prop_set_str("value", "");
			let new_card = NewCard{
				user_id: 0,
				title: &title_value,
				description: &description_value,
				status: 0,
			};
			let card_json = serde_json::to_string(&new_card).unwrap();
			let card: Card = serde_json::from_str(&wire::http_post("addcard", &card_json)).unwrap();
			let id = format!("list-card-{}", card.id);
			cardlist.html_append(&view::card(&card, &id).into_string());
			let selector = document.element_query(".new-status").unwrap();
			selector.class_remove("new-status");
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

			let button = document.element_query(".new-button").unwrap();
			button.class_remove("new-button");
			button.on("click", enclose!{
				(card, document, id) move |_:Event| {
					let url = format!("deletecard/{}", card.id);
					let _ = wire::http_get(&url);
					let id = format!("#{}", id);
					let card_element = document.element_query(&id).unwrap();
					card_element.remove_self();
				}
			});
		}
	})
}

