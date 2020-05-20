// Compare this(https://github.com/seanmonstar/warp/blob/master/examples/file.rs) with the entire project.

// If you want to deploy it with Docker, refer to
// https://github.com/steadylearner/Rust-Full-Stack/tree/master/React_Rust/server/actix

// If you want to modulize it, refer to
// https://github.com/steadylearner/Rust-Full-Stack/tree/master/warp/hello_world

use warp::{self, path, Filter};

use console::Style;

// How to serve React or other single page apps?
// 1. Serve production index.html file from them for speicfic routes.
// 2. Find how to serve files in public/ or static/.

#[tokio::main]
async fn main() {
    pretty_env_logger::init();    

    let target: String = "0.0.0.0:8000".parse().unwrap();
    let blue = Style::new()
        .blue();

    // 1. $curl 0.0.0.0:8000
    // p.s If you liked the React app here and need a help with it,
    // please contact me with https://www.linkedin.com/in/steady-learner-3151b7164/
    // (I can work with React, Node, Rust, Python, (Golang))

    // When I visit the 0.0.0.0:8000, it shows the React app.
    // $curl 0.0.0.0:8000 also return index.html page
    let single_page_app = warp::get()
        .and(warp::path::end())
        .and(warp::fs::file("./public/index.html"));

    // 2. $curl 0.0.0.0:8000/public/index.html
    // dir already includes GET / ...
    // (You don't have to prefix warp::get("/")" here)

    // They all work when manually visiting them with a local browser and CURL.
    // $curl 0.0.0.0:8000/vendors.js
    // $curl 0.0.0.0:8000/main.js
    // $curl 0.0.0.0:8000/main.css
    // $curl 0.0.0.0:8000/index.html
    // $curl 0.0.0.0:8000/src/images/rust-chat-app.png
    let public_files = warp::fs::dir("./public/");

    // GET / => ./public/index.html
    // GET /public/... => ./public/..

    // https://github.com/seanmonstar/warp/issues/420
    
    // First attempt to find the errors with its author
    // $RUST_LOG=warp_react=info cargo run --release to find the origin of 404 errors from loadtest.
    // let routes = single_page_app.or(public_files).with(warp::log("warp_react"));

    // With $RUST_LOG=warp::filters::fs=info cargo run --release again.
    let routes = single_page_app.or(public_files).with(warp::log("warp::filters::fs"));
    // Could find them with more details from fs module from its API
    // INFO  warp::filters::fs > 127.0.0.1:59704 "GET / HTTP/1.1" 200 "-" "loadtest/4.0.0" 295.867072ms
    // ERROR warp::filters::fs > file open error (path="./public/index.html"): Too many open files (os error 24) 

    
    println!("\nRust Warp Server ready at {}", blue.apply_to(&target));
    println!("Use $curl http://0.0.0.0:8000 to test the end point.");

    warp::serve(routes).run(([0, 0, 0, 0], 8000)).await;
}
