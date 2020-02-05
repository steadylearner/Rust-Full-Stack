<!--
    Post{
        subtitle: "Render markdown files for your website with Rust Yew.",
        image: "post/web/rust-frontend-markdown-example.png",
        image_decription: "Image by Steadylearner",
        tags: "How markdown Rust Yew",
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

[Rust Blog Example]: https://github.com/steadylearner/Rust-Full-Stack/tree/master/web/before/rust_blog

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

[How to use Python in JavaScript]: https://www.steadylearner.com/blog/read/How-to-use-Python-in-JavaScript

<!-- / -->

<!-- Steadylearner Twitter and LinkedIn  -->

[Twitter]: https://twitter.com/steadylearner_p
[LinkedIn]: https://www.linkedin.com/in/steady-learner-3151b7164/

<!--  -->

In the previous post [How to use NPM packages with Rust Frontend], we learnt how to use JavaScript in Rust frontend project.

You could use both Rust crates and JavaScript modules whenever you want.

In this post, we will learn how to use **markdown with code snippets** in Rust frontend app. You will find we can render a text, video, image, code and whatever you want with Rust.

If you visit [Rust Full Stack], you will find that we already have examples for that. We will focus on **How to use markdown** here because it will show you can use other visual HTML elements with Rust also.

If you want to see the result first, please visit [Rust Blog Example] and its **code.rs**. It will help you to render blog posts with **fetch** and **mount** API from Yew.

<br />

<h2 class="red-white">[Prerequisite]</h2>

1. [How to install Rust]
2. [Yew]
3. [Yew inner HTML example]
4. [How to use Rust Yew]
5. [Fullstack Rust with Yew]
6. [How to use NPM packages with Rust Frontend]
7. [React Easy Markdown] or [Marked]

---

I want you already have Rust installed in your machine. The blog post [How to install Rust] will help you for that.

If you haven't setup development environment for [Yew], please read the previous post [How to use Rust Yew]. Then, visit [Fullstack Rust with Yew] and [How to use NPM packages with Rust Frontend].

Read more [Rust blog posts] and [Yew inner HTML example]. They will help you to find this post better.

We will use [Marked] NPM module in this post to render markdown files. You may read its source code or visit [React Easy Markdown] that we will directly use(copy and paste).

If you could build your [Rust Full Stack] project, you can easily deploy it with [How to deploy Rust Web App].

<br />

<h2 class="blue">Table of Contents</h2>

1. How to use **inner HTML** in Rust Frontend
2. Write Rust code to use **Marked**
3. Prepare **CSS** for it
4. **Conclusion**

---

You can skip the part for **CSS** if you want to use yours for markdown files.

<br />

## 1. How to use inner HTML in Rust Frontend

When we use JavaScript to use markdown files, we can render them with pure JavaScript with **inner HTML** DOM api and [Marked] NPM package.

Therefore, what we need to render markdown in Rust frontend will be also find

1. How to use **inner HTML** in it.

2. How to use [Marked] with it.

We already learnt [How to use NPM packages with Rust Frontend] so we just need to find how to solve **1.**.

We will find the hint for that with [Yew inner HTML example].

Its payload is similar to this

```rust
const SVG: &str = r#"<code>pass markdown value with &str later</code>"#;

impl Renderable<Model> for Model {
    fn view(&self) -> Html<Self> {
        let js_svg = js! {
            var div = document.createElement("div");
            div.innerHTML = @{SVG.to_string()};
            console.log(div);
            return div;
        };
        eprintln!("js_svg: {:?}", js_svg);
        let node = Node::try_from(js_svg).expect("convert js_svg");
        let vnode = VNode::VRef(node);
        eprintln!("svg: {:?}", vnode);
        vnode
    }
}
```

The **js!** macro from [stdweb] is used again.

The important part here is **JavaScript code** and others are just to make [FFI between JavaScript and Rust][How to use NPM packages with Rust Frontend].

It will be sufficient to find you can use **inner HTML** in Rust Yew or other Rust frontend with this.

<br />

## 2. Write Rust code to use Marked

We already have the working example for this at [Rust Full Stack] so we will handle only important parts.

To make [marked] work, you first need to include

```html
<script src="https://cdn.jsdelivr.net/npm/marked/marked.min.js"></script>
```

in your **/web/static/index.html**.

You can use [Browserify] or others also for [this purpose][How to use NPM packages with Rust Frontend].

Then, render markdown in **Rust frontend** with

```rust
// /web/src/components/code.rs
use stdweb::web::Node;
use stdweb::unstable::TryFrom;
use yew::{html, Html};
use yew::virtual_dom::VNode;

use crate::Model; // 1.

pub fn view_code(value: &str) -> Html<Model> {
    // 2.
    let markdown = js! {
        const div = document.createElement("div");
        div.className = "markdown"; // 3.
        const code = @{&value};

        const options = {
            gfm: true,
            tables: true,
            breaks: false,
            pedantic: false,
            sanitize: false, // 4.
            smartLists: true,
            smartypants: false,
            // 5.
            langPrefix: "hljs ",
            highlight: (code, lang) => {
                if (!!(lang && hljs.getLanguage(lang))) {
                    return hljs.highlight(lang, code).value;
                }

                return code;
            }
            //
        };

        marked.setOptions(options);

        div.innerHTML = marked(code);
        return div;
    };

    eprintln!("markdown: {:?}", markdown);
    let node = Node::try_from(markdown).expect("convert markdown");
    let vnode = VNode::VRef(node);
    eprintln!("div: {:?}", vnode);

    html! {
       { vnode }
    }
}
```

**js!** is payload for all of this process. We could just copy and paste some parts of code snippet from [React Easy Markdown].

The important parts are

1. Include **use crate::Model;** because it is not a component and used to render the main **Model** struct that represent the entire Rust Yew Frontend.

2. Those **JavaScript** variables in **js!** are not usable in JavaScript global scope(window etc) after this process.

3. You will want your custom CSS to render markdown. Include it similiar to **div.className = "markdown"**.

4. When you use **false**, it will allow HTML and otherwise won't.

5. It is to parse code snippets with hljs and give CSS names use CSS for them.

That was all.

You can test it with [Rust Blog Example] or **yarn watch:rs** in **web** directory after clone [Rust Full Stack].

For example,

```md
I can use **markdown** in my Rust frontend app.
```

in the input part of your app.

[![Rust full stack chat app](https://www.steadylearner.com/static/images/post/web/full-stack-rust-chat-app-by-steadylearner.png)](https://www.steadylearner.com/static/images/post/web/full-stack-rust-chat-app-by-steadylearner.png)

If you want to write more custom JavaScript code for your markdown renderer, please refer to [React Easy Markdown].

<br />

## 3. Prepare CSS for it

We will use [Steadylearner CSS] files used to render [Steadylearner]. You can use yours instead and refer to [Rust Full Stack] **/web/static** folder.

The CSS file to render markdown will be similar to

```js
const ContentCSS = `
    & > * {

    }
    // more CSS extension syntax here
`;

export default ContentCSS;
```

and used this syntax to include it in other React **Styled Component** CSS with ${ContentCSS} because it is easily reusable.

For example,

```js
import React from "react";
import styled from "styled-components";

import ContentCSS from "../../CSS/ContentCSS";

const BlogCSS = styled.article`
    .blog-contents {
        ${ContentCSS}
    }
`;

export default BlogCSS;
```

That was to help you to find how it was used in **React** app such as [Steadylearner].

If you are familiar with CSS or whatever its extensions, you could find it is just SCSS when it is separated from styled components API.

Copy only CSS relevant parts from the file before and write SCSS similar to

```scss
.markdown {
    & > * {

    }
    // more CSS extension syntax here
}
```

It has become SCSS file with **.markdown** class. We can use it to help **div.className = "markdown";** we made before in **web/src/components/code.rs** to work.

What you need next is to find how to convert them it to **CSS**.

[You can search on your own](https://www.google.com/search?q=sass+to+css+online) and use what you want.

Then, include all the relevant files to it in **index.html**.

```html
  <link href="https://fonts.googleapis.com/css?family=Inconsolata" rel="stylesheet" />
  <link rel="stylesheet" type="text/css" href="markdown.css" />
  <link rel="stylesheet" href="https://cdnjs.cloudflare.com/ajax/libs/highlight.js/9.13.1/styles/foundation.min.css" />
  <script src="https://cdn.jsdelivr.net/npm/marked/marked.min.js"></script>
```

and [verify the result][Rust Blog Example]. It will be similar to the main image of this post.

<br />

## 4. Conclusion

[![react-easy-md-example](https://www.steadylearner.com/static/images/post/web/code-with-you.png)](https://github.com/steadylearner/react-easy-md)

In this post, we learnt how to use **markdown** files with code snippets in Rust Yew Frontend. You could verify it work with [Rust Blog Example].

In the next [Rust blog posts], we will learn how to make components in Rust [Yew] and modulize the project. Then, we will write full stack Rust chat app with Rust frontend and server side code.

You can learn it with documentation from [Yew], [How to start Rust Chat App] and [Rust Full Stack].

If you need **a full stack Rust developer**, Contact me with [LinkedIn] or be one of them.

Follow me at [Twitter] if you want the latest contents.

**Thanks and please share this post with others.**