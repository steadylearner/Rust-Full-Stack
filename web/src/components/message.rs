use yew::{html, Html};

use crate::Model;

// find how to include text, image, video and use match

pub fn view_message(value: &str, message_type: &str) -> Html<Model> {
    if !(&value.is_empty()) {
        let message = match message_type {
            "image" => {
                image(&value)
            }
            "video" => {
                video(&value)
            }
            // "code" => {
                // code(&value)
            // }
            _ => {
                text(&value)
            }
        };

        html! {
            <li>
                <span> { "You: " }</span>
                { message }
            </li>
        }
    } else {
        html! {
            { "" }
        }
    }
}

