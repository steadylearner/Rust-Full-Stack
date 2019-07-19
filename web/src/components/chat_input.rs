// https://github.com/DenisKolodin/yew/tree/master/examples/custom_components/src

use yew::events::IKeyboardEvent;
use yew::{html, Callback, Component, ComponentLink, Html, Renderable, ShouldRender};
use stdweb::js;

pub struct ChatInput {
    value: String,
    disabled: bool,
    onsignal: Option<Callback<(String)>>,
}

pub enum Msg {
    Update(String),
    Submit,
    Nope,
}

#[derive(PartialEq, Clone)]
pub struct Props {
    pub disabled: bool,
    pub onsignal: Option<Callback<(String)>>,
}

impl Default for Props {
    fn default() -> Self {
        Props {
            disabled: false,
            onsignal: None,
        }
    }
}

// https://docs.rs/yew/0.6.0/yew/html/trait.Component.html
impl Component for ChatInput {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        ChatInput {
            value: "".to_string(),
            disabled: false,
            onsignal: props.onsignal,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Update(val) => {
                self.value = val;
            }
            Msg::Submit => {
                if let Some(ref callback) = self.onsignal { // use this syntax just to use None at the beginning
                    let message = self.value.clone();
                    callback.emit(message);
                    // self.value.clear(); does not work here(callback is async so shows problem here?)
                    js! {
                        setTimeout(() => {
                            document.querySelector("#chat-input").value = "";
                            window.scrollTo({ top: window.innerHeight, behavior: "auto" });
                            // temporary solution, use number you like or find other ways
                        }, 10);
                    }
                }
            }
            Msg::Nope => {}
        }
        false
    }

    // This is for props

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.disabled = props.disabled;
        self.onsignal = props.onsignal;
        true
    }
}

impl Renderable<ChatInput> for ChatInput {
    fn view(&self) -> Html<Self> {
        html! {
            <input
                id="chat-input",
                type="text",
                placeholder="Type here to start to talk with others and enter to submit",
                title="You should enter the chat before you type.",
                autocomplete="off",
                disabled=self.disabled,
                value=&self.value,
                oninput=|e| Msg::Update(e.value),
                onkeypress=|e| {
                    if e.key() == "Enter" { Msg::Submit } else { Msg::Nope }
                }, 
            />
        }
    }
}