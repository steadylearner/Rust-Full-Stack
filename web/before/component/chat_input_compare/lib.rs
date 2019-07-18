#![recursion_limit="512"]
#![feature(rustc_private)]

#[macro_use]
extern crate stdweb;
// use stdweb::js;

extern crate yew;
use yew::events::IKeyboardEvent;
use yew::{html, Component, ComponentLink, Html, Renderable, ShouldRender};
use yew::services::{ConsoleService};

mod state;
mod components;
mod npm;

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
};

pub struct Model {
    console: ConsoleService,
    state: State,
    emoji: EmojiService,
}

pub enum Msg {
    Update(String),
    Submit,
    Type(String), // use enum later?
    Exit,
    Nope,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _link: ComponentLink<Self>) -> Self {
        let state = State {
            value: "".to_string(),
            response: "".to_string(),
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
            Msg::Update(val) => {
                self.state.value = val
            }
            Msg::Submit => {
                let before = format!("{}", &self.state.value);
                let emojified = self.emoji.emojify(before.to_string());

                self.state.response = emojified;
                // conditional or timeout here later?
                self.state.value.clear(); // Use this instead of self.state.value = "".to_string();
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
        // let State { value, response, message_type } = &self.state;
        let State { value, response, message_type } = &self.state;
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
                        { view_message(response, message_type) }
                    </ul>
                    <section
                        id="form",
                        class=("chat-input", "flex", "center"),
                    >
                        <UseCode: disabled={message_type != "code"}, onsignal=Msg::Type, />
                        <ChatInput: value=value, oninput=Msg::Submit, onkeypress=Msg::Update, />
                        // { self.chat_input() }
                        <UseImage: disabled={message_type != "image"}, onsignal=Msg::Type, />
                        <UseVideo: disabled={message_type != "video"}, onsignal=Msg::Type, />
                    </section>
                </section>
            </>
        }
    }
}

// How to make component for this instead of impl?
impl Model {
    fn chat_input(&self) -> Html<Model> {
        html! {
            <input
                id="msg",
                type="text",
                placeholder="Type here to start to talk with others and enter to submit",
                title="You should enter the chat before you type.",
                autocomplete="off",
                value=&self.state.value,
                oninput=|e| Msg::Update(e.value),
                onkeypress=|e| {
                    if e.key() == "Enter" { Msg::Submit } else { Msg::Nope }
                }, 
            />
        }
    }
}