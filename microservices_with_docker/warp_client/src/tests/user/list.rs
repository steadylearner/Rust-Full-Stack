use super::*;

#[tokio::test]
async fn list_users() {
    let _ = pretty_env_logger::init();

    let list_users = user_route::list()
        .and_then(user_handler::list);

    let res = warp::test::request()
        .method("GET")
        .path("/api/user/v1") // 1. Define path with datas
        .reply(&list_users) // 2. Use the exact payload you want to test and reply can be target, responder, reply_with.
        .await;

    assert_eq!(res.status(), 200, "Should return 200 OK.");
    println!("{:#?}", res.body());
}

