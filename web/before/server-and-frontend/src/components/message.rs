use yew::{html, Html};

use crate::Model;

// super here is mod.rs in this folder(components), you should write pub mod <name> first
use super::{
    image::view_image,
    video::view_video,
    code::view_code,
    text::view_text,
};

// You can write less code here if you build users, login, before and after pages.

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

        // Use this if you want to test with idx or id before you write code for users
        // if idx % 2 != 0 {
        //     html! {
        //         <li class="red-white", >
        //             <span> { "Steadylearner: " }</span>
        //             { message }
        //         </li>
        //     }
        // } else {
        //     html! {
        //         <li>
        //             <span> { "You: " }</span>
        //             { message }
        //         </li>
        //     }
        // }

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
            if let Some(_user_id) = user  { // Use this not to show any message when user leave the chat
                html! {
                    <li class="blue", >
                        { message } 
                    </li>
                }
            } else {
                html! {
                    { "" }
                }
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

