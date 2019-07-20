// https://github.com/DenisKolodin/yew/tree/master/examples/custom_components/src

use yew::events::IKeyboardEvent;
use yew::{html, Callback, Component, ComponentLink, Html, Renderable, ShouldRender};

pub struct Input {
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
            disabled: true,
            onsignal: None,
        }
    }
}

// https://docs.rs/yew/0.6.0/yew/html/trait.Component.html
impl Component for Input {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Input {
            value: "".into(),
            disabled: props.disabled,
            onsignal: props.onsignal,
        }
    }

    // this is for methods
    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Update(val) => {
                self.value = val;
            }
            Msg::Submit => {
                
                if let Some(ref callback) = self.onsignal { // use this syntax just to use None at the beginning
                    let message = self.value.clone();
                    // self.value = "".to_string(); // why not working?
                    callback.emit(message); // callback is async so shows problem here?
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

impl Renderable<Input> for Input {
    fn view(&self) -> Html<Self> {
        html! {
            <input
                id="msg",
                type="text",
                title="You should enter the chat before you type.",
                disabled=self.disabled,
                placeholder="Type here to start to talk with others and enter to submit",
                autocomplete="off",
                value=&self.value,
                oninput=|e| Msg::Update(e.value),
                onkeypress=|e| {
                    if e.key() == "Enter" { Msg::Submit } else { Msg::Nope }
                },
            />
        }
    }
}