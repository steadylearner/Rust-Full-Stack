use yew::{html, Callback, Component, ComponentLink, Html, Renderable, ShouldRender};

// implementation for this will be very simple
// should be folder later with other type of modal and modal_type etc in modal.rs in main folder
// or modal_type and conditional statements

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
        let mut modal_class = "modal ".to_string();
        
        if self.show {
            modal_class.push_str("inherit-display");
        } else {
            modal_class.push_str("x-display");
        };

        let src = "https://www.steadylearner.com/static/images/brand/code.png".to_string();

        let shadow_class="width-vw height-vh cursor-pointer absolute";
        let image_class="max-width box-shadow-white relative modal-content--image";

        // It compiles when you have errors in format for Yew or stdweb html!

        html! {
            <section class=modal_class, id="modal", title="Click this to close modal.", >
                <section class=shadow_class, onclick=|_| Msg::Set, ></section>// It compiles without / at the end > so verify it
                <img class=image_class, src=src, />
            </section>
        }
    }
}