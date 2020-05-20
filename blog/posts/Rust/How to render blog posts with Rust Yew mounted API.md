<!--
    Post{
        subtitle: "Learn how to use Rust Yew mounted API with it.",
        image: "post/web/rust-blog-example.png",
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
[How to use markdown with code snippets in Rust Yew Frontend]: https://www.steadylearner.com/blog/read/How-to-use-markdown-with-code-snippets-in-Rust-Yew-Frontend
[How to modulize your Rust Frontend]: https://www.steadylearner.com/blog/read/How-to-modulize-your-Rust-Frontend
[How to write Full Stack Rust Code]: https://www.steadylearner.com/blog/read/How-to-write-Full-Stack-Rust-code
[How to use a modal in Rust]: https://www.steadylearner.com/blog/read/How-to-use-a-modal-in-Rust
[How to use routers in Rust Frontend]: https://www.steadylearner.com/blog/read/How-to-use-routers-in-Rust-Frontend
[How to serve static files with Rust]: https://www.steadylearner.com/blog/read/How-to-serve-static-files-with-Rust
[How to use a single page app wtih Rust]: https://www.steadylearner.com/blog/read/How-to-use-a-single-page-app-with-Rust
[How to use Rust Tera for undefined paths]: https://www.steadylearner.com/blog/read/How-to-use-Rust-Tera-for-undefined-paths
[How to make JSON Webservice with Rust and YouTube API]: https://www.steadylearner.com/blog/read/How-to-make-JSON-Webservice-with-Rust-and-YouTube-API
[How to use CORS and OPTIONS HTTP request with Rust Rocket]: https://www.steadylearner.com/blog/read/How-to-use-CORS-and-OPTIONS-HTTP-request-with-Rust-Rocket
[How to render a YouTube vlog with Rust Yew fetch API]: https://www.steadylearner.com/blog/read/How-to-render-a-YouTube-vlog-with-Rust-Yew-fetch-API

[How to use Python in JavaScript]: https://www.steadylearner.com/blog/read/How-to-use-Python-in-JavaScript
[How to use React Spring to animate your message]: https://medium.com/@steadylearner/how-to-use-react-spring-to-animate-your-message-2bd2a7e62a5a

<!-- / -->

<!-- Steadylearner Twitter and LinkedIn  -->

[Twitter]: https://twitter.com/steadylearner_p
[LinkedIn]: https://www.linkedin.com/in/steady-learner-3151b7164/

<!--  -->

In this post, we will learn how to render blog posts with [Rust Yew Frontend] and its **mounted** and fetch API.

If you want the result first, visit [Rust Blog Example].

You can use

1. [Steadylearner Blog](https://www.steadylearner.com/blog/search/Rust) for a complete single page app blog example.

2. [Steadylearner YouTube Vlog](https://www.steadylearner.com/blog/search/Rust) if you want to use videos in your blog also.

<br />

<h2 class="red-white">[Prerequisite]</h2>

1. [How to install Rust]
2. [How to use Rust Yew]
3. [How to use markdown in Rust Frontend][How to use markdown with code snippets in Rust Yew Frontend]
4. [How to use Rust Yew fetch API][How to render a YouTube vlog with Rust Yew fetch API]

---

If you haven't installed Rust in your machine, read [How to install Rust].

We will use [Rust Yew Frontend]. Refer to [How to use Rust Yew] and [How to write Full Stack Rust Code].

The markdown files and **FetchService** API from Yew will be used for this post. Therefore, learn how to use them first with [How to use markdown in Rust Frontend][How to use markdown with code snippets in Rust Yew Frontend] and [How to use Rust Yew fetch API][How to render a YouTube vlog with Rust Yew fetch API].

We already have many [Rust Full Stack] projects and [Rust blog posts] for them.

[You can deploy your Rust projects easily.][How to deploy Rust Web App]

<br />

<h2 class="blue">Table of Contents</h2>

1. Prepare a blog post
2. Render it with Rust Yew mounted and fetch API
3. **Conclusion**

---

<br />

## 1. Prepare a blog post

Before we learn how to build Rust Frontend app to render blog posts with markdown files, we need to find where to get the datas for it.

In this post, we will use [README file](https://raw.githubusercontent.com/steadylearner/Rust-Full-Stack/master/README.md) from [Rust Full Stack] for that purpose, the page will show the raw text file similar to this.

```md
## Start

First, clone this repository with

**$git clone https://github.com/steadylearner/Rust-Full-Stack.git**

Then,

1. **$./install.sh** in **web** folder and **$./run-local.sh** for a full stack Rust chat app.
2. **$cd web/before/static_files && $cargo run --release** for JavaScript frontend and Rust server side web app.
3. **$cd before/JSON_Webservice && $./run-local.sh** for YouTube vlog example with JSON Webservice.
4. **$cd web/before/rust_blog && $./install.sh && yarn watch:rs** for a Rust blog example.
```

You can use yours instead or test the project with [this post at GitHub](https://raw.githubusercontent.com/steadylearner/code/master/post/Rust/How%20to%20render%20blog%20posts%20with%20Rust%20Yew.md). It will be similar to the main image of this post.

If you are reading this post with your desktop, [you can verify the how the result will be fast with this](https://www.steadylearner.com/markdown).

<br />

## 2. Render it with Rust Yew mounted and fetch API

The main file of our [Rust Blog Example] will be **lib.rs**. We will build it with the codes we made in the previous [Rust blog posts] and Rust Yew **mounted** and **fetch** API.

If you have experience with React, you can see **mounted** and **update** are very similar to [componetDidMount](https://reactjs.org/docs/react-component.html#componentdidmount) and [componentDidUpdate](https://reactjs.org/docs/react-component.html#componentdidupdate) from it.

[It will be helpful to find information about the lifecycle of single page apps before you read on.](https://www.google.com/search?&q=react+lifecycle).

The parts relevant to this post will be similar to this for the logic

```rust
use yew::services::{
    ConsoleService,
    fetch::{FetchService, FetchTask, Request, Response},
};

use yew::events::IKeyboardEvent;

use yew::format::{Text, Nothing}; // 1.

use yew::{html, Component, ComponentLink, Html, Renderable, ShouldRender};

mod code;

use self::{
    code::view_code,
};

pub struct Model {
    fetch_service: FetchService,
    console: ConsoleService,

    link: ComponentLink<Model>,
    fetching: bool, // 2.

    edit_value: String,
    value: String,

    // 3.
    data: Option<String>,

    ft: Option<FetchTask>,
}

// 4.
pub enum Msg {
    Update(String),
    Submit,
    // 1.
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
            // 5.
            value: "https://raw.githubusercontent.com/steadylearner/Rust-Full-Stack/master/README.md".to_string(),
            link,
            data: None,

            ft: None,
        }
    }

    fn mounted(&mut self) -> ShouldRender {
        self.fetch_data(); // 5.

        true
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            // 8.
            Msg::Update(val) => {
                self.edit_value = val;
            }
            Msg::Submit => {
                self.value = self.edit_value.clone();
                self.fetch_data();
            }
            Msg::FetchReady(response) => {
                self.fetching = false;
                self.data = response.map(|data| data).ok(); // 6.
            }
            Msg::Ignore => {
                return false;
            }
            Msg::Nope => {}
        }
        true
    }
}

```

and the view part.

```rust
impl Renderable<Model> for Model {
    fn view(&self) -> Html<Self> {
        // 7.
        let mut subnav_class = "sub theme-black border-white center ".to_string();

        if self.fetching {
            subnav_class.push_str("x-display");
        } else {
            subnav_class.push_str("inherit-display");
        }

        html! {
            <section>
                // 8.
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
                { self.view_data() }
                // 7.
                <footer class=subnav_class, >
                    { "© Steadylearner" }
                </footer>
            </section>
        }
    }
}

impl Model {

    fn view_data(&self) -> Html<Model> {
        if let Some(value) = &self.data {
            html! {
                { view_code(&value) } // 6.
            }
        } else {
            html! {
                { "Loading..." } // 2.
            }
        }
    }

    // 4.
    fn fetch_data(&mut self) {
        self.fetching = true;

        let callback = self.link.send_back(
            move |response: Response<Text>| { // 1.
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
            .uri(self.value.clone()) // 5.
            .body(Nothing) // 1.
            .unwrap();

        let task = self.fetch_service.fetch(request, callback);
        self.ft = Some(task);
    }
}
```

There are many codes in this example. You could modulize it first with [How to modulize your Rust Frontend].

Many of them are already explained before with [YouTube vlog example][How to render a YouTube vlog with Rust Yew fetch API] and [Full Stack Rust Chat App we made before][How to write Full Stack Rust Code].

Therefore, we will only handle the parts relevant to this post.

**1.** We already verifired the data we will get from the GitHub will be **the raw text** in the previous part. We normally send no data in the body part of **GET request**. So, we will use **Nothing** API from the authors for that purpose.

**2.** **self.fecthing** will be used to show loader when data is not fetched yet. It will be **"Loading..."** in this project.

**3.** [In the previous post][How to render a YouTube vlog with Rust Yew fetch API], we used **Video** struct to type the YouTube JSON data. For we know that GitHub page will return the raw texts, we will use **String** to represent it.

**4.** We don't have **FetchData** message type here compared to the previous post. We extract it inside **impl Model** to make it more reusable.

**5.** We use the default value instead of **None** or **"".to_string()** to use it with Yew **mounted** API.

```rust
fn mounted(&mut self) -> ShouldRender {
    self.fetch_data();

    true
}
```

It is to render the blog post when user visit the webpage.

You can compare it with [YouTue vlog example][JSON Webservice] without this part.(The user won't have to click anything to render datas with the **mounted** API.)

**6.** This is the payload to render the view part of your app. For we already made [Rust Yew module to render markdown files][How to use markdown with code snippets in Rust Yew Frontend], you can just pass the **data** with **view_code(&value)**.

**7.** When you make a single page app with dynamic contents, it may be not easy to handle **footer** or other less important parts that affect the layout of the entire webpage.

Use this logic to show it only after the more important parts of your app are rendered first.

**8.** It is the same code from the **input** part of [Rust Full Stack] chat app. You can see that many files of it are reusable.

Run the [Rust Blog Example] app with

```console
$install.sh && yarn watch:rs
```

if you haven't yet.

It will show you the blog contents similar to this if you are at [Steadylearner].

<br />

## 3. Conclusion

I hope you could render your markdown files for blog posts with Rust Yew **mounted** API. Refer to [Steadylearner] to build a full stack Rust blog with it.

With **mounted** and **update** API from [Rust Yew Frontend], you could prototype the visual part of your app with [server side rendering][How to use Rust Tera for undefined paths] first and turn it into Rust single page app easily.

For that to work

**1.** [Test your app with server side rendering][How to use Rust Tera for undefined paths] with [Tera] or other Rust template engine.

**2.** [Build the end points in your Rust server.][How to make JSON Webservice with Rust and YouTube API]

**3.** [Enable CORS/OTPIONS for them if necessary.][How to use CORS and OPTIONS HTTP request with Rust Rocket]

**4.** Include code to get data from the server in your Rust frontend. Find API similar to **self.fetch_data()** in **mounted** Yew lifecycle part and **update**.

We already have sufficient examples to build a full stack website similar to [Steadylearner] with Rust only.

We will learn how to manage the blog posts and other files in GitHub with its API. Then, we will find how to include databases in our projects, build CLI, login page with session and JSON Web Token etc with Rust.

If you want the latest contents from Steadylearner, follow me at [Twitter].

Do you need **a Full Stack Rust Developer**? Contact me with [LinkedIn] and I will help you.

**Thanks and please share this post with others**.
