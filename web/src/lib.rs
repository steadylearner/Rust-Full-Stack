#![recursion_limit="512"]
#![feature(rustc_private)]

#[macro_use]
extern crate stdweb;
// use stdweb::js;

extern crate yew;
use yew::{html, Component, ComponentLink, Html, Renderable, ShouldRender};
use yew::services::{ConsoleService};

mod state;
mod components;
mod npm;

mod http_model;

use self::{
    state::State,

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
        website::{
            steadylarner_blog,
            social,
        }
    },

    npm::{
        emoji::EmojiService,
    },

    http_model::{
        websocket_json::WebSocketRequest,
    },
};

pub struct Model {
    state: State,
    
    console: ConsoleService,
    
    // NPM
    emoji: EmojiService,
}

pub enum Msg {
    Submit(String),
    Type(String), // use enum later?
    Connect,
    Disconnect,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _link: ComponentLink<Self>) -> Self {
        let state = State {
            responses: Vec::new(),
            message_type: "text".to_string(),
            connection: true,
        };

        Model {
            state,

            // Yew
            console: ConsoleService::new(),

            // NPM
            emoji: EmojiService::new()
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Submit(val) => {
                // use this or if val == "!clear"
                match val.as_ref() {
                    "!clear" => {
                        self.state.responses.clear();
                    }
                    "!exit" => {
                        self.state.connection = false;
                    }
                    _ => {
                        let before = format!("{}", &val);
                        let emojified = self.emoji.emojify(before.to_string());

                        let message_type = self.state.message_type.clone(); 

                        // https://serde.rs/
                        let request = WebSocketRequest {
                            value: emojified,
                            message_type,
                        };

                        // Convert the WebSocketRequest to a JSON string.
                        // use it when you send JSON or other data 
                        let serialized = serde_json::to_string(&request).unwrap();

                        self.console.log(&serialized);

                        self.state.responses.push(serialized);

                        if &self.state.message_type != "text" {
                            self.state.message_type = "text".to_string();
                        }
                    }
                }
            }
            Msg::Type(val) => {
                self.state.message_type = val
            }
            Msg::Connect => {
                self.state.connection = true;
            }
            Msg::Disconnect => {
                self.state.connection = false;
            }
        }
        true
    }
}

// Make Enter and Exit components later
impl Renderable<Model> for Model {
    fn view(&self) -> Html<Self> {
        let State { responses, message_type, connection } = &self.state;
        html! {
            <>
                { social() }
                <section>
                    <nav id="nav", class=("nav", "flex", "center"), >
                        { steadylarner_blog() }
                        <Connect: disabled={*connection}, onsignal=|_| Msg::Connect, />
                        <Disconnect: disabled={!*connection}, onsignal=|_| Msg::Disconnect, />
                    </nav>
                    <ul
                        id="messages",
                    >
                        // { view_message(response, message_type) }
                        { 
                            for responses
                            .iter()
                            .enumerate()
                            .map(|(idx, response)| {
                                    // https://serde.rs/, use it before you use data, other names for them later not to be confused
                                    let deserialized: WebSocketRequest = serde_json::from_str(&response).unwrap();
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
                        <ChatInput: disabled={!*connection}, onsignal=Msg::Submit, />
                        // or { self.chat_input() } - refer to /before/component/chat_input_compare folder
                        <UseImage: disabled={message_type != "image"}, onsignal=Msg::Type, />
                        <UseVideo: disabled={message_type != "video"}, onsignal=Msg::Type, />
                    </section>
                </section>
            </>
        }
    }
}
