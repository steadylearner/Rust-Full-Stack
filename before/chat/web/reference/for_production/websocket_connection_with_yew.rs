#![recursion_limit="512"]
#![feature(rustc_private)]

#[macro_use]
extern crate yew;

use yew::services::Task; // Is it used here?
use yew::{html, Component, ComponentLink, Html, Renderable, ShouldRender};

// https://github.com/DenisKolodin/yew/blob/master/src/format/json.rs
// https://github.com/DenisKolodin/yew/blob/master/src/format/macros.rs
use yew::format::{Json};
use yew::services::{ConsoleService};
use yew::services::websocket::{WebSocketService, WebSocketTask, WebSocketStatus};


#[macro_use]
extern crate failure;
use failure::Error;

extern crate serde_derive;
use serde_derive::{Deserialize, Serialize};

type UseBinary = bool;
type Data = String;

// should separate user_id and messages struct? and separate it with another API?
pub struct Model {
  ws_service: WebSocketService,
  console: ConsoleService,
  ws: Option<WebSocketTask>,
  link: ComponentLink<Model>, // Required when you use service?
  user_id: Option<String>, // should be state or here?
  data: Option<String>,

//   state: State,

}

// pub struct State {
//     value: String,
//     edit_value: String,
// }

pub enum WebSocketAction {
    Connect,
    SendData(UseBinary, Data),
    Disconnect,
    Lost,
}

pub enum Msg {
    WebSocketAction(WebSocketAction),
    WebSocketReady(Result<WebSocketResponse, Error>),
    Ignore,

    Clear,
}

impl From<WebSocketAction> for Msg {
    fn from(action: WebSocketAction) -> Self {
        Msg::WebSocketAction(action)
    }
}

/// What they do?, should be vector of &u16 ? stream?
/// This type is used as a request which sent to websocket connection.
/// Should use JSON anyway?, test it with JSON to connect or it may be fomrat is just help you write the data in structure
/// Test it with byte macro in Yew
#[derive(Serialize, Debug)] // send
struct WebSocketRequest {
    // value: String,
    // value: Vec<u8>,
    value: String,
}

/// This type is an expected response from a websocket connection.
#[derive(Deserialize, Debug)] // receive
pub struct WebSocketResponse {
    value: String,
}

// lack any function here?

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        // let state = State {
        //     value: "".into(),
        //     edit_value: "".into(),
        // };
        Model {
            ws_service: WebSocketService::new(),
            console: ConsoleService::new(),
            ws: None,
            link,
            user_id: None,
            data: None,

            // state,
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
                        let task = self.ws_service.connect("ws://127.0.0.1:7777/ws", callback, notification); // 1.
                        self.ws = Some(task);
                    }
                    WebSocketAction::SendData(binary, _data) => {
                        let request = WebSocketRequest {
                            value: "Test".to_string(), 
                        }; // send {"value": "Test"} to websocket server
                        if binary {
                            // println!("Send Json format binary data to websocket");
                            self.ws.as_mut().unwrap().send_binary(Json(&request));
                        } else {
                            // println!("Use if you want to use text data for the websocket instead");
                            unimplemented!();
                        }
                    }
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
                self.data = response.map(|data| data.value).ok(); // and update app state?
            }
            Msg::Ignore => {
                return false;
            }
            Msg::Clear => {
                self.console.log("The user wants to clear the messages.");
                // method to remove every messages in the state
            }
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
                    // should be in messages
                    { self.ws_connection() }
                    { self.ws_data() }
                </ul>
                // Use React like form handler with state? Then, it becomes complicated.
                // https://github.com/DenisKolodin/yew/blob/master/examples/todomvc/src/lib.rs
                <form
                    id="form",
                    class=("chat-input", "flex", "center"),
                    // onsubmit=|e| WebSocketAction::SendData(true, self.state.value).into(),
                    // message to vector of string?
                >
                    <img
                        id="code",
                        class=("flex", "center", "rust-icon", "hover", "cursor-pointer", "transition-half"),
                        title="Use this for whatever you want",
                        src="https://www.steadylearner.com/static/images/code/Rust.svg",
                    />
                    <input
                        id="msg",
                        type="text",
                        placeholder="Type here to start to talk with others.",
                        // oninput=|e| Msg::UpdateMessage(e.value), // should edit state.value
                        autocomplete="off",
                    />
                    // remove this if it complicates the app
                    <button
                        class=("blue", "cursor-pointer"),
                        disabled=self.ws.is_none(),
                        // onclick=|_| WebSocketAction::SendData(true, "test".to_string()).into(),
                        // It work in JSON format
                    >
                        { "Send" }
                    </button>
                    <button
                        id="clear",
                        class=("margin-left-one", "red-white", "cursor-pointer"),
                        type="button",
                        onclick=|_| Msg::Clear,
                    >
                        { "Clear" }
                    </button>
                </form>
            </section>
        }
    }
}

// To test Websocket funtion
impl Model {
    // 1.
    fn ws_connection(&self) -> Html<Model> {
        if let Some(value) = &self.ws {
            html! {
                <li>{ "You entered to the chat. Say something." }</li>
            } // should be list of messages with messages: Vec<String>
        } else {
            html! {
                <li>{ "You are not connected to websocket. Click [Enter] button for that." }</li>
            }
        }
    }
    fn ws_data(&self) -> Html<Model> {
        if let Some(value) = &self.data { // payload?
            // can I combine codes here?
            html! {
                <li>{ value }</li>
            } 
            // should be list of messages with messages: Vec<String>
        } else {
            html! {
                <li>{ "There is no messages yet." }</li>
            }
        }
    }
}

// WebSocket
// refer to dashboard example in Yew

// 1. Make a connection - Done

// In Rocket Server
// Accepted a new tcp connection from 127.0.0.1:39660.
// Browser Request from "http://127.0.0.1:8080"
// Client found is None
// 127.0.0.1:39660 entered and the number of live connections is 1

// In Yew Chat Client
// Enter and Exit work with
// console.log("Websocket is ready. Start to chat with others.");
// console.log("The user wants to leave this page.");

// In JavaScript Chat Cliente
// When Click Enter in Yew client, show message below
// 127.0.0.1:39768 entered and the number of live connections is 2

// 2. receive data from websocket and render it to app? - Done

// found what is payload for this app

// Yew(specific state management and its methods)

// Refer to todomvc and humps

// 1. send data from Yew app with form in yew(similar to React) and ws_data part work or not

// 2. Read more documentation and codes and refactor app?
// 3. Find how to handle data in input and send, it will be simialr to React?
// 4. Write yew code for specific messages
// 5. Refactor App














// When you end your app, follow the structure of other projects to refactor app

// mod components;
// mod services;
// mod util;

// use self::{
// components::{controls::Controls, messages::Messages, stats::Stats},
// services::{window};
// util::*,
// };

// https://docs.rs/yew/0.6.0/yew/services/websocket/enum.WebSocketStatus.html
