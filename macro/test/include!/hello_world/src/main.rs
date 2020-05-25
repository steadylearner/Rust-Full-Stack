use warp::{
    self,
    Filter,
};

// It will only work with $cargo test
// For example, $cargo test hello -- --nocapture
#[cfg(test)] mod tests;

mod routes;
mod handlers; // This is the payload of this framework.
mod models;

use self::{
    routes::{
        hello_route,
        message_route,
    },
    handlers::{
        hello_handler,
        message_handler,
    },
};

#[tokio::main]
async fn main() {
    let end = include!("routers/hello.include")
        .or(include!("routers/message.include"));

    warp::serve(end).run(([0, 0, 0, 0], 8000)).await;
}
