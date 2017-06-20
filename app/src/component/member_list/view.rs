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
                    div.list-add-card {
                        div.list-add-new {"New Card:"}
                        div.list-add-card-title {
                            label { "Title:" } 
                            input id="add-card-title" type="text" /
                        }
                        div.list-add-card-description {
                            label { "Description:" } 
                            input id="add-card-description" type="text" / 
                        }
                        div.list-add-card-button {
                            button id="add-card-button" { "Add Card" }
                        }
                    }
                    ul.list-cards {
                    }
                }
            }
        }
    }
}

pub fn card(card: &Card, card_id: &str) -> Markup{
    html! {
        li.list-card id=(card_id){
            div {
                button.list-card-delete.new-button {
                    i.fa.fa-times aria-hidden="true" {}
                    span.list-delete-tooltip { "Delete Card" }
                }
            }
            div.list-card-title{
                h3 { (card.title) }
                div.list-card-status{
                    @if card.status == 0 {
                        span {"Status:"} 
                        select.new-status {
                            option value="Incomplete" { "Incomplete" }
                            option value="Complete" { "Complete" }
                        }
                    } @else {
                        span {"Status:"} 
                        select.new-status {
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