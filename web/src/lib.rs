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
    Type(String), // use enum later?
    Exit
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _link: ComponentLink<Self>) -> Self {
        let state = State {
            value: "".to_string(),
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
                let before = format!("{}", &val);
                let emojified = self.emoji.emojify(before.to_string());

                self.state.value = emojified
            }
            Msg::Type(val) => {
                self.state.message_type = val
            }
            Msg::Exit => {
                self.console.log("The user wants to leave this.")
            }
        }
        true
    }
}

// Make Enter and Exit components later
impl Renderable<Model> for Model {
    fn view(&self) -> Html<Self> {
        let State { value, message_type } = &self.state;
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
                        { view_message(value, message_type) }
                    </ul>
                    <section
                        id="form",
                        class=("chat-input", "flex", "center"),
                    >
                        <UseCode: disabled={message_type != "code"}, onsignal=Msg::Type, />
                        <ChatInput: value=value, onsignal=Msg::Update, />
                        <UseImage: disabled={message_type != "image"}, onsignal=Msg::Type, />
                        <UseVideo: disabled={message_type != "video"}, onsignal=Msg::Type, />
                    </section>
                </section>
            </>
        }
    }
}

// 1. Read more documentation and organize code
// 2. Write blog post "How to use markdown in Rust Frontned"
//    (code with marked in JavaScript or with Rust)
// 3. Use this to chat app in separate project or make it to chat app? and whenever they submit back to test default