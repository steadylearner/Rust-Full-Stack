use super::*;

// 1. get_user in database
//      -> Should return 200 OK
//      -> The full_name made from its first_name and last name should be equal to the hashed value
//         when verified with verify_with_argon2
// 2. get_user not in database
//      -> fail with 404 'Not Found'(This is what I expect)
// (When I use CURL or Browser, it returns 405 'HTTP method not allowed')
//      -> Should solve this problem with the author of the crate or there is a problem with my code?

#[tokio::test]
async fn get_user() {
    let _ = pretty_env_logger::init();

    let get_user = user_route::get()
        .and_then(user_handler::get);

    // 1.
    let res = warp::test::request()
        .method("GET")
        .path("/api/user/v1/steadylearner")
        .reply(&get_user)
        .await;

    assert_eq!(res.status(), 200, "Should return 200 'OK'");
    // res.body here is b"" utf8 bytes
    let body_str_from_byte = str::from_utf8(&res.body()).unwrap();
    let UserSuccess { full_name } = serde_json::from_str(&body_str_from_byte).unwrap();

    assert!(verify_with_argon2(&full_name, "steady learner".as_bytes()));

    // 2.
    let res = warp::test::request()
        .method("GET")
        .path("/api/user/v1/d631208e-57ce-4838-9368-db401519ebb8")
        .reply(&get_user)
        .await;

    assert_eq!(res.status(), 404, "Should return 404 'Not Found'");
}
