#![recursion_limit = "256"]

use yew::services::{
    ConsoleService,
    storage::{Area, StorageService},
    fetch::{FetchService, FetchTask, Request, Response},
};

use stdweb::js;

use yew::events::IKeyboardEvent;

// https://docs.rs/yew/0.4.0/i686-pc-windows-msvc/yew/format/type.Text.html
// type Text = Result<String, Error>;
use yew::format::{Text, Json, Nothing};

use yew::{html, Component, ComponentLink, Html, Renderable, ShouldRender};

mod code;

use self::{
    code::view_code,
};

const NAME: &'static str = "rust.yew.blog.example.by.www.steadylearner.com";

pub struct Model {
    fetch_service: FetchService,
    console: ConsoleService,
    storage: StorageService,

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
        let storage = StorageService::new(Area::Local);

        let data = {
            if let Ok(restored_model) = storage.restore(NAME) {
                Some(restored_model)
            } else {
                None
            }
        }; 

        Model {
            fetch_service: FetchService::new(),
            console: ConsoleService::new(),
            storage,

            fetching: false,
            edit_value: "".to_string(),
            value: "https://raw.githubusercontent.com/steadylearner/Rust-Full-Stack/master/README.md".to_string(),
            link,
            data,
            // data: None,
            
            ft: None,
        }
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

                js! {
                    window.localStorage.setItem(@{NAME}, @{&self.data});
                }
            }
            Msg::Ignore => {
                return false;
            }
            Msg::Nope => {}
        }
        true
    }

    fn mounted(&mut self) -> ShouldRender {
        if let Ok(restored_model) = self.storage.restore(NAME) { 
            self.console.log("There is already data for this page. So use it instead of fetch.");
            self.console.log(&restored_model);
        } else {
            // You will also see loading... in the page first. or test later with localStorage.clear();
            self.console.log("There is no data for this page in web storage yet. So fetch data first.");
            self.fetch_data();
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
                <section class=("main-width", "flex-column", "center-auto-margin"), >
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
            let class="red-white margin-left-a-quarter margin-top-one".to_string();

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
