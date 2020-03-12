<!--
    Post{
        subtitle: "Include Rust components only visible in specific conditions",
        image: "post/web/Rust-Modal-by-Steadylearner.png",
        image_decription: "Image by Steadylearner",
        tags: "How Rust model code",
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
[How to write Full Stack Rust Code]: https://www.steadylearner.com/blog/read/How-to-write-Full-Stack-Rust-code

[How to use Python in JavaScript]: https://www.steadylearner.com/blog/read/How-to-use-Python-in-JavaScript

<!-- / -->

<!-- Steadylearner Twitter and LinkedIn  -->

[Twitter]: https://twitter.com/steadylearner_p
[LinkedIn]: https://www.linkedin.com/in/steady-learner-3151b7164/

<!--  -->

In the previous post [How to write Full Stack Rust Code], we learnt how to connect Rust server and client code and complete your Rust app.

In this post, we will learn how to write code to build modals in Rust frontend. Read this post briefly and apply it to more frontend components you already had. 

If you want to save your time, clone [Rust Full Stack] and follow the instruction there. Then, test it on your own. 

<br />

<h2 class="red-white">[Prerequisite]</h2>

1. [How to install Rust]
2. [Yew]
3. [Fullstack Rust with Yew]
4. [How to modulize your Rust Frontend]
5. [How to write Full Stack Rust Code]

---

I want you already have Rust installed in your machine. The blog post [How to install Rust] will help you for that.

If you haven't setup development environment for [Yew], please read the previous post [How to use Rust Yew]. Then, visit [Fullstack Rust with Yew] and [How to use NPM packages with Rust Frontend].

I hope you already read the previous [Rust blog posts] and especially [How to write Full Stack Rust Code]. The entire code used here is based on that post and [How to modulize your Rust Frontend] will also help you.

You will also need to know **CSS** and [how to include it in your Rust full stack project][How to use markdown with Rust Frontend].

If you could build your [Rust Full Stack] project, you can deploy it with [How to deploy Rust Web App].

<br />

<h2 class="blue">Table of Contents</h2>

1. CSS 
2. State
3. Component
4. **Conclusion**

---

<br />

## 1. CSS

We already had various CSS files in **static** folder. We will write **modal.css** in it. 

It will be similar to 

```css
@keyframes modal {
    0%{
        transform: scale(0);
        opacity: 0;
    }
    1% {
        display: flex;
        opacity: 0;
    }
    100% {
        transform: scale(1);
        opacity: 1;
    }
}

.modal {
    position: fixed;
    z-index: 1;
    left: 0;
    top: 0;
    width: 100vw;
    height: 100vh;
    background-color: rgba(0, 0, 0, 0.7);
    animation-duration: 0.6s;
    animation-name: modal;
}

.modal-content--image {
    margin: 5.8rem auto auto;
    display: flex;
    max-width: 100%;
    height: calc(100% - 11.6rem);
}
```

It is from [Steadylearner] for an image modal. You may use yours.

The important parts are

**1.** Separte CSS code for **modal wrapper with animation** and **its content**. It will help you to easily include more components in your modal later.

**2.** We use **modal.css** and will write Rust file with name **modal.rs** etc. It will help you to compare those files more easily.

<br />

## 2. State

We already have CSS for animation and visual part of our Rust frontend modal.

Then, we need code to define its state and control and show it to users.

We will start with **modal.rs** and will be similar to

```rust
// 1.
#[derive(Debug)]
pub struct Modal {
  pub show: bool,
  pub location: String,
}

// 2.
impl Default for Modal {
  fn default() -> Self {
    let modal = Self {
      show: false,
      location: "".to_string(),
    };
    modal
  }
}
```

We separate it from **state.rs** as we did with **ws_rs.rs** in the preivous [Rust blog posts] because they are not relevant to the main state of your app.

You can see in **1.** that its state need only a few parts. In **2.** we used **impl Default** to write less code in **lib.rs**.

You will need simple code similar to this after you write code to import **modal.rs** in it.

```rust
    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let modal = Modal::default();

        Model {
            modal,
        }
    }
```

That is all for the state of our modal.

You can see that we just need to call **Modal::default();** to use it because we already prepared it in modal.rs file.

We already have various files to handle the state of your app. You may build **state** folder and include them there and refactor your app.

<br />

## 3. Component

What left are Rust code to control the state of your modal and show it to users.

For that purpose, we need to write some more codes in **lib.rs** first. Then we will write **modal.rs** in **components** folder.

The paylodas will be

```rust
pub enum Msg {
    Set(String)
}

fn update(&mut self, msg: Self::Message) -> ShouldRender {
    match msg {
        Msg::Set(val) => {
            if val.is_empty() {
                self.modal.show = false;
                self.modal.location = "".to_string();
                self.console.log("The user wants to close the modal");
            } else {
                self.modal.show = true;
                self.modal.location = val;
                self.console.log("The user wants to use the modal");
            }
        }
    }
    true
}

impl Renderable<Model> for Model {
    fn view(&self) -> Html<Self> {
        let Modal { show, location } = &self.modal;

        html! {
            <ImageModal: show={show}, location={location}, onsignal=Msg::Set, />
        }
    }
}
```

and it won't be difficult to find what they do.

We just wrote simple instructions for our modal. Then, we pass it to **ImageModal** component we will made with Rust [Yew] syntax.

If you need more information, please refer to [Rust Full Stack] repository or [How to modulize your Rust Frontend].

For we already separated state of our modal, you can reuse those logics in other project also.

Then, we will write **components/modal.rs**.

Its payload will be

```rust
pub enum Msg {
    Set, // 1.
}

impl Renderable<ImageModal> for ImageModal {
    fn view(&self) -> Html<Self> {
        let mut modal_class = "modal ".to_string(); // 2.

        // 3.
        if self.show {
            modal_class.push_str("inherit-display");
        } else {
            modal_class.push_str("x-display");
        };

        // You can test with whatever image you want
        let src = "https://www.steadylearner.com/static/images/brand/code.png".to_string();

        // 4.
        let shadow_class="width-vw height-vh cursor-pointer absolute";
        // 5.
        let image_class="max-width box-shadow-white relative modal-content--image";

        html! {
            <section 
                class=modal_class, 
                id="modal", 
            >
                <section 
                    title="Click this to close modal.", 
                    class=shadow_class, 
                    onclick=|_| Msg::Set, 
                /> // 6. 
                <img class=image_class, src=src, />
            </section>
        }
    }
}
```

The majority of CSS parts are from **steadylearner.css**. You can refer to it in **static** folder and use yours instead.

You can compare **1.** with similar part in **lib.rs**. We don't need to type it with **Set(String)** here because it will be used just to call the callback function we passed with **onsignal=Msg::Set** before.

You can use **"modal "** instead of **"modal"** in **2.**. It will help you to write code similar to **3.** better.

In the first part we separated CSS for **the modal wrapper with animation** and **its component**(image here).

In **3.**, We write code for the wrapper for visibilty with Rust.

**4.** is to close the modal with **shadow_class**. Then, **5.** is the main part to show **content** of your modal.

Put your custom **modal-content--image** at the last part. This will help it to have priority to the other CSS names you have.

In **6.** I found that it compiles without / at the end >. So follow **the rule of api authors**(macro here) well.

Have a special caution when you write frontend code with various languages and frameworks.

<br />

## 4. Conclusion

I intentionally used **modal** for the file names for **CSS** and **Rust frontend state** and **component** in this post.

I want you to find all of them are used for the same purpose.

What we made is a simple modal. But, you should have learnt you can build **log in**, **sidebar** or whatever you want with the logic and the code example used here.

What you need are just

1. CSS for animation

2. Rust for logic instead of JavaScript

In the next [Rust blog posts], we will learn **how to use routers** for **Rust frontend**. If you want to save your time, refer to **before** folder at [Rust Full Stack] repository.

**Thanks and share this post with others.**