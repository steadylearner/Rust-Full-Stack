use yew::{html, Html};

use crate::http_model::websocket_json::WebSocketResponse;
use crate::Model;

// You(client_id = user_id): black
// others(client_id != user_id): red-white
// Server(client_id = None): blue

// later make struct or database for users and use their images instead

// https://www.steadylearner.com/static/images/brand/code.png
pub fn view_response(_idx: usize, response: &Option<String>, user: &Option<String>) -> Html<Model> {

    let mut class = "".to_string();

    // should organize code here
    if let Some(value) = response { // You may delete this?

        let ws_response: WebSocketResponse = serde_json::from_str(&value).unwrap();

        let WebSocketResponse { value, client, .. } = ws_response;

        if let Some(client_id) = client {
            if let Some(user_id) = user {
                if user_id.clone() == client_id {
                    class.push_str(" black"); // use user instead later

                    if value.starts_with("https") {
                        html! {
                            <li class=class, >
                                <span> { "You: " }</span>
                                <img class=("flex", "margin-top-two-and-a-half"), src=value, ></img>
                            </li>
                        }
                    } else {
                        html! {
                            <li class=class, >
                                <span>{ format!("You: {}", value) }</span>
                            </li>
                        }
                    }

                } else {
                    class.push_str(" red-white"); // use others
                    if value.starts_with("https") {
                        html! {
                            <li class=class, >
                                <span> { format!("{}: ", client_id) } </span>
                                <img class=("flex", "margin-top-two-and-a-half"), src=value, ></img>
                            </li>
                        }
                    } else {
                        html! {
                            <li class=class, >
                                <span>{ format!("{}: {}", client_id, value) }</span>
                            </li>
                        }
                    }
                }
            } else {
                // class.push_str(" blue");
                html! {
                    { "" }
                    // <li class=class, > // use server 
                    //     <span>{ format!("{}", value) }</span>
                    // </li>
                }
            }
        } else {
            html! { 
                { "" }
            }
        }
    } else {
        html! {
            { "" }
        }
    }
}