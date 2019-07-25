use yew::{html, Callback, Component, ComponentLink, Html, Renderable, ShouldRender};

pub struct Disconnect {
    disabled: bool,
    onsignal: Option<Callback<()>>,
}

pub enum Msg {
    Disconnect,
}

#[derive(PartialEq, Clone)]
pub struct Props {
    pub disabled: bool,
    pub onsignal: Option<Callback<()>>,
}

impl Default for Props {
    fn default() -> Self {
        Props {
            disabled: true,
            onsignal: None,
        }
    }
}

impl Component for Disconnect {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Disconnect {
            disabled: props.disabled,
            onsignal: props.onsignal,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Disconnect => {
                if let Some(ref mut callback) = self.onsignal {
                    callback.emit(());
                }
            }
        }
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.disabled = props.disabled;
        self.onsignal = props.onsignal;
        true
    }
}

impl Renderable<Disconnect> for Disconnect {
    fn view(&self) -> Html<Self> {
        html! {
            <button
                id="exit",
                class=("margin-right-one", "white", "cursor-pointer", "hover", "transition", "theme-black"),
                onclick=|_| Msg::Disconnect,
                disabled={self.disabled},
                title="Click this to disconnect from the Rust chat app.",
            >
                { "Exit" }
            </button>
        }
    }
}