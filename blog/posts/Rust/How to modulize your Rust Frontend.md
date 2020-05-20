<!--
    Post{
        subtitle: "Learn how to use function and components in Yew",
        image: "post/web/modulize-code-with-yew.png",
        image_decription: "Image by Steadylearner",
        tags: "How Rust Yew Frontend",
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

[React Easy Markdown]: https://github.com/steadylearner/react-easy-md/blob/master/src/MarkdownPreview.js
[Marked]: https://github.com/markedjs/marked

<!-- / -->

<!-- package.json

cargo web start(include build), cargo web deploy
yarn watch:rs for devlopment then yarn prod(include build) for production

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

[How to use Python in JavaScript]: https://www.steadylearner.com/blog/read/How-to-use-Python-in-JavaScript

<!-- / -->

<!-- Steadylearner Twitter and LinkedIn  -->

[Twitter]: https://twitter.com/steadylearner_p
[LinkedIn]: https://www.linkedin.com/in/steady-learner-3151b7164/

<!--  -->

In the previous post [How to use markdown with Rust Frontend], we learnt how to render markdown in Rust frontend and include CSS files from your previous frontend project.

You can use a text, image, video, markdown or any HTML elements you want for your Rust frontend.

In this post, we will learn how to **modulize** your Rust [Yew] frontend app. It will be easy after you learn how to use **impl**, **function**, and **components** for that.

You can find examples for them at [Rust Full Stack] or [Yew Examples].

If you want to save your time, clone [Rust Full Stack] repository, use its bash files to install depenedencies and test it on your own.

<br />

<h2 class="red-white">[Prerequisite]</h2>

1. [How to install Rust]
2. [Yew]
3. [How to use Rust Yew]
4. [Fullstack Rust with Yew]

---

I want you already have Rust installed in your machine. The blog post [How to install Rust] will help you for that.

If you haven't setup development environment for [Yew], please read the previous post [How to use Rust Yew]. Then, visit [Fullstack Rust with Yew] and [How to use NPM packages with Rust Frontend].

I hope you already read the previous [Rust blog posts]. They will help you to find this post better.

Read [Yew Custom Components example] first. You can learn how to use **components** with it.

If you could build your [Rust Full Stack] project, you can deploy it with [How to deploy Rust Web App].

<br />

<h2 class="blue">Table of Contents</h2>

1. impl
2. Functions
3. Components
4. **Conclusion**

---

You can read only parts that interests you. This post is just to help others to write [Rust Full Stack] code.

<br />

## 1. impl

You can find **impl Model** or **impl State** are used many times in [Yew Examples].

It will be the starting point to have less code in

`impl Renderable<Model> for Model` part in your **lib.rs**.

For exmaple,

```rust
impl Renderable<Model> for Model {
    fn view(&self) -> Html<Self> {
        html! {
            <section>
                // <input
                //     value=&self.state.value,
                //     oninput=|e| Msg::Update(e.value),
                //     onkeypress=|e| {
                //         if e.key() == "Enter" { Msg::Submit } else { Msg::Nope }
                //     },
                // />

                // or

                { self.chat_input() }
            </section>
        }
    }
}

impl Model {
    fn chat_input(&self) -> Html<Model> { // (&self) is used here
        html! {
            <input
                value=&self.state.value, // to use &self for functions inside impl Model
                oninput=|e| Msg::Update(e.value),
                onkeypress=|e| {
                    if e.key() == "Enter" { Msg::Submit } else { Msg::Nope }
                },
            />
        }
    }
}
```

You can test your Rust code for HTML first in

`impl Renderable<Model> for Model` then move it to **impl Model**.

It will be the best solution when

1. Before you learn how to use function and components or make them
2. What you want is a complete single file example.
3. You don't want to care for the ownership and lifetime of variables to modulize the project.
4. You like **impl** or object oriented programming.

But it will be better to have less lines of code in a single file. It will help you to find errors of your code and edit it better.

Learn how to use it more with [Yew Examples] before you use functions and components. What they do are the same.

<br />

## 2. Functions

They will be the best solution when what you want is to render **static HTML elements**. You can find examples at [Rust Full Stack].

For example,

```rust
// /web/src/components/website.rs
use yew::{html, Html};
use crate::Model; // You will need it because they will be used for Model struct

pub fn steadylarner_blog() -> Html<Model> {
    html! {
        <a
            target="_blank", rel="noopener noreferrer",
            href="https://www.steadylearner.com/blog/search/Rust",
        >
            <span>
                <img
                    src="https://www.steadylearner.com/static/images/code/Rust.svg",
                />
                { "Full Stack Rust Chat App" }
            </span>
        </a>
    }
}

// https://github.com/steadylearner/code/blob/master/src/metatag/index.js
pub fn social() -> Html<Model> {
    html! {
        <>
            <title>{ "Full Stack Rust Chat App" }</title>
            <meta name="description", content="Rust Full Stack Website by Steadylearner", />
            <meta name="thumbnail", content="https://avatars0.githubusercontent.com/u/32325099?s=460&v=4", />
        </>
    }
}
```

You can see that you could use them to render **link** and **meta tags**. They don't need specific methods and they will be always similar with static datas. So you could separate them from the **lib.rs** file.

You can use them with ease. For example,

```rust
// /web/src/lib.rs

mod components;
use self::{
    components::{
        website::{
            steadylarner_blog,
            social,
        }
    },
};

impl Renderable<Model> for Model {
    fn view(&self) -> Html<Self> {
        let State { responses, message_type, connection } = &self.state;
        html! {
            <>
                { social() }
                { steadylarner_blog() }
            </>
        }
    }
}
```

They also became more reusable and you could copy and paste them in other projects.

It is not difficult to use them instead of **impl**. You can also use functions with arguments for the same purpose. You just need to define types for them before you use. They are just **Rust functions** that return **HTML elements**.

For example, in **/web/src/lib.rs** you can find

```rust
{
    for responses
    .iter()
    .enumerate()
    .map(|(idx, response)| {
            let deserialized: WebSocketRequest = serde_json::from_str(&response).unwrap();
            view_message(
                // 1.
                &idx, // : &usize
                &deserialized.value, // : &str
                &deserialized.message_type // &str
            )
        }
    )
}
```

and **/web/src/components/message.rs**

```rust
pub fn view_message(idx: &usize, value: &str, message_type: &str) -> Html<Model>
```

It is easy to build it when you know how to use types in your Rust project.

What you need are

1. **payload** to render HTML elements

2. **built in Rust types** for its arguments

<br />

## 3. Components

In the first part, you already read simplified code for **chat_input** to learn how to use **impl**. We will make it to a **ChatInput** component and you could apply it to write other components also.

Please, read it briefly with notion for `<Props>` and `<Methods>` in comments.

It will be similar to this and you will find the pattern.

```rust
// /web/src/components/chat_input.rs
use yew::events::IKeyboardEvent;
use stdweb::js;

pub struct ChatInput {
    // <Prop and its own state>
    value: String, // you can use the data only for this component also
    disabled: bool,
    onsignal: Option<Callback<(String)>>, // 1.
    // </Prop and its own state>
}

pub enum Msg {
    // <Method>
    Update(String),
    Submit,
    Nope,
    // </Method>
}

#[derive(PartialEq, Clone)]
pub struct Props {
    // <Props>
    pub disabled: bool,
    pub onsignal: Option<Callback<(String)>>,
    // </Props>
}

impl Default for Props {
    fn default() -> Self {
        // <Props>
        Props {
            disabled: false,
            onsignal: None,
        }
        // </Props>
    }
}

// https://docs.rs/yew/0.6.0/yew/html/trait.Component.html
impl Component for ChatInput {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        // <Prop and its own state>
        ChatInput {
            value: "".to_string(),
            disabled: false,
            onsignal: props.onsignal,
        }
        // </Prop and its own state>
    }

    // This is for methods
    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        // <Method>
        match msg {
            Msg::Update(val) => {
                self.value = val;
            }
            Msg::Submit => {
                if let Some(ref callback) = self.onsignal { // 2.
                    let message = self.value.clone();
                    callback.emit(message); // 3.
                    // self.value.clear(); work with impl but not here with components
                    js! { // 4.
                        setTimeout(() => {
                            document.querySelector("#chat-input").value = "";
                            window.scrollTo({ top: window.innerHeight, behavior: "auto" });
                            // temporary solution, use number you like or find other ways
                        }, 10);
                    }
                }
            }
            Msg::Nope => {}
        }
        // </Method>
        false
    }

    // This is for props
    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        // <Props>
        self.disabled = props.disabled;
        self.onsignal = props.onsignal;
        // </Props>
        true
    }
}

impl Renderable<ChatInput> for ChatInput {
    fn view(&self) -> Html<Self> {
        html! {
            <input
                disabled=self.disabled,
                value=&self.value,
                oninput=|e| Msg::Update(e.value),
                onkeypress=|e| {
                    if e.key() == "Enter" { Msg::Submit } else { Msg::Nope }
                },
            />
        }
    }
}
```

I hope you could find the pattern. You write the similar codes for the same thing.

The **html!** part will be almost same when you use it with **impl** with a little difference.

[Rust Full Stack] already have many examples for this. But a little help will be useful.

1. Syntax to use **callback** function you pass in **lib.rs**.

2. You can see that this was used to make **1.** work.

3. This is **payload** for the function you passed with other props.

4. You can use **js!** whenever you need it.

Before you make components, you should have tested it with **impl Model** and have **lib.rs** file for them.

It will be similar to

```rust
// /web/src/lib.rs
pub enum Msg {
    // <Methods>
    Submit(String), // 1.
    // </Methods>
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _link: ComponentLink<Self>) -> Self {
        let state = State {
            // <Props>
            responses: Vec::new(),
            connection: true,
            // </Props>
        };

        Model {
            state,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        // <Methods>
        match msg {
            Msg::Submit(val) => {
                match val.as_ref() {
                    _ => {
                        self.state.responses.push(val);
                    }
                }
            }
        }
        // </Methods>
        true
    }
}

impl Renderable<Model> for Model {
    fn view(&self) -> Html<Self> {
        let State { responses, connection } = &self.state;
        html! {
            <ChatInput: disabled={!*connection}, onsignal=Msg::Submit, />
        }
    }
}
```

and you will already have made methods and the datas to pass for your component in your **lib.rs**.

Use them for your [Yew] components. For exmaple,

```rust
<ChatInput: // 2.
    disabled={!*connection},
    onsignal=Msg::Submit, // 1.
/>
```

and you should care for

**1.** We made

```rust
pub enum Msg {
    Submit(String)
}
```

but we could pass the prop **Msg::Submit** without (String) part. Then, we wrote code to use it in the **ChatInput** component.

**2.** It is easy to forget to use : and , in your Yew component and help Rust compiler.

<br />

## 4. Conclusion

I want you learnt how to modulize your Rust Frontend project with this post. You could use **impl**, **functions** and **components** for that purpose and others will be up to your Rust skill.

It won't be easy when you start. But you already have [Rust Full Stack] and [Yew Examples] to help you.

**We can write frontend code with Rust**.

In the next [Rust blog posts], we will learn **how to connect Rust server side and frontend part**. We will include **Websocket** for our project and make local chat app with it. You can refer to [How to start Rust Chat App] first.

Then, we may challenge the phrase **'JavaScript is the only programming language that can be used to write both server and frontend code.'**.

If you want the latest contents from [Steadylearner], you may follow me at [Twitter].

**Do you need a Full Stack Rust and JavaScript Developer**? You may contact me with [LinkedIn] or be one of them.

**Thanks and please share this post with others.**