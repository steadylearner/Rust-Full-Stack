<!--
    Post{
        subtitle: "Learn how to use Rust Yew fetch and webstorage API with it.",
        image: "post/web/rust-json-webservice-example.png",
        image_decription: "Image by Steadylearner",
        tags: "How Rust YouTube fetch",
    }
-->

<!-- Link -->

[Steadylearner]: https://www.steadylearner.com
[Rust Website]: https://www.rust-lang.org/

[Rust Rocket]: https://rocket.rs/
[Rocket Getting Started]: https://rocket.rs/v0.4/guide/getting-started
[Rocket JSON Example]: https://github.com/SergioBenitez/Rocket/tree/master/examples/json
[Redirect]: https://api.rocket.rs/v0.4/rocket/response/struct.Redirect.html
[Tera]: https://tera.netlify.com/
[Rocket Tera example]: https://github.com/SergioBenitez/Rocket/tree/master/examples/tera_templates

[CORS]: https://developer.mozilla.org/en-US/docs/Web/HTTP/Access_control_CORS
[OPTIONS]: https://developer.mozilla.org/en-US/docs/Web/HTTP/Methods/OPTIONS
[Rocket CORS]: https://github.com/lawliet89/rocket_cors
[Rocket CORS examples]: https://github.com/lawliet89/rocket_cors/tree/master/examples
[Rocket CORS fairing example]: https://github.com/lawliet89/rocket_cors/blob/master/examples/fairing.rs
[Rocket CORS fairng test example]: https://github.com/lawliet89/rocket_cors/blob/master/tests/fairing.rs

[Rust Yew Examples]: https://github.com/yewstack/yew/tree/master/examples

[hyper]: https://github.com/hyperium/hyper

[Rust Dotenv]: https://crates.io/crates/dotenv
[Reqwest]: https://docs.rs/reqwest/0.9.18/reqwest/

[stdweb]: https://github.com/koute/stdweb

[YouTube API]: https://developers.google.com/youtube/v3/getting-started#before-you-start
[How to use YouTube API for developers]: https://www.google.com/search?q=how+to+use+youtube+api+for+developers

[Rust Full Stack]: https://github.com/steadylearner/Rust-Full-Stack
[JSON Webservice]: https://github.com/steadylearner/Rust-Full-Stack/tree/master/before/JSON_Webservice

[Rust Blog Example]: https://github.com/steadylearner/Rust-Full-Stack/tree/master/web/before/rust_blog

[CRA]: https://github.com/facebook/create-react-app

<!-- / -->

<!-- Steadylearner Post -->

[Rust blog posts]: https://www.steadylearner.com/blog/search/Rust
[How to install Rust]: https://www.steadylearner.com/blog/read/How-to-install-Rust
[Rust Chat App]: https://www.steadylearner.com/blog/read/How-to-start-Rust-Chat-App
[Rust Yew Frontend]: https://github.com/yewstack/yew
[Yew Counter]: https://www.steadylearner.com/yew_counter
[How to use Rust Yew]: https://www.steadylearner.com/blog/read/How-to-use-Rust-Yew
[How to deploy Rust Web App]: https://www.steadylearner.com/blog/read/How-to-deploy-Rust-Web-App
[How to start Rust Chat App]: https://www.steadylearner.com/blog/read/How-to-start-Rust-Chat-App
[Fullstack Rust with Yew]: https://www.steadylearner.com/blog/read/Fullstack-Rust-with-Yew
[How to use NPM packages with Rust Frontend]: https://www.steadylearner.com/blog/read/How-to-use-NPM-packages-with-Rust-Frontend
[How to use markdown with Rust Frontend]: https://www.steadylearner.com/blog/read/How-to-use-markdown-with-Rust-Frontend
[How to modulize your Rust Frontend]: https://www.steadylearner.com/blog/read/How-to-modulize-your-Rust-Frontend
[How to write Full Stack Rust Code]: https://www.steadylearner.com/blog/read/How-to-write-Full-Stack-Rust-code
[How to use a modal in Rust]: https://www.steadylearner.com/blog/read/How-to-use-a-modal-in-Rust
[How to use routers in Rust Frontend]: https://www.steadylearner.com/blog/read/How-to-use-routers-in-Rust-Frontend
[How to serve static files with Rust]: https://www.steadylearner.com/blog/read/How-to-serve-static-files-with-Rust
[How to use a single page app wtih Rust]: https://www.steadylearner.com/blog/read/How-to-use-a-single-page-app-with-Rust
[How to use Rust Tera for undefined paths]: https://www.steadylearner.com/blog/read/How-to-use-Rust-Tera-for-undefined-paths
[How to make JSON Webservice with Rust and YouTube API]: https://www.steadylearner.com/blog/read/How-to-make-JSON-Webservice-with-Rust-and-YouTube-API
[How to use CORS and OPTIONS HTTP request with Rust Rocket]: https://www.steadylearner.com/blog/read/How-to-use-CORS-and-OPTIONS-HTTP-request-with-Rust-Rocket

[How to use Python in JavaScript]: https://www.steadylearner.com/blog/read/How-to-use-Python-in-JavaScript
[How to use React Spring to animate your message]: https://medium.com/@steadylearner/how-to-use-react-spring-to-animate-your-message-2bd2a7e62a5a

<!-- / -->

<!-- Steadylearner Twitter and LinkedIn  -->

[Twitter]: https://twitter.com/steadylearner_p
[LinkedIn]: https://www.linkedin.com/in/steady-learner-3151b7164/

<!--  -->

In this post, we will make a YouTube vlog page with [Rust Yew Frontend] and its **fetch and WebStorage** API.

We already have [JSON Webservice][How to make JSON Webservice with Rust and YouTube API] with [CORS/OPTIONS and tested the end points][How to use CORS and OPTIONS HTTP request with Rust Rocket] for the Rust server side. Therefore, we will only need to write the frontend part.

If you want the result first, visit [JSON Webservice].

You can also refer to

1. [Video pages at Steadylearner](https://www.steadylearner.com/video) for a YouTube vlog example.

2. [Rust Blog Example] if you want to use markdown files for the vlog description part.

<br />

<h2 class="red-white">[Prerequisite]</h2>

1. [How to install Rust]
2. [How to use Rust Yew]
3. [How to write Full Stack Rust Code]
4. [Rust Yew Examples]
5. [How to use CORS and OPTIONS HTTP request with Rust Rocket]

---

If you haven't installed Rust in your machine, read [How to install Rust].

We will use [Rust Yew Frontend]. Please read [How to use Rust Yew] and [How to write Full Stack Rust Code] first.

[How to use CORS and OPTIONS HTTP request with Rust Rocket] will help you find this post better. The Yew allows the users to send fetch requests to servers only when **CORS/OPTIONS** is enabled in them.

Read [Rust Yew Examples] and find where **fetch** and **localStorage** API are used.

We already have many [Rust Full Stack] projects and [Rust blog posts] to learn how to use them.

[You can deploy them.][How to deploy Rust Web App]

<br />

<h2 class="blue">Table of Contents</h2>

1. Yew Fetch API
2. Render a vlog with it
3. **Conclusion**

---

<br />

## 1. Yew Fetch API

Before we build a vlog page with Rust, we need to learn how to get data from the server for the frontend part first. The [FetchService](https://github.com/yewstack/yew/blob/master/src/services/fetch.rs) from [Rust Yew Frontend] will help you for that.

Those services are just JavaScript Web API wrappers that authors prepared for its users.(You can build your own services also).

It will be sufficient to know how to use [FETCH](https://developer.mozilla.org/en-US/docs/Web/API/Fetch_API) in JavaScript for this post.

We will start with [the official example that uses it.](https://github.com/yewstack/yew/blob/master/examples/dashboard/src/lib.rs)

If you delete all the codes not relevant to the fetch API, the payload will be similar to this.

```rust
#![recursion_limit = "256"] // 1.

use failure::Error;
use serde_derive::{Deserialize, Serialize};
use yew::format::{Json, Nothing};
use yew::services::fetch::{FetchService, FetchTask, Request, Response};
use yew::services::Task;
use yew::{html, Component, ComponentLink, Html, Renderable, ShouldRender};

pub struct Model {
    fetch_service: FetchService,
    link: ComponentLink<Model>,
    fetching: bool,
    data: Option<u32>,
    ft: Option<FetchTask>,
}

pub enum Msg {
    FetchData,
    FetchReady(Result<DataFromFile, Error>), // 2.
    Ignore,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Model {
            fetch_service: FetchService::new(),
            link,
            fetching: false,
            data: None,
            ft: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        // 3.
        match msg {
            Msg::FetchData => {
                self.fetching = true; // 4.

                let callback = self.link.send_back(
                    move |response: Response<Json<Result<DataFromFile, Error>>>| { // 2.
                        let (meta, Json(data)) = response.into_parts();
                        if meta.status.is_success() {
                            Msg::FetchReady(data)
                        } else {
                            Msg::Ignore
                        }
                    },
                );
                let request = Request::get("/data.json").body(Nothing).unwrap(); // 5.

                let task = self.fetch_service.fetch(request, callback);
                self.ft = Some(task);
            }
            Msg::FetchReady(response) => {
                self.fetching = false; // 4.
                self.data = response.map(|data| data.value).ok(); // 6.
            }
            Msg::Ignore => {
                return false;
            }
        }
        true
    }
}
```

It has became much simpler and we use **fetch** instead of websocket that we used for [How to write Full Stack Rust Code].

**1.** **recursion_limit** is related to macro in Rust and ! to affect the entire file.

**2.** We made **Video** struct for the [YouTube JSON webservice][How to make JSON Webservice with Rust and YouTube API] before.

[Rust code used in server can be reused in frontend also][How to write Full Stack Rust Code]. Therefore, what you need is just **copy and paste** the code relevant to **Video** and use it isntead DataFromFile later.

**3.** **FetchData** and **FetchReady** will be the payload for the entire Rust frontend part to work.

**4.** You can use **self.fetching** part to render simple loaders before the data is fetched. When it is true, it will mean the data is not ready and vice versa when it is false. You can show the **Loading...** only when it is true.

**5.** [We built the end points in Rust server][How to use CORS and OPTIONS HTTP request with Rust Rocket]. We will use the end points we made before instead of the static file path.

**6.** the data in **self.data** will be what you really use for the view part of your app.

<br />

## 2. Render a vlog with it

We already have many [Rust blog posts] to learn [How to write Full Stack Rust Code]. 

Therefore, we will handle only **web/lib.rs** file in [JSON Webservice], the payload of your [Rust Yew Frontend] app.

The important parts will be

```rust
#![recursion_limit = "256"]

use failure::Error;
use yew::format::{Json, Nothing};
use yew::services::{
    ConsoleService, // 1.
    storage::{Area, StorageService}, // 2.
    fetch::{FetchService, FetchTask, Request, Response},
};

use yew::{html, Component, ComponentLink, Html, Renderable, ShouldRender};

mod components;
mod http_model;

#[macro_use]
extern crate serde_derive;
extern crate serde_json;

// 2.
const NAME: &'static str = "rust.yew.fetch.webstorage.example.by.www.steadylearner.com";

use self::{
    // 3.
    http_model::youtube_video::Video,
    components::video::view_video,
};

pub struct Model {
    fetch_service: FetchService,
    console: ConsoleService, // 1.
    storage: StorageService, // 2.

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
        // 2.
        let storage = StorageService::new(Area::Local);

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
                        if meta.status.is_success() {
                            Msg::FetchReady(data)
                        } else {
                            Msg::Ignore
                        }
                    },
                );

                // 4.
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
                self.console.log("YouTube data is ready."); // 1.

                self.data = response.map(|data| data).ok(); // 5.

                self.storage.store(NAME, Json(&self.data)); // 2.
            }
            Msg::Ignore => {
                return false;
            }
        }
        true
    }
}
```

and the code snippet for the view part of it.

```rust
impl Renderable<Model> for Model {
    fn view(&self) -> Html<Self> {
        html! {
            <section >
                <nav>
                    <span
                        onclick=|_| Msg::FetchData,
                        title="Click this to show the Rust video.",
                    >
                    </span>
                </nav>
                { self.view_data() }
            </section>
        }
    }
}

impl Model {
    fn view_data(&self) -> Html<Model> {

        if let Some(video) = &self.data { // 6.

        let payload = &video.items.as_ref().unwrap()[0];

        // Use destructure with Video here if you want;
        // let Video { id, title, description, ... } = payload;

        let video_id = &payload.id;
        let video_title = &payload.snippet.title;
        let video_description = &payload.snippet.description;

            html! {
                <section >
                    { &video_title }
                    { view_video(&video_id) }
                    { &video_description }
                </section>
            }
        } else { // 7.
            html! {
                <p>
                    { "Data hasn't fetched yet." }
                </p>
            }
        }
    }
}
```

There are many codes here but it will be easy after we learn how to use them.

**1.** We need console API from [Rust Yew Frontend] to debug your app easily with browser.

**2.** This is optional. Use it not to show the loader("Data hasn't fetchted yet here") everytime.

If you want to use it, start the main data(self.data) of your app with **None** or **Option(value)** and substitute it with the webstorage or other database relevant API later.

**3.** We already made them in the previous [Rust blog posts]. Reuse them wherever you want because [we invested our time to learn how to modulize them][How to modulize your Rust Frontend].

**4.** We already tested the relevant API in server with [hyper] in [How to use CORS and OPTIONS HTTP request with Rust Rocket]. Here, [http](https://github.com/hyperium/http) API from the same authors is used. Therefore, they are very similar.

**5.** **self.data** is the payload to render the view part of your project. Nothing complciated here. It will be **Video** data from the JSON Webservice server we made before.

**6.** It means **self.data = Some(Video)** and **self.fetching** is false. The data for the YouTube video is ready. Use it to render the view part.

**7.** You can use your loaders while the data is not ready for your app. The simple message **Loading...** with CSS will be sufficient to start your [Rust Full Stack] web project.

Test [your full stack Rust app][JSON Webservice] in your machine if you haven't yet.

It will show you "Data hasn't fetched yet." for the data is not ready when you first render it. Then, click **This video in Rust** and it will show the page similar to the main image of this post.

<br />

## 4. Conclusion

I hope you could render [Rust Full Stack] vlog with this post. You can also refer to [Rust Blog Example] and [Steadylearner] to build a full stack Rust blog with it.

In the next [Rust blog posts], we will learn how to use mount API from [Rust Yew Frontend] to render blog posts. We will use the markdown files form the GitHub. Then, we will find how to include the database in our full stack Rust project, build CLI, login page, JSON Web Token etc with Rust.

If you want the latest contents from Steadylearner, follow me at [Twitter].

Do you need **a Full Stack Rust Developer**? Contact me with [LinkedIn] and I will help you.

**Thanks and please share this post with others**.
