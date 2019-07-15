// https://github.com/DenisKolodin/yew/tree/master/examples/custom_components/src

use yew::{html, Callback, Component, ComponentLink, Html, Renderable, ShouldRender};

pub struct Input {
    value: String,
    onsignal: Option<Callback<(String)>>,
}

pub enum Msg {
    Update(String),
}

#[derive(PartialEq, Clone)]
pub struct Props {
    pub value: String,
    pub onsignal: Option<Callback<(String)>>,
}

impl Default for Props {
    fn default() -> Self {
        Props {
            value: "".to_string(),
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
            onsignal: props.onsignal,
        }
    }

    // this is for methods
    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Update(val) => {
                if let Some(ref callback) = self.onsignal { // use this syntax just to use None at the beginning
                    callback.emit(val); // callback is async so shows problem here?
                }
            }
        }
        false
    }

    // This is for props

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.value = props.value;
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
                placeholder="Type here to start to talk with others and enter to submit",
                title="You should enter the chat before you type.",
                autocomplete="off",
                value=&self.value,
                oninput=|e| Msg::Update(e.value),
            />
        }
    }
}