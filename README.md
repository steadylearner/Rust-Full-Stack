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

<!-- package.json

cargo web start(include build), cargo web deploy
yarn watch:rs for devlopment then yarn prod(include build) for production

<!-- Steadylearner Post -->

[Rust blog posts]: https://www.steadylearnerc.om/blog/search/Rust
[How to install Rust]: https://www.steadylearner.com/blog/read/How-to-install-Rust
[Rust Chat App]: https://www.steadylearner.com/blog/read/How-to-start-Rust-Chat-App
[Steadylearner Rust Blog Posts]: https://www.steadylearner.com/blog/search/Rust
[Yew Counter]: https://www.steadylearner.com/yew_counter
[How to use Rust Yew]: https://www.steadylearner.com/blog/read/How-to-use-Rust-Yew
[How to deploy Rust Web App]: https://www.steadylearner.com/blog/read/How-to-deploy-Rust-Web-App
[How to start Rust Chat App]: https://www.steadylearner.com/blog/read/How-to-start-Rust-Chat-App

<!-- / -->

In the previous post [How to use Rust Yew], we learnt how to prepare minimal files to build webassembly files with Yew for **Rust frontend**.

We will advance it with some Rust server side code and bash file to automate the process.

If you are competent in **Rust** and other programming, you may use the commands below to save your time.

```console
$git clone https://github.com/steadylearner/Rust-Full-Stack.git
```

then inside **Web** folder

```console
$yarn
$rustup default set nightly
$cargo install cargo-web
```

and

1. **$yarn watch:rs** to test Rust frontend with [Yew] or use $cargo run** also in **server** folder for server side code.

2. For production files before you [deploy][How to deploy Rust Web App] them, use **./run-local.sh**.

Then, write more Rust codes.

<br />

<h2 class="red-white">[Prerequisite]</h2>

1. [How to install Rust]
2. [Yew]
3. [Rocket]
4. [How to use Rust Yew]
5. [Rocket Yew starter pack]
6. [Bash for beginners]
7. [How to start Rust Chat App]

---

I want you already have Rust installed in your machine. The blog post [How to install Rust] will help you to learn how to do that or visit [Rust Website] for more information.

If you haven't setup development environment for [Yew], please read the previous post [How to use Rust Yew].

We will use [Rocket] for our Rust backend. What we will do is just **copy and paste** but reading documents from it will hep you and there are many examples.

What you will get here is just modified version of [Rocket Yew starter pack]. So you may read its code first if you want.

We will use bash commands to automize the process to copy static files made from Yew and will serve it with routes in [Rocket].

Having a little experience in [Bash][Bash for beginners] or Linux commands will help you.(Whenever you have doubts, you can use **$man** command.)

The **HTML and CSS** files we will use in this post is based on [How to start Rust Chat App].

You may read it and will help you in this post and others later for Rust full stack app we will build later.

<br />

<h2 class="blue">Table of Contents</h2>

1. Frontend with Yew
2. Backend with Rocket
3. **Conclusion**

---

You can skip first part if you are already familar with [Yew] or what you want is just to find how to have a minimal [Rust Full Stack] project.

<br />

## 1. Frontend with Yew

I hope you already read [How to use Rust Yew] and read the source code of [Rust Full Stack] repository.

You can easily find that the major differences between them are only **lib.rs and state.rs** file.

First, you may separate state of the entire app with **state.rs**.

For the app used is here is very simple, you may not need them. But it will help you as your app grows.

It will be similar to

```rust
#[derive(Debug)]
pub struct State {
  pub value: String,
}
```

and you will not care much for it except that you will use it with **self.state.value** later in **lib.rs** file or refer to example files in [Yew] documenation.

Then, **lib.rs** will be similar to and

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

**1.** In [Yew], there are prebuilt modules called [Service][Yew Service] to help you and you can use them for JavaScript codes not relevant to render and update components.

When you can't solve the problem with them, think that you may not need them and can do the same with **js!** from [stdweb].

For example, there is no **Widnow Serivce** in [Yew] yet. So you may be confused and want to find the module or documenation for that from framework first.

But you can just write

```rust
js! {
    setTimeout(() => window.scrollTo({ top: window.innerHeight, behavior: "auto" }), 10);
};
```

It is also important to find that Rust Yew code will be used at browser and it will be better to use **console** instead of macros such as println!, println! etc from Rust.

**2.** If you have experience in frontend framework, You will already know that finding how to use a form on your own is not easy.

Fortunately, [Yew] already has [examples][Yew examples] for that and the code snippet above shows you how to get value from input and use it to render other parts of the Yew app.

You can also see that **Model** here has its own methods to render components with **impl**.

You can use them here or find how to extract them as separate components with [documenation][Yew examples].

<br />

## 2. Backend with Rocket

If you spent some time to read codes from [Rocket Yew starter pack], you will find that its payload is **run-local.bash**.

It helps you to move static files made from frontend to bakcend and easily serve those files with [Rocket].

For its frontend used in it is different from [How to use Rust Yew], we have to modify **run-local.bash** to

```sh
#!/bin/bash

set -e #$help set 1.

# build frontend assets and put them in a place the Rocket server
# expects

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

It will be easy to find what they do if you already know how to use POSIX system. But a little help will be useful.

1. **set -e** make the bash file fail when there is a problem.(You will not want your entire website break just to automate the process to copy files and run the files with a few commands.)

2. Commands to copy production static files to server automatically.

Then, we should also build correspondent routes to serve those copied files with [Rocket].

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
```

Once you define those routes to serve production static files from Yew, you will not need to care for them anymore.

Then, other parts will be just a matter of **copy and paste**.

You had to think the entire project as a whole. Then, you wrote corresponding codes to make other parts work.

## 3. Conclusion

If you read the previous post [How to use Rust yew], following this post would be easy.

What we need for our [Rust Full Stack] project to work are

1. [Yew] to write Frotnend code with webassembly and other static files

2. [Rocket] or other Rust framework to serve those files and bash file to automate the process.

The Yew code used here is very simple but if you read the **src** and **examples** form [Yew] and [stdweb]. You can apply it to the Frontend code used at [How to start Rust chat app]. Then, you may build the full stack Rust chat app.

I want you to spend some time to practice [Yew] with it or your previous frontend code. You will find that it is not so difficult.

The benefits of using [Yew] you may find are

1. It is **Rust** code and you do not need **lint** to find errors in your code and compiler will help you how to correct them also.

2. It is difficult to write error prone code because it won't compile if there is any.

3. It takes long when you compile at first. But you don't have to wait a lot after you make your project compile once after you modify them.

4. You can easily use NPM packages if you can't find Rust crates to make your project work.

You can see that many of them are the same from using **Rust** for your project.

We will find how to use **NPM** package in **Rust** Yew frontend in [the next post][Rust blog posts].

**Thanks and please share this post with other**