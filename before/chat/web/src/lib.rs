#![recursion_limit="512"]
#![feature(rustc_private)]

#[macro_use]
extern crate stdweb;
// use stdweb::web::{document, window, IParentNode, IElement};
use stdweb::js;
// items from traits can only be used if the trait is in scope, IParentNode and IElement are traits?

#[macro_use]
extern crate yew;

use yew::services::Task;
use yew::{html, Component, ComponentLink, Html, Renderable, ShouldRender};

use yew::format::{Json};
use yew::events::IKeyboardEvent;
use yew::services::{ConsoleService};
use yew::services::websocket::{WebSocketService, WebSocketTask, WebSocketStatus};
use yew::services::storage::{StorageService, Area};

const NAME: &'static str = "steadylearner.full.stack.rust.chat.app";

extern crate failure;
use failure::Error;

extern crate serde_derive;
// use serde_derive::{Deserialize, Serialize};

mod state;

mod http_model;
mod services;
mod components;

mod npm;

use self::{
    state::State,
    http_model::websocket_json::{WebSocketRequest, WebSocketResponse},
    components::{
        input::Input, 
        messages::view_response
    },
    npm::{
        emoji::EmojiService, 
    },
};

pub struct Model {
    ws_service: WebSocketService,
    ws: Option<WebSocketTask>,
    console: ConsoleService,
    link: ComponentLink<Model>, // Required when you use service?

    state: State,
    storage: StorageService,

    emoji: EmojiService,   
}

pub enum WebSocketAction {
    Connect,
    Disconnect,
    Lost,
}

pub enum Msg {
    WebSocketAction(WebSocketAction),
    WebSocketReady(Result<WebSocketResponse, Error>),
    Ignore,
    
    Update(String),
    Submit,
    Nope,
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
        let storage = StorageService::new(Area::Local);
        let ws_responses = {
            if let Json(Ok(restored_model)) = storage.restore(NAME) {
                restored_model
            } else {
                Vec::new()
            }
        };

        let state = State {
            user_id: None,
            user_inputs: Vec::new(),
            ws_responses,
            value: "".to_string(),
        };

        Model {
            ws_service: WebSocketService::new(),
            ws: None,
            console: ConsoleService::new(),
            link,

            state,
            storage,

            // NPM
            emoji: EmojiService::new(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            // WebSocket, should find how to remove them from here, move them to state?
            // make functions for them
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
                    WebSocketAction::Disconnect => {

                        self.state.user_id = None;
                        self.ws.take().unwrap().cancel();

                        // self.console.clear();

                        // self.console.log("You are disconnected");
                    }
                    WebSocketAction::Lost => {
                        self.state.user_id = None;
                        // self.ws.as_mut().unwrap().send_binary(Json(&request));

                        self.ws = None;
                    }
                }
            }
            Msg::WebSocketReady(response) => { // payload?
                self.console.log("Websocket is ready. Start to chat with others.");
                let socket_input = response.map(|data| data).ok();

                let serialized = serde_json::to_string(&socket_input).unwrap(); // value in view_response

                self.console.log(&serialized);

                let for_if = serialized.clone();

                let ws_response: WebSocketResponse = serde_json::from_str(&for_if).unwrap();
                let WebSocketResponse { value, client, number_of_connection: _ } = ws_response;

                if self.state.user_id == None {
                    self.state.user_id = client;
                }

                if value == "!clearall" {
                    self.state.ws_responses.clear();
                } 

                // if number_of_connection == 5 {

                // }

                self.state.ws_responses.push(Some(serialized));
            }

            Msg::Ignore => {
                return false;
            }

            // Below are for View and other Controllers 

            Msg::Update(val) => {
                self.state.value = val
            }

            Msg::Submit => {
                // use more condtionals later

                // refactor this code and extract self.state.value.clear(); and other duplicate parts
                if self.state.value.is_empty() {
                                  
                } else if &self.state.value == "!clear" {
                    self.state.ws_responses.clear();   
                    self.state.value.clear();
                } else if &self.state.value == "!clearall" {
                    let user_id = self.state.user_id.clone();
                    let request = WebSocketRequest {
                        value: "!clearall".into(),
                        user_id,
                    };
                    
                    self.ws.as_mut().unwrap().send_binary(Json(&request));
                    self.state.value.clear();
                } else if &self.state.value == "!exit" {
                    self.state.user_id = None;
                    self.ws.take().unwrap().cancel();
                } else {
                    let user_id = self.state.user_id.clone();

                    let before = format!("{}", &self.state.value);
                    let emojified = self.emoji.emojify(before.to_string());
                    
                    let request = WebSocketRequest {
                        value: emojified,
                        user_id,
                    };
                    
                    self.ws.as_mut().unwrap().send_binary(Json(&request));
                    
                    // let inner_height = window().inner_height(); // u32 to string for console to use
                    
                    // let inner_height = window().inner_height().to_string(); // u32 to string for console to use
                    // self.console.log(&inner_height); It shows inner_height 

                    // https://docs.rs/stdweb/0.4.17/stdweb/web/trait.IElement.html#method.set_scroll_top

                    // should find yew way to do this.
                    // let body = document().query_selector("body").unwrap().unwrap();
                    // body.set_scroll_top(inner_height.into());

                    // or use javascript with js! because it is not relevant to components
                    js! {
                        setTimeout(() => window.scrollTo({ top: window.innerHeight, behavior: "auto" }), 50);
                        console.log("Find the number you think best for users");
                        // console.log(emoji); It shows emoji module from node-emoji
                        // You can use Rust code here instead of EmojiService from npm/emoji.rs
                    };

                    self.state.value.clear();
                }

                // use serialized JSON with user_id, when you have login function etc
                // let user_input = format!("{}", &value); 
                // self.state.user_inputs.push(user_input.into());
            }
            Msg::Nope => {}
        }
        // Test it with !clear and without it
        self.storage.store(NAME, Json(&self.state.ws_responses));
        true
    }
}

// should write more components to remove this part
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
                    { 
                        for self.state.ws_responses
                        .iter()
                        .enumerate()
                        .map(|(idx, response)| { 
                            view_response(idx, response, &self.state.user_id) }
                        ) 
                    }
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
                        onclick=|_| Msg::Nope,
                    />
                    { self.view_input() }
                    // <Input: disabled=self.ws.is_none(), onsignal=Msg::Submit, />
                    // but value does not become "" after submit
                    // to test it, use files in /reference/for_production/with_input_component folder 
                </section>
            </section>
        }
    }
}

// test with clear and remove them with component
impl Model {
    fn view_input(&self) -> Html<Model> {
        html! {
            <input
                id="msg", 
                type="text",
                placeholder="Type here to start to talk with others and enter to submit",
                title="You should enter the chat before you type.",
                autocomplete="off",
                disabled=self.ws.is_none(),
                value=&self.state.value,
                oninput=|e| Msg::Update(e.value),
                onkeypress=|e| {
                    if e.key() == "Enter" { Msg::Submit } else { Msg::Nope }
                }, 
            />
        }
    }
}