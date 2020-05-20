use std::error::Error;
use warp::{self, Filter};

#[macro_use]
extern crate lazy_static;

mod routes;
mod handlers;
mod api;

mod template_setup;

use self::{
    routes::{
        hello_route,
        hi_route,
    },
    handlers::{
        hello_handler,
        hi_handler,
    },
};

use console::Style;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let target: String = "0.0.0.0:8000".parse().unwrap();
    let blue = Style::new()
        .blue();

    // Use $RUST_LOG=warp::filters::fs=info cargo run --release if you see the problem with this.
    // curl 0.0.0.0:8000/rust_example.png
    let public_files = warp::fs::dir("./public/");

    let end = hello!()
        .or(hi!())
        .or(public_files);

    println!("\nRust Warp Server ready at {}\n", blue.apply_to(&target));

    println!("$curl 0.0.0.0:8000/hello/www.steadylearner.com to test the minimal end point.");
    println!("$curl 0.0.0.0:8000/hi/www.steadylearner.com to test the Tera template views/");
    println!("$curl 0.0.0.0:8000/rust_teloxide_example.png to test the public/.");

    Ok(warp::serve(end).run(([0, 0, 0, 0], 8000)).await)
}