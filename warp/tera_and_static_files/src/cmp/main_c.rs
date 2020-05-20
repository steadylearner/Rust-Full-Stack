// use warp::{self, Filter};


use tera::{Context, Tera};
use warp::{self, filters::BoxedFilter, path, Filter};
use warp::{
    reject::{custom, Reject},
    reply, Rejection, Reply,
};
use std::error::Error;

#[macro_use]
extern crate lazy_static;

use console::Style;

mod routes;
mod handlers;
mod api;
mod template_setup;

use self::{
    routes::{
        hello_route,
        // hi_route,
    },
    handlers::{
        hello_handler,
        // hi_handler,
    },
    template_setup::{tera::{create_template_filter, render}}
};

// It will only work with $cargo test
// For example, $cargo test hello -- --nocapture
#[cfg(test)] mod tests;

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
        .or(public_files).with(warp::log("warp::filters::fs"));

    println!("\nRust Warp Server ready at {}", blue.apply_to(&target));

    println!("$curl 0.0.0.0:8000/hello/www.steadylearner.com to test the minimal end point.");
    println!("$curl 0.0.0.0:8000/hi/www.steadylearner.com to test the Tera template views/");
    println!("$curl 0.0.0.0:8000/rust_example.png to test the public/.");

    Ok(warp::serve(end).run(([0, 0, 0, 0], 8000)).await);
}

//

// Single palyer Game.(You should start to use transcation here.)

// [Webserver with Warp]

// 1. Serve templates with warp at views/
// 2. Show the public data of users at /users
// 3. Show the ranking of usesrs at /user/ranking?

// [Data]

// 1. Include table for rank.
// 2. Search git tag and more.
// 3. Organize the project.(Extract common parts to functions.
//    Find how to reuse SQLite connection. Should I remove build.rs and move it to function?)
//    Make a profit function and method.

// Question he wants to contiune.

// Multipalyer Game.(You should start to use transcation here for the game results.)
