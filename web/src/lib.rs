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
    console: ConsoleService,
    state: State,
    emoji: EmojiService,
}

pub enum Msg {
    Submit(String),
    Type(String), // use enum later?
    Exit,
    Nope,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _link: ComponentLink<Self>) -> Self {
        let state = State {
            responses: Vec::new(),
            message_type: "text".to_string(),
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
                let before = format!("{}", &val);
                let emojified = self.emoji.emojify(before.to_string());

                let message_type = self.state.message_type.clone(); 

                let request = WebSocketRequest {
                    value: emojified,
                    message_type,
                };

                // Convert the WebSocketRequest to a JSON string.
                // When you send, use it
                let serialized = serde_json::to_string(&request).unwrap();

                self.state.responses.push(serialized);

                if &self.state.message_type != "text" {
                    self.state.message_type = "text".to_string();
                }
            }
            Msg::Type(val) => {
                self.state.message_type = val
            }
            Msg::Exit => {
                self.console.log("The user wants to leave this.")
            }
            Msg::Nope => {}
        }
        true
    }
}

// Make Enter and Exit components later
impl Renderable<Model> for Model {
    fn view(&self) -> Html<Self> {
        let State { responses, message_type } = &self.state;
        html! {
            <>
                { social() }
                <section>
                    <nav id="nav", class=("nav", "flex", "center"), >
                        { steadylarner_blog() }
                        <button
                            id="connect",
                            class=("margin-right-one", "white", "cursor-pointer", "hover", "transition", "theme-black"),
                        >
                            { "Enter" }
                        </button>
                        <button
                            id="exit",
                            class=("margin-right-one", "white", "cursor-pointer", "hover", "transition", "theme-black"),
                            onclick=|_| Msg::Exit,
                        >
                            { "Exit" }
                        </button>
                    </nav>
                    <ul
                        id="messages",
                    >
                        // { view_message(response, message_type) } // use for here instead
                        { 
                            for responses
                            .iter()
                            .enumerate()
                            .map(|(_idx, response)| {  // use idx later
                                // use it before you use, other names for them later not to be confused
                                let deserialized: WebSocketRequest = serde_json::from_str(&response).unwrap();
                                view_message(&deserialized.value, &deserialized.message_type) }
                            ) 
                        }
                    </ul>
                    <section
                        id="form",
                        class=("chat-form", "flex", "center"),
                    >
                        <UseCode: disabled={message_type != "code"}, onsignal=Msg::Type, />
                        <ChatInput: onsignal=Msg::Submit, />
                        // or { self.chat_input() } - refer to /before/component/chat_input_compare folder
                        <UseImage: disabled={message_type != "image"}, onsignal=Msg::Type, />
                        <UseVideo: disabled={message_type != "video"}, onsignal=Msg::Type, />
                    </section>
                </section>
            </>
        }
    }
}

// 1. Write post for "How to use components in Rust Yew"
// 2. Update response to Vec<String> or Vec<Option<String>> and write echo chat app without websocket connection with index
// 3. Write code for server side and web for chat app with ws-rs
// 4. Write blog post "Fullstack Rust Chat App"
// 5. End the "Fullstack Rust Web App" series
