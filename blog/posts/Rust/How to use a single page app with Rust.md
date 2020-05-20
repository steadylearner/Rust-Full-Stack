<!--
    Post{
        subtitle: "Learn how to serve single page app production files with Rust",
        image: "post/web/react-with-rust.png",
        image_decription: "Image by Steadylearner",
        tags: "How React Rust code",
    }
-->

<!-- Link -->

[Steadylearner]: https://www.steadylearner.com
[Rust Website]: https://www.rust-lang.org/

[Rust Rocket]: https://rocket.rs/
[Rocket Getting Started]: https://rocket.rs/v0.4/guide/getting-started
[Redirect]: https://api.rocket.rs/v0.4/rocket/response/struct.Redirect.html
[Tera]: https://tera.netlify.com/

[Rust Full Stack]: https://github.com/steadylearner/Rust-Full-Stack

[CRA]: https://github.com/facebook/create-react-app

<!-- / -->

<!-- Steadylearner Post -->

[Rust blog posts]: https://www.steadylearner.com/blog/search/Rust
[How to install Rust]: https://www.steadylearner.com/blog/read/How-to-install-Rust
[Rust Chat App]: https://www.steadylearner.com/blog/read/How-to-start-Rust-Chat-App
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

[How to use Python in JavaScript]: https://www.steadylearner.com/blog/read/How-to-use-Python-in-JavaScript
[How to use React Spring to animate your message]: https://medium.com/@steadylearner/how-to-use-react-spring-to-animate-your-message-2bd2a7e62a5a

<!-- / -->

<!-- Steadylearner Twitter and LinkedIn  -->

[Twitter]: https://twitter.com/steadylearner_p
[LinkedIn]: https://www.linkedin.com/in/steady-learner-3151b7164/

<!--  -->

In the previous post [How to serve static files with Rust], we learnt how to use HTML, CSS, JavaScript, images etc with [Rust Rocket].

In this post, we will learn how to use a single page app with Rust. We will use React and [CRA] for this post.

If you don't have production files yet, [you can use this](https://github.com/steadylearner/Rust-Full-Stack/tree/master/web/before/static_files/React).

You may verify the result of this post first at [React Single Page App with Rust](https://www.steadylearner.com/react_form) example.

If you want other Rust web projects, please refer to [Rust Full Stack] repostiroy and its **before** folders. [How to use Rust Yew] and other [Rust blog posts] will help you to use Rust instead of React to make single page apps also.

<br />

<h2 class="red-white">[Prerequisite]</h2>

1. [How to install Rust]
2. [How to serve static files with Rust]
3. [How to deploy Rust Web App]

---

I want you already have Rust installed in your machine. The blog post [How to install Rust] will help you for that. Then, read [How to serve static files with Rust] if you haven't yet.

You can deploy your single page app with Rust with [How to deploy Rust Web App]. 

<br />

<h2 class="blue">Table of Contents</h2>

1. Production files with CRA
2. Routes to serve them
3. Conclusion

---

If you already have production files, you won't need the first part. Use them instead.

If you just want to test it work in your machine, use files at [server](https://github.com/steadylearner/Rust-Full-Stack/tree/master/web/before/static_files/server).

Then, **$cargo run --release** and visit **/single_page_app**.

<br />

## 1. Production files with CRA

[![React with Rust example](https://www.steadylearner.com/static/images/post/React/react-form-example.png)](https://www.steadylearner.com/react_form)

First, we need them before we test React with Rust. We will use [CRA] because you won't need to spend your time to prepare the development environment with it.

If you haven't made your React project yet, use **/web/before/static_files** in [Rust Full Stack]. **$yarn && yarn start** to test it work in your local machine.

Then, **yarn build** and you will have the production files in your **build** folder.

It will be similar to the image above. I prepared it to apply for a job. You can use it for whatever purpose. It will be responsive in mobile also.

When the process ends, you will see the message similar to this.

```console
Compiled successfully.

File sizes after gzip:

The project was built assuming it is hosted at /static/single_page_app/.
You can control this with the homepage field in your package.json.

The build folder is ready to be deployed.

Find out more about deployment here: https://bit.ly/CRA-deploy
```

There is **The project was built assuming it is hosted at /static/react-form/.** part above. It is just to help you to deploy the files easily with your server side framework.

You can modify it in your package.json file.

```json
"homepage": "https://www.yourwebsite.com/static/single_page_app"
```

Only parts after `https://www.yourwebsite.com` will affect the result. You can use what you think the best for your website.

The production files in **build** folder will be similar to

```console
├── index.html
└── static
    ├── css
    │   ├── main.css
    │   └── main.css.map
    └── js
        └── main.js
```

They are just static files and we already learnt [How to serve static files with Rust].

(We don't need most of manifest and map.js files only to test your React single page apps work with Rust.)

We will modify index.html that links all other files before we write route for it later.

It will be similar to

```html
<!doctype html>
<html lang="en">
    <head>
        <meta charset="utf-8">
        <meta name="viewport" content="width=device-width,initial-scale=1,shrink-to-fit=no">
        <title>©ode Sign Up</title>
        <!-- 1. -->
        <link href="/static/single_page_app/static/css/main.production.chunk.css" rel="stylesheet">
    </head>
    <body>
        <div id="root"></div>
        <script>!function(l){return f.p="/single_page_app/"}</script> <!-- 2. -->
        <!-- 3. -->
        <script src="/static/single_page_app/static/js/production.chunk.js"></script>
        <script src="/static/single_page_app/static/js/main.production.chunk.js"></script>
        <!-- / -->
    </body>
</html>
```

and the important parts will be

1. Include **CSS**

2. Function for [CRA] to work

3. Include **JavaScript** production files

**/static/react_form/** is prefixed for them if you have used them in **package.json** before.

If you don't like **static** is repeated here. You can use **public** instead to serve static files in server side.

<br />

## 2. Routes to serve them

We already learnt [How to serve static files with Rust] and set up the project for that. 

Therefore, this part will be very simple because you don't have to make routes for every files anymore.

First, move your React production files you made before to **server/static/single_page_app** folder. Then, edit **get.rs** and **main.rs** you had before.

```rust
use std::io;
use rocket::response::{NamedFile, Redirect};

#[get("/")]
pub fn index() -> io::Result<NamedFile> {
    NamedFile::open("static/index.html")
}

#[get("/single_page_app")]
pub fn single_page_app() -> io::Result<NamedFile> {
    NamedFile::open("static/single_page_app/index.html")
}

#[get("/favicon.ico")]
pub fn favicon() -> Redirect {
    Redirect::to("/static/steadylearner_favicon.png")
}
```

and

```rust
#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;

mod routes;
use crate::routes::{ static_files, get };

fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .mount(
            "/",
            routes![
                static_files::file,
                get::index,
                get::single_page_app,
                get::favicon,
            ],
        )
}

fn main() {
    rocket().launch();
}
```

You made a new route for **single_page_app** and [Redirect] with **favicon** for the single page app. Then, include them in the main process to serve the **index.html**.

**$cargo run main --relase** in your console and you can verify the result at **/single_page_app**.

I hope you could make it work. You may include **manifest.json and map.js** etc relevant codes if you think they are necessary for your project.

<br />

## 3. Conclusion

We already had the Rust project to serve any static files. Therefore, the entire process to serve a single page app with Rust was very simple.

What you need are

**1.** Prepare React production files and edit **index.html** file to be compatible with Rust server side web framework.

**2.** Make a route for that and include it in your main process(main.rs).

Compare the process with the Rust single page app you made with [How to use Rust Yew].

In the next [Rust blog posts], we will learn how to use [Tera] to handle undefined paths. It ill help you to learn how to use server side rendering with Rust. Then, we will build **a login page** with [Rust Frontend][How to write Full Stack Rust Code] and JSON Webservice at server side.

If you want the latest contents from Steadylearner, follow me at [Twitter].

Do you need a Full Stack Rust and JavaScript Developer? contact me with [LinkedIn] or be one of them.

**Thanks and please share this post with others**.
