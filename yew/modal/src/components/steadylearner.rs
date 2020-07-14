use yew::{html, Callback, Component, ComponentLink, Html, Renderable, ShouldRender};

pub struct Steadylearner {
    onsignal: Option<Callback<String>>,
}

pub enum Msg {
    Set(String),
}

#[derive(PartialEq, Clone)]
pub struct Props {
    pub onsignal: Option<Callback<String>>,
}

impl Default for Props {
    fn default() -> Self {
        Props {
            onsignal: None,
        }
    }
}

// https://docs.rs/yew/0.6.0/yew/html/trait.Component.html
impl Component for Steadylearner {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Steadylearner {
            onsignal: props.onsignal,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Set(val) => {
                if let Some(ref callback) = self.onsignal { // use this syntax just to use None at the beginning
                    callback.emit(val);
                }
            }
        }
        false
    }

    // This is for props

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.onsignal = props.onsignal;
        true
    }
}

impl Renderable<Steadylearner> for Steadylearner {
    fn view(&self) -> Html<Self> {
        let src = "https://www.steadylearner.com/static/images/brand/code.png".to_string(); // payload here
        let class = "width-two-and-a-half theme-white border-round margin-right-half hover cursor-pointer";

        html! {
            <span 
                class=("white", "bold", "flex", "center", "right-auto"), 
            >
                <img 
                    class=class,
                    title="Click this to use modal.",
                    src="https://www.steadylearner.com/static/images/code/Rust.svg",
                    onclick=|_| Msg::Set(src.clone()),
                />
                <a
                    class=("no-text-decoration", "white"),
                    href="https://www.steadylearner.com/blog/search/Rust",
                    title="Click it to learn how to code this.",
                    target="_blank",
                    rel="noopener noreferrer",
                >
                    { "Full Stack Rust Chat App" }
                </a>
            </span>
        }
    }
}