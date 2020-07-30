// We will make Warp version of
// https://github.com/steadylearner/Rust-Full-Stack/tree/master/actix/src/database
// You should refer to
// https://github.com/steadylearner/Rust-Full-Stack/tree/master/microservices_with_docker/warp_client/src
use warp::{self, Filter};

use console::Style;

mod routes;
mod handlers; // This is the payload of this framework.
use self::{
    routes::{
        post_route,
    },
    handlers::{
        post_handler
    },
};
mod api; // With this, you can use macros in main.rs and tests/

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate diesel;

mod schema;
mod models;
mod db;

// It will only work with $cargo test
// For example, $cargo test list_post -- --nocapture
#[cfg(test)] mod tests;

// Use some test driven development with files at tests/ and CURL commands.
// If you still can't make it work, don't modulze the Warp API and test them here first.
#[tokio::main]
async fn main() {
    let target: String = "0.0.0.0:8000".parse().unwrap();
    let blue = Style::new()
        .blue();

    // curl 0.0.0.0:8000/repeat/www.steadylearner.com
    // Use this to debug and verify API chaining work or not.
    // let repeat = post_route::repeat()
    //     .and_then(post_handler::repeat);

    let post_api = list_posts!()
        .or(get_post!())
        .or(create_post!())
        .or(update_post!())
        .or(delete_post!());

    let end = post_api.with(warp::log("post_api"));

    println!("\nRust Warp Server ready at {}", blue.apply_to(&target));
    println!("Use $curl http://0.0.0.0:8000/api/post/v1 to test the end point.");

    warp::serve(end).run(([0, 0, 0, 0], 8000)).await;
}
