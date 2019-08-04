use yew::{html, Callback, Component, ComponentLink, Html, Renderable, ShouldRender};

pub struct Connect {
    disabled: bool,
    onsignal: Option<Callback<()>>,
}

pub enum Msg {
    Connect,
}

#[derive(PartialEq, Clone)]
pub struct Props {
    pub disabled: bool,
    pub onsignal: Option<Callback<()>>,
}

impl Default for Props {
    fn default() -> Self {
        Props {
            disabled: false,
            onsignal: None,
        }
    }
}

impl Component for Connect {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Connect {
            disabled: props.disabled,
            onsignal: props.onsignal,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Connect => {
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

impl Renderable<Connect> for Connect {
    fn view(&self) -> Html<Self> {
        html! {
            <button
                id="connect",
                class=("margin-right-one", "white", "cursor-pointer", "hover", "transition", "theme-black"),
                onclick=|_| Msg::Connect,
                disabled={self.disabled},
                title="Click this to connect to Rust chat app.",
            >
                { "Enter" }
            </button>
        }
    }
}
