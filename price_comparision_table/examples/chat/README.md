<!--
 Post{
   title: "How to start Rust Chat App",
   subtitle: "Build your local chat application with it.",
   image: "post/chat/rust_chat_title.png",
   image_decription: "Rust Chat App by Steadylearner",
   tags: "Rust chat app code",
   theme: "rust",
 }
-->

<!-- Link here -->

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

<!-- Post for this series -->

<!-- / -->

# How to start Rust Chat App

You can also find post for this at [Steadylearner Post for this](https://www.steadylearner.com/blog/read/How-to-start-Rust-Chat-App).

<h2 class="red-white">[Prerequisite]</h2>

1. [How to install Rust]
2. [Websocket API]
3. [ws-rs]
4. [ws-rs-html-example]
5. [Steadylearner Chat]
6. [browserify]
7. [node-emoji]
8. [Thread in Rust](https://doc.rust-lang.org/std/thread/)

---

You will use **Rust** mainly to build socket server to handle messages. So you need to install it following the instruction from [its official website][How to install Rust].

You also need to understand what is [Websocket API] to understand what you need to connect, send, receive and close the websocket for your chat app.

(You may also visit [socket-io] JavaScript framework if you are more familar with JavaScript to understand the topic better.)

We will use ws-rs Rust crate for this post. So I want you to visit [its website][ws-rs] and read its API and documenation first.

The chat app we will build here is just the improved version of [ws-rs-html-example]. Therefore, I want you to visit it and read its source code.

I also prepared the examples to compare [ws-rs] to [socket-io] at [socket-io-and-ws-rs-comparision] GitHub page. You may refer to it if you are already knew  API of [socket-io].

Whenever you have doubt about **Rust**, please visit [Rust Documentation] page and seach on your own. It will be better for you to read [Concurrency in Rust] and search other articles if you want to understand why Rust is fast.

(If you were completely new at Rust, but familar with JavaScript, you may read [TypeScript](https://www.typescriptlang.org/docs/index.html) documentation first because the purpose of both langauges is to have strong type system. It will help you to learn **Rust** if you haven't any experience in Rust.)

<br />

<h2 class="blue">Table of Contents</h2>

1. Setup [Rocket] to serve files
2. ws-rs for socket server
3. HTML and CSS for layout
4. JavaScript for chat app users
5. Conclusion

---

<br />

## 1. Setup Rocket to serve files

(You can skip this part if you already downloaded [GitHub Respoitory][Steadylearner Chat] and understand how to structure Rust application.)

If you have tested code at [ws-rs-html-example] before you read on, you should have found that a single Rust(.rs) file does everything to render html, serve file, and exchange messages.

It may be a decent single file example but it will be difficult to build more complicated app later. Therefore, we will pass its role to serve files to [Rocket] web framework.

(I prefer [it][Rocket] for it has many examples but you may use whatever framework.)

We can start this from our **Cargo.toml** file like the code snippet below or use **$cargo add ws-rs rocket**.(You can visit [cargo-edit] page for this command)

```toml
[package]
name = "chat_app_with_rust_by_steadylearner"
version = "0.1.0"
edition = "2018"

[dependencies]
ws = "0.8.1"
rocket = "0.4.1"
```

Nothing complicated for our Cargo.toml, What we need is to notify **Rust** that we need [Rocket] to serve our files such as **HTML** file to work as socket client, CSS, JavaScript, images etc at webpage. 

Then, [ws-rs] crate to work as socket server to handle chat messages.

(If you are not familar with [.toml files](https://doc.rust-lang.org/cargo/reference/manifest.html), you may think it is similar to **package.json** in JavaScript)

Then we will build **main.rs** file first to be starting point for our Rust app.(Please, refer to the repository I gave before whenever you have doubts.)

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

The major part of the [Rocket] relevant code will be the boilerplate if you want to write minimum webpage with it. So you don't have to understand all at once.

But the important points here are

1. We use [thread](https://doc.rust-lang.org/std/thread/) to separate chat server and not to affect(break) the main server that manages your website(It is Rocket here)

2. You can assign **.stack_size(83886 * 1024)** here if you want to be serious with your chat app later.(You can search "How many resources chat app need" at your search engine.)

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

They are to serve routes and init your app with them.

The important part here will be **static_files** and **ws_rs**.

When you see the source code for static_routes first,

```rust
use std::path::{Path, PathBuf};
use rocket::response::NamedFile;

#[get("/static/<file..>")]
pub fn file(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(file)).ok()
}
```

It is just to serve every files in static folder. You can just copy and paste it when you need to serve every files in static folder.

(You can use simplified API in newer version of [Rocket] if you want.)

for **ws_rs** route, important part is

```rust
#[get("/chat")]
pub fn chat() -> io::Result<NamedFile> {
    NamedFile::open("static/chat/index.html")
}
```

and it is to serve **HTML** files for chat app.

For example, https://www.yourwebsite.com/chat

The file will help users to connect socket and have their separtate data with JavaScript and brwoser API later. We will learn how to do that in the last part of this post.

You can refer to [Steadylearner Chat] and [Rocket] Documentation for more information.

<br />

## 2. ws-rs for socket server

In this part, we will learn how to build server socket with Rust.

If you see the code snippet for [ws-rs](https://github.com/steadylearner/Chat/blob/master/ws_rs_with_rocket/src/chat/ws_rs.rs), it will be similar to this and not so different to [ws-rs-html-example] from its official website.

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
                // Uncomment this and find what you can do with them when you develope
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

The code snippet is a little bit long but the important parts will be

1. [You need on_request part just once and you don't have to reconnect later](https://blog.stanko.io/do-you-really-need-websockets-343aed40aa9b).

2. Use them to verify what you can do when the first socket connection between server and client happen and read [the documenation for them](https://ws-rs.org/api_docs/ws/struct.Request.html)

3. We need to count how many connections there are because it affects connection quality. You may use the `number_of_connection` variable with conditional statement.(We will write code for that in client side later. You may use your own code.)

4. **This is the most important part.** Even though we use localhost first and not real users, there should be some ways to differenciate the users from one another. So We will use return value of `&handshake.peer_addr.unwrap()` for the purpose and also `number of connection` inside `fn on_open`. (If you open various windows for http://localhost:8000/chat later, You can see that it always return different values in your CLI.)

5. This is where you can do various things with messages from users. You can use database to save messages from users here. You may write experimental code, for example, to send **warning** received from other users to everyone connected to the server socket.(You may test it with **!warn** in socket client later.)

6. `self.out.broadcast(message)` used to send messages to all users. It is the last API used before the messages from the server arrive to clients connected to socket.

7. `self.count.set(self.count.get() - 1)` is used to recalculate the total number of user when some client close the connection.

I hope it helped you to understand this code snippet. You will need to find what are the uses of [Sender](https://doc.rust-lang.org/std/sync/mpsc/struct.Sender.html), [Rc](https://doc.rust-lang.org/std/rc/index.html) and [Cell](https://doc.rust-lang.org/std/cell/index.html) on your own if you want to understand it fully.

It is also important for you to understand that the most of the code used above are relevant to the JavaScript client code we will use later.

(It is important to think that Frontend and Backend code all in one.)

<br />

## 3. HTML and CSS for layout

![Rust Chat App](https://www.steadylearner.com/static/images/post/chat/rust_chat.png)

We briefly learnt the Rust serverside code to build our chat app. It is time to build Frotnend part to help users.

If you see the [index.html](https://github.com/steadylearner/Chat/blob/master/ws_rs_with_rocket/static/chat/index.html) file, The important part will be

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

The main points here are

1. They are CSS files for layout. You can edit or use your own if you want.

2. We will use various ids inside html file to select and manipulate them with JavaScript later.(We will use id to make our simple Rust chat app development easy without Frontend framework)

3. The chat message will be written here inside `<li>` wrappers and it is important to know that they are under `<ul id="messages" >` to be deleted easily later with JavaScript `removeMessages` function later.

4. We will use [browserify] to bundle our JavaScript file with NPM modules later. It will help us to use emoji .

They are to be used with JavaScript and layout and when you run your app later it will be similar to the image you saw.

(**steadylearner.css** above is especially used for React and [prop-passer] package that I wrote, it is just the bunch of class names that does one thing at a time. I want you not to be confused with many CSS classes when you see the source code. You can verify how they work better at [Steadylearner])

<br />

## 4. JavaScript for chat app users

We prepared a lot to do something interesting with our Rust chat app. In this part, we will write a JavaScript code that will help users to use it in there browser.

If you already have experience with chat app. The part 1. and 2. is already sufficient for you to start your Rust chat app.

So before you read on I want you know the two points.

1. I haven't written code for chat app before this post. So please use this just for reference.

2. I decided not to use Frontend Frameworks here because I wanted to find that I can write something useful without them.

Therefore, the code below may not be well-organized and want you to use framework if you want to make it advanced with database, login, send and receive video and image etc.

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
  //   alert("You sent warning to the other users");
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

    // We pick the first(id for user and will be You with JavaScript)
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

    p.textContent = `Your id is ${userId} and "You" will be used in this page instead`;
    p.className = "blue";
    li.append(p)
    messages.append(li);
    return;
  } else {
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

    // Give color and "You" for a user when author of the messages is the user.
    if (authorOfMessage === userId) {
      li.className = "red-white";
      fromServer = fromServer.replace(userId, "You");
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
  const closeMessage = event.data === undefined ? "Server, You or another user closed the connection." : "WebSocket is closed now."
  const messages = document.getElementById("messages");

  const li = document.createElement("li");
  li.append(closeMessage)
  li.className = "blue";
  messages.append(li);

  localStorage.setItem("userInputs", `[${userInputs}]`);
  localStorage.setItem("server", `[${server}]`);
};
```

The code snippet is a little bit long and I will explain only important parts here.

1. We import modules you need later to use emojis in your chat app and connect your client to the web socket server with `new WebSocket("ws://127.0.0.1:7777/ws");`.(You can test it in your browser later)

2. We define custom functions to help log time when the user send messages and remove messages. Then we make the default state for client that we will manipulate with JavaScript later.

3. We assign roles for HTML code with id **clear** and **exit** we wrote in **index.html** before. It won't be difficult to understand what they do and you can find the codes that does similar things in **4.** .

4. We find HTML element with id **form** with `document.getElementById`. Then we define what should happen when users type to it. We save the user input with time(**userInputs.push(userInputWithTime);**) and send it to the server with `socket.send`.(You can see that you can write some features before user input is sent to the socket server such as "clear" and "exit" here.)

5. **This is the most important**. We defined some variables in **2.** and we can use it to assign **id** to user with **let separate = event.data.split(" ");** and **userId = separate[0];**. Then, verify the user is already connected to server or not with JavaScript(socket is open or not). You can see that we turn **open = false;** to **open = true;** inside it and the client side code will execute code inside **if(!open)** only once.

6. We didn't write code what to do when there are more users than allowed before at Rust server side. So we make client to leave the connection with JavaScript here.

7. Use this part to allow users to type emojis easily. Please, read the doucmenation for them.([node-emoji], [has-emoji])

8. When socket closes, we notify users that the socket connection is closed and save messages from the user, other users and server to the localStorage.(You can use your database API instead.)

You can modify and bundle them with [Browserify] `$browserify index.js > bundle.js` after you install them with `$sudo npm install -g browserify`.

Then you can run your chat app with `$cargo run` and verify the result and test them with various windows open while you type commands defined here and click the components.

<br />

## 4. Conclusion

I hope the post was helpful to start your own chat app with Rust. It may not be sufficient to call it complete chat application. But it would be a starting point to write chat app with Rust.

It was also my first trial to write chat app. So please contribute to Steadylearner Chat repository or Steadylearner Post for this blog post if you find something to improve.

If you want to know someone who wants to improve his coding skill in Rust, JavaScript and Python everyeday, please contact me with [LinkedIn] and [Twitter]

(I would be grateful also if someone give me a chance to know who can help me to imporve Rust code skill for it is difficult to find one here.)

I am planning to convert Frontend code(HTML and JavaScript) used here to Rust with one of its web frameworks later and may write post for that also.

We might learn how to write Rust code for Frontend and Backend code later with this example.

**Thanks and please share this post with others**
