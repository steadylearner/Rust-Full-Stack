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

[How to install Rust]: https://www.steadylearner.com/blog/read/How-to-install-Rust
[Rust Chat App]: https://www.steadylearner.com/blog/read/How-to-start-Rust-Chat-App
[Steadylearner Rust Blog Posts]: https://www.steadylearner.com/blog/search/Rust
[Yew Counter]: https://www.steadylearner.com/yew_counter
[How to use Rust Yew]: https://www.steadylearner.com/blog/read/How-to-use-Rust-Yew
[How to deploy Rust Web App]: https://www.steadylearner.com/blog/read/How-to-deploy-Rust-Web-App
[How to start Rust Chat App]: https://www.steadylearner.com/blog/read/How-to-start-Rust-Chat-App
[Fullstack Rust with Yew]: https://www.steadylearner.com/blog/read/Fullstack-Rust-with-Yew
[How to use NPM packages with Rust Frontend]: https://www.steadylearner.com/blog/read/How-to-use-NPM-packages-with-Rust-Frontend
[How to use markdown with Rust Frontend]: https://www.steadylearner.com/blog/read/How-to-use-markdown-with-Rust-Frontend
[How to modulize your Rust Frontend]: https://www.steadylearner.com/blog/read/How-to-modulize-your-Rust-Frontend

[How to use Python in JavaScript]: https://www.steadylearner.com/blog/read/How-to-use-Python-in-JavaScript

[How to build a static sitemap.xml with Rust]: https://www.steadylearner.com/blog/read/Static-sitemap.xml-with-Rust
[How to build a sitemap.xml with dynamic contents in Rust]: https://www.steadylearner.com/blog/read/How-to-make-sitemap-with-dynamic-contents-in-Rust
[How to build a sitemap for images with Rust]: https://www.steadylearner.com/blog/read/How-to-build-sitemap-for-images-with-Rust
[How to automate building sitemaps with Rust]: https://www.steadylearner.com/blog/read/How-to-automate-building-sitemaps-with-Rust

[How to write Full Stack Rust code]: https://www.steadylearner.com/blog/read/How-to-write-Full-Stack-Rust-code

[How to use a modal in Rust]: https://www.steadylearner.com/blog/read/How-to-use-a-modal-in-Rust
[How to use routers in Rust Frontend]: https://www.steadylearner.com/blog/read/How-to-use-routers-in-Rust-Frontend

[How to serve static files with Rust]: https://www.steadylearner.com/blog/read/How-to-serve-static-files-with-Rust
[How to use a single page app with Rust]: https://www.steadylearner.com/blog/read/How-to-use-a-single-page-app-with-Rust

[How to use Rust Tera for undefined paths]: https://www.steadylearner.com/blog/read/How-to-use-Rust-Tera-for-undefined-paths
[How to make Rust JSON Webservice with YouTube API]: https://www.steadylearner.com/blog/read/How-to-make-Rust-JSON-Webservice-with-YouTube-API

[How to use gRPC with Rust Tonic and Postgresql database]: https://www.steadylearner.com/blog/read/How-to-use-gRPC-with-Rust-Tonic-and-Postgresql-database

[How to use Python Scrapy to crawl This Week in Rust]: https://www.steadylearner.com/blog/read/How-to-use-Python-Scrapy-to-crawl-static-websites
[How to use React with Rust Actix]: https://www.steadylearner.com/blog/read/How-to-use-React-with-Rust-Actix

[How to use Docker commands]: https://www.steadylearner.com/blog/read/How-to-use-Docker-commands
[How to use Docker with Rust]: https://www.steadylearner.com/blog/read/How-to-use-Docker-with-Rust

[donation]: (https://www.paypal.com/cgi-bin/webscr?cmd=_s-xclick&hosted_button_id=HLNVQJ2L2YYZU)

<!-- / -->

[![Donate](https://img.shields.io/badge/Donate-PayPal-green.svg)](https://www.paypal.com/cgi-bin/webscr?cmd=_s-xclick&hosted_button_id=HLNVQJ2L2YYZU)

# How to be a full stack Rust Developer

Read [the Rust blog posts at Steadylearner][Steadylearner Rust Blog Posts]. 

[![Rust full stack chat app](https://www.steadylearner.com/static/images/post/web/full-stack-rust-chat-app-by-steadylearner.png)](https://www.steadylearner.com/blog/read/Fullstack-Rust-with-Yew)

## Start

First, clone this repository with

```console
$git clone https://github.com/steadylearner/Rust-Full-Stack.git
```

Prepare Rust development environment with [How to install Rust] if you haven't yet.

Then, use one of these commands.

1. **$./install.sh** in **web** folder and **$./run-local.sh** for a full stack Rust chat app. 
2. **$cd static_files && $cargo run --release** for JavaScript frontend and Rust server side web app.
3. **$cd JSON_Webservice && $./run-local.sh** for YouTube vlog example with JSON Webservice.
4. **$cd yew/rust_blog && $./install.sh && yarn watch:rs** for a Rust blog example.
5. **$cd React_Rust** to test a React app with **Rocket**, **Actix**, Express, Restify, Django, Golang, Vibora etc.
6. **$cd sitemap** if you want to build sitemap automatically with a database.

You can **like** and share it with others. 

## Install and Deploy Rust

1. [How to install Rust]
2. [How to deploy Rust Web App]

## Docker and AWS

[![Docker and Rust by Steadylearner](https://www.steadylearner.com/static/images/post/Rust/docker-rust.png)](https://www.steadylearner.com/blog/read/How-to-use-Docker-with-Rust)

1. [How to use Docker commands]
2. [How to use Docker with Rust]

## Frontend

[![NPM and Rust by Steadylearner](https://www.steadylearner.com/static/images/post/web/npm-and-rust-by-Steadylearner.png)](https://www.steadylearner.com/blog/read/How-to-use-NPM-packages-with-Rust-Frontend)

1. [How to use Rust Yew]
2. [How to use a modal in Rust]
3. [How to use routers in Rust Frontend]
4. [How to modulize your Rust Frontend]
5. [How to use NPM packages with Rust Frontend]
6. [How to use markdown with Rust Frontend]

## Server

[![Rust equivalent server and client code](https://www.steadylearner.com/static/images/post/web/client-server-equal-rust-code.png)](https://www.steadylearner.com/blog/read/How-to-write-Full-Stack-Rust-code)

1. [How to use Rust Tera for undefined paths]
2. [How to make Rust JSON Webservice with YouTube API]
3. [How to use CORS and OPTIONS HTTP request with Rust Rocket](https://www.steadylearner.com/blog/read/How-to-use-CORS-and-OPTIONS-HTTP-request-with-Rust-Rocket)
4. [How to serve static files with Rust]
5. [How to use a single page app with Rust]
6. [How to use gRPC with Rust Tonic and Postgresql database]
7. [How to use React with Rust Actix]

## Full Stack

[![Rust Router Example](https://www.steadylearner.com/static/images/post/web/rust-router-example.png)](https://www.steadylearner.com/blog/read/How-to-use-routers-in-Rust-Frontend)

1. [Rust Chat App]
2. [Fullstack Rust with Yew]
3. [How to write Full Stack Rust code]
4. [How to render a YouTube vlog with Rust Yew fetch API](https://www.steadylearner.com/blog/read/How-to-render-a-YouTube-vlog-with-Rust-Yew-fetch-API)
5. [How to render blog posts with Rust Yew mounted API](https://www.steadylearner.com/blog/read/How-to-render-blog-posts-with-Rust-Yew-mounted-API)

## Sitemap

[![Sitemaps with Rust](https://www.steadylearner.com/static/images/post/sitemap/automate-sitemap-rust.png)](https://www.steadylearner.com/blog/read/How-to-automate-building-sitemaps-with-Rust)

1. [How to build a static sitemap.xml with Rust]
2. [How to build a sitemap.xml with dynamic contents in Rust]
3. [How to build a sitemap for images with Rust]
4. [How to automate building sitemaps with Rust]

## Web Scrapers

1. [How to use Python Scrapy to crawl This Week in Rust]

## What you will learn with it

1. How to use HTML, CSS and Markdown in Rust Frontend
2. How to include NPM packages and JavaScript modules in it
3. How to use Rust [Yew]
4. How to write components and organize the Rust project
5. How to build complete **Full Stack Rust Web Application**
6. How to modulize your Rust project
7. How to deploy it
8. How to write sitemaps and metatags for it

## CWD

Rust login features prototype at **auth/javascript/express**.

## Contact

Send messages with them if you need a help from a full stack developer.

1. [LinkedIn](https://www.linkedin.com/in/steady-learner-3151b7164/) 
2. [Twitter](https://twitter.com/steadylearner_p)


