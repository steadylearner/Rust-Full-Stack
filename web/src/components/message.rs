use yew::{html, Html};

use crate::Model;

// super here is mod.rs in this folder(components), you should write pub mod <name> first
use super::{
    image::view_image,
    video::view_video,
    code::view_code,
    text::view_text,
};

// function and conditional statements to make it simple

pub fn view_message(_idx: &usize, response: &str, message_type: &str, client: &Option<String>, user: &Option<String>) -> Html<Model> {
    
    if !response.is_empty() {
        let message = match message_type {
            "image" => {
                view_image(&response)
            }
            "video" => {
                view_video(&response)
            }
            "code" => {
                view_code(&response)
            }
            _ => {
                view_text(&response)
            }
        };

        if let Some(client_id) = client {
            if let Some(user_id) = user  {
                if client_id == user_id {
                    html! {
                        <li>
                            { author() }
                            { message }
                        </li>
                    }
                } else {
                    html! { 
                        <li class="red-white", >
                            { others(&client_id) }
                            { message }
                        </li>
                    } 
                }
            } else {
                // user is not connected to the web socket
                // so do not show any messages(this is temporay solution and you should make the pages before user enter and leave)
                
                html! {
                    { "" }
                }
            }
        } else {
            html! {
                <li>
                    { message } // this will be from Server with client_id None, use this instead of I want to use Rust Full Stack App
                </li>
            }
        }
    } else {
        html! {
            { "" }
        }
    }
}

fn author() -> Html<Model> {
    html! {
        <span> { "You: " }</span>
    }
}

fn others(client_id: &str) -> Html<Model> {
    html! {
        <span> { format!("{}: ", client_id) }</span>
    }
}
