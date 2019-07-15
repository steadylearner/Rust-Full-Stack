use yew::{html, Html};

use crate::Model;

pub fn view_text(value: &str) -> Html<Model> {
    html! {
        <span> { &value }</span>
    }
}

