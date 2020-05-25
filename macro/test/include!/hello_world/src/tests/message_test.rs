// https://github.com/steadylearner/Rust-Full-Stack/blob/master/microservices_with_docker/warp_client/src/tests/user/create.rs

use warp::Filter;

use crate::{
    handlers::message_handler,
    routes::message_route,
    models::MessageCreateRequest,
};

// $cargo test message -- --show-output
#[tokio::test]
async fn message() {
    let message_create_request = MessageCreateRequest {
        text: "Send a message to Rust Warp server.".into(),
    };

    let res = warp::test::request()
        .method("POST")
        .path("/api/v1/message")
        .json(&message_create_request)
        .reply(&include!("../routers/message.include"))
        .await;

    assert_eq!(res.status(), 200, "Should return 200 OK.");
    println!("{:#?}", res.body());
}