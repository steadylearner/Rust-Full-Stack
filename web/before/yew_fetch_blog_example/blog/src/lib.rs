#![recursion_limit = "256"]

use failure::Error;
use yew::services::{
    ConsoleService,
    fetch::{FetchService, FetchTask, Request, Response},
};

use yew::format::{Nothing};

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
    data: Option<String>,

    ft: Option<FetchTask>,
}

pub enum Msg {
    FetchReady(Result<String, Error>),
    Ignore
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {

        Model {
            fetch_service: FetchService::new(),
            console: ConsoleService::new(),

            fetching: false,
            link,
            data: None,
            
            ft: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::FetchReady(response) => {
                self.fetching = false;
                self.data = response.map(|data| data).ok();
            }
            Msg::Ignore => {
                return false;
            }
        }
        true
    }

    fn mounted(&mut self) -> ShouldRender {
        // use stdweb::js;
        // use stdweb::Value;
        // use stdweb::unstable::TryInto;
        
        // We won't expect user want to cancel when the app renders first time.
        // https://developer.mozilla.org/en-US/docs/Web/API/Fetch_API/Using_Fetch#Processing_a_text_file_line_by_line       
        // https://www.steadylearner.com/blog/read/How-to-use-NPM-packages-with-Rust-Frontend

        self.fetch_data();

        true
    }
}

impl Renderable<Model> for Model {
    fn view(&self) -> Html<Self> {
        html! {
            <section class=("max-width", "main-width", "flex-column", "center-auto-margin"), >
                { self.view_data() }
            </section>
        }
    }
}

impl Model {

    fn view_data(&self) -> Html<Model> {
        if let Some(value) = &self.data {
            html! {
                <section class="flex-column", >
                    { view_code(&value) }
                    // { &value }
                </section>
            }
        } else {
            html! {
                <p class="red-white margin-left-a-quarter", >
                    { "Data hasn't fetched yet." }
                </p>
            }
        }
    }

    fn fetch_data(&mut self) {
        self.fetching = true;
                
        let callback = self.link.send_back(
            move |response: Response<Result<String, Error>>| {
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
            .uri("https://raw.githubusercontent.com/steadylearner/Rust-Full-Stack/master/README.md")
            .body(Nothing)
            .unwrap();

        let task = self.fetch_service.fetch(request, callback);
        self.ft = Some(task);
    }
}
