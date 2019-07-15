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
        input::Input,
        message::{view_message}
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
            Msg::Type(val) => {
                self.state.message_type = val
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
                // Nav
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
                // 
                <ul
                    id="messages",
                >
                    { view_message(&self.state.value, &self.state.message_type) }
                </ul>
                // Input
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
                    <Input: value=&self.state.value, onsignal=Msg::Update, />
                    // { self.view_input() }
                </section>
            </section>
        }
    }
}

// Should remove code here as much as possible with Yew components and functions
impl Model {
    // Let this here before you completely find how components work
    // fn view_input(&self) -> Html<Model> {
    //     html! {
    //         <input
    //             id="msg",
    //             type="text",
    //             placeholder="Type here to start to talk with others and enter to submit",
    //             title="You should enter the chat before you type.",
    //             autocomplete="off",
    //             value=&self.state.value,
    //             oninput=|e| Msg::Update(e.value),
    //         />
    //     }
    // }
}

// 1. input to component - Done
// 2. Write more component and view part for video and image
// 3. Test it work with Type and write CSS for them
// 4. Write blog post "How to use components in Rust" with, video, text and image
//    (code with marked in JavaScript or with Rust)
// 5. Use this to chat app in separate project or make it to chat app?