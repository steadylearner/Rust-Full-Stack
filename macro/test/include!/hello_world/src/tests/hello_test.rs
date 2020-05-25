use warp::Filter;

use crate::{
    handlers::hello_handler,
    routes::hello_route,
};

// $cargo test hello -- --show-output
#[tokio::test]
async fn hello() {
    let res = warp::test::request()
        .method("GET")
        .path("/hello/www.steadylearner.com") // 1. [Client] - Define request(path with datas) until this
        .reply(&include!("../routers/hello.include")) // 2. [Server] - How will you respond to it? With what?
        .await;

    assert_eq!(res.status(), 200, "Should return 200 OK.");
    println!("{:#?}", res.body());
}
