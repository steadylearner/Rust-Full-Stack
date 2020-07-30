use warp::Filter;

use crate::{
    handlers::post_handler,
    routes::post_route,
    // macros
    list_posts,
    get_post,
    // create_post,
    // update_post,
    // delete_post,
};

// $cargo test -- --nocapture if you want to use println! etc.

// or test just one function each time.
// For example, $cargo test list_post and it passes.

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    // Refer to curl commands in main.rs

    #[tokio::test]
    async fn list_post() {
        let res = warp::test::request()
            .method("GET")
            .path("/api/post/v1") // 1. [Client] - Define request(path with datas) until this
            .reply(&list_posts!()) // 2. [Server] - How will you respond to it? With what?
            .await;

        assert_eq!(res.status(), 200, "Should return 200 OK.");
        println!("{:#?}", res.body());
    }

    #[tokio::test]
    async fn get_post() {
        let res = warp::test::request()
            .method("GET")
            .path("/api/post/v1/1")
            .reply(&get_post!())
            .await;

        assert_eq!(res.status(), 200, "Should return 200 OK.");
        println!("{:#?}", res.body());
    }

    // Refer to this to make tests for create_post, delete_post, update_post
    // https://github.com/steadylearner/Rust-Full-Stack/blob/master/microservices_with_docker/warp_client/src/tests/user_test_monolithic.rs
    // Otherwise, simply use CURL commands in main.rs file.

    // #[tokio::test]
    // async fn create_post() {

    // }

    // #[tokio::test]
    // async fn update_post() {

    // }

    // #[tokio::test]
    // async fn delete_post() {

    // }
}