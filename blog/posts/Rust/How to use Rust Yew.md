<!--
    Post{
        subtitle: "Prepare development environment for Rust yew",
        image: "post/web/how-to-use-rust-yew.png",
        image_decription: "Image made with CSS by Steadylearner",
        tags: "How Rust Yew code",
    }
-->

<!-- Link -->

[Steadylearner]: https://www.steadylearner.com
[Steadylearner Web]: https://github.com/steadylearner/Webassembly
[Rust Website]: https://www.rust-lang.org/
[Cargo Web]: https://github.com/koute/cargo-web
[Yew]: https://github.com/DenisKolodin/yew
[Yew Documenation]: https://docs.rs/yew/0.6.0/yew/
[Build a rust frontend with Yew]: https://dev.to/deciduously/lets-build-a-rust-frontend-with-yew---part-2-1ech
[rollupjs]: https://github.com/rollup/rollup

[Rocket Yew starter pack]: https://github.com/anxiousmodernman/rocket-yew-starter-pack
[Web completely in Rust]: https://medium.com/@saschagrunert/a-web-application-completely-in-rust-6f6bdb6c4471

<!-- / -->

<!-- package.json

cargo web start(include build), cargo web deploy
yarn watch:rs for devlopment then yarn prod(include build) for production

<!-- Steadylearner Post -->

[How to install Rust]: https://www.steadylearner.com/blog/read/How-to-install-Rust
[Rust Chat App]: https://www.steadylearner.com/blog/read/How-to-start-Rust-Chat-App
[Steadylearner Rust Blog Posts]: https://www.steadylearner.com/blog/search/Rust
[Yew Counter]: https://www.steadylearner.com/yew_counter

<!-- / -->

In this post, we will prepare development environment for Rust **Yew**. Then, we will write minimal code with it and learn how to deploy it in [your website][Steadylearner] also.

If you just want to see what you will get with this post, please visit [Yew Counter] in [Steadylearner].

If you want to save your time and have experience in Rust, just clone the [Steadylearner Web] repository with

```console
$git clone https://github.com/steadylearner/Webassembly.git
```

then inside Yew folder

```console
$yarn
$rustup default nightly
$cargo install cargo-web
```

and **$yarn watch:rs** for development and **$yarn prod** for production files in **release**.

Then, make a route to serve **index.html** and modify paths to link them if you find a problem with it.

<br />

<h2 class="red-white">[Prerequisite]</h2>

1. [How to install Rust]
2. [Build a rust frontend with yew]
3. [Cargo Web]
4. [Yew]
5. [rollupjs]

---

I want you to install Rust first if you haven't yet. The blog post [How to install Rust] will help you to learn how to do that or visit [Rust Website] for more information.

This post is based on the information from another blog post for **Yew** [Build a rust frontend with Yew]. You can read until second part of it. You will find that this post will have many common parts with it.

With this post, you will have everything ready to develop Rust **Yew** Web application. Before you invest more time for it, read the documentations from [Yew](especially **src** and **examples** folders). If you see the source code of it, you can see that many of them are from [Cargo Web]. So it will be helpful for you to read its documentation also.

In case of [rollupjs], it will be sufficient for you to know that

**'To use it with a configuration file, pass the --config or -c flags.'**(https://rollupjs.org/guide/en#configuration-files)

If you want to use your own favicon after you read this post or [Build a rust frontend with Yew], please clear cache of your webrowser first and use your file with name favicon.ico instead in **static** folder.

(**$grep "favicon.ico" .** and delete files relevant to it  will not help you.)

<br />

<h2 class="blue">Table of Contents</h2>

1. Install Cargo Web to use Yew
2. How to prepare minimal files for it
3. Rust Yew example from its official website
4. Improve it
5. How to upload it in your website
6. **Conclusion**

---

You can skip some parts if you already read [Build a rust frontend with Yew].

<br />

## 1. Install Cargo Web to use Yew

Before this post, I doubted that my machine with Linux Ubuntu 18.04 would work with **Yew** or not. When you read the documentation from the [Cargo Web], you can see that there are many options and it was difficult to decide what to use.

So I found a blog post such as [Build a rust frontend with Yew]. I hope you already read it and may skip this part. Otherwise, read on and execute commands below.

(I will suppose that you already have Rust installed in your machine with [How to install Rust] or other guides.)

```console
$rustup default nightly // 1.
$cargo install cargo-web // 2.
```

1. You will need nightly features of Rust to use **Yew**. You can use override to make nightly directory specific instead of glboal.

2. It will take long. You may use this time to read [Build a rust frontend with Yew] post and its relevant information.

When you see that [Cargo Web] installation is completed. You have everything ready to write **Rust yew**. Its main role is to help you use JavaScript, HTML etc in Rust **Yew**.

You may follow the instructions from the first part of [Build a rust frontend with Yew] at this point.

It will be to execute command below to make [Cargo Web] work well.

```console
$echo 'default-target = "wasm32-unknown-unknown"' > Web.toml
```

Then, build some files and install NPM packages to make the entire Rust Yew project work. It is just matter of **copy and paste**.

If you want to save time, you can clone [Steadylearner Web] repository and use **Yew** folder.

Then, open the console in it and type **$yarn watch:rs** or **$yarn prod** and you will see the latest **Yew** example from [Steadylearner] in your browser.

<br />

## 2. How to prepare minimal files for it

If you read [the blog post][Build a rust frontend with Yew] or cloned [Steadylearner Web], you should have everything to start development with Rust Yew already.

Having the minimal folder structure to start with will always be helpful.

It will be similar to

```console
├── Cargo.toml
├── package.json
├── rollup.config.js
├── src
│   ├── components
│   ├── lib.rs
│   └── main.rs
├── static
│   ├── favicon.ico
│   ├── index.css
│   ├── index.html
├── Web.toml
```

We don't need to edit **Web.toml** and you may modify folder or file names for **Cargo.toml**, **rollup.config.js** when you want to customize them.

So what left are **package.json**, and **src** and **static** folders. I already gave you a information about **favicon.ico** you can use your image instead of it.

You don't have to care for **components** and **lib.rs** file also. You may delete all files in **components** and empty **lib.rs** file but don't delete it because it is included in **Cargo.toml**.

In those processes, we removed some options and what we need to care for are only

```console
├── package.json
├── src
│   └── main.rs
├── static
│   ├── index.css
│   ├── index.html
```

We have payloads for this project. We will start from **package.json**.

If you see the scripts and the devDependencies parts of **package.json** from the [the blog post][Build a rust frontend with Yew]

```json
{
  "scripts": {
    "build:js": "rollup -c",
    "build:rs": "cargo web deploy --release",
    "build:scss": "node-sass --include-path scss scss/hunt.scss css/hunt.css",
    "build:css": "postcss --use autoprefixer -o static/hunt.css css/hunt.css",
    "build:style": "run-s build:scss build:css",
    "build:copy": "cp target/deploy/hunt.css release/ && cp target/deploy/hunt.wasm release/ && cp target/deploy/index.html release/ && cp target/deploy/favicon.ico release/",
    "build": "run-s clean:deploy build:rs build:js build:style build:copy",
    "clean:deploy": "rm -rf /release",
    "prod": "run-s build serve",
    "serve": "serve -p 8080 release",
    "watch:rs": "cargo web start --release",
    "test": "echo \"Error: no tests!\" && exit 1"
  },
  "devDependencies": {
    "@babel/core": "^7.4.5",
    "@babel/preset-env": "^7.4.5",
    "autoprefixer": "^9.6.0",
    "node-sass": "^4.12.0",
    "nodemon": "^1.19.1",
    "npm-run-all": "^4.1.5",
    "postcss": "^7.0.17",
    "postcss-cli": "^6.1.2",
    "rollup": "^1.15.6",
    "rollup-plugin-babel": "^4.3.2",
    "rollup-plugin-postcss": "^2.0.3",
    "rollup-plugin-uglify": "^6.0.2",
    "rollup-plugin-wasm": "^3.0.0",
    "serve": "^11.0.2"
  }
}
```

It is not easy to find what they do. If you spent time for that, you may find that you can remove many of them if you don't use sass etc to write CSS.

Therefore, if you remove parts for them it will be

```json
{
    "scripts": {
        "build:js": "rollup -c",
        "build:rs": "cargo web deploy --release",
        "build:copy": "cp target/deploy/index.css release/ && cp target/deploy/index.wasm release/ && cp target/deploy/index.html release/ && cp target/deploy/favicon.ico release/",
        "build": "run-s clean:deploy build:rs build:js build:copy",
        "clean:deploy": "rm -rf /release",
        "prod": "run-s build serve",
        "serve": "serve -p 8080 release",
        "watch:rs": "cargo web start --release",
        "test": "echo \"Error: no tests!\" && exit 1"
    },
    "devDependencies": {
        "@babel/core": "^7.1.6",
        "@babel/preset-env": "^7.1.6",
        "autoprefixer": "^9.3.1",
        "nodemon": "^1.18.6",
        "npm-run-all": "^4.1.3",
        "rollup": "^0.67.3",
        "rollup-plugin-babel": "^4.0.3",
        "rollup-plugin-uglify": "^6.0.0",
        "rollup-plugin-wasm": "^3.0.0",
        "serve": "^11.0.2"
    }
}
```

and what you need are

1. **$yarn watch:rs** when you develope

2. **$yarn prod** before you prepare production files.

and you don't need some folders for scss etc to work and project becomes simpler.

We have index.html, index.css and main.rs at this point. If you have experience with frontend, you won't need many explantion for how **index.html, index.css** work. You can just refer the files in [Steadylearner Web] repository or write index.html file similar to this.

```html
<!DOCTYPE html>
<html lang="en">

<head>
  <meta charset="utf-8">
  <meta http-equiv="X-UA-Compatible" content="IE=edge">
  <meta name="viewport" content="width=device-width, initial-scale=1.0">

  <!-- https://github.com/steadylearner/code/blob/master/src/metatag/index.js -->

  <title>Web by Steadylearner</title>
  <meta name="description" content="Yew example made by https://www.steadylearner.com" />
  <meta name="thumbnail" content="https://avatars0.githubusercontent.com/u/32325099?s=460&v=4" />

  <!-- / -->

  <!-- Custom CSS -->

  <link rel="stylesheet" type="text/css" href="steadylearner.css" />

  <!-- / -->

  <link rel="stylesheet" type="text/css" href="index.css" />
</head>

<body>
  <script src="custom.js"></script>
  <script src="index.js"></script>
</body>

</html>
```

We have only **main.rs** to care for and we will write code for it in the next part.

<br />

## 3. Rust Yew example from its official website

[Yew] has many examples. But the purpose of this post is to help you have minimal setup and start the development with it.

So we will take minimal example from from its author.

It will be similar to the code snippet below and copy it your **main.rs** file.

```rust
#[macro_use]
extern crate yew;
use yew::prelude::*;

// M in MVC

struct Model {
    value: i64,
}

// C in MVC
enum Msg {
    DoIt,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self {
            value: 0,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::DoIt => self.value = self.value + 1
        }
        true
    }
}

// V in MVC
impl Renderable<Model> for Model {
    fn view(&self) -> Html<Self> {
        html! {
            <div>
               <button onclick=|_| Msg::DoIt,>{ "+1" }</button>
                <p>{ self.value }</p>
            </div>
        }
    }
}

fn main() {
    yew::initialize();
    App::<Model>::new().mount_to_body();
    yew::run_loop();
}
```

If you are familar with other frontend framework such as [React](https://www.steadylearner.com/blog/search/React) or read the documentation I gave you above.

You should know that

```rust
fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
    Self {
        value: 0,
    }
}

fn update(&mut self, msg: Self::Message) -> ShouldRender {
    match msg {
        Msg::DoIt => self.value = self.value + 1
    }
    true
}
```

is payload to control the app and

```rust
html! {
    <div>
        <button onclick=|_| Msg::DoIt,>{ "+1" }</button>
        <p>{ self.value }</p>
    </div>
}
```

is to render the view of it.

The entire main.rs file is the minimal example to show how to control state with [Yew].

We do not need to edit **fn create** part that much because we just need them only when we start it.

If you use **$yarn watch:rs or yarn prod** in your folder it will show you counter app in your [localhost](http://localhost:8080/).

If you made it, you have minimal development environment ready to start to use **Yew**.

You may refer to [Yew] documentation and its examples or follow the next part.

<br />

## 4. Improve it

You are already ready to write codes for Rust with the previous parts. You may have found that what you need to do is just

1. Modify your html to be used inside Rust html! macro

2. Write JavaScript equivalent mehtods to update state in Rust **Yew** way.

But you haven't seen how to use CSS and what if you want to use various methods instead of just update it?

Then, you can refer to the example below.

```rust
#[macro_use]
extern crate yew;
use yew::prelude::*;

struct Model {
    value: i64,
}

// 1.
enum Msg {
    Plus,
    Minus,
    Zero,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();
    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self {
            value: 0,
        }
    }

    // 2.
    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Plus => self.value = self.value + 1,
            Msg::Minus => self.value = self.value - 1,
            Msg::Zero => self.value = 0,
        }
        true
    }
}

impl Renderable<Model> for Model {
    fn view(&self) -> Html<Self> {
        // 3.
        html! {
            <section class=("flex", "center", "height-vh"), >
               <section>
                    <button
                        class=("hover-blue", "cursor-pointer"),
                        onclick=|_| Msg::Plus,
                        title="Click this to plus one",
                    >
                        { "+1" }
                    </button>
                    <button
                        class=("hover-red", "cursor-pointer"),
                        onclick=|_| Msg::Minus,
                        title="Click this to minus one",
                    >
                        { "-1" }
                    </button>
                   <p
                        class=("flex", "center", "cursor-pointer"),
                        onclick=|_| Msg::Zero,
                        title="Click this back to zero",
                    >
                        { self.value }
                    </p>
               </section>
            </section>
        }
    }
}

fn main() {
    yew::initialize();
    App::<Model>::new().mount_to_body();
    yew::run_loop();
}
```

You can see that

1. We write more options for **enum Msg**

2. Modify update parts to make work with it

3. Include some class names, title and event handlers for html! macro to work

For **html!** is macro, we don't need to invest time to find what they do, just follow the rules.

It is important to notice that you should write **,** when ever you write prop(atrribute) for your html tags.

You can live edit your app with **$yarn watch:rs**. Whenever you modify the html example, you will see message similar to this

```console
==== Triggering `cargo build` ====
   Compiling index v0.1.0 (/steadylearner/Yew)
```

and it take a little bit long because **Rust** will statically verify your web application and wouldn't compile if there is a problem in your code.

I hope you made it. You could include **steadylearner.css** in [Steadylearner Web] repository for your index.css or use your own CSS if you want.

I hope you made it and we will find how to upload it in the website such as [Yew Counter] in [Steadylearner].

If you build the file with **$yarn prod** at this point, you will see console message similar to

```console
$cp target/deploy/index.css release/ && cp target/deploy/
```

and your production files ready at **release** folder.

```console
├── favicon.ico
├── index.css
├── index.html
├── index.js
└── index.wasm
```

It won't be that different from **static** folder and we will learn how to modify it to work for in [our Rust Website][Steadylearner].

The entire app is simple but you may call yourself "Full Stack Rust Devloper" after you deploy more advanced apps later.

(You may edit frontend part of [Rust Chat App] with [Yew] if you want a challenge.)

## 5. How to upload it in your website

The files we made before are just static files.

If you have experience in single page apps and how to make them work in your website, you will know that what you need are

1. Make a route for the **index.html** and verify it work

2. Edit path for other static files such as **index.css**, **index.js**, **index.wasm** and **favicon.ico** etc.

In this part, we will use **Rust** Rocket framework and its relevant codes.

I do not have any favor of it. Just use it for [Steadylearner] and [blog posts](https://www.steadylearner.com/blog) because

1. It has many examples.

2. It is better to use **path** first framework when you need many webpages with different paths.

You can use whatever web framework and languages you want and just refer the process here.

If you want the entire example for this part, you may refer to files and folder structure used for [Rust Chat App] and use static files you made.

We will first start with get.rs to write a route to serve **index.html** file we made before.

```rust
use std::io;
use std::path::{PathBuf};
use rocket::response::{NamedFile, Redirect};

// [Web]

#[get("/yew_counter")]
pub fn yew_counter() -> io::Result<NamedFile> {
    NamedFile::open("static/yew_counter/index.html")
}
```

You can infer that you should incldue all the static files you made before in **static/yew_counter** folder.

and **main.rs** to serve routes and start your application.

```rust
#![feature(proc_macro_hygiene, decl_macro, custom_attribute, rustc_private, type_ascription, async_await)]
#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;

mod route;
use crate::route::{static_files, get};

fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .mount(
            "/",
            routes![
                static_files::file,

                get::index,

                // Web, I use it instead of Webassembly.

                get::yew_counter,
            ],
        )
        .register(catchers![error::not_found])
}

fn main() {
    rocket().launch();
}
```

They are simplified and you can use your own codes instead.

You are ready with your serverside code.

I hope you already made **yew_counter** folder and we should edit **index.html** and **index.js** file.

Because the environment to serve file is different between development and production, we have to modify paths to make everything work in serverside.

You can refer to [Steadylearner Web] repository and **static_files_in_server_example** in **Yew/referenece** folder if you want the entire example.

Inside **index.html** it have parts to link CSS files and JavaScript files to it.

```html
<link rel="stylesheet" type="text/css" href="steadylearner.css" />
<link rel="stylesheet" type="text/css" href="index.css" />

<body>
  <script src="index.js"></script>
</body>
```

You can modify it to

```html
<link rel="stylesheet" type="text/css" href="/static/css/steadylearner.css" />
<link rel="stylesheet" type="text/css" href="/static/yew_counter/index.css" />

<body>
  <script src="/static/yew_counter/index.js"></script>
</body>
```

I had already equivalent CSS for [Steadylearner] so I used path for it instead of **yew_counter** folder not to duplicate here.

It would not be difficult to find what happens here if [you know how to deploy a website](https://www.google.com/search?q=how+to+deploy+Rust).

and in your **index.js**, you can see that code to serve webassembly files similar to

```js
if (typeof process === "object") {
    var path = require("path");

    var wasm_path = path.join(__dirname, "index.wasm");
    var buffer = fs.readFileSync(wasm_path);
    var mod = new WebAssembly.Module(buffer);
    var wasm_instance = new WebAssembly.Instance(mod, instance.imports);
    return instance.initialize(wasm_instance);
} else {
    var file = fetch("index.wasm", {
    credentials: "same-origin"
});
```

It may not work in your serverside so you may modify it to

```js
if (typeof process === "object") {
    var wasm_path = "/static/yew_counter/index.wasm";
    var buffer = fs.readFileSync(wasm_path);
    var mod = new WebAssembly.Module(buffer);
    var wasm_instance = new WebAssembly.Instance(mod, instance.imports);
    return instance.initialize(wasm_instance);
} else {
    var file = fetch("/static/yew_counter/index.wasm", {
    credentials: "same-origin"
});
```

It is JavaScript but what we do is the same. You just modify paths to work well inside with other files in your machine.

You may use static files as they are and worked. But hope this example helpful whenever you find a path relevant problem.

Everything is ready. You can use **$cargo run --bin main** or other command to start your app in production.

You will see your webpage similar to [Yew Counter] in [Steadylearner].

I used term **production** here because what you need to do after this process is just to copy and paste them if you use virtual machine for Linux to deploy your website.

You can see [the exact example][Yew Counter] at [Steadylearner] and its many webpages.

## 6. Conclusion

I hope you made it.

We could set up development environment for Rust [Yew]. Then, we improved the official example. We could learn [how to deploy it][Yew Counter].

If you had already [a website built with Rust][Steadylearner], it wouldn't be difficult to follow this post. Because what we made are just static files and there is no difference from other CSS, HTML and JavaScirpt files etc.

In the next post, we will learn how to build [Rust Chat App] with [Yew] instead of JavaScript. It would not be difficult because we already have the minimal code and know how to deploy it.

If you want more advanced example, you may visit [Rocket Yew starter pack] or [Web completely in Rust].

I want to people know that type of framework and languages are not important but programmers with these posts.

**Thanks and please share this post with others.**