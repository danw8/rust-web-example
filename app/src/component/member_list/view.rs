use maud::{Markup};
use super::Card;

pub fn member_list() -> Markup{
    html! {
        div.center.margin-md  {
            div.list-container {
                div{
                    h1 {
                        "My Cards"
                    }
                    ul.list-cards {
                    }
                }
            }
        }
    }
}

pub fn card(card: &Card) -> Markup{
    html! {
        li.list-card{
            div.list-card-title{
                h3 { (card.title) }
                div.list-card-status{
                    @if card.status == 0 {
                        span {"Status:"} 
                        select.update-and-remove-me {
                            option value="Incomplete" { "Incomplete" }
                            option value="Complete" { "Complete" }
                        }
                    } @else {
                        span {"Status:"} 
                        select.update-and-remove-me {
                            option value="Complete" { "Complete" }
                            option value="Incomplete" { "Incomplete" }
                        }
                    }
                }
            }
            div.list-card-description{
                (card.description)
            }
        }
    }
}