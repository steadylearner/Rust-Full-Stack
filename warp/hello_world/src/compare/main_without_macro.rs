// Start with $cargo doc -p warp --open
// To see its documentation.
// We will follow the structure of Express app because these two frameworks can be structured similarly.
use warp::{self, Filter};

use console::Style;

mod routes;
// Files in handlers/ are what implements "Result<impl warp::Reply, warp::Rejection>"
// It will be similar to controllers in Express and you will edit the folder most of time with models/
mod handlers; // This is the payload of this framework.
use self::{
    routes::{
        hello_route,
    },
    handlers::{
        hello_handler
    },

};

// It will only work with $cargo test
// For example, $cargo test hello -- --nocapture
#[cfg(test)] mod tests;

#[tokio::main]
async fn main() {
    let target: String = "0.0.0.0:8000".parse().unwrap();
    let blue = Style::new()
        .blue();

    // GET /hello/www.steadylearner.com => 200 OK with body "Hello, www.steadylearner.com!"
    // $curl 0.0.0.0:8000/hello/www.steadylearner.com
    let hello = hello_route::hello()
        .and_then(hello_handler::hello);

    // What is the point of this framework? Separate all features and make them reusable.

    // Why all this closures work without typing?
    // Because Rust compiler solves typing the codes automatically instead of you when you use it.
    // It is easy and similar to use JavaScript arrow functions.
    // But, it is not that reusable and a little bit difficult to see what happens behind.
    // Therefore, we make them in functions and separate in each folder.
    // They are modulized, reusable and testable.

    // There are route part and handler part. Similar to request(res) and response(req)?
    // let hello = path!("hello" / String) // 1. Compare it to the routes/hello_route.rs
    //    .map(|name| format!("Hello, {}!", name)); // 2. Compare it to the handlers/hello_handler.rs

    // Refer to https://github.com/steadylearner/Rust-Full-Stack/blob/master/microservices_with_docker/warp_client/src/main.rs
    let end = hello.with(warp::log("hello"));

    println!("\nRust Warp Server ready at {}", blue.apply_to(&target));
    println!("Use $curl 0.0.0.0:8000/hello/www.steadylearner.com to test the end point.");

    warp::serve(end).run(([0, 0, 0, 0], 8000)).await;
}

// Compare this(https://github.com/seanmonstar/warp#example) with the entire project.

// use warp::{self, path, Filter};

// #[tokio::main]
// async fn main() {
//    // GET /hello/warp => 200 OK with body "Hello, warp!"
//    let hello = path!("hello" / String)
//        .map(|name| format!("Hello, {}!", name));
//
//    warp::serve(hello).run(([127, 0, 0, 1], 3030)).await;
// }