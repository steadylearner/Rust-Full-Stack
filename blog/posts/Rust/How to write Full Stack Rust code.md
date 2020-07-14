<!--
    Post{
        subtitle: "Learn how to connect server and client with Rust",
        image: "post/web/full-stack-rust-chat-app-by-steadylearner.png",
        image_decription: "Image by Steadylearner",
        tags: "How write Rust code",
    }
-->

<!-- Link -->

[Steadylearner]: https://www.steadylearner.com
[Steadylearner CSS]: https://github.com/steadylearner/code/blob/master/CSS/
[Steadylearner Web]: https://github.com/steadylearner/Webassembly
[Rust Website]: https://www.rust-lang.org/
[Cargo Web]: https://github.com/koute/cargo-web
[stdweb]: https://github.com/koute/stdweb
[Yew]: https://github.com/DenisKolodin/yew
[Yew Documenation]: https://docs.rs/yew/0.6.0/yew/
[Yew Service]: https://github.com/DenisKolodin/yew/tree/master/src/services
[Yew Examples]: https://github.com/DenisKolodin/yew/tree/master/examples
[Yew NPM example]: https://github.com/DenisKolodin/yew/tree/master/examples/npm_and_rest
[Yew inner HTML example]: https://github.com/DenisKolodin/yew/blob/master/examples/inner_html/src/lib.rs
[Yew Custom Components example]: https://github.com/DenisKolodin/yew/tree/master/examples/custom_components/src

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
[ws-rs]: https://github.com/housleyjk/ws-rs
[serde]: https://serde.rs/derive.html

[React Easy Markdown]: https://github.com/steadylearner/react-easy-md/blob/master/src/MarkdownPreview.js
[Marked]: https://github.com/markedjs/marked

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

[How to use Python in JavaScript]: https://www.steadylearner.com/blog/read/How-to-use-Python-in-JavaScript

<!-- / -->

<!-- Steadylearner Twitter and LinkedIn  -->

[Twitter]: https://twitter.com/steadylearner_p
[LinkedIn]: https://www.linkedin.com/in/steady-learner-3151b7164/

<!--  -->

In the previous post [How to modulize your Rust Frontend], we learnt how to use **impl**, **functions** and [Yew] components. They help you to find errors and organize your Rust frontend project.

In this post, we will include server side code with [ws-rs]. It will help us to build complete [Rust Full Stack] chat app similar to what we made at [How to start Rust Chat App].

You will find writing full stack Rust code is not difficult. You can even just **copy and paste** [server side Rust code](https://github.com/steadylearner/Rust-Full-Stack/blob/master/server/src/http_model/websocket_json.rs) to [Rust frontend](https://github.com/steadylearner/Rust-Full-Stack/blob/master/web/src/http_model/websocket_json.rs) also.

**You will verify that they work.**

<br />

<h2 class="red-white">[Prerequisite]</h2>

1. [How to install Rust]
2. [Yew]
3. [ws-rs]
4. [serde]
5. [How to start Rust Chat App]
6. [How to use Rust Yew]
7. [Fullstack Rust with Yew]

---

I want you already have Rust installed in your machine. The blog post [How to install Rust] will help you for that.

If you haven't setup development environment for [Yew], please read the previous post [How to use Rust Yew]. Then, visit [Fullstack Rust with Yew] and [How to use NPM packages with Rust Frontend].

I hope you already read the previous [Rust blog posts] and especially [How to start Rust Chat App]. They will help you to find this post better.

We will use [serde] to serialize and deserialize many times in both server and client and [ws-rs] to build **websocket server** and connect it from client.

The source code of [The simulated Rust chat app without web socket](https://github.com/steadylearner/Rust-Full-Stack/blob/master/web/before/component/) will help you if you find the current [Rust Full Stack] code complicated or want example wihtout websocket.

If you could build your [Rust Full Stack] project, you can deploy it with [How to deploy Rust Web App].

<br />

<h2 class="blue">Table of Contents</h2>

1. **websocket_json.rs** to connect server and client
2. WebSocket Server with [ws-rs]
3. Client with Rust
4. **Conclusion**

---

<br />

## 1. websocket_json.rs to connect server and client

[![Rust equivalent server and client code](https://www.steadylearner.com/static/images/post/web/client-server-equal-rust-code.png)](https://www.steadylearner.com/static/images/post/web/client-server-equal-rust-code.png)

For our goal is to write [Rust Full Stack] code, we need files to help you to use **Rust** in both server and frontend.

We will write **websocket_json.rs** in **server/src/http_model** folder.

You can also find equivalent code in **web/src/http_model**.

```rust
// https://serde.rs/derive.html
use serde_derive::{Deserialize, Serialize};

// Into WebSocket - Message
#[derive(Serialize, Deserialize, Debug)]
pub struct WebSocketRequest {
    pub value: String, // 1.
    pub message_type: String, // 2.
    pub client: Option<String>, // 3.
}

// From WebSocket Server - Broadcast
#[derive(Serialize, Deserialize, Debug)]
pub struct WebSocketResponse {
    pub value: String,
    pub message_type: String,
    pub client: Option<String>,

    // from websocket
    pub number_of_connection: u32, // 4.
}
```

In **WebSocket client**, We use

1. **WebSocketRequest** to send the message from **the user**.

2. WebSocketResponse to receive the messages from **all the users connected to the server**.

It will be vice versa in **WebSocket server**.

I hope you read and test the entire [Rust Full Stack] project to find how they work better.

We will only handle why we use each data types here for simplicity.

1. The client will send the data in text format and server will interpret it before it send it to all the clients connected to it.

2. You can send a text, image, video and code separately with it and each message will have their own type when they are sent by users or server.

3. Before clients connect to websocket, there is no data(None) for them. So use this type.

4. Only server have information for it and they send it for users to use.

You can use Serialize or Deserialze only depending on the location of this file(inside server or web folder). We will not care for it to help you copy and paste them easily between server and client.

Later, we will use [serde] and its api to send and receive datas with them.

You should **serialize** when you send the data and **deserialize** when you want to use its members.

<br />

## 2. WebSocket Server with ws-rs

If you compare **ws_rs.rs** from [How to start Rust Chat App] and [Rust Full Stack], you will find that there are a few differences.

Its payload will be similar to

```rust
use super::super::http_model; // 1.

use self::{
    http_model::{
        websocket_json::{
            WebSocketRequest,
            WebSocketResponse,
        },
    },
};

impl Handler for WebSocket {
    fn on_open(&mut self, handshake: Handshake) -> Result<()> {
        let client = handshake.peer_addr.expect("Websocket failed to locate the client").to_string();

        let ws_resp = WebSocketResponse {
            value: format!("I want to use a chat app completely built in Rust.").to_string(),
            message_type: "text".to_string(), // 2.
            client: Some(client), // 3.
            number_of_connection,
        };

        // 4.
        println!("{:#?}", &ws_resp);
        let serialized = serde_json::to_string(&ws_resp).unwrap();
        println!("serialized = {}", serialized);

        self.out.broadcast(serialized)
    }

    fn on_message(&mut self, message: Message) -> Result<()> {
        let number_of_connection = self.count.get();

        let raw_message = message.into_text()?;
        println!("The message from the client is {:#?}", &raw_message);

        // 5.
        let ws_request: WebSocketRequest = serde_json::from_str(&raw_message).unwrap();
        println!("ws_request = {:?}", &ws_request);
        let WebSocketRequest { value, message_type, client } = ws_request;

        let client = if value == "!clearall" {
            None
        } else {
            client
        };

        let ws_resp = WebSocketResponse {
            value,
            message_type,
            client,
            number_of_connection,
        };

        // 6.
        println!("{:#?}", &ws_resp);
        let serialized = serde_json::to_string(&ws_resp).unwrap();
        println!("serialized = {}", serialized);

        self.out.broadcast(serialized)
    }
}
```

We won't care for much with **on_request**, **on_close** and **on_error** and it will be the same for the frontend client part later.

You shouldn't edit code at **on_request** if you are not expert with this. Just use the code the author of [ws-rs] prepared for its users.

Both **on_close** and **on_error** part will be only useful when you already have code for **the real users**.

It will be much simpler to find what this file do without them.

**1.** We need it for **websocket_json.rs** we made before. The module system in **Rust** may confuse you. Then, you should verify you wrote **pub mod** or **mod** for lib.rs for the files you want to import and use **super** keyword for them.

It will be better with examples to find how they work. So refer to [Rust Full Stack] and find the documentations for it.

**2.** on_open is used when **WebSocket** server send the first message to its client. The default message type will be **text** in both server and client. You can edit this part more with login, the databases you have etc.

**3.** There is **no user part** to differenciate one from another here yet.(What we want is just local chat app)

With the previous post [How to start Rust Chat App], you can solve it.

**4.** We made JSON data with **WebSocketResponse**.

We should **serialize** it with **serde_json::to_string(&ws_resp).unwrap();** api from [serde] before we send it to client. Then, we can use it in client with **deserialize** later with ease.

**5.** on_message will be the most important part for your [Rust Full Stack] chat app to work.

**serde_json::from_str(&raw_message).unwrap();** is used for the data from websocket clients. Then, you can write some handlers and conditional statements for them.

**6.** We need to serialize them again before we send it to clients.

It will be easy if you read and test [Rust Full Stack].

Because it is full stack project, edit it also whenever you write relevant codes at frontned.

It is the **single source of truth for messages user will receive and render in the next part**.

<br />

## 3. Client with Rust

We will improve the previous Rust frontend code from [Rust Full Stack]. You can find that it is not that simple as it was in [How to use Rust Yew] or [Yew Counter].

Therefore, we will care for only important files for this post. They will be **lib.rs**, **state.rs**, **ws_rs.rs**, **message.rs** and websocket_json.rs.

We already know that **websocket_json.rs** file is equal in both Rust server and client code. So we don't need to handle it.

You can compare them to each part of ws_rs.rs in **server** folder and what you read before. We will start with **state.rs**.

```rust
#[derive(Debug)]
pub struct State {
  pub ws_responses: Vec<Option<String>>, // 1.
  pub message_type: String, // 2.
  pub client: Option<String>, // 3.
}

// 4.
impl State {
    pub fn lost(&mut self) {
        self.client = None;
    }
}
```

**1.** We need to save the responses from the server and learn how to render them with one of [Yew Examples] later.

**2.** You can use this to each message the client send to the WebSocket server to have different message_type("text", "image", "video", "code").

**3.** Before users connect to the websocket, there will be no data about client. So use it with None and Some("client").

**4.** I let this **impl** parts in **state.rs and ws_rs.rs** to make **lib.rs** file simpler. We use this instead of function because you won't need to reuse it in other parts.

Find what they do.

Then, **ws_rs.rs** will be similar to

```rust
use yew::services::websocket::{WebSocketService, WebSocketTask};
use yew::services::Task;
use yew::format::{Json};

use super::{
    http_model::{
        websocket_json::{
            WebSocketRequest,
        }
    },
};

pub struct WebSocket {
    pub ws_service: WebSocketService,
    pub ws: Option<WebSocketTask>,
}

// 1.
pub enum WebSocketAction {
    Connect,
    Disconnect,
    Lost,
}

// 2.
impl WebSocket {
    pub fn connect(&mut self, task: WebSocketTask) {
        self.ws = Some(task);
    }

    // 1.
    pub fn send(&mut self, test: Json<&WebSocketRequest>) {
        self.ws.as_mut().unwrap().send_binary(test);
    }

    pub fn disconnect(&mut self) {
        self.ws.take().unwrap().cancel(); // You can use it with Task
    }

    pub fn lost(&mut self) {
        self.ws = None;
    }
}
```

Separate **ws_rs.rs** file similar to **state.rs** to make the [Yew exmaple with websocket](https://github.com/DenisKolodin/yew/blob/master/examples/dashboard/src/lib.rs) simple.

You can see that there are a few differences.

**1.** There is no **Send** part here and use it inside **impl WebSocket**.

Find that **Connect**, **Disconnect**, **Lost** and its correspondent contents of **connect**, **disconnect** and **lost** function will be always similar.

You can easily use **send** when it is separated from **impl WebSocket**.

If you find the better way, please contribute to [Rust Full Stack].

**2.** You can compare it with code in **impl Handler for WebSocket** in **ws_rs.rs**.

We use those files to handle the state of user and websocket connection in the client.

The rest are **lib.rs** to use all others files we have and **message.rs** to render **messages** from websocket.

There are many files and the code snippets for them are not short. That is why we learnt [How to modulize your Rust Frontend].

We will handle simplified version of **lib.rs** first. You can briefly read it. Then, test it in your local machine.

```rust
pub struct Model {
    state: State,
    ws_rs: WebSocket,
    link: ComponentLink<Model>, // 1.
    console: ConsoleService,
    emoji: EmojiService,
}

pub enum Msg { // 2.
    WebSocketAction(WebSocketAction),
    WebSocketReady(Result<WebSocketResponse, Error>),
    Ignore,
}

impl From<WebSocketAction> for Msg { // 2.
    fn from(action: WebSocketAction) -> Self {
        Msg::WebSocketAction(action)
    }
}

const WEBSOCKET: &'static str = "ws://127.0.0.1:7777/ws";

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self { 
        // 3.
        let state = State {
            ws_responses: Vec::new(),
            message_type: "text".to_string(),
            client: None,
        };

        let ws_rs = WebSocket {
            ws_service: WebSocketService::new(),
            ws: None,
        };

        Model {
            state,
            ws_rs,
            link,
            console: ConsoleService::new(),
            emoji: EmojiService::new()
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::WebSocketAction(action) => {
                match action {
                    // 4.
                    WebSocketAction::Connect => {
                        let callback = self.link.send_back(|Json(data)| Msg::WebSocketReady(data));  
                        let notification = self.link.send_back(|status| {
                            match status {
                                WebSocketStatus::Opened => Msg::Ignore,
                                WebSocketStatus::Closed | WebSocketStatus::Error => WebSocketAction::Lost.into(),
                            }
                        });
                        let task = self.ws_rs.ws_service.connect(WEBSOCKET, callback, notification);
                        self.ws_rs.connect(task);
                    }
                    WebSocketAction::Disconnect => {
                        self.state.lost();
                        self.ws_rs.disconnect();
                        // self.console.log("No connection to WebSocket anymore");
                    }
                    WebSocketAction::Lost => {
                        self.state.lost();
                        // self.console.log("No connection to WebSocket anymore");
                        self.ws_rs.lost();
                    }
                }
            }

            // 5. 
            Msg::WebSocketReady(response) => { // payload, should edit here most of the time
                self.console.log("Websocket is ready. Start to chat with others.");
                let ws_response = response.map(|data| data).ok();

                let serialized = serde_json::to_string(&ws_response).unwrap(); // value in view_response
                self.console.log(&serialized);

                let ws_response: WebSocketResponse = serde_json::from_str(&serialized).unwrap();
                let WebSocketResponse { value, message_type, client, number_of_connection, } = ws_response;

                // should use login page or oauth later instead of this
                // and self.state.client = None when disconnect
                if self.state.client == None {
                    self.state.client = client;
                    let user = self.state.client.clone();
                    let ws_response = WebSocketResponse {
                        value: format!("Your id is {:#?} and {} in total for this page", &user.unwrap(), &number_of_connection),
                        message_type,
                        client: None,
                        number_of_connection,
                    };

                    let serialized = serde_json::to_string(&ws_response).unwrap();
                    self.state.ws_responses.push(Some(serialized));
                } else {
                    // write equivalent condtional for all users from server here server/src/chat/ws_rs.rs
                    match value.as_ref() {
                        "!clearall" => {
                            self.state.ws_responses.clear();
                        }
                        _ => {
                            self.state.ws_responses.push(Some(serialized));
                        }
                    }
                }
            }

            Msg::Ignore => {
                return false;
            }

            // Client

            // 6.
            Msg::Submit(val) => {
                match val.as_ref() {
                    "" => {}
                    "!clear" => {
                        // similar to !clearall in Msg::WebSocketReady(response)
                        self.state.ws_responses.clear();
                    }
                    "!exit" => {
                        // Equal to WebSocketAction::Disconnect
                        self.state.lost();
                        self.ws_rs.disconnect();
                    }
                    _ => {
                        let State { ws_responses: _ , message_type, client } = &self.state;

                        let emojified = self.emoji.emojify(val.to_string());

                        self.console.log(&emojified);

                        let message_type = message_type.clone();
                        let client = client.clone();

                        let request = WebSocketRequest {
                            value: emojified,
                            message_type,
                            client
                        };

                        self.ws_rs.send(Json(&request));

                        if &self.state.message_type != "text" {
                            self.state.message_type = "text".to_string();
                        }
                    }
                }
            }
            Msg::Type(val) => {
                self.state.message_type = val
            }
        }
        true
    }
}

impl Renderable<Model> for Model {
    fn view(&self) -> Html<Self> {
        let State { ws_responses, message_type, client: _ } = &self.state;
        let WebSocket { ws, ws_service: _ } = &self.ws_rs;
        html! {
            <>
                <Connect: disabled={ws.is_some()}, onsignal=|_| WebSocketAction::Connect.into(), />
                {
                    // 7.
                    for ws_responses
                    .iter()
                    .enumerate()
                    .map(|(idx, response)| {
                            let message = response.clone();
                            let deserialized: WebSocketResponse = serde_json::from_str(&message.unwrap()).unwrap();
                            let WebSocketResponse { value, message_type, client, number_of_connection: _, } = deserialized;
                            // To be explicit here, you can use other variable name or just pass &self.state.client
                            let user = self.state.client.clone();
                            view_message(
                                &idx,
                                &value,
                                &message_type,
                                &client,
                                &user,
                            )
                        }
                    )
                }
            </>
        }
    }
}
```

The important parts will be

**1.** Use **ComponentLink** to asynchronously update the yew component. We use messages from websocket server for that.

**2.** You can see that you already had **WebSocketAction** in **ws_rs.rs** file.

We code `impl From<WebSocketAction>` part and **from** function inside it.

It helps you to use **WebSocketAction::Connect.into()** in

`<Connect: onsignal=|_| WebSocketAction::Connect.into(), />`

Read [From trait](https://doc.rust-lang.org/std/convert/trait.From.html).

**3.** We start data for the **ws_rs.rs** and **state.rs** for the **Model** here. It is to render the entire app in **main.rs**.

**4.** WebSocketAction::Connect is the payload to connect your WebSocket client to WebSocket server. You won't need to edit code here and compare it with **ws_rs.rs** in **server** folder.

**5.** This is the payload to handle responses from WebSocket Server after connection to it.

You can write correspondent code for **on_message** in **ws_rs.rs** here. They are very similar except you give id to user with the response from the server when there is no id for the client yet.

Read [How to start Rust Chat App] for this if you haven't yet. You can see that the code here is more organized.

It is the **single source of truth for messages(server responses) user will receive in the client**. We will use it to render view in the next part**.

**6.** You can write some conditional statements here before the user send a message to the server. You can write code only available for the author of it.

If you are familar with system programming, you may think it is similar to **shell**. Then, websocket server will be **kernel** and clients will be processes.

**7.** This is to use **ws_responses** we defined in **state.rs**. You can see this many times in [Yew Examples].

There are already almost same variables **self.state.client** and **client**. We used them to make server and client easily compatible. But, you can use other variable names for them also.

That was all important part for **lib.rs** file. If you need more information, please read [Rust Full Stack] code and [Rust blog posts].

We yet have **message.rs** to render messages.

It will be similar to

```rust
pub fn view_message(
    _idx: &usize, 
    response: &str, 
    message_type: &str, 
    client: &Option<String>, 
    user: &Option<String>,
) -> Html<Model> {
    // 1.
    if !response.is_empty() {
        // 2.
        let message = match message_type {
            "image" => {
                view_image(&response)
            }
            "video" => {
                view_video(&response)
            }
            "code" => {
                view_code(&response)
            }
            _ => {
                view_text(&response)
            }
        };

        // 3.
        if let Some(client_id) = client {
            // 4.
            if let Some(user_id) = user  {
                if client_id == user_id {
                    html! {
                        <li>
                            { author() }
                            { message }
                        </li>
                    }
                } else {
                    html! {
                        <li class="red-white", >
                            { others(&client_id) }
                            { message }
                        </li>
                    } 
                }
            } else {
                html! {
                    { "" }
                }
            }
        } else {
            // 5.
            if let Some(_user_id) = user  { // Use this not to show any message when user leave the chat
                html! {
                    <li class="blue", >
                        { message }
                    </li>
                }
            } else {
                html! {
                    { "" }
                }
            }
        }
    } else {
        html! {
            { "" }
        }
    }
}
```

THe there are many **html! { { "" } }** part. It is to render nothing but be compatible with `HTML<Model>` return type.

**1.** We will return when responses are **""**(empty message). You may remove it if you think it is unnecessary or already handle them in other conditionals.

**2.** Each message has its own data type that was cloned from the client state when user send the message. The default is text and we use **view_text** function for that.

**3.** If the message is sent from the user. It will always with client(user_id). It is from the server when the response is without it.

**4.** When the client and the user_id is equal, use **You** instead.

**5.** When users are connected to server, they could see the message from it. Otherwise, they shouldn't be available to read any messages from it.

Write the code for database, user, login, redirect page and other business logics you want to include for your company. Then, use them instead of conditionals.

That was all to write full stack Rust code and build your [Rust Full Stack] chat app.

<br />

## 4. Conclusion

I intentionally repeated same words in this post.

Find how similar Rust frontend and server side code can be.

We can just copy and paste server side code to frontend or vice versa with **websocket_json.rs**.

**We can write frontend Rust code and its correspondent one in server**.

If you made [Rust Full Stack] project work in your machine, challenge the phrase 'JavaScript is the only programming language that can be used in both server side and frontend.'

**Rust** will be better at speed at least.

**We are already web**.

We will learn how to make **JSON web service** in server and how to use it in frontend with Rust. Please, read more [Rust blog posts].

If you want the latest contents from [Steadylearner], follow me at [Twitter].

**Do you need a Full Stack Rust and JavaScript Developer**? contact me with [LinkedIn] or be one of them.

**Please, share this post with others.**