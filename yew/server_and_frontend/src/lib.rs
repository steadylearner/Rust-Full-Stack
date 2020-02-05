#![recursion_limit="512"]
#![feature(rustc_private)]

#[macro_use]
extern crate stdweb;
// use stdweb::js;

extern crate yew;
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
mod modal;

mod http_model;

use self::{
    state::State,
    ws_rs::{WebSocket, WebSocketAction},

    modal::Modal,

    components::{
        chat_input::ChatInput,
        message::{view_message},
        buttons::{
            use_image::UseImage,
            use_video::UseVideo,
            use_code::UseCode,
            connect::Connect,
            disconnect::Disconnect,
        },
        // modal::Modal,
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

    modal: Modal,

    // Yew
    console: ConsoleService,

    // NPM
    emoji: EmojiService,
}

pub enum Msg {
    WebSocketAction(WebSocketAction),
    WebSocketReady(Result<WebSocketResponse, Error>),
    Ignore,

    // Client
    Submit(String),
    Type(String),
}

// For into in WebSocketAction::Connect.into() and others to work
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

        let modal = Modal::default();

        Model {
            state,

            // WebSocekt
            ws_rs,
            link, //  // remove _ from _link: ComponentLink<Self> to make it work
            //

            modal,

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
                        self.ws_rs.connect(task);
                    }
                    WebSocketAction::Disconnect => {
                        self.state.lost();
                        self.ws_rs.disconnect();
                        // self.console.log("No connection to WebSocket anymore");
                    }
                    WebSocketAction::Lost => {
                        self.state.lost();
                        // self.console.log("No connection to WebSocket anymore");
                        self.ws_rs.lost();
                    }
                }
            }

            Msg::WebSocketReady(response) => { // payload, should edit here most of the time
                self.console.log("Websocket is ready. Start to chat with others.");
                let ws_response = response.map(|data| data).ok();

                let serialized = serde_json::to_string(&ws_response).unwrap(); // value in view_response
                self.console.log(&serialized);

                // shadow variable here
                let ws_response: WebSocketResponse = serde_json::from_str(&serialized).unwrap();
                let WebSocketResponse { value, message_type, client, number_of_connection, } = ws_response;

                // should use login page or oauth later instead of this
                // and self.state.client = None when disconnect
                if self.state.client == None {
                    self.state.client = client;
                    let user = self.state.client.clone();
                    // should modify response here or in server?
                    // write server side code first or write code for users
                    // or show total number of connection?
                    let ws_response = WebSocketResponse {
                        value: format!("Your id is {:#?} and {} in total for this page", &user.unwrap(), &number_of_connection),
                        message_type,
                        client: None,
                        number_of_connection,
                    };

                    let serialized = serde_json::to_string(&ws_response).unwrap();
                    self.state.ws_responses.push(Some(serialized));
                } else {
                    // write equivalent condtional for all users from server here server/src/chat/ws_rs.rs
                    // Find the better solution than this
                    match value.as_ref() {
                        "!clearall" => {
                            self.state.ws_responses.clear();
                        }
                        _ => {
                            self.state.ws_responses.push(Some(serialized));
                        }
                    }
                }
            }

            Msg::Ignore => {
                return false;
            }

            // Client

            Msg::Submit(val) => {
                match val.as_ref() {
                    "" => {}
                    "!clear" => {
                        // similar to !clearall in Msg::WebSocketReady(response)
                        self.state.ws_responses.clear();
                    }
                    "!exit" => {
                        // Equal to WebSocketAction::Disconnect
                        self.state.lost();
                        self.ws_rs.disconnect();
                    }
                    _ => {
                        let State { ws_responses: _ , message_type, client } = &self.state;

                        let emojified = self.emoji.emojify(val.to_string());

                        self.console.log(&emojified);

                        let message_type = message_type.clone();
                        let client = client.clone();

                        // https://serde.rs/
                        let request = WebSocketRequest {
                            value: emojified,
                            message_type,
                            client
                        };

                        self.ws_rs.send(Json(&request));

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
        let WebSocket { ws, ws_service: _ } = &self.ws_rs;
        // test disabled part with cargo run in server directory
        // (You should first start socket and rocket server before you test it)
        // Because it is full stack project
        html! {
            <>
                { social() }
                <section>
                    <nav id="nav", class=("nav", "flex", "center"), >
                        { steadylarner_blog() }
                        <Connect: disabled={ws.is_some()}, onsignal=|_| WebSocketAction::Connect.into(), />
                        <Disconnect: disabled={ws.is_none()}, onsignal=|_| WebSocketAction::Disconnect.into(), />
                    </nav>
                    // <Modal used={used} />
                    <ul
                        id="messages",
                    >
                        {
                            for ws_responses
                            .iter()
                            .enumerate()
                            .map(|(idx, response)| {
                                    let message = response.clone();
                                    // https://serde.rs/, use it before you use data, other names for them later not to be confused
                                    let deserialized: WebSocketResponse = serde_json::from_str(&message.unwrap()).unwrap();
                                    let WebSocketResponse { value, message_type, client, number_of_connection: _, } = deserialized;
                                    // To be explicit here, you can use other variable name or just pass &self.state.client
                                    let user = self.state.client.clone();
                                    view_message(
                                        &idx,
                                        &value,
                                        &message_type,
                                        &client,
                                        &user,
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
                        <ChatInput: disabled={ws.is_none()}, onsignal=Msg::Submit, />
                        <UseImage: disabled={message_type != "image"}, onsignal=Msg::Type, />
                        <UseVideo: disabled={message_type != "video"}, onsignal=Msg::Type, />
                    </section>
                </section>
                // <Modal />
            </>
        }
    }
}
