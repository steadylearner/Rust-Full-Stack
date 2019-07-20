#![recursion_limit="512"]
#![feature(rustc_private)]

// #[macro_use]
extern crate yew;

use yew::services::Task; // Is it used here?
use yew::{html, Component, ComponentLink, Html, Renderable, ShouldRender};

// https://github.com/DenisKolodin/yew/blob/master/src/format/json.rs
// https://github.com/DenisKolodin/yew/blob/master/src/format/macros.rs
use yew::format::{Json};
use yew::events::IKeyboardEvent;
use yew::services::{ConsoleService};
use yew::services::websocket::{WebSocketService, WebSocketTask, WebSocketStatus};

// #[macro_use]
extern crate failure;
use failure::Error;

extern crate serde_derive;
use serde_derive::{Deserialize, Serialize};

// should separate user_id and messages struct? and separate it with another API?
pub struct Model {
  ws_service: WebSocketService,
  console: ConsoleService,
  ws: Option<WebSocketTask>,
  link: ComponentLink<Model>, // Required when you use service?

  state: State,
}

// messages or data here should be single source of truth to show chat messages

type Message = String;

#[derive(Serialize, Deserialize, Debug)]
pub struct State {
    user_id: Option<String>,
    user_inputs: Vec<String>,
    ws_responses: Vec<Option<String>>, // is this right approach?
    value: String,
}

pub enum WebSocketAction {
    Connect,
    // SendData(Message), // Doesn't work well to type
    Disconnect,
    Lost,
}

pub enum Msg {
    WebSocketAction(WebSocketAction),
    WebSocketReady(Result<WebSocketResponse, Error>),
    Ignore,

    Update(Message),
    Submit,
    Clear,

    Nope,
}

impl From<WebSocketAction> for Msg {
    fn from(action: WebSocketAction) -> Self {
        Msg::WebSocketAction(action)
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct WebSocketRequest {
    value: String,
    user_id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WebSocketResponse {
    value: String,
    client: Option<String>,
    number_of_connection: u32,
}

// lack any function here?

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let state = State {
            user_id: None,
            user_inputs: Vec::new(),
            ws_responses: Vec::new(), // is this right approach?
            value: "".into(),
        };
        Model {
            ws_service: WebSocketService::new(),
            console: ConsoleService::new(),
            ws: None,
            link,

            state,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            // WebSocket
            Msg::WebSocketAction(action) => {
                match action {
                    WebSocketAction::Connect => {
                        let callback = self.link.send_back(|Json(data)| Msg::WebSocketReady(data));  // payload?
                        let notification = self.link.send_back(|status| {
                            match status {
                                WebSocketStatus::Opened => Msg::Ignore,
                                WebSocketStatus::Closed | WebSocketStatus::Error => WebSocketAction::Lost.into(),
                            }
                        });
                        let task = self.ws_service.connect("ws://127.0.0.1:7777/ws", callback, notification); // 1.
                        self.ws = Some(task);
                    }
                    // WebSocketAction::SendData(data) => {
                    //     let request = WebSocketRequest {
                    //         value: data, 
                    //     }; // send {"value": "Test"} to websocket server
                    //     self.ws.as_mut().unwrap().send_binary(Json(&request));
                    // }
                    WebSocketAction::Disconnect => {
                        self.ws.take().unwrap().cancel();
                        self.console.clear();
                        self.console.log("The user wants to leave this page.");
                    }
                    WebSocketAction::Lost => {
                        self.ws = None;
                    }
                }
            }
            Msg::WebSocketReady(response) => { // payload?
                self.console.log("Websocket is ready. Start to chat with others.");
                // let socket_input = response.map(|data| data.value).ok();
                // self.state.ws_responses.push(socket_input);
            }
            Msg::Ignore => {
                return false;
            }

            // Below are for View and other Controllers 

            Msg::Update(val) => {
                self.state.value = val;
            }
            Msg::Submit => {
                // should write conditionals here
                let value = self.state.value.clone();
                let user_id = self.state.user_id.clone();

                // self.console.log(&value);
                // They work
                let request = WebSocketRequest {
                    value,
                    user_id,
                };
                self.ws.as_mut().unwrap().send_binary(Json(&request));
                 
                self.state.value = "".to_string();
            }
            Msg::Clear => {
                self.console.log("The user wants to clear the messages.");
                // method to remove every messages in the state
            }
            Msg::Nope => {}
        }
        true
    }
}

impl Renderable<Model> for Model {
    fn view(&self) -> Html<Self> {
        html! {
            <section>
                <nav id="nav", class=("nav", "flex", "center"), >
                    <a
                        class=("flex", "no-text-decoration", "hover", "cursor-pointer", "transition-half", "right-auto"),
                        href="https://www.steadylearner.com/blog",
                        title="Click it to learn how to code this.",
                    >
                        <span class=("white", "bold"), >{ "© Rust Chat App" }</span>
                    </a>
                    <button
                        id="connect",
                        class=("margin-right-one", "white", "cursor-pointer", "hover", "transition", "theme-black"),
                        disabled=self.ws.is_some(),
                        onclick=|_| WebSocketAction::Connect.into(),
                    >
                       { "Enter" }
                    </button>
                    <button
                        id="exit",
                        class=("margin-right-one", "white", "cursor-pointer", "hover", "transition", "theme-black"),
                        disabled=self.ws.is_none(),
                        onclick=|_| WebSocketAction::Disconnect.into(),
                    >
                       { "Exit" }
                    </button>
                </nav>
                <ul
                    id="messages",
                >
                    { for self.state.ws_responses.iter().enumerate().map(view_response) }
                </ul>
                <section
                    id="form",
                    class=("chat-input", "flex", "center"),
                >
                    <img
                        id="code",
                        class=("flex", "center", "rust-icon", "hover", "cursor-pointer", "transition-half"),
                        title="Use this for whatever you want",
                        src="https://www.steadylearner.com/static/images/code/Rust.svg",
                        onclick=|_| Msg::Clear,
                    />
                    <input
                        id="msg", 
                        type="text",
                        placeholder="Type here to start to talk with others and enter to submit",
                        autocomplete="off",
                        value=&self.state.value,
                        oninput=|e| Msg::Update(e.value),
                        onkeypress=|e| {
                            if e.key() == "Enter" { Msg::Submit } else { Msg::Nope }
                        }, 
                    />
                </section>
            </section>
        }
    }
}

fn view_response((_idx, response): (usize, &Option<String>)) -> Html<Model> {
    if let Some(value) = response {
        html! {
            <li>{ value }</li>
        }
    } else {
        html! {
            <li>{ "Enter the chat." }</li>
        }
    }
}