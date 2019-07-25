use yew::{html, Callback, Component, ComponentLink, Html, Renderable, ShouldRender};

// implementation for this will be very simple

pub struct Modal {
    show: bool,
    location: Option<String>,
    // type: String with "text", "image", "video" etc or enum
    onsignal: Option<Callback<()>>,
}

pub enum Msg {
    Set, // show: true, location: Some("www.steadylearner.com") => show: false, lcoation: None,
}

#[derive(PartialEq, Clone)]
pub struct Props {
    pub show: bool,
    pub location: Option<String>,
    pub onsignal: Option<Callback<()>>,
}

impl Default for Props {
    fn default() -> Self {
        Props {
            show: true, 
            location: None,
            onsignal: None,
        }
    }
}

// https://docs.rs/yew/0.6.0/yew/html/trait.Component.html
impl Component for Modal {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Modal {
            show: false,
            location: None,
            onsignal: props.onsignal,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Set => {
                if let Some(ref callback) = self.onsignal { // use this syntax just to use None at the beginning
                    callback.emit(());
                }
            }
        }
        false
    }

    // This is for props

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.show = props.show;
        self.location = props.location;
        self.onsignal = props.onsignal;
        true
    }
}

impl Renderable<Modal> for Modal {
    fn view(&self) -> Html<Self> {
        html! {
            <section>
                { "I am a modal."}
            </section>
        }
    }
}