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

#[macro_use]
extern crate diesel;

mod schema;
mod models;
mod db_connection;

// It will only work with $cargo test
// For example, $cargo test list_post -- --nocapture
#[cfg(test)] mod tests;

// Use some test driven development with files at tests/ and CURL commands.
// If you still can't make it work, don't modulze the Warp API and test them here first.

// Make data first with post.sql
// Use macros to ignore type problems and make them sharable to tests/
#[macro_export]
macro_rules! list_posts {
    () => {
        // $curl 0.0.0.0:8000/api/post/v1
        post_route::list()
            .and_then(post_handler::list)
    }
}

#[macro_export]
macro_rules! get_post {
    () => {
        // $curl 0.0.0.0:8000/api/post/v1/1
        post_route::get()
            .and_then(post_handler::get)
    }
}

#[macro_export]
macro_rules! create_post {
    () => {
        // curl -X POST localhost:8000/api/post/v1 -H "Content-Type: application/json" -d '{ "title": "When can I work with programming?", "body": "Should find someone recognize my skills." }'
        // and $curl 0.0.0.0:8000/api/post/v1
        // To see it created.
        post_route::create()
            .and_then(post_handler::create)
    }
}

#[macro_export]
macro_rules! update_post {
    () => {
        // curl -X PUT 0.0.0.0:8000/api/post/v1/1 -H "Content-Type: application/json" -d '{ "title": "test update", "body": "test update" }'
        // and $curl 0.0.0.0:8000/api/post/v1/1
        // To see it updated.
        post_route::update()
            .and_then(post_handler::update)
    }
}

#[macro_export]
macro_rules! delete_post {
    () => {
        // $curl -X DELETE 0.0.0.0:8000/api/post/v1/1 and $curl 0.0.0.0:8000/api/post/v1
        // To see it deleted.
        post_route::delete()
            .and_then(post_handler::delete)
    }
}

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
