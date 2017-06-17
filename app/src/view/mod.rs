use maud::{Markup};
use wire;
use serde_json;

pub fn member_home() -> Markup{
    let username = wire::http_get("getuser");
    println!("{}", username);
    html! {
        div.center.margin-md  {
            div.member-container {
                div{
                    h1 {
                        "Welcome, " (username)
                    }
                }
            }
        }

    }
}

#[derive(Deserialize)]
struct Card {
	pub id: i32,
	pub user_id: i32,
	pub title: String,
	pub description: String,
    status: i32,
}

pub fn member_list() -> Markup{
    let cards : Vec<Card> = serde_json::from_str(&wire::http_get("usercards")).unwrap();
    html! {
        div.center.margin-md  {
            div.list-container {
                div{
                    h1 {
                        "My Cards"
                    }
                    ul {
                        @for card in &cards {
                            li{
                                div{
                                    (card.title)
                                }
                                div{
                                    "Description:" (card.description)
                                }
                                div{
                                    @if card.status == 0 {
                                        "Status: Incomplete"
                                    } @else {
                                        "Status: Complete"
                                    }
                                }
                            } 
                        }
                    }
                }
            }
        }
    }
}