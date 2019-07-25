// https://github.com/DenisKolodin/yew/tree/master/examples/custom_components/src

use yew::{html, Callback, Component, ComponentLink, Html, Renderable, ShouldRender};

pub struct UseImage {
    disabled: bool,
    onsignal: Option<Callback<(String)>>,
}

pub enum Msg {
    Type(String),
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
impl Component for UseImage {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        UseImage {
            disabled: props.disabled,
            onsignal: props.onsignal,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Type(val) => {
                if let Some(ref callback) = self.onsignal { // use this syntax just to use None at the beginning
                    callback.emit(val); // callback is async so shows problem here?
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

impl Renderable<UseImage> for UseImage {
    fn view(&self) -> Html<Self> {
        let mut class = "fas hover fa-file-image white transition-half width-five flex text-center".to_string();
        // use this or think other logics
        if self.disabled {
            class.push_str(" cursor-pointer white")
        } else {
            class.push_str(" link--active-blue")
        }

        html! {
            // It compiled with UseImage also. Verify it well and use button.
            <button
                onclick=|_| Msg::Type("image".to_string()),
                disabled={self.disabled},
                title="Use this to send images.(https://www.steadylearner.com/static/images/post/web/full-stack-rust-chat-app-by-steadylearner.png)",
            >
                <i
                    class=class,
                />
            </button>
        }
    }
}