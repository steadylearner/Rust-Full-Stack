<!--
    Post{
        subtitle: "Learn how to use server side rendering with Rust,
        image: "post/web/rust-tera.png",
        image_decription: "Image by Steadylearner",
        tags: "How Tera Rust server",
    }
-->

<!-- Link -->

[Steadylearner]: https://www.steadylearner.com
[Rust Website]: https://www.rust-lang.org/

[Rust Rocket]: https://rocket.rs/
[Rocket Getting Started]: https://rocket.rs/v0.4/guide/getting-started
[Redirect]: https://api.rocket.rs/v0.4/rocket/response/struct.Redirect.html
[Tera]: https://tera.netlify.com/
[Rocket Tera example]: https://github.com/SergioBenitez/Rocket/tree/master/examples/tera_templates

[Rust Full Stack]: https://github.com/steadylearner/Rust-Full-Stack

[CRA]: https://github.com/facebook/create-react-app

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
[How to use a modal in Rust]: https://www.steadylearner.com/blog/read/How-to-use-a-modal-in-Rust
[How to use routers in Rust Frontend]: https://www.steadylearner.com/blog/read/How-to-use-routers-in-Rust-Frontend
[How to serve static files with Rust]: https://www.steadylearner.com/blog/read/How-to-serve-static-files-with-Rust
[How to use a single page app wtih Rust]: https://www.steadylearner.com/blog/read/How-to-use-a-single-page-app-with-Rust

[How to use Python in JavaScript]: https://www.steadylearner.com/blog/read/How-to-use-Python-in-JavaScript
[How to use React Spring to animate your message]: https://medium.com/@steadylearner/how-to-use-react-spring-to-animate-your-message-2bd2a7e62a5a

<!-- / -->

<!-- Steadylearner Twitter and LinkedIn  -->

[Twitter]: https://twitter.com/steadylearner_p
[LinkedIn]: https://www.linkedin.com/in/steady-learner-3151b7164/

<!--  -->

In the previous post, we learnt [How to use a single page app wtih Rust].

We already learnt we can build website with Rust in various ways.

They are

1. [Plain HTML, CSS, JavaScript][How to serve static files with Rust]

2. [JavaScript Single Page App][How to use a single page app wtih Rust]

3. [Rust Webassembly][Fullstack Rust with Yew]

We will learn how to use [server side rendering with Rust][Tera] in this post.

It will help you to build a static website with ease. You can also write protoypes of a single page app with [Rust][Fullstack Rust with Yew] or [JavaScript][How to use a single page app wtih Rust] with it.

If you just want to verify the result from this post first,

1. Refer to [static_files folder](https://github.com/steadylearner/Rust-Full-Stack/tree/master/web/before/static_files/React) in [Rust Full Stack] repository.

2. Visit [undefined](www.steadylearner.com/undefined) path in [Steadylearner].

If you want other Rust web projects, use [the repostiory][Rust Full Stack] and its **before** folders.

<br />

<h2 class="red-white">[Prerequisite]</h2>

1. [How to install Rust]
2. [How to serve static files with Rust]
3. [How to use a single page app wtih Rust]
4. [How to deploy Rust Web App]

---

If you haven't installed Rust in your machine yet. The blog post [How to install Rust] will help you for that.

Read [Rocket Tera example] example briefly and [template part of Tera documenation](https://tera.netlify.com/docs/templates/#templates). We will use **default**, **truncate** filters in this post and want you to spend more time to find how they work.

When you complete your [Rust Full Stack] website, you can deploy it with [How to deploy Rust Web App].

<br />

<h2 class="blue">Table of Contents</h2>

1. Prepare Rust Rocket project
2. Route for server side rendering to work
3. Tera template engine files for it
4. Conclusion

---

You can use Rust server side frameworks instead of Rocket and only read parts relevant to [Tera].

The **main.html.tera** and **error.html.tera** we will build with it can be used with any of them.

<br />

## 1. Prepare Rust Rocket project

We need to set up the Rust Rocket project to make it usable with Tera template engine first. I will assume that you already read [How to serve static files with Rust] and minimal [Rust Rocket] project ready.

We will start with the **Cargo.toml** file that is similar to **package.json** in JavaScript.

It will be similar to this

```toml
<!-- 1. -->
[dependencies]
rocket = "0.4.2"
serde = "1.0"
serde_derive = "1.0"
serde_json = "1.0"

<!-- 2. -->
[dependencies.rocket_contrib]
version = "0.4.2"
default-features = false
features = ["tera_templates"]
```

and the important parts will be

1. To follow the official example and future usage. It is optional for this post.

2. [rocket_contrib](https://crates.io/crates/rocket_contrib) will be the payload for [Tera] to work with [Rust Rocket].

Then, we will edit **main.rs**

```rust
#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;

use rocket_contrib::templates::Template; // 1.

mod routes;
use crate::routes::{ static_files, get, error }; // 2.

fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .attach(Template::fairing()) // 1.
        .register(catchers![error::not_found]) // 2.
}
```

1. Import and include [Tera] to the main process of [Rust Rocket].

2. We will build **error.rs** to use server side rendering for undefined paths in the next part.

<br />

## 2. Routes for server side rendering to work

We already learnt how to use [Rust Rocket] routes in the previous [Rust blog posts].

Build **error.rs** to serve [Tera] template engine files for undefined routes.

```rust
// /server/src/route/error.rs
use rocket::Request;
use rocket_contrib::templates::Template; // 1.

#[catch(404)]
pub fn not_found(req: &Request) -> Template {
    let mut map = std::collections::HashMap::new();
    map.insert("path", req.uri().path());
    Template::render("error/not-found", &map) // 2.
}
```

The important points here are

**1.** You need to include it to **type** the return value of **not_found** to **Template**.

**2.** **Template::render("error/not-found", &map)** will use the value of path(**req.uri().path()**) to render **/templates/error/not-found.html.tera** we will make in the next part.

Then, you need to include **error.rs** in **route/mod.rs** with **pub mod error** to make it usbable in the **main.rs**. 

We have everything ready for [Rust Rocket] to work with [Tera].

<br />

## 3. Tera template engine files for it

Following the [Rocket Tera example], we will build **main.html.tera** and **error/not-found.html.tera** in **templates** folder. We will start with **main.html.tera**.

It will be similar to

```html
<!doctype html>
 {# title,css, main #}
<html lang=en>
    <head>
        {# 1. #}
        <title>{% block title %} Steadylearner {% endblock title %}</title>
        {# 2. #}
        {% block css %}<link rel="stylesheet" href="/static/css/blog.css">{% endblock css %}
    </head>
    <body>
    {# 3. #}
    <main role="main">
        {% block main %}{% endblock main %}
    </main>
    </body>
</html>
```

and nothing complicated. It will be the base of other Tera files and you can easily substitute parts in **{% block name % }{% endblock name %}** and reuse other [Tera] files with **{% extends "file" %}**.

We will learn how to use it more with **not-found.html.tera**.

```html
{# 1. #}
{% extends "main" %}

{# 2. #}
{% block title %} [404] Not Found {% endblock title %}

{# 3. #}
{% block css %}
  <link rel="stylesheet" href="/static/css/not-found.css">
{% endblock css %}

{% block main %}
  <section class="not-found">
    {# 4. #}
    <h1 class="not-found--title red-white" title={{path}} >No contents for {{
      path | truncate(length=25) | default(value="undefined")
    }}</h1>
    <h2 class="not-found--content" title="Redriect to /">
      <a class="blue hover no-text-decoration" href="/" >
        Please,
        <i class="fas fa-mouse-pointer"></i>
        this to back to the
        <i class="fas fa-home" ></i>
        page
      </a>
    </h2>
  </section>
{% endblock main %}
```

It will be used to handle undefined paths for your [Rust Rocket] app.

The important points here are

**1.** You can reuse codes in **main.html.tera** with this.

**2.** You can use **[404] Not Found** instead of the default value in **main.html.tera** because they are inside **{% block name % }{% endblock name %}**.

**3.** This is CSS specific part and you can use yours instead.

**4.** We use filter **truncate** and **default** here with | to shorten the **path**. It is not to affect the layout of your website.

You can also pass those values from **template engine** easily for HTML attributes. For example, refer to **title={{path}}**. It was used to show the entire path if users want to verify it when the path is truncated.

Verify it work in your machine with **$cargo run**. Then, visit whatever path you haven't defined in your project. It will be similar to the main image of this post.

<br />

## 4. Conclusion

We learnt we can use server side rendering with Rust easily.

What we need are

1. Rust server side framework

2. Perpare routes to use it with templates

3. Write files with [template engine][Tera]

In the next [Rust blog posts], we will build **a login page** with [Rust Frontend][How to write Full Stack Rust Code] and JSON Webservice at server side.

If you want the latest contents from Steadylearner, follow me at [Twitter].

Do you need a Full Stack Rust and JavaScript Developer? Contact me with [LinkedIn] or be one of them.

**Thanks and please share this post with others**.
