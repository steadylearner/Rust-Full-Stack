use warp::Filter;

use crate::{
    handlers::hello_handler,
    routes::hello_route,
    hello,
};

// $cargo test -- --nocapture if you want to use println! etc.

// or test just one function each time.
// For example, $cargo test hello and it passes.

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    // Refer to curl commands in main.rs
    #[tokio::test]
    async fn hello() {
        // let hello = hello_route::hello()
        //     .and_then(hello_handler::hello);

        let res = warp::test::request()
            .method("GET")
            .path("/hello/www.steadylearner.com") // 1. [Client] - Define request(path with datas) until this
            .reply(&hello!()) // 2. [Server] - How will you respond to it? With what?
            .await;

        assert_eq!(res.status(), 200, "Should return 200 OK.");
        println!("{:#?}", res.body());
    }
}