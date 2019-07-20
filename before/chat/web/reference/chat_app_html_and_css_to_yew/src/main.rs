#[macro_use]
extern crate yew;
use yew::prelude::*;

struct Model {
    value: i64,
}

enum Msg {
    Plus,
    Minus,
    Zero,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();
    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self {
            value: 0,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Plus => self.value = self.value + 1,
            Msg::Minus => self.value = self.value - 1,
            Msg::Zero => self.value = 0,
        }
        true
    }
}

// Refer to HTML file at https://www.steadylearner.com/blog/read/How-to-start-Rust-Chat-App

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
                    <span
                        class=("margin-right-one", "white", "cursor-pointer", "hover", "transition"),
                        id="exit",
                    >
                       { "Exit" }
                    </span>
                </nav>
                <ul
                    id="messages",
                >
                </ul>
                <form id="form", class=("chat-input", "flex", "center"), >
                    <img
                        id="code",
                        class=("flex", "center", "rust-icon", "hover", "cursor-pointer", "transition-half"),
                        title="Use this for whatever you want",
                        src="Rust.svg",
                    />
                    <input
                        id="msg",
                        type="text",
                        placeholder="Type here to start to talk with others.",
                        autocomplete="off",
                    />
                    <button class=("blue", "cursor-pointer"), >{ "Send" }</button>
                    <button
                        id="clear",
                        class=("margin-left-one", "red-white", "cursor-pointer"),
                        type="button",
                    >
                        { "Clear" }
                    </button>
                </form>
            </section>
        }
    }
}

fn main() {
    yew::initialize();
    App::<Model>::new().mount_to_body();
    yew::run_loop();
}
