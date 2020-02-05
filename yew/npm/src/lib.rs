#![recursion_limit="512"]
#![feature(rustc_private)]

#[macro_use]
extern crate stdweb;
// use stdweb::js;

extern crate yew;
use yew::{html, Component, ComponentLink, Html, Renderable, ShouldRender};
use yew::services::{ConsoleService};

mod state;
mod npm;

use self::{
    state::State,

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
            console: ConsoleService::new(),
            state,

            // NPM
            emoji: EmojiService::new()
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Update(val) => {
                let before = format!("{}", &val);
                let emojified = self.emoji.emojify(before.to_string());

                // or use js! here
                
                self.state.value = emojified
            }
            Msg::Exit => {
                self.console.log("The user wants to leave this.")
                // or 
                // js! {
                //     console.log("The user wants to leave this.")
                // }
            }
        }
        true
    }
}

// should write more components to remove this part
impl Renderable<Model> for Model {
    fn view(&self) -> Html<Self> {
        // descturture state before you use it here?
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
                    { self.view_message() }
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
                    />
                    { self.view_input() }
                </section>
            </section>
        }
    }
}

impl Model {
    fn view_message(&self) -> Html<Model> {
        if !(&self.state.value.is_empty()) {
            html! {
                <li>
                    <span> { format!("You: {}", &self.state.value) }</span>
                </li>
            }
        } else {
            html! {
                { "" }
            }
        }
    }

    fn view_input(&self) -> Html<Model> {
        html! {
            <input
                id="msg",
                type="text",
                placeholder="Type here to start to talk with others and enter to submit",
                title="You should enter the chat before you type.",
                autocomplete="off",
                value=&self.state.value,
                oninput=|e| Msg::Update(e.value),
            />
        }
    }
}
