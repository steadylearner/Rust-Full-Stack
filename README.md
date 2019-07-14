<!--
    Post{
        subtitle: "Learn to inlcude JavaScript modules in your Rust Frontend",
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
[Yew Service]: https://github.com/DenisKolodin/yew/tree/master/src/services
[Yew Examples]: https://github.com/DenisKolodin/yew/tree/master/examples
[Yew NPM example]: https://github.com/DenisKolodin/yew/tree/master/examples/npm_and_rest

[Build a rust frontend with Yew]: https://dev.to/deciduously/lets-build-a-rust-frontend-with-yew---part-2-1ech
[rollupjs]: https://github.com/rollup/rollup

[Rocket Yew starter pack]: https://github.com/anxiousmodernman/rocket-yew-starter-pack
[Web completely in Rust]: https://medium.com/@saschagrunert/a-web-application-completely-in-rust-6f6bdb6c4471

[Rocket]: https://rocket.rs/
[Bash for beginners]: http://www.tldp.org/LDP/Bash-Beginners-Guide/html/
[Rust Full Stack]: https://github.com/steadylearner/Rust-Full-Stack
[Browserify]: https://github.com/browserify/browserify
[unpkg]: https://unpkg.com/
[The C programming language]: https://www.google.com/search?q=the+c+programming+language

[node-emoji]: https://www.npmjs.com/package/node-emoji
[actix]: [https://actix.rs/]

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
[Fullstack Rust with Yew]: https://www.steadylearner.com/blog/read/Fullstack-Rust-with-Yew

[How to use Python in JavaScript]: https://www.steadylearner.com/blog/read/How-to-use-Python-in-JavaScript

<!-- / -->

In the previous post [Fullstack Rust with Yew], we learnt how to prepare minimal files to build full stack Rust web app.

You can build whatever Rust allows with it.

But, what if there are no crates or examples in Rust for what you want to build yet? I know that you can eventually make it work and Rust language and its community will help you.

That is important but it will take you some time to make it happen. So in this post we will learn how to use NPM packages directly in your Rust frontend web app. Here, we will use [Yew], [stdweb] but you may use whatever Rust modules relevant to webassembly and JavaScript.

You can think that this post is just [the previous post][Fullstack Rust with Yew] and [browserify] to serve **NPM** files for your Rust [Yew] frontend webassembly.

If you want to save your time, you may clone [Rust Full Stack] and spend time to find what it does.

<br />

<h2 class="red-white">[Prerequisite]</h2>

1. [How to install Rust]
2. [Yew]
3. [How to use Rust Yew]
4. [Fullstack Rust with Yew]
5. [How to use Python in JavaScript]
6. [How to deploy Rust Web App]
7. [How to start Rust Chat App]

---

I want you already have Rust installed in your machine. The blog post [How to install Rust] will help you for that.

If you haven't setup development environment for [Yew], please read the previous post [How to use Rust Yew]. Then, you may read [Fullstack Rust with Yew].

The main point in this post is how to use **FFI**(foreign function interface) between **Rust** and **JavaScript**. I already wrote [How to use Python in JavaScript] for JavaScript and Python and it may help you for this post. You may also read the what **js!** does in [stdweb].

When your full stack Rust app is ready, you can deploy it with [How to deploy Rust Web App].

The **HTML and CSS** files we will use in this post is based on [How to start Rust Chat App].

You may read it and will help you in this post and others later for Rust full stack app we will build later.

<h2 class="blue">Table of Contents</h2>

1. What happens when you include JavaScript files in html
2. Browserify to use NPM modules in Rust Frontend
3. Rust Frotend code to use them
4. Edit Rust server side
5. **Conclusion**

---

You may skip first and second part you already know JavaScript and browserify well.

If you spend some time in Rust and webassembly, you will find that there is no difference between using **Rust and JavaScript** for frontend. What you make are just static files.

You may apply what you learn here when you use JavaScript to build web app also.

<br />

## 1. What happens when you include JavaScript files in html

If you read the documentations from [Yew], you should have found that it already has the example for [NPM][Yew NPM example].

It shows you can use NPM packages with it. But, you will find that it is not a perfect solution for every NPM modules and there was no documentation for that yet.

I hope you tested it in your machine and invested it. You will find that its payload is

```rust
use stdweb::Value;
use stdweb::unstable::TryInto;

#[derive(Default)]
pub struct CcxtService(Option<Value>);

impl CcxtService {
    // 1.
    pub fn new() -> Self {
        let lib = js! {
            return ccxt;
        };
        CcxtService(Some(lib))
    }

    // 2.
}
```

and

```html
<!-- /static/index.html -->
<script type="text/javascript" src="https://unpkg.com/ccxt"></script>
```

If you read the previous post [Fullstack Rust with Yew], you know that those modules with name **Service** are just made from authors to help and you can do the same with **js!** macro from [stdweb].

You will find that you can **copy and paste** the major part of Rust code there.

The important points here are

1. **pub fn new()** is used to start to use **NPM** modules in Rust(lib.rs).

2. Then, you define methods only what you want to use from it in Rust.

You can see that what really work is **js!** and you can do the same without those **Service**. They are there just to make them reusable as it is in other **Service** modules in [Yew].

We will take care of it later with more details.

and You can easily suppose that **https://unpkg.com/ccxt"** in **index.html** help you to use the NPM modules in the global scope of **JavaScript** in browser.

It is not sufficient to find what happens here.

So you may visit the https://unpkg.com/ccxt and you will find that it relocates you to **ccxt.browser.js** file and there are parts

```js
/*  A entry point for the browser bundle version. This gets compiled by:
    browserify --debug ./ccxt.browser.js > ./dist/ccxt.browser.js
*/
window.ccxt = require ('./ccxt')
```

and you can see this is payload to make everything work.

It may not easy to find what it does. You may test it in your browser with these files

```html
<!DOCTYPE html>
<html lang="en">

<head>
  <meta charset="utf-8">
  <meta http-equiv="X-UA-Compatible" content="IE=edge">
  <meta name="viewport" content="width=device-width, initial-scale=1.0">
  <title>What happens when you include js files in your html file</title>
</head>

<body>
  <h1>Open your browser and hello(); or window.hello();</h1>
  <script src="index.js"></script>
</body>

</html>
```

<br />

```js
console.log("Hello from Steadylearner(www.steadylearner.com)")

const hello = () => console.log("Thank for using JavaScript in html. You can use it easily in browser with 'window.code = code' syntax");

window.hello = hello;

// Test hello(); in your console after you open index.html with your browser
```

You will see that those JavaScript files included in index.html will be executed when you open it and you can use variables defined there in your **window**(browser JavaScript global scope).

You may test with your JavaScript code also and test it in your console.

## 2. Browserify to use NPM modules in Rust Frontend

In the previous part, there was **browserify --debug ./ccxt.browser.js > ./dist/ccxt.browser.js**. You can find that [Browserify] was there to help modules in [unpkg] work only with its link.

Having spent some time with [it][unpkg], I found that not every module in it has **browserify** relevant code to make the Rust Frotnend code we read before work.

So we will write code to use [Browserify] on our own instead of using [unpkg]. That will be **payload** your Rust frontend to work.

If you haven't used [Browserify] yet, read [How to start Rust Chat App] or [its documentation][Browserify].

First, start with installing it in your machine.

```console
$npm install -g browserify
```

Then, install NPM modules you want to use. For example,

```console
$yarn add node-emoji
```

We will use **node-emoji** here because we used it in [How to start Rust Chat App] and also visually easy to verify this work.

It will help you find that those **NPM packages** with name **node** or only seem to be useful for **node** environment can also be used in browser for your frontend app.

then write in **/web/static/npm.js**

```js
const emoji = require("node-emoji");

// 1.
window.emoji = emoji;

// 2.
// console.log(emoji);
// console.log(emoji.emojify);
// console.log(emoji.emojify("I :heart: Rust - or use whatever you want"));
```

and in **index.html** to link **node_modules** folder to your **Rust Frontend** code later

```html
<head>
    <script src="bundle.js"></script>
</head>
```

then you can end this whole process for **static** files with **browserify npm.js > bundle.js**.

You can see that **1.** was the payload and uncomment codes in **2.** and test it with your browser console.

<br />

If you want to use more **NPM** moduels later, just **copy and paste** those syntax with more modules.

<br />

## 3. Rust Frontend code to use them

If you haven't read previous post [Fullstack Rust with Yew], please read that first. Otherwise, you may read code of [Rust Full Stack].

We will first build **web/npm/EmojiService.rs** file to follow the rule of [Yew] framework.

```rust
use stdweb::Value;
use stdweb::unstable::TryInto;

#[derive(Default)]
pub struct EmojiService(Option<Value>);

impl EmojiService {
    pub fn new() -> Self {
        let lib = js! {
            return emoji;
        };
        EmojiService(Some(lib))
    }

    pub fn emojify(&mut self, message: String) -> String {
        let lib = self.0.as_ref().expect("node-emoji library object lost");
        let v: Value = js! {
            // 1.
            var emoji = @{lib};
            console.log(emoji);
            return emoji.emojify(@{message});
        };
        // 2.
        let v: String = v.try_into().expect("can't convert to emoji");
        v
    }
}
```

You may use **/web/service** folder instead. You can also use **js!** wherever you want to use in your Rust file instead of writing specific folder and file for them.

What is important here is **js!** macro. You can see that **pub fn new()** part will be always similar. Then, in **1.** only difference is module name and its methods.

If you testsed previous examples here, you can find that you just modifed JavaScript code you used for **Rust**.

**Value**, **String** etc are to use **JavaScript values in Rust** and **@** in **@{lib}** syntax is used to pass value from **Rust to JavaScript**.

If you want more information for them, read the documenation from [its author][stdweb].

Then in **lib.rs**

```rust
fn update(&mut self, msg: Self::Message) -> ShouldRender {
    match msg {
        Msg::Update(val) => {
            self.state.value = val
        }
    true
}
```

to

```rust
fn update(&mut self, msg: Self::Message) -> ShouldRender {
    match msg {
        Msg::Update(val) => {
            let before = format!("{}", &val);
            let emojified = self.emoji.emojify(before.to_string());

            // or use js! here

            self.state.value = emojified
        }
    true
}
```

then **$yarn watch:rs** and test it with **I :heart: Rust** or [node-emoji][whatever you want]. You will find that your input and your message are emojified.

That was all to use **NPM packages** in **Rust Frontend**.

## 4. Edit Rust server side for that

We already prepared all the code for Rust frontend part. We should write more server side code for them to make it a full stack Rust project.

We will edit **run-local.sh** first.

We made **bundle.js** to use **node_modules** in Rust Frontend. So we write code for them in it.

It will be similar to

```sh
#!/bin/bash

set -e #$help set

# build frontend assets and put them in a place the Rocket server
# expects


echo "building web"
pushd web #$help pushd
yarn build
popd #$help popd
echo "web build complete"

cp web/target/wasm32-unknown-unknown/release/index.js server/web/index.js
cp web/target/wasm32-unknown-unknown/release/index.wasm server/web/index.wasm
cp web/static/index.html server/web/index.html
cp web/static/index.css server/web/index.css
cp web/static/favicon.ico server/web/favicon.ico

cp web/static/normalize.css server/web/normalize.css
cp web/static/steadylearner.css server/web/steadylearner.css

cp web/static/bundle.js server/web/bundle.js
cp -R web/static/node_modules server/web/node_moduels

(
  echo "running server"
  cd server
  cargo run --release
)
```

and you can find that

```sh
cp web/static/bundle.js server/web/bundle.js
cp -R web/static/node_modules server/web/node_moduels
```

are used to copy **bundle.js** file and **node_moduels** directory.

Then, write more codes to serve them for **web.rs** we made before similar to

```rs
use std::io;
use std::path::{Path, PathBuf};
use rocket::response::{NamedFile};

// For browserify and NPM to work and it is optional

#[get("/bundle.js")]
pub fn browserify() -> io::Result<NamedFile> {
    NamedFile::open("web/bundle.js")
}

#[get("/node_modules/<file..>")]
pub fn npm(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("web/node_modules/").join(file)).ok()
}
```

and if you already read the code to serve every files in **static** folder with [Rocket]

```rs
// static_files.rs
use std::path::{Path, PathBuf};
use rocket::response::NamedFile;

#[get("/static/<file..>")]
pub fn file(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(file)).ok()
}
```

It is just the variation of the codes you already had.

Then, you may find how it is easy to serve all the files in directory with Rust [Rocket]. It is just matter of **copy and paste**.

If you want more, you can learn how pointers work with [The C programming language].

and include those routes you made in **main.rs**

```rs
fn rocket() -> rocket::Rocket {
    let rocket_routes = routes![
        static_files::file,
        //
        get::index,
        //
        web::web,
        web::web_index_js,
        web::web_wasm,
        web::web_index_css,
        web::web_favicon,
        web::steadylearner_css,
        web::normalize_css,
        // npm
        web::browserify,
        web::npm,
    ];

    rocket::ignite()
        .mount("/", rocket_routes)
}
```

You can test it work with **./run-local.sh** or **cargo c** in server directory.

If you want to use other web frameworks with Rust or other language for server side, just find the [equivalent code](https://actix.rs/docs/static-files/).

## 5. Conclusion

[![Rust full stack chat app](https://www.steadylearner.com/static/images/post/web/full-stack-rust-chat-app-by-steadylearner.png)](https://www.steadylearner.com/static/images/post/web/full-stack-rust-chat-app-by-steadylearner.png)

In this post, we learnt how to use NPM modules in Rust frontend. You can use your **JavaScript** code also. You are already [Rust Full Stack] developer and just write more Rust code to support the term.

If you want to use the code used at [How to start Rust Chat App] and layout of the [Rust Full Stack], you may refer to **main.rs**.
in it. You can follow the process left there and write your full stack Rust chat app.

You can also **copy and paste** codes from [websocket example](https://github.com/DenisKolodin/yew/tree/master/examples/dashboard) and [form example](https://github.com/DenisKolodin/yew/tree/master/examples/todomvc).

You can also read [actix] documentation and its [chat example][https://github.com/actix/actix/tree/master/examples/chat] and will help you learn how socket-client and server communicate.
