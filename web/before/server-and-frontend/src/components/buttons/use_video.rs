// https://github.com/DenisKolodin/yew/tree/master/examples/custom_components/src

use yew::{html, Callback, Component, ComponentLink, Html, Renderable, ShouldRender};

pub struct UseVideo {
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
impl Component for UseVideo {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        UseVideo {
            disabled: props.disabled,
            onsignal: props.onsignal,
        }
    }

    // this is for methods
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

    // This is for props

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.disabled = props.disabled;
        self.onsignal = props.onsignal;
        true
    }
}

impl Renderable<UseVideo> for UseVideo {
    fn view(&self) -> Html<Self> {
        let mut class = "fab fa-youtube hover flex transition-half text-center width-two flex margin-right-one".to_string();
        // use disabled or think another logics, is cursor-pointer necessary here?
        if self.disabled {
            class.push_str(" cursor-pointer white")
        } else {
            class.push_str(" red-white")
        }

        html! {
            <button
                onclick=|_| Msg::Type("video".to_string()),
                disabled={self.disabled},
                title="Use this to send videos with YouTube video id(s7TVVyTyReU)",
            >
                <i
                    class=class,
                />
            </button>
        }
    }
}
