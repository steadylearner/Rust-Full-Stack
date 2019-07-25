use yew::{html, Callback, Component, ComponentLink, Html, Renderable, ShouldRender};

// implementation for this will be very simple
// should be folder later with other type of modal and modal_type etc in modal.rs in main folder

pub struct ImageModal {
    show: bool,
    location: String,
    // type: String with "text", "image", "video" etc or enum
    onsignal: Option<Callback<String>>,
}

pub enum Msg {
    Set, // show: true, location: "" => show: false, lcoation: None,
}

#[derive(PartialEq, Clone)]
pub struct Props {
    pub show: bool,
    pub location: String,
    pub onsignal: Option<Callback<String>>,
}

impl Default for Props {
    fn default() -> Self {
        Props {
            show: true, 
            location: "".to_string(),
            onsignal: None,
        }
    }
}

// https://docs.rs/yew/0.6.0/yew/html/trait.Component.html
impl Component for ImageModal {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        ImageModal {
            show: false,
            location: "".to_string(),
            onsignal: props.onsignal,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Set => {
                if let Some(ref callback) = self.onsignal { // use this syntax just to use None at the beginning
                    callback.emit("".to_string());
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

impl Renderable<ImageModal> for ImageModal {
    fn view(&self) -> Html<Self> {
        let class = if self.show {
            "".to_string()
        } else {
            "x-display".to_string()
        };

        html! {
            <section class=class, onclick=|_| Msg::Set, id="modal", >
                { "I will be a modal" }
            </section>
        }
    }
}