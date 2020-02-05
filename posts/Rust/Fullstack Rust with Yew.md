<!--
    Post{
        subtitle: "Have minimal files to be a fullstack Rust developer.",
        image: "post/web/full-stack-rust-with-yew.png",
        image_decription: "Image by Steadylearner",
        tags: "How Rust Yew code",
    }
-->

<!-- Link -->

[Steadylearner]: https://www.steadylearner.com
[Steadylearner Web]: https://github.com/steadylearner/Webassembly
[Rust Website]: https://www.rust-lang.org/
[Cargo Web]: https://github.com/koute/cargo-web
[stdweb]: https://github.com/koute/stdweb
[Yew]: https://github.com/DenisKolodin/yew
[Yew Documenation]: https://docs.rs/yew/0.6.0/yew/
[Build a rust frontend with Yew]: https://dev.to/deciduously/lets-build-a-rust-frontend-with-yew---part-2-1ech
[rollupjs]: https://github.com/rollup/rollup

[Rocket Yew starter pack]: https://github.com/anxiousmodernman/rocket-yew-starter-pack
[Web completely in Rust]: https://medium.com/@saschagrunert/a-web-application-completely-in-rust-6f6bdb6c4471

[Rocket]: https://rocket.rs/

[Bash for beginners]: http://www.tldp.org/LDP/Bash-Beginners-Guide/html/

[Rust Full Stack]: https://github.com/steadylearner/Rust-Full-Stack

[Yew Service]: https://github.com/DenisKolodin/yew/tree/master/src/services
[Yew Examples]: https://github.com/DenisKolodin/yew/tree/master/examples

<!-- / -->

<!-- Steadylearner Post -->

[Rust blog posts]: https://www.steadylearner.com/blog/search/Rust
[How to install Rust]: https://www.steadylearner.com/blog/read/How-to-install-Rust
[Rust Chat App]: https://www.steadylearner.com/blog/read/How-to-start-Rust-Chat-App
[Steadylearner Rust Blog Posts]: https://www.steadylearner.com/blog/search/Rust
[Yew Counter]: https://www.steadylearner.com/yew_counter
[How to use Rust Yew]: https://www.steadylearner.com/blog/read/How-to-use-Rust-Yew
[How to deploy Rust Web App]: https://www.steadylearner.com/blog/read/How-to-deploy-Rust-Web-App
[How to start Rust Chat App]: https://www.steadylearner.com/blog/read/How-to-start-Rust-Chat-App

<!-- / -->

In the previous post [How to use Rust Yew], we learnt how to prepare minimal files to build webassembly files with Yew for **Rust frontend**.

We will advance it with some Rust server side code and write a bash file to automate the process.

If you are competent in **Rust**, use the commands below to save your time.

```console
$git clone https://github.com/steadylearner/Rust-Full-Stack.git
$rustup default nightly
$cargo install cargo-web
```

Then, **$yarn** inside **Web** to use **rollup** to use live Rust Yew editor and in **Web/static** folder **$yarn** again to use **NPM packages** for Rust Frontend with **Browserify**.

When you install all of them, you can use

1. **$yarn watch:rs** if you want only test frontend with [Yew] or use **$cargo run** also in **server** folder for server side Rust code.

2. Before you [deploy][How to deploy Rust Web App] production files, use **./run-local.sh**.

<br />

<h2 class="red-white">[Prerequisite]</h2>

1. [How to install Rust]
2. [Yew]
3. [Rocket]
4. [How to use Rust Yew]
5. [Rocket Yew starter pack]
6. [Bash for beginners]
7. [How to deploy Rust Web App]
8. [How to start Rust Chat App]

---

I want you already have Rust installed in your machine. The blog post [How to install Rust] will help you to learn how to do that or visit [Rust Website] for more information.

If you haven't setup development environment for [Yew], please read the previous post [How to use Rust Yew].

We will use [Rocket] for our Rust backend. What we will do is just **copy and paste** but read its documentation.

What you will get here is a modified version of [Rocket Yew starter pack]. So you may read its code first.

Having a little experience in [Bash][Bash for beginners] or Linux commands will help you.(Whenever you have doubts, you can use **$man** command.)

When your full stack Rust app is ready, you can deploy it with [How to deploy Rust Web App].

The **HTML and CSS** files we will use in this post is based on [How to start Rust Chat App].

Read it and will help you in this and when we build a **Rust full stack chat app** better than this later.

<br />

[![Rust full stack chat app](https://www.steadylearner.com/static/images/post/web/full-stack-rust-chat-app-by-steadylearner.png)](https://www.steadylearner.com/static/images/post/web/full-stack-rust-chat-app-by-steadylearner.png)

<br />

<h2 class="blue">Table of Contents</h2>

1. Frontend with Yew
2. Backend with Rocket
3. **Conclusion**

---

You can skip first part if you are already familar with [Yew].

<br />

## 1. Frontend with Yew

I hope you already read [How to use Rust Yew] and read the source code of [Rust Full Stack] repository.

You can easily find that the major differences between them are only **lib.rs and state.rs**.

First, you may separate state of the entire app with **state.rs**.

For the app used is here is very simple, you may not need it at this point. But it will help you as your app grows.

It will be similar to

```rust
#[derive(Debug)]
pub struct State {
  pub value: String,
}
```

and use it with **self.state.value** later in **lib.rs** file or refer to example files in [Yew] documenation.

Then, **lib.rs** will be similar to

```rust
#![recursion_limit="512"]
#![feature(rustc_private)]

// #[macro_use]
// extern crate stdweb;
// use stdweb::js;

extern crate yew;
use yew::{html, Component, ComponentLink, Html, Renderable, ShouldRender};
use yew::services::{ConsoleService}; // 1.

mod state;
use self::{
    state::State,
};

pub struct Model {
    console: ConsoleService,
    state: State,
}

pub enum Msg {
    Update(String),
    Exit
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _link: ComponentLink<Self>) -> Self {
        let state = State {
            value: "".to_string(),
        };

        Model {
            console: ConsoleService::new(),
            state,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Update(val) => {
                self.state.value = val
            }
            Msg::Exit => {
                self.console.log("The user wants to leave this.")
                // or
                // js! {
                //     console.log("The user wants to leave this.")
                // }
            }
        }
        true
    }
}

// should write more components to remove this part
impl Renderable<Model> for Model {
    fn view(&self) -> Html<Self> {
        html! {
            <section>
                <nav id="nav", class=("nav", "flex", "center"), >
                    <a
                        class=("flex", "no-text-decoration", "hover", "cursor-pointer", "transition-half", "right-auto"),
                        href="https://www.steadylearner.com/blog",
                        title="Click it to learn how to code this.",
                    >
                        <span class=("white", "bold"), >{ "© Rust Chat App" }</span>
                    </a>
                    <button
                        id="connect",
                        class=("margin-right-one", "white", "cursor-pointer", "hover", "transition", "theme-black"),
                    >
                       { "Enter" }
                    </button>
                    <button
                        id="exit",
                        class=("margin-right-one", "white", "cursor-pointer", "hover", "transition", "theme-black"),
                        onclick=|_| Msg::Exit,
                    >
                       { "Exit" }
                    </button>
                </nav>
                <ul
                    id="messages",
                >
                    { self.view_message() }
                </ul>
                <section
                    id="form",
                    class=("chat-input", "flex", "center"),
                >
                    <img
                        id="code",
                        class=("flex", "center", "rust-icon", "hover", "cursor-pointer", "transition-half"),
                        title="Use this for whatever you want",
                        src="https://www.steadylearner.com/static/images/code/Rust.svg",
                    />
                    { self.view_input() }
                </section>
            </section>
        }
    }
}

// 2.
impl Model {
    fn view_message(&self) -> Html<Model> {
        if !(&self.state.value.is_empty()) {
            html! {
                <li>
                    <span> { format!("You: {}", &self.state.value) }</span>
                </li>
            }
        } else {
            html! {
                { "" }
            }
        }
    }

    fn view_input(&self) -> Html<Model> {
        html! {
            <input
                id="msg",
                type="text",
                placeholder="Type here to start to talk with others and enter to submit",
                title="You should enter the chat before you type.",
                autocomplete="off",
                value=&self.state.value,
                oninput=|e| Msg::Update(e.value),
            />
        }
    }
}
```

and payloads will be

**1.** In [Yew], there are prebuilt modules called [Service][Yew Service] to help you. You can use them for JavaScript codes not relevant to render and update components.

When you can't solve the problem with them, think that you can find how with **js!** from [stdweb].

For example, there is no **Window Serivce** in [Yew] yet. So you may be confused and want to find the module or documenation for that from framework first.

But you can just write

```rust
js! {
    setTimeout(() => window.scrollTo({ top: window.innerHeight, behavior: "auto" }), 10);
};
```

Rust Yew code will be used at browsers so use **console** instead of macros such as **print!, println!** etc from Rust.

**2.** If you have experience in any frontend framework, you will know finding how to use a form is not easy.

Fortunately, [Yew] already has [examples][Yew examples] for that.

The code snippet above shows you

1. How to get value from input

2. Then, use it to render other parts of the Yew app.

You can also see that **Model** here has its own methods to render components with **impl**.

You can use them but find how to make separate components with [documenation][Yew examples].

<br />

## 2. Backend with Rocket

If you read codes from [Rocket Yew starter pack], you will find that its payload is **run-local.bash**.

It helps you to copy static files made from Rust Yew frontend to server. Then, you can use [Rocket] or other Rust framework and write routes to serve them.

For its frontend used in it is different from [How to use Rust Yew], we have to modify **run-local.bash** to

```sh
#!/bin/bash

set -e #$help set 1.

# build static files with HTML, CSS, Webassembly from Rust and JavaScript to link them.

echo "building web"
pushd web #$help pushd
yarn build
popd #$help popd
echo "web build complete"

# 2.
cp web/target/wasm32-unknown-unknown/release/index.js server/web/index.js
cp web/target/wasm32-unknown-unknown/release/index.wasm server/web/index.wasm
cp web/static/index.html server/web/index.html
cp web/static/index.css server/web/index.css
cp web/static/favicon.ico server/web/favicon.ico

cp web/static/normalize.css server/web/normalize.css
cp web/static/steadylearner.css server/web/steadylearner.css

(
  echo "running server"
  cd server
  cargo run --release
)

```

1. **set -e** makes the bash file fail when there is a problem.(You will not want your entire website to break to automate the simple process.)

2. Commands to copy production static files from Rust Yew to server automatically.

Then, we should also build correspondent routes to serve them with [Rocket] or other Rust frameworks.

It will be similar to

```rust
// web.rs
use std::io;
use rocket::response::NamedFile;

#[get("/web")]
pub fn web() -> io::Result<NamedFile> {
    NamedFile::open("web/index.html")
}

#[get("/index.js")]
pub fn web_index_js() -> io::Result<NamedFile> {
    NamedFile::open("web/index.js")
}

#[get("/index.wasm")]
pub fn web_wasm() -> io::Result<NamedFile> {
    NamedFile::open("web/index.wasm")
}

#[get("/index.css")]
pub fn web_index_css() -> io::Result<NamedFile> {
    NamedFile::open("web/index.css")
}

#[get("/favicon.ico")]
pub fn web_favicon() -> io::Result<NamedFile> {
    NamedFile::open("web/favicon.ico")
}

#[get("/steadylearner.css")]
pub fn steadylearner_css() -> io::Result<NamedFile> {
    NamedFile::open("web/steadylearner.css")
}

#[get("/normalize.css")]
pub fn normalize_css() -> io::Result<NamedFile> {
    NamedFile::open("web/normalize.css")
}

// I use web instead of webassembly here because it is short and easy to find what it does
```

Once you define those routes to serve production static files, you will not need to care for them anymore.

Then, other parts will be just a matter of **copy and paste**.

When you write a full stack code, **you should think the entire project as a whole**. Then, edit corresponding parts to make others work with it.

<br />

## 3. Conclusion

If you read the previous post [How to use Rust yew], following this post would be easy.

For your [Rust Full Stack] project to work,

1. [Yew] to write Rust frontend code with webassembly and other static files

2. [Rocket] or other server side framework to serve those files and bash file to automate the process.

The Yew code used here is simple but with **src** and **examples** from [Yew] and [stdweb], you can build more complicated app.

Spend some time to practice [Yew] with frontend code at [How to start Rust chat app] or your previous projects.

**You can make Rust full stack web app.**

The benefits of using [Yew] you will find are

1. It is **Rust** code and you do not need **lint** to find errors in your code and compiler will help you how to correct them also.

2. It is difficult to write error prone code because Rust won't make it compile if there is any.

3. It takes long when you compile at first. But you don't have to wait to start a project a lot after that.(Different from some JavaScript bundlers that make you wait the exact same time whenever you start your frontend.)

4. [You can easily use NPM packages also](https://www.steadylearner.com/blog/read/How-to-use-NPM-packages-with-Rust-Frontend) if you can't find Rust crates to make your project work.

5. You can render a text, image, video and [markdown files](https://www.steadylearner.com/blog/read/How-to-use-markdown-with-Rust-Frontend) with ease.

You can see that many of them are from using **Rust**.

We will find how to use **Rust** frontend more in [the next posts][Rust blog posts].

**Thanks and please share this post with others.**
