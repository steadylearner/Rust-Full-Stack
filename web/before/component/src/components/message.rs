// use name messages instead when you use vector and for to render them
use yew::{html, Html};

use crate::Model;

// super here is mod.rs in this folder(components), you should write pub mod <name> first
use super::{
    image::view_image,
    video::view_video,
    code::view_code,
    text::view_text,
};

pub fn view_message(idx: &usize, value: &str, message_type: &str) -> Html<Model> {
    if !(&value.is_empty()) {
        let message = match message_type {
            "image" => {
                view_image(&value)
            }
            "video" => {
                view_video(&value)
            }
            "code" => {
                view_code(&value)
            }
            _ => {
                view_text(&value)
            }
        };

        // Remove this when you use real chat app
        if idx % 2 != 0 {
            html! {
                <li class="red-white", >
                    <span> { "Steadylearner: " }</span>
                    { message }
                </li>
            }
        } else {
            html! {
                <li>
                    <span> { "You: " }</span>
                    { message }
                </li>
            }
        }
    } else {
        html! {
            { "" }
        }
    }
}

