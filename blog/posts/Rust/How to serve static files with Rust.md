<!--
    Post{Learn how to serve HTML, CSS, JavaScript, Images etc with Rust.
        subtitle: "Learn how to serve HTML, CSS, JavaScript, Images etc with Rust.",
        image: "post/web/static-files-with-rust.png",
        image_decription: "Image by Steadylearner",
        tags: "How Rust serve static",
    }
-->
<!-- Link -->

[Steadylearner]: https://www.steadylearner.com
[Rust Website]: https://www.rust-lang.org/

[Rust Rocket]: https://rocket.rs/
[Rocket Getting Started]: https://rocket.rs/v0.4/guide/getting-started
[Tera]: https://tera.netlify.com/

[Rust Full Stack]: https://github.com/steadylearner/Rust-Full-Stack

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
[How to use a single page app with Rust]: https://www.steadylearner.com/blog/read/How-to-use-a-single-page-app-with-Rust
[How to use Rust Tera for undefined paths]: https://www.steadylearner.com/blog/read/How-to-use-Rust-Tera-for-undefined-paths

[How to use Python in JavaScript]: https://www.steadylearner.com/blog/read/How-to-use-Python-in-JavaScript
[How to use React Spring to animate your message]: https://medium.com/@steadylearner/how-to-use-react-spring-to-animate-your-message-2bd2a7e62a5a

<!-- / -->

<!-- Steadylearner Twitter and LinkedIn  -->

[Twitter]: https://twitter.com/steadylearner_p
[LinkedIn]: https://www.linkedin.com/in/steady-learner-3151b7164/

<!--  -->

We learnt how to write frontend routers for your [Rust Full Stack] web app with [How to use routers in Rust Frontend] before.

In this post, you will learn how to serve static files with [Rust Rocket] framework. You will have minimum website structure to serve static files such as HTML, CSS, JavaScript, images etc with it.

The contents of this post is for the beginners of Rust language.

You can easily verify the result of this post [here](https://github.com/steadylearner/Rust-Full-Stack/tree/master/web/before/static_files). Clone the repository and **$cargo run or $cargo run --release**. Verify it work and you will have minimum setup to start web development with Rust.

If you want other Rust web projects, please refer to [Rust Full Stack] repostiory and its **before** folders.

You can also read [How to use a single page app with Rust] with React CRA example and [How to use Rust Tera for undefined paths]. These posts are extensions of this post. They will help you to build a full stack Rust web project instantly.

<br />

<h2 class="red-white">[Prerequisite]</h2>

1. [How to install Rust]
2. [Rocket Getting Started]
3. [How to deploy Rust Web App]
4. [How to use React Spring to animate your message]

---

I want you already have Rust installed in your machine. The blog post [How to install Rust] will help you for that. Then, setup development environment for [Rust Rocket]. [Rocket Getting Started] from the author will help you.

You can use other Rust web framework such as [actix] also. The important point here is to learn the workflow so use what you want.

The result of this post will be similar to [How to use React Spring to animate your message]. It won't be the exactly same but you can refer to it first if you are familiar with React.

When you want to deploy your Rust website later. Refer to [How to deploy Rust Web App].

If you are reading this post at [Steadylearner], there will be no better other examples than it.

<br />

<h2 class="blue">Table of Contents</h2>

1. HTML
2. CSS
3. JavaScript
4. Rust Rocket routes to serve them
5. Conclusion

---

You can only read **Rust Rocket to serve them** part if you already have static files and learnt how to structure Rust Rocket project before.

<br />

## 1. HTML

To make static files work is not Rust specific problem. They will work on their own in your browser withtout it. So we will build them first and test it with Rust Rocket later.

We will start with **index.html**. You should have followed [Rocket Getting Started] before you read on.

It will be similar to

```html
<!doctype html>
<html lang=en>

<head>
    <meta charset="UTF-8" >
    <meta name="viewport" content="width=device-width,initial-scale=1" >
    <meta http-equiv="X-UA-Compatible" content="ie=edge" >
    <meta name="author" content="www.steadylearner.com" >
    <title>Rust Website with static files</title>
    <link rel="icon" href="/static/steadylearner_favicon.png"> <!-- 1. -->
    <link rel="stylesheet" href="/static/main.css" > <!-- 2. -->
</head>

<body>
    <h1
        id="message"
        class="center-percent-absolute hover hover--red cursor-pointer transition--default font-two"
    >
        You know what? <!-- 3. -->
    </h1>
    <script type="text/javascript" src="/static/main.js"></script> <!-- 3. -->
</body>
</html>
```

Nothing complicated here. Your **index.html** file will be a linker for all other files you will have.

1. This will be your favicon for your website. It will also show you can serve images for your Rust backend website.

2. It will help you to visually decorate your html elements.

3. You will use JavaScript **alert API** to show another message.

That was all for your **index.html**. Refer to [How to use React Spring to animate your message] if you want to write more complicated code later.

<br />

## 2. CSS

You could see that there is **class="center-percent-absolute"** and more for in the previous index.html. We will write CSS file for them to work.

Your **main.css file** will be similar to

```css
html, body {
  width: 100%;
  height: 100%;
  margin: 0;
  padding: 0;
}

body {
  font-family: helvetica;
  font-size: 62.5%;
}

.font-normal {
  font-size: 1.6rem;
}

/* 1. */
.center-percent-absolute {
  position: absolute;
  margin: 0;
  top: 50%;
  left: 50%;
  transform: translate(-50%, -50%);
}

/* 2. */
.transition--default {
  transition: 1s;
}

.hover:hover {
  opacity: 0.8;
}

.hover--red:hover {
  color: #ff7676;
}

.cursor-pointer {
  cursor: pointer;
}
```

You can use **1.** to center HTML elements on your device wherever they are. **2.** is just for visual effects when your user put cursor on your message.

You can use your CSS instead.

<br />

## 3. JavaScript

Our purpose is just to show you can serve static files.

We won't need complicated JavaScript file. Therfore, You may write main.js file similar to this

```javascript
const message = document.getElementById("message");

message.onclick = () => { alert("I love You 💗") };
```

and it will show your true message when a user click the html element with id "message".

We already have all our HTML, CSS, JavaScript file ready.

You could test them without server side framework if you use path to serve **main.css** and **main.js** without /static/ prefix in **index.html**.

It will be similar to this.

[![react-easy-md-example](https://www.steadylearner.com/static/images/post/web/rust-static-files.png)](https://www.steadylearner.com/static/images/post/web/rust-static-files.png)

<br />

## 4. Rust Rocket routes to serve them

If you already have Cargo installed in your machine, you can use **$cargo new --bin staitc_files** to start your Rust web project.

Then, edit **Cargo.toml** in your folder similar to this. You can think that it is similar to **package.json** if you are familiar with JavaScript.

We don't need many dependencies to make [Rust Rocket] work.

```toml
[package]
name = 'static_files_react_tera_with_rust'
version = "0.1.0"
authors = ["www.steadylearner.com"]
edition = "2018"

[dependencies]
rocket = "0.4.2"
```

Then, we will edit **main.rs** file first in your **src** folder.

```rust
#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;

// 1.
use crate::routes::{ static_files, get };
mod routes;

fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .mount(
            "/",
            // 2.
            routes![
                static_files::file,
                get::index,
            ],
        )
}

fn main() {
    rocket().launch();
}
```

**1.** is used to import the **routes** we will build later.

If you not familiar with Rust yet, please refer to [Rust Rocket] documenation and read [How to modulize your Rust Frontend] and other [Rust blog posts].

**2.** is the payload of your Rust Rocket app. What you need is to include the routes you will make later inside routes! macro.

(Later, we will use **$cargo run** or **cargo run --release** to compile the result binary file and serve your files with the framework. It will show you the same result at your browser if you tested it manually in the previous part.)

Then, we need to make those routes in **routes** folder and it will be easy.

First, you need **mod.rs** file to represent the **routes** folder.

It will be similar to

```rust
pub mod get;
pub mod static_files;
```

and nothing special here. You can see that Rust want you to be explicit when you export modules also with **pub mod**.

If you are familar with JavaScript, you can compare it with **index.js** that only import and export other JavaScript files.

For example,

```JavaScript
import get from "./get";
import staticFiles from "./staticFiles";

export {
  get,
  staticFiles,
}
```

Then, write **get.rs**

```rust
use std::io;
use rocket::response::{NamedFile}; // 1.

#[get("/")]
pub fn index() -> io::Result<NamedFile> {
    NamedFile::open("static/index.html") // 2.
}
```

1. You can see **NamedFile** API here. You will need it whenever you want to serve static files.

2. The **"static/index.html"** is used because instead of **index.html**. It will work after you write code for **static_files.rs**.

It will be similar to

```rust
use std::path::{Path, PathBuf};
use rocket::response::NamedFile;

#[get("/static/<file..>")]
pub fn file(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(file)).ok()
}
```

and you can compare it with **2.** in **get.rs**.

The author of [Rust Rocket] and its contributors already made [the way to simplify the process](https://api.rocket.rs/v0.4/rocket_contrib/serve/struct.StaticFiles.html). You may use it. You can also use **public** etc instead of **static** here.

You have all the files for your Rust project to serve static files.

What you need is to move all of them (index.html, main.css, main.js, favicon.png) inside your **static** folder.

Then, use **cargo run** to debug or **cargo run --release** for production mode to verify the result.

You will see the message with link you can visit after long wait.

Verify that you could serve static files with Rust server side web framework with the results.

1. **HTML** element **You know what?**

2. **CSS** for it is centered

3. **JavaScript** because it shows message with alert API when you click it

4. **Image** with **favicon.png** served with the title of your index.html

<br />

## 5. Conclusion

You could learn how to serve static files with **Rust**.

You can apply the same process to serve production files form Rust or JavaScript frontend framework because they are just static files also.

With this post, [How to use a single page app with Rust], [How to use Rust Tera for undefined paths] and [How to write Full Stack Rust Code], we already learnt we can write frontend of [Rust Full Stack] website in various ways.

In the later [Rust blog posts], we will learn how to build JSON Web Service, Database, CLI, REST API etc in server and will focus more on server side programming with Rust and combine them what we learnt before. 

If you want the latest contents from Steadylearner, follow me at [Twitter].

Do you need a Full Stack Rust and JavaScript Developer? contact me with [LinkedIn] or be one of them.

**Thanks and please share this post with others.**
