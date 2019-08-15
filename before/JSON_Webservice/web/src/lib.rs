#![recursion_limit = "256"]

use failure::Error;
// use serde_derive::{Deserialize, Serialize};
use yew::format::{Json, Nothing};
use yew::services::{
    ConsoleService,
    storage::{Area, StorageService},
    fetch::{FetchService, FetchTask, Request, Response},
};

// use yew::services::Task;
use yew::{html, Component, ComponentLink, Html, Renderable, ShouldRender};

mod components;
mod http_model;

#[macro_use]
extern crate serde_derive;
extern crate serde_json;

const NAME: &'static str = "rust.yew.fetch.webstorage.example";

use self::{
    http_model::youtube_video::Video,
    components::video::view_video,
};

pub struct Model {
    fetch_service: FetchService,
    console: ConsoleService,
    storage: StorageService,

    link: ComponentLink<Model>,
    fetching: bool,
    data: Option<Video>,
    ft: Option<FetchTask>,
}

pub enum Msg {
    FetchData,
    FetchReady(Result<Video, Error>),
    Ignore,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let storage = StorageService::new(Area::Local);

        // You don't have to refresh the page with this
        // once you fetch the youtube data
        let data = {
            if let Json(Ok(restored_model)) = storage.restore(NAME) {
                Some(restored_model)
            } else {
                None
            }
        }; 

        Model {
            storage,
            fetch_service: FetchService::new(),
            console: ConsoleService::new(),

            link,
            fetching: false,
            data,
            // data: None 
            ft: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::FetchData => {
                self.fetching = true;
                self.console.log("Browser uses Rust Yew Fetch API for YouTube data");
                
                let callback = self.link.send_back(
                    move |response: Response<Json<Result<Video, Error>>>| {
                        let (meta, Json(data)) = response.into_parts();
                        // println!("META: {:?}, {:?}", meta, data);
                        if meta.status.is_success() {
                            Msg::FetchReady(data)
                        } else {
                            Msg::Ignore // FIXME: Handle this error accordingly.
                        }
                    },
                );

                // CORS error, how to solve this?
                // https://github.com/yewstack/yew/blob/master/src/services/fetch.rs
                // https://www.keycdn.com/support/cors#a-real-world-example-of-how-cors-works
                // .header("Access-Control-Allow-Origin", "http://127.0.0.1:8000/")

                // Error: No matching routes for "OPTIONS" /video_search_by_id/s7TVVyTyReU.
                // https://github.com/lawliet89/rocket_cors should learn how to use it for this
                // https://developer.mozilla.org/en-US/docs/Web/HTTP/Methods/OPTIONS
                // https://developer.mozilla.org/en-US/docs/Web/HTTP/CORS

                // https://docs.rs/yew/0.8.0/yew/services/fetch/struct.Request.html
                // https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Access-Control-Allow-Origin
                
                // https://github.com/hyperium/http
                let request = Request::builder()
                    .method("GET")
                    .uri("http://localhost:8000/video_search_by_id/8EPsnf_ZYU0")
                    .header("Access-Control-Allow-Origin", "http://127.0.0.1:8000")
                    .body(Nothing)
                    .unwrap();

                let task = self.fetch_service.fetch(request, callback);
                self.ft = Some(task);
            }
            Msg::FetchReady(response) => {
                self.fetching = false;
                self.console.log("YouTube data is ready.");
                self.data = response.map(|data| data).ok();
                self.storage.store(NAME, Json(&self.data));
            }
            Msg::Ignore => {
                return false;
            }
        }
        true
    }
}

impl Renderable<Model> for Model {
    fn view(&self) -> Html<Self> {
        let youtube_class = "red center fab fa-youtube font-two-and-a-half";

        html! {
            <section class=("max-width", "main-width", "flex-column", "center-auto-margin"), >
                <nav>
                    <h1 class=("font-four", "flex"), >
                        <span 
                            class=("cursor-pointer", "hover", "transition-half"), 
                            onclick=|_| Msg::FetchData,
                            title="Click this to show the Rust video.",
                        >
                            <span> { "This " } </span>
                            <i class=youtube_class,  />
                            <span> { " with Rust " } </span>
                        </span>
                        <a 
                            class=("no-text-decoration", "blue", "left-auto", "hover", "transition-half"),
                            href="https://www.steadylearner.com/blog/search/Rust",
                            title="Click it to learn how to code this.",
                            target="_blank",
                            rel="noopener noreferrer",
                        > 
                            { "©ode" } 
                        </a>
                    </h1>
                    
                </nav>
                { self.view_data() }
            </section>
        }
    }
}

impl Model {
    fn view_data(&self) -> Html<Model> {
        // video is relevant to 
        // let body = res.body_string().unwrap(); 
        // let video: Video = serde_json::from_str(&body).unwrap();
        // in /server/tests.rs
        
        if let Some(video) = &self.data {
    
        let payload = &video.items.as_ref().unwrap()[0]; // video_item or instead of payload

        let video_id = &payload.id;
        // let video_title = &payload.snippet.title;

        // let video_title_class = "font-two-and-a-half red-white margin-left-a-quarter";
 
            html! {
                <section class="flex-column", >
                    // <h1 class=video_title_class, >
                    //     { &video_title }
                    // </h1>
                    { view_video(&video_id) }
                </section>

                // equals to what we tested in server.rs
                // <p>{ &video_title }</p>
                // <p> { &video_id } </p>

            }
        } else {
            html! {
                <p class="red-white margin-left-a-quarter", >
                    { "Data hasn't fetched yet." }
                </p>
            }
        }
    }
}
