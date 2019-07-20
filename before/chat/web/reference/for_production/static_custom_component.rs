// https://github.com/DenisKolodin/yew/tree/master/examples/custom_components/src

use yew::events::IKeyboardEvent;
use yew::{html, Callback, Component, ComponentLink, Html, Renderable, ShouldRender};

pub struct Input {
    value: String,
    // onsignal: Option<Callback<()>>,
}

pub enum Msg {
    Update(String),
}

impl Component for Input {
    type Message = Msg;
    type Properties = ();

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Input {
            value: "".into(),
            // onsignal: props.onsignal,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            // Msg::Submit => {
            //     if let Some(ref mut callback) = self.onsignal {
            //         callback.emit(());
            //     }
            // }
            Msg::Update(val) => {
                self.value = val;
            }
        }
        false
    }
}

impl Renderable<Input> for Input {
    fn view(&self) -> Html<Self> {
        html! {
            <input
                id="msg",
                type="text",
                placeholder="Type here to start to talk with others and enter to submit",
                autocomplete="off",
                value=&self.value, // should be self
                oninput=|e| Msg::Update(e.value), // should be self
                // onkeypress=|e| { // should be prop
                //     if e.key() == "Enter" { Msg::Submit } else { Msg::Nope }
                // },
            />
        }
    }
}