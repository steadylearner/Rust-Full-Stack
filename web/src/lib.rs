#![recursion_limit="512"]
#![feature(rustc_private)]

#[macro_use]
extern crate stdweb;
// use stdweb::js;

extern crate yew;
use yew::services::Task;
use yew::format::{Json};
use yew::{html, Component, ComponentLink, Html, Renderable, ShouldRender};
use yew::services::{
    ConsoleService,
    websocket::{WebSocketService, WebSocketStatus},
};

extern crate failure;
use failure::Error;

const WEBSOCKET: &'static str = "ws://127.0.0.1:7777/ws";

mod state;
mod ws_rs;

mod components;
mod npm;

mod http_model;

use self::{
    state::State,
    ws_rs::{WebSocket, WebSocketAction},

    components::{
        // chat_input::ChatInput,
        message::{view_message},
        buttons::{
            use_image::UseImage,
            use_video::UseVideo,
            use_code::UseCode,
            // connect::Connect,
            // disconnect::Disconnect,
        },
        website::{
            steadylarner_blog,
            social,
        }
    },

    npm::{
        emoji::EmojiService,
    },

    http_model::{
        websocket_json::{
            WebSocketRequest,
            WebSocketResponse,
        }
    },
};

pub struct Model {
    state: State,

    // To update component wiht message from WebSocket
    ws_rs: WebSocket,
    link: ComponentLink<Model>,

    // Yew
    console: ConsoleService,

    // NPM
    emoji: EmojiService,
}

pub enum Msg {
    // Websocket

    WebSocketAction(WebSocketAction),
    WebSocketReady(Result<WebSocketResponse, Error>),
    Ignore,

    // Client

    Submit(String),
    Type(String),
}

impl From<WebSocketAction> for Msg {
    fn from(action: WebSocketAction) -> Self {
        Msg::WebSocketAction(action)
    }
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let state = State {
            ws_responses: Vec::new(),
            message_type: "text".to_string(),
            client: None,
        };

        let ws_rs = WebSocket {
            ws_service: WebSocketService::new(),
            ws: None,
        };

        Model {
            state,

            // WebSocekt
            ws_rs,
            link, //  // remove _ from _link: ComponentLink<Self> to make it work

            // Yew
            console: ConsoleService::new(),

            // NPM
            emoji: EmojiService::new()
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
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
                        let task = self.ws_rs.ws_service.connect(WEBSOCKET, callback, notification); // 1.
                        self.ws_rs.ws = Some(task);
                    }
                    WebSocketAction::Disconnect => {
                        self.state.client = None;
                        self.ws_rs.ws.take().unwrap().cancel();
                    }
                    WebSocketAction::Lost => {
                        self.state.client = None;
                        self.ws_rs.ws = None;
                    }
                }
            }

            Msg::WebSocketReady(response) => { // payload, should edit here
                self.console.log("Websocket is ready. Start to chat with others.");
                let socket_input = response.map(|data| data).ok();

                let serialized = serde_json::to_string(&socket_input).unwrap(); // value in view_response

                self.console.log(&serialized);

                let for_if = serialized.clone();

                let ws_response: WebSocketResponse = serde_json::from_str(&for_if).unwrap();
                let WebSocketResponse { value, message_type, client, number_of_connection: _ } = ws_response;

                if self.state.client == None {
                    self.state.client = client;
                }

                if value == "!clearall" {
                    self.state.ws_responses.clear();
                }

                self.state.ws_responses.push(Some(serialized));
            }

            Msg::Ignore => {
                return false;
            }

            // Client 

            Msg::Submit(val) => {
                // use this or if val == "!clear"
                match val.as_ref() {
                    "!clear" => {
                        self.state.ws_responses.clear();
                    }
                    "!exit" => {
                        // self.state.connection = false;
                    }
                    _ => {
                        let before = format!("{}", &val);
                        let emojified = self.emoji.emojify(before.to_string());

                        let message_type = self.state.message_type.clone();

                        // https://serde.rs/
                        let request = WebSocketRequest {
                            value: emojified,
                            client: Some("steadylearner".to_string()),
                            message_type,
                        };

                        // Convert the WebSocketRequest to a JSON string.
                        // use it when you send JSON or other data
                        let serialized = serde_json::to_string(&request).unwrap();

                        self.console.log(&serialized);

                        // self.state.ws_responses.push(Some(serialized)); move this to WebSocket Ready
                        // and write code to send messages to WebSocket in Server

                        if &self.state.message_type != "text" {
                            self.state.message_type = "text".to_string();
                        }
                    }
                }
            }
            Msg::Type(val) => {
                self.state.message_type = val
            }
        }
        true
    }
}

// Make Enter and Exit components later
impl Renderable<Model> for Model {
    fn view(&self) -> Html<Self> {
        let State { ws_responses, message_type, client: _ } = &self.state;
        html! {
            <>
                { social() }
                <section>
                    <nav id="nav", class=("nav", "flex", "center"), >
                        { steadylarner_blog() }
                        // <Connect: disabled={*connection}, onsignal=|_| Msg::Connect, />
                        // <Disconnect: disabled={!*connection}, onsignal=|_| Msg::Disconnect, />
                    </nav>
                    <ul
                        id="messages",
                    >
                        { 
                            for ws_responses
                            .iter()
                            .enumerate()
                            .map(|(idx, response)| {
                                    // https://serde.rs/, use it before you use data, other names for them later not to be confused
                                    let message = response.clone();
                                    let deserialized: WebSocketRequest = serde_json::from_str(&message.unwrap()).unwrap();
                                    view_message(
                                        &idx, 
                                        &deserialized.value,
                                        &deserialized.message_type
                                    ) 
                                }
                            ) 
                        }
                    </ul>
                    <section
                        id="form",
                        class=("chat-form", "flex", "center"),
                    >
                        <UseCode: disabled={message_type != "code"}, onsignal=Msg::Type, />
                        // <ChatInput: disabled={!*connection}, onsignal=Msg::Submit, />
                        // or { self.chat_input() } - refer to /before/component/chat_input_compare folder
                        <UseImage: disabled={message_type != "image"}, onsignal=Msg::Type, />
                        <UseVideo: disabled={message_type != "video"}, onsignal=Msg::Type, />
                    </section>
                </section>
            </>
        }
    }
}
