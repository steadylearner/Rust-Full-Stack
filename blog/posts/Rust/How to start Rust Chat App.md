<!--
  Post{
    subtitle: "Build your local chat application with it.",
    image: "post/chat/rust_chat_title.png",
    image_decription: "Rust Chat App by Steadylearner",
    tags: "Rust How start code",
  }
-->

<!-- Link  -->

[Steadylearner]: https://www.steadylearner.com
[Steadylearner Github Repository]: https://github.com/steadylearner/Steadylearner
[Steadylearner Chat]: https://github.com/steadylearner/Chat
[How to deploy Rust Web App]: https://medium.com/@steadylearner/how-to-deploy-rust-web-application-8c0e81394bd5?source=---------9------------------
[Steadylearner Post]: https://github.com/steadylearner/Steadylearner/tree/master/post
[LinkedIn]: https://www.linkedin.com/in/steady-learner-3151b7164/
[Twitter]: https://twitter.com/steadylearner_p

[prop-passer]: https://github.com/steadylearner/prop-passer

[How to install Rust]: https://www.rust-lang.org/learn/get-started
[Rust Documentation]: https://doc.rust-lang.org/std/
[cargo-edit]: https://github.com/killercup/cargo-edit
[Rocket]: https://rocket.rs/
[Concurrency in Rust]: https://blog.rust-lang.org/2015/04/10/Fearless-Concurrency.html

[Websocket API]: https://developer.mozilla.org/en-US/docs/Web/API/WebSockets_API
[ws-rs]: https://ws-rs.org/
[ws-rs-html-example]: https://github.com/housleyjk/ws-rs/blob/master/examples/html_chat.rs
[socket-io]: https://socket.io/
[socket-io-and-ws-rs-comparision]: https://github.com/steadylearner/Chat/tree/master/socket_io

[browserify]: https://github.com/browserify/browserify
[node-emoji]: https://www.npmjs.com/package/node-emoji
[has-emoji]: https://www.npmjs.com/package/has-emoji

[Rust Full Stack]: https://github.com/steadylearner/Rust-Full-Stack

<!-- / -->

<!-- Post -->

<!-- / -->

In this post, we will learn how to build simple chat app in your local machine with **Rust** and simple JavaScript.

If We already have experience in Rust, We can just clone the [Steadylearner Chat].

Then, `$yarn` in /static/chat folder to download NPM packages and `$cargo run` for **Rust** crates in **ws_rs_with_rocket** folder.

It will show We the chat app with **Rust** Backend at http://localhost:8000/chat.

Refer to [Rust Full Stack] for full stack Rust version of it.

<br />

<h2 class="red-white">[Prerequisite]</h2>

1. [How to install Rust](https://www.steadylearner.com/blog/read/How-to-install-Rust)
2. [Websocket API]
3. [ws-rs]
4. [ws-rs-html-example]
5. [Steadylearner Chat]
6. [browserify]
7. [node-emoji]
8. [Thread in Rust](https://doc.rust-lang.org/std/thread/)

---

We will use **Rust** mainly to build socket server to handle messages. So We need to install it first following the instruction from [its official website][How to install Rust].

We also need to find what is [Websocket API] to connect, send, receive and close the websocket for your chat app.

Refer to [socket-io] if you are more familar with JavaScript.

We will use ws-rs Rust crate for this post. So visit [its website][ws-rs] and read its documenation first.

The chat app we will build here is just the improved version of [ws-rs-html-example]. Read its code.

I also prepared the examples to compare [ws-rs] to [socket-io] at [socket-io-and-ws-rs-comparision]. Refer to it also.

Whenever you doubt about **Rust**, please visit [Rust Documentation] page. Read [Concurrency in Rust] and search other articles and find why Rust is faster.

If you are completely new at Rust, but familar with JavaScript, read [TypeScript](https://www.typescriptlang.org/docs/index.html) documentation first because the purpose of both langauges is to have strong type system. It will help you to learn **Rust** better.

<br />

<h2 class="blue">Table of Contents</h2>

1. Setup [Rocket] to serve files
2. ws-rs for websocket server
3. HTML and CSS for layout
4. JavaScript for the chat app users
5. Conclusion

---

Skip the first part if you already downloaded [GitHub Respoitory][Steadylearner Chat] and learnt how to structure Rust application.

<br />

## 1. Setup Rocket to serve files

If you have tested code at [ws-rs-html-example] before, you should have learnt that a single Rust(.rs) file does everything to render html, serve file, and exchange messages.

It may be a decent single file example but it will be difficult to build more complicated apps later. Therefore, we will pass its role to serve files to [Rocket].

We use [it][Rocket] because it has many examples but you may use another Rust framework.

We will start the project with **Cargo.toml**.

Make it similar to the code snippet below. We can use [cargo-edit] also.

```toml
[package]
name = "chat_app_with_rust_by_steadylearner"
version = "0.1.0"
edition = "2018"

[dependencies]
ws = "0.8.1"
rocket = "0.4.1"
```

Nothing complicated. If you are not familar with [toml](https://doc.rust-lang.org/cargo/reference/manifest.html), think it is similar to **package.json** in JavaScript.

What we need is to notify **Rust** that we will use [Rocket] to serve our files such as **HTML** file to work as socket client and link CSS, JavaScript, images etc with it.

Then, [ws-rs] crate to work as a socket server to handle chat messages. Then, we will build **main.rs** file first to be the starting point for our project.

```rust
#![feature(
    proc_macro_hygiene,
    decl_macro,
    custom_attribute,
    rustc_private,
    type_ascription
)]
#[macro_use]
extern crate rocket;

extern crate ws;

use std::thread;

mod route;
use crate::route::{get, static_files};

mod chat;
use crate::chat::ws_rs;

fn rocket() -> rocket::Rocket {
    let rocket_routes = routes![
        static_files::file,
        //
        get::index,
        get::chat,
    ];

    rocket::ignite()
        .mount("/", rocket_routes)
}

fn main() {
    // 1.
    thread::Builder::new()
        .name("Thread for Rust Chat with ws-rs".into())
        // 2.
        .spawn(|| {
            ws_rs::websocket();
        })
        .unwrap();

    rocket().launch();
}
```

This [Rocket] relevant code will be what we need when we want to write minimum web projects with it.

The important parts are

1. We use [thread](https://doc.rust-lang.org/std/thread/) to separate chat server and not to affect(break) the main server that manages your website(It is Rocket here).

2. We can assign **.stack_size(83886 * 1024)** here if We want to be serious with your chat app later.(Search "How many resources chat app need" at your search engine.)

The part of the code snippet above includes

```rust
mod route;
use crate::route::{get, static_files};

mod chat;
use crate::chat::ws_rs;

fn rocket() -> rocket::Rocket {
    let rocket_routes = routes![
        static_files::file,
        //
        get::index,
        get::chat,
    ];

    rocket::ignite()
        .mount("/", rocket_routes)
}
```

They are to serve routes and start the chat app with them.

What important are **static_files** and **ws_rs**.

When We see the source code for static files first,

```rust
use std::path::{Path, PathBuf};
use rocket::response::NamedFile;

#[get("/static/<file..>")]
pub fn file(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(file)).ok()
}
```

It is to serve every files in static folder. Use it whenever you need to serve all files in static folder.

For **ws_rs** route, the important part is

```rust
#[get("/chat")]
pub fn chat() -> io::Result<NamedFile> {
    NamedFile::open("static/chat/index.html")
}
```

and it is to serve **HTML** files for the chat app.

For example, `https://www.yourwebsite.com/chat`.

The file will help users to connect their websockets and have the separtate data with JavaScript and brwoser API later. We will learn how to do that later.

Refer to [Steadylearner Chat] and [Rocket] documentation for more information.

<br />

## 2. ws-rs for websocket server

In this part, we will learn how to build websocket server with Rust.

Read the code snippet for [ws-rs](https://github.com/steadylearner/Chat/blob/master/ws_rs_with_rocket/src/chat/ws_rs.rs). It will be similar to this and similar to [ws-rs-html-example].

```rust
use ws::{
    listen,
    CloseCode,
    Error,
    Handler,
    Handshake,
    Message,
    Request,
    Response,
    Result,
    Sender,
};

use std::cell::Cell;
use std::rc::Rc;

// Server web application handler
struct Server {
    out: Sender,
    count: Rc<Cell<u32>>,
}

impl Handler for Server {
    // 1.
    fn on_request(&mut self, req: &Request) -> Result<(Response)> {
        match req.resource() {
            "/ws" => {
                // 2.
                println!("Browser Request from {:?}", req.origin().unwrap().unwrap());
                // Uncomment this and find what We can do with them when We develope
                // println!("Client found is {:?}", req.client_addr().unwrap());
                // println!("{:?} \n", &resp);
                let resp = Response::from_request(req);
                resp
            }

            _ => Ok(Response::new(404, "Not Found", b"404 - Not Found".to_vec())),
        }
    }

    fn on_open(&mut self, handshake: Handshake) -> Result<()> {
        // 3.
        self.count.set(self.count.get() + 1);
        let number_of_connection = self.count.get();

        if number_of_connection > 5 {
            // panic!("There are more user connection than expected.");
        }

        // 4.
        let open_message = format!("{} entered and the number of live connections is {}", &handshake.peer_addr.unwrap(), &number_of_connection);
        // println!("{}", &handshake.local_addr.unwrap());

        println!("{}", &open_message);
        self.out.broadcast(open_message);

        Ok(())
    }

    fn on_message(&mut self, message: Message) -> Result<()> {
        let raw_message = message.into_text()?;
        println!("The message from the client is {:#?}", &raw_message);

        // 5.
        let message = if raw_message.contains("!warn") {
            let warn_message = "One of the clients sent warning to the server.";
            println!("{}", &warn_message);
            Message::Text("There was warning from another user.".to_string())
        } else {
            Message::Text(raw_message)
        };

        // 6.
        // Broadcast to all connections
        self.out.broadcast(message)
    }

    fn on_close(&mut self, code: CloseCode, reason: &str) {
        match code {
            CloseCode::Normal => println!("The client is done with the connection."),
            CloseCode::Away => println!("The client is leaving the site."),
            CloseCode::Abnormal => {
                println!("Closing handshake failed! Unable to obtain closing status from client.")
            },
            _ => println!("The client encountered an error: {}", reason),
        }

        // 7.
        self.count.set(self.count.get() - 1)
    }

    fn on_error(&mut self, err: Error) {
        println!("The server encountered an error: {:?}", err);
    }
}

pub fn websocket() -> () {
  println!("Web Socket Server is ready at ws://127.0.0.1:7777/ws");
  println!("Server is ready at http://127.0.0.1:7777/");

  // Rc is a reference-counted box for sharing the count between handlers
  // since each handler needs to own its contents.
  // Cell gives us interior mutability so we can increment
  // or decrement the count between handlers.

  // Listen on an address and call the closure for each connection
  let count = Rc::new(Cell::new(0));
  listen("127.0.0.1:7777", |out| { Server { out: out, count: count.clone(), } }).unwrap()
}
```

The important parts will be

1. [We need on_request part just once and We don't have to reconnect later](https://blog.stanko.io/do-We-really-need-websockets-343aed40aa9b). Never edit this part if We are not an expert with this.

2. Use them to verify what We can do when the first socket connection between server and client happen and read [the documenation for them](https://ws-rs.org/api_docs/ws/struct.Request.html)

3. We need to count how many connections there are because it affects connection quality. Use the `number_of_connection` variable with conditional statement.(We will write code for that in client side later. We may use your own code.)

4. **This is the most important part.** Even though we use localhost first and not real users, there should be some ways to differenciate the users from one another. So We will use return value of `&handshake.peer_addr.unwrap()` for that and also `number of connection` inside `fn on_open`. (If We open various windows for http://localhost:8000/chat later, We can see that it always return different values in your CLI.)

5. This is where We can do use your logics with the messages from users. We can use database to save messages from users and write experimental code, for example, to send **warning** received from other users to everyone connected to the server socket.(We may test it with **!warn** in socket client later.)

6. `self.out.broadcast(message)` used to send messages to all. It is the last API used before the messages from the server arrive to clients connected to socket.

7. `self.count.set(self.count.get() - 1)` is used to recalculate the total number of user when some client close the connection.

Find what are the uses of [Sender](https://doc.rust-lang.org/std/sync/mpsc/struct.Sender.html), [Rc](https://doc.rust-lang.org/std/rc/index.html) and [Cell](https://doc.rust-lang.org/std/cell/index.html) on your own.

(The chat server is a single threaded, share the count value among clients(shared value in Rust is not mutable) but we yet want to edit it with **Rc** and **get and set API** from **Cell** in Rust.)

The majority of the code used above are relevant to the JavaScript client code we will write later.

<br />

## 3. HTML and CSS for layout

![Rust Chat App](https://www.steadylearner.com/static/images/post/chat/rust-chat.png)

We learnt how to make the Rust websocket server to build our chat app. It is time to build frontend part to help users.

If We see the [index.html](https://github.com/steadylearner/Chat/blob/master/ws_rs_with_rocket/static/chat/index.html) file, The important part will be

```html
<head>
  <!-- 1. -->
  <link rel="stylesheet" href="/static/css/normalize.css" />
  <link rel="stylesheet" href="/static/css/steadylearner.css" />
  <link rel="stylesheet" href="/static/chat/index.css" />
</head>
<body>
  <section>
    <nav id="nav" class="nav flex center">
      <a
        href="https://www.steadhylearner.com/blog"
        title="Click it to learn how to code this."
        class="flex no-text-decoration hover cursor-pointer transition-half right-auto"
      >
        <span class="white" >Rust Chat App</span>
      </a>
      <!-- 2. -->
      <span
        id="exit"
        class="margin-right-one white cursor-pointer hover transition"
      >
        Exit
      </span>
    </nav>
    <ul
      id="messages"
    >
      <!-- 3. -->
    </ul>
    <form id="form" class="chat-input flex center">
      <img
        id="code"
        class="flex center rust-icon hover cursor-pointer transition-half"
        title="Click this to use markdown or not"
        src="/static/images/Rust.svg"
      />
      <input
        id="msg"
        type="text"
        placeholder="Type here to start to talk with others."
        autocomplete="off"
      >
      <button class="blue cursor-pointer" >Send</button>
      <button
        id="clear"
        type="button"
        class="margin-left-one red-white cursor-pointer"
      >
        Clear
      </button>
    </form>
  </section>
  <!-- 4. -->
  <script
    src="/static/chat/bundle.js"
    type="text/javascript"
  >
  </script>
</body>
</html>
```

The main points are

1. They are CSS files for layout. Edit or use yours if you want.

2. We will use various ids inside html file to select and manipulate them with JavaScript.

3. The chat message will be written here inside `<li>` wrappers. It is important that they are under `<ul id="messages" >` to be deleted easily later with JavaScript `removeMessages` function.

4. We will use [browserify] to bundle our JavaScript files with NPM modules. It will help us to use emoji.

They are to be used with JavaScript and for the layout of our proejct.

When we build the app later, it will be similar to the image.

**steadylearner.css** above is especially used for React and [prop-passer] package.

It is just the bunch of class names that does one thing at a time. Verify how they work better at [Steadylearner]

<br />

## 4. JavaScript for the chat app users

We prepared a lot. In this part, we will write a JavaScript code and will help users to use it in their browsers.

If you already have experience with chat app. The part 1. and 2. is already sufficient to start your Rust chat app.

So I want you know the two points.

1. Use this just for a reference.

2. No frontend frameworks here because I wanted to find I can write something useful without them.

The code below will not be well-organized. Use framework and JSON format to structure your data instead. Refer to [Rust Full Stack] if you want the Rust version of this.

The **index.js** file we will use will be similar to the code below.

```js
// 1.
const emoji = require("node-emoji");
const hasEmoji = require("has-emoji");

const socket = new WebSocket("ws://127.0.0.1:7777/ws");

// 2.

function getDateTime() {
  const today = new Date();
  const date = today.getFullYear() + '-' + (today.getMonth() + 1) + '-' + today.getDate();
  const time = today.getHours() + ":" + today.getMinutes() + ":" + today.getSeconds();
  const payload = date + ' ' + time;
  return payload;
}

function removeMessages() { // remove child elements from its parent element
  const messages = document.getElementById("messages");
  while (messages.firstChild) {
    messages.removeChild(messages.firstChild);
  }
}

let open = false;

let userId = "";
let userInputs = [];

let server = []

socket.addEventListener('open', function (event) {
  // socket.send('Start to chat');
  console.log("Start to chat");
});

// 3.

const clear = document.getElementById("clear");
clear.onclick = removeMessages;

const exit = document.getElementById("exit");
exit.onclick = function () {
  socket.close();
}

// 4.

const form = document.getElementById("form");

form.onsubmit = function (event) {
  event.preventDefault();
  const input = document.getElementById("msg");

  if (input.value === "") {
    return;
  }

  if (input.value === "!clear") {
    removeMessages()
    input.value = "";
    return;
  }

  if (input.value === "!exit") {
    socket.close();
    return;
  }

  const userInputWithTime = `${userId} typed ${input.value} at ${getDateTime()}`;
  userInputs.push(userInputWithTime);

  socket.send(`${userId}: ${input.value}`);
  input.value = "";
  // Comment it and find the difference(It scroll down the window when user type)
  setTimeout(() => window.scrollTo({ top: window.innerHeight, behavior: "auto" }), 10);
};

socket.onmessage = function (event) {
  // To save what server sent to localStorage, use database in production
  const messagefromServer = `Server ${event.origin} sent ${event.data} at ${getDateTime()}`
  server.push(messagefromServer);

  // if (userInputs[userInputs.length - 1] === "!warn") {
  //   alert("We sent warning to the other users");
  // }

  if (event.data.includes("!clearall")) {
    removeMessages();
    return;
  }

  if (event.data.includes("!exitall")) {
    socket.close();
    return;
  }

  if (event.data.includes("!x-opacity")) {
    const messages = document.getElementById("messages");
    if (messages.className === "x-opacity") { messages.className = ""; } else { messages.className = "x-opacity" }
    return;
  }

  // 5.
  if (!open) {
    // To give id to user and verify the maximum number, only work once

    // See the Rust code we defined before and find that they are relevant.

    // We pick the first(id for user and will be We with JavaScript)
    // and the last part(number of connection) from
    // open_message variable with JavaScript

    // fn on_open(&mut self, handshake: Handshake) -> Result<()> {
    //     self.count.set(self.count.get() + 1);
    //     let number_of_connection = self.count.get();

    //     let open_message = format!("{} entered and the number of live connections is {}", &handshake.peer_addr.unwrap(), &number_of_connection);
    //     self.out.broadcast(open_message); -> becomes event.data

    //     Ok(())
    // }

    let separate = event.data.split(" ");
    userId = separate[0];

    const messages = document.getElementById("messages");
    const li = document.createElement("li");
    const p = document.createElement("p");

    let totalNumber = separate[separate.length - 1];
    // 6.
    if (totalNumber > 5) {
      p.textContent = `${totalNumber - 1} is maximum user allowed. Wait for others exit the chat.`;
      p.className = "red-white";
      li.append(p)
      messages.append(li);
      socket.close();
      return;
    }

    open = true;

    p.textContent = `Your id is ${userId} and "We" will be used in this page instead`;
    p.className = "blue";
    li.append(p)
    messages.append(li);
    return;
  } else {
    // use JSON format and its api here instead

    let fromServer = event.data;
    const beforePayload = fromServer.split(" ")[0];
    const authorOfMessage = beforePayload.slice(0, beforePayload.length - 1); // to get the id part of the message

    // if (authorOfMessage !== userId && fromServer.includes(`!exclude ${userId}`)) {
    if (fromServer.includes(`!exclude ${userId}`)) {
      socket.close();
      return;
    }

    const messages = document.getElementById("messages");
    const li = document.createElement("li");

    // Give color and "We" for a user when author of the messages is the user.
    if (authorOfMessage === userId) {
      li.className = "red-white";
      fromServer = fromServer.replace(userId, "We");
    }

    // 7.
    const includeEmoji = hasEmoji(emoji.emojify(fromServer));
    afterEmoji = includeEmoji ? emoji.emojify(fromServer) : fromServer;

    const p = document.createElement("p");
    p.append(afterEmoji)
    li.append(p);
    messages.append(li);
    return;
  }
};

// 8.
socket.onclose = function (event) {
  const closeMessage = event.data === undefined ? "Server, We or another user closed the connection." : "WebSocket is closed now."
  const messages = document.getElementById("messages");

  const li = document.createElement("li");
  li.append(closeMessage)
  li.className = "blue";
  messages.append(li);

  localStorage.setItem("userInputs", `[${userInputs}]`);
  localStorage.setItem("server", `[${server}]`);
};
```

The payloads are.

1. Import modules to use emojis in the chat app and connect clients to the websocket server with `new WebSocket("ws://127.0.0.1:7777/ws");`.

2. We define custom functions to log time when the users send messages and remove messages. Then, we make the default state for clients and we will manipulate it with JavaScript.

3. We assign roles for HTML code with id **clear** and **exit** we made in **index.html**. Find what they do and the codes that does similar things in **4.**.

4. We find HTML element with id **form** with `document.getElementById`. Then, we define what should happen when users type with it. We save the user input with time(**userInputs.push(userInputWithTime);**) and send it to the server with `socket.send`.(We can write some features before user input is sent to the socket server such as "clear" and "exit" here.)

5. **This is the payload**. We defined some variables in **2.**. Then, we can use it to assign **id** to user with **let separate = event.data.split(" ");** and **userId = separate[0];**. Verify the user is already connected to server or not with JavaScript(socket is open or not). We turn **open = false;** to **open = true;** inside it and the client side code will execute the codes inside **if(!open)** only once.

6. We didn't write code what to do when there are more users than allowed at Rust server side before. Instead, we make client to leave the connection with JavaScript here.

7. Use this part to allow users to send emojis. Read the doucmenation [node-emoji] and [has-emoji].

8. When socket closes, we notify users that the socket connection is closed and save messages from **the user, other users and server** to the localStorage. Use your database API instead.

Install [Browserify] with `$sudo npm install -g browserify`. Then, bundle NPM packages and your index.js with it.

```console
$browserify index.js > bundle.js`
```

Start the project with `$cargo run` and verify the result. Test it with various windows connected to it and type commands defined here and click the components.

<br />

## 4. Conclusion

I hope the post was helpful to start your first chat app with Rust. You can find the [Rust Full Stack] version of it also.

If you want the latest contents, please follow me at [Twitter]. Contact me with [LinkedIn] if you need a [Rust Full Stack] developer.

**Thanks and please share this post with others.**