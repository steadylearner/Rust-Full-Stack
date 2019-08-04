use yew::{html, Html};

use crate::Model;

// consider test with Self and Component method later?
pub fn view_text(value: &str) -> Html<Model> {
    html! {
        <span>{ &value }</span>
    }
}

