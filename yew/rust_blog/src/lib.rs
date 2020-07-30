#![recursion_limit = "256"]

use yew::services::{
    ConsoleService,
    fetch::{FetchService, FetchTask, Request, Response},
};

use yew::events::IKeyboardEvent;

// You can also
// uncomment # failure = "0.1.5" in Cargo.toml use failure::Error; and Result<String, Error> instead;
use yew::format::{Text, Nothing}; 

use yew::{html, Component, ComponentLink, Html, Renderable, ShouldRender};

mod code;

use self::{
    code::view_code,
};

pub struct Model {
    fetch_service: FetchService,
    console: ConsoleService,

    link: ComponentLink<Model>,
    fetching: bool,
    
    edit_value: String,
    value: String,
    data: Option<String>,
    
    ft: Option<FetchTask>,
}

pub enum Msg {
    Update(String),
    Submit,
    FetchReady(Text),
    Ignore,
    Nope,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {

        Model {
            fetch_service: FetchService::new(),
            console: ConsoleService::new(),

            fetching: false,
            edit_value: "".to_string(),
            value: "https://raw.githubusercontent.com/steadylearner/Rust-Full-Stack/master/README.md".to_string(),
            link,
            data: None,
            
            ft: None,
        }
    }

    fn mounted(&mut self) -> ShouldRender {
        self.fetch_data();

        true
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Update(val) => {
                self.edit_value = val;
            }
            Msg::Submit => {
                self.value = self.edit_value.clone();
                self.fetch_data();
            }
            Msg::FetchReady(response) => {
                self.fetching = false;
                self.data = response.map(|data| data).ok();
            }
            Msg::Ignore => {
                return false;
            }
            Msg::Nope => {}
        }
        true
    }
}

impl Renderable<Model> for Model {
    fn view(&self) -> Html<Self> {
        let rust_image_class = "width-two-and-a-half theme-white border-round";
        let mut subnav_class = "sub theme-black border-white center ".to_string();

        if self.fetching {
            subnav_class.push_str("x-display");
        } else {
            subnav_class.push_str("inherit-display");
        }

        html! {
            <section>
                <section id="content", class=("main-width", "flex-column", "center-auto-margin"), >
                    <section
                        class=("post-form", "flex", "center"),
                    >
                        <a
                            class=("no-text-decoration", "hover", "transition-half", "center"),
                            href="https://www.steadylearner.com/blog/search/Rust",
                            title="Click it to learn how to code this.",
                            target="_blank",
                            rel="noopener noreferrer",
                        >
                            <img
                                src="https://www.steadylearner.com/static/images/code/Rust.svg",
                                class=rust_image_class,
                            />
                        </a>
                        <input
                            type="text",
                            autocomplete="off",
                            disabled=self.fetching,
                            value=&self.value,
                            oninput=|e| Msg::Update(e.value),
                            onkeypress=|e| {
                                if e.key() == "Enter" { Msg::Submit } else { Msg::Nope }
                            },
                        />
                    </section>
                    { self.view_data() }
                    <footer class=subnav_class, >
                        <a
                            class=("flex", "no-text-decoration", "hover", "cursor-pointer", "transition-half", "auto", "margin-left-one"),
                            href="https://www.steadylearner.com/blog/search/Rust",
                            title="Click it to learn how to code this.",
                            target="_blank",
                            rel="noopener noreferrer",
                        >
                            <span 
                                class=("white", "bold", "flex", "center"), 
                            >
                                { "© Steadylearner" }
                            </span>
                        </a>
                    </footer>
                </section>
            </section>
        }
    }
}

impl Model {

    fn view_data(&self) -> Html<Model> {
        if let Some(value) = &self.data {
            html! {
                <section>
                    { view_code(&value) }
                </section>
            }
        } else {
            let class = "red-white margin-left-a-quarter margin-top-one".to_string();
            
            html! {
                <p class=class, >
                    { "Loading..." }
                </p>
            }
        }
    }

    fn fetch_data(&mut self) {
        self.fetching = true;
                
        let callback = self.link.send_back(
            move |response: Response<Text>| {
                let (meta, data) = response.into_parts();
                
                if meta.status.is_success() {
                    Msg::FetchReady(data)
                } else {
                    Msg::Ignore
                }
            },
        );
        
        let request = Request::builder()
            .method("GET")
            .uri(self.value.clone())
            .body(Nothing)
            .unwrap();

        let task = self.fetch_service.fetch(request, callback);
        self.ft = Some(task);
    }
}
