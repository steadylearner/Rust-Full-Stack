<!--
    Post{
        subtitle: "Learn how to build JSON Web Service with Rust",
        image: "post/web/rust-youtube-by-Steadylearner.png",
        image_decription: "Image by Steadylearner",
        tags: "How Rust Webservice YouTube",
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

[Rust Dotenv]: https://crates.io/crates/dotenv
[Reqwest]: https://docs.rs/reqwest/0.9.18/reqwest/

[YouTube API]: https://developers.google.com/youtube/v3/getting-started#before-you-start
[How to use YouTube API for developers]: https://www.google.com/search?q=how+to+use+youtube+api+for+developers

[Rust Full Stack]: https://github.com/steadylearner/Rust-Full-Stack
[JSON Webservice]: https://github.com/steadylearner/Rust-Full-Stack/tree/master/server/before/JSON_Webservice

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

[How to use Python in JavaScript]: https://www.steadylearner.com/blog/read/How-to-use-Python-in-JavaScript
[How to use React Spring to animate your message]: https://medium.com/@steadylearner/how-to-use-react-spring-to-animate-your-message-2bd2a7e62a5a

<!-- / -->

<!-- Steadylearner Twitter and LinkedIn  -->

[Twitter]: https://twitter.com/steadylearner_p
[LinkedIn]: https://www.linkedin.com/in/steady-learner-3151b7164/

<!--  -->

In the previous post, we learnt [how to use Rust server to render view part of your Rust web project][How to use Rust Tera for undefined paths].

In this post, we will learn how to make **JSON Web Service with Rust** and **YouTube API**.

If you want to verify the result from this post first,

1. Learn [How to use YouTube API for developers] and [its documenation][YouTube API].

2. Clone [Rust Full Stack]. Then, visit **/server/before/JSON_Webservice** folder.

3. Include what you made in **1.** to [/server/.env][Rust Dotenv].

It is a simple [Rust Full Stack] project and will show you the result similar to this.

[![Rust JSON Webservice example by Steadylearner](https://www.steadylearner.com/static/images/post/web/rust-json-webservice-example.png)][JSON Webservice]

You can also refer to [video](https://www.steadylearner.com/video) parts and [Rust Blog Example].

If you want other Rust web projects, use [the repostiory][Rust Full Stack] and its **before** folders.

<br />

<h2 class="red-white">[Prerequisite]</h2>

1. [How to install Rust]
2. [Rust Rocket]
3. [Rocket JSON Example]
4. [How to use YouTube API for developers]

---

If you haven't installed Rust in your machine yet, read [How to install Rust].

We will use [Rust Rocket] framework here. If you haven't used it before, visit [Rocket Getting Started] first.

Then, clone [Rocket JSON example] and test it with

```console
$cargo run
$cargo test
```

in your machine and find what they do.

We already have many [Rust Full Stack] projects and [Rust blog posts] to learn how to use them. Deploy it with [How to deploy Rust Web App].

Read [How to use YouTube API for developers] and [YouTube API] if you want to use the exact example used for this post. Otherwise, learn the workflow and use your datas instead to build JSON Webservice with Rust or other programming languages.

<br />

<h2 class="blue">Table of Contents</h2>

1. Prepare Rust Rocket project
2. Type YouTube API data
3. Build a route for it
4. Test
5. **Conclusion**

---

<br />

## 1. Prepare Rust Rocket project

We need to set up the project first to make JSON Webservice with it.

I will assume that you already read [How to use Rust Tera for undefined paths] and learnt how to setup minimal [Rust Rocket] project.

We will start with the **Cargo.toml** file.

```toml
[package]
name = 'rust_json_web_service'
version = "0.1.0"
authors = ["www.steadylearner.com"]
edition = "2018"

[dependencies]
rocket = "0.4.2"
serde = "1.0"
serde_derive = "1.0"
serde_json = "1.0"
<!-- 1. -->
dotenv = "0.14.1"
reqwest = "0.9.19"
<!-- 2. -->
rocket_cors = "0.5.0"

[dependencies.rocket_contrib]
version = "0.4.2"
default-features = false
features = ["tera_templates", "json"] <!-- 3. -->
```

and the important parts for this post are

**1.** This is to use .env file with [YouTube API] to serve datas for our [Rust Full Stack] app.

I hope you already read [How to use YouTube API for developers] and setup **.env** in your /server folder with it.

We will use [Reqwest] to get datas from the [YouTube API]. Then, we will build JSON Webservice with it to return them with a [Rust Rocket] routes.

**2.** We need it for [Rust Yew Frontend] in **/web** folder to work.

We will learn more about it in the later [Rust blog posts].

**3.** Enable **json** feature for [Rust Rocket] with this.

Then, we will make **main.rs**().

```rust
// Refer to /server/src/main_without_cors_options_for_json.rs

#[cfg(test)] mod tests; // 1.
// 2.
mod http_model;
mod web_service;

use self::{
    web_service::youtube_video::video_search_by_id,
};

use rocket::{get, routes};

#[get("/")]
fn hello() -> &'static str {
    "Hello from www.steadylearner.com"
}

fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .mount(
            "/",
            routes![
                hello,
                video_search_by_id::webservice, // 2.
            ],
        )
}

fn main() {
    rocket().launch();
}
```

and will be easy if you already know [how to use modules in Rust][How to modulize your Rust Frontend].

The important parts are

**1.** It will make **tests.rs** work only when you use **$cargo test**.

**2.** We will build **http_modle** and **web_service**.

The route **webservice** will be our payload to serve JSON Webservice from [YouTube API].

<br />

## 2. Type YouTube API data

In the [How to write Full Stack Rust Code], we learnt [how to type the JSON data for websocket server and client](https://github.com/steadylearner/Rust-Full-Stack/blob/master/server/src/http_model/websocket_json.rs)

For Rust is statically typed language, we should type the JSON datas from [YouTube API] first before we use them.

We will build **http_model/youtube_video.rs** for that.

```rust
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)] // 1.
pub struct Video {
    pub items: Option<Vec<VideoItem>>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct VideoItem {
    pub id: String,
    pub snippet: Snippet,
}

// 2. https://serde.rs/attr-rename.html
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Snippet {
    pub published_at: String,
    pub channel_id: String,
    pub title: String,
    pub description: Option<String>,
    pub thumbnails: Thumbnails,
    pub channel_title: String,
    pub tags: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Thumbnails {
    pub default: Thumbnail,
    pub high: Thumbnail,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Thumbnail {
    pub url: String,
    pub width: u16,
    pub height: u16,
}
```

It will be used for the **JSON** data by **YouTube Video Id**.

The important parts are

**1.** **derive(PartialEq, Clone)** to use them in **tests.rs**.

**2.** **rename_all = "camelCase"** to satisfy the Rust compiler and use it in frontend without modification.

We won't talk about how they were made. It is better with the example to save time. You can make your own type definition for them if you want.

<br />

## 3. Build a route to serve it

What left is to build a [Rust Rocket] route to return [YouTube API] data in JSON format.

We will build **video_search_by_id.rs** in /webservice folder for that purpose.

It will be similar to

```rust
use dotenv::dotenv;
use std::env;

use rocket_contrib::json::{
    Json,
    // JsonValue
};

// This is relevant to mod http_model; in main.rs
use crate::http_model::youtube_video::Video; // 1.

#[get("/video_search_by_id/<q>")] // 8EPsnf_ZYU0
pub fn webservice(q: String) -> Result<Json<Video>, reqwest::Error> { // 2.
    dotenv().ok();
    let youtube_key = env::var("YOUTUBE_KEY").expect("Should be set already");

    let request_url = format!(
      "https://www.googleapis.com/youtube/v3/videos?part=snippet&id={}&key={}",
      q, youtube_key
    );

    println!("{}", request_url);
    let video_search_by_id: Video = reqwest::get(&request_url)?.json()?;

    println!("{:?}", &video_search_by_id); // 3.

    Ok(Json(video_search_by_id)) // 2.
}
```

The payloads are

**1.** We already have typed the data from the [YouTube API] with **Video** in youtube_video.rs file. We can use it wherever we want.

**2.** We use [Reqwest] api to get data here. Therefore, error parts in **Result** is **reqwest::Error**.

**3.** It will show you the **Video** typed result in your console.

We already have all the code to use JSON Webservice with Rust. We used the datas from [YouTube API] here. But, you can use ones from your database instead. We will learn more about that in the later [Rust blog posts].

<br />

## 4. Test

In [Rocket JSON Example], there is **tests.rs** example to test its end points already.

We will apply it to what we made before and edit the example similar to this.

```rust
use crate::rocket; // 1.
use crate::serde_json;
use crate::http_model::youtube_video::Video;

use rocket::local::Client;
use rocket::http::{Status, ContentType};

#[test]
fn invalid_youtube_id() {
    let client = Client::new(rocket()).unwrap(); // 1.

    let invalid_youtube_id = "invalid_youtube_id";

    // 1.
    let request_url = format!(
        "/video_search_by_id/{}",
        invalid_youtube_id,
    );

    let mut res = client.get(&request_url).header(ContentType::JSON).dispatch();

    assert_eq!(res.status(), Status::Ok);

    let body = res.body_string().unwrap();

    // 2.
    let video: Video = serde_json::from_str(&body).unwrap();

    // should include PartialEq, Clone for http_model::youtube_video::Video
    // to use it.
    assert_eq!(video.items.unwrap(), [].to_vec());
}

#[test]
fn valid_youtube_id() {
    let client = Client::new(rocket()).unwrap();

    let valid_youtube_id = "8EPsnf_ZYU0";

    let request_url = format!(
        "/video_search_by_id/{}",
        &valid_youtube_id,
    );

    let mut res = client.get(&request_url).header(ContentType::JSON).dispatch();

    assert_eq!(res.status(), Status::Ok);

    let body = res.body_string().unwrap();
    let video: Video = serde_json::from_str(&body).unwrap();

    // 3.
    let payload = &video.items.unwrap()[0];

    // 4.
    assert_eq!(payload.id, valid_youtube_id.to_string());
    assert!(payload.snippet.title.contains("Rust"));
}
```

The important parts are

**1.** In main.rs there is

```rust
fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .mount(
            "/",
            routes![
                hello,
                video_search_by_id::webservice, // 2.
            ],
        )
}
```

**rocket** in **use crate::rocket** indicate this code snippet.

and you can infer you can test only the routes you included in **routes!**.

**2.** We deserialize data in **body** variable here with **Video** to easily use it in other parts.

**3.** This is the payload to render **YouTube Video** datas in your frontend.

You may read **Snippet** part in youtube_video.rs file again.

**4.** Test the video data we received was from the same video we requested with this.

Use **$cargo test** and you will see the result similar to this.

```console
running tests
test tests::invalid_youtube_id

test result: ok
```

If you want the simpler way, **$cargo run --release** and

```curl
$curl http://localhost:8000/video_search_by_id/invalid_youtube_id
$curl http://localhost:8000/video_search_by_id/8EPsnf_ZYU0
```

in another console. Then, verify the results.

<br />

## 5. Conclusion

We learnt **How to make JSON Web service** with Rust in this post.

In the next [Rust blog posts], we will learn **How to enable CORS/OPTIONS with Rust Rocket**, **How to use Fetch API in Rust Yew**, **How to render blog posts with Rust Yew Frontend**.

Then, we will find how to include the database in our [Rust Full Stack] project, build CLI, login page, JSON Web Token etc with Rust.

If you want the latest contents from Steadylearner, follow me at [Twitter].

Do you need **a Full Stack Rust Developer**? Contact me with [LinkedIn] and I will help you.

**Thanks and please share this post with others**.