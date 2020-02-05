use super::*;

// 1. Create user succesfully.
//      -> success with "Create 1 user with id a80ca026-5ea1-4047-a18c-4a596e410322."
// 2. Create user with incorrect date format.
//      -> Fail with 405 'HTTP method not allowed' when curl is used.
// (Should give some restrctions. There is no proper fields for them yet.)
//      -> For example, duplicate username etc.

#[tokio::test]
async fn create_user() {
    let _ = pretty_env_logger::init();

    let create_user = user_route::create()
        .and_then(user_handler::create);

    let user_with_correct_date_of_birth = NewUser {
        first_name: "steady".into(),
        last_name: "learner".into(),
        date_of_birth: "2019-01-01".into(),
    };

    // 1. header("Content-Length", "0") // It is automatically set by curl command with -d with JSON datas.
    //    If you don't use it it will return 411 cotent-length not set.
    let res = warp::test::request()
        .method("POST")
        .header("Content-Length", "0")
        .path("/api/user/v1")
        .json(&user_with_correct_date_of_birth)
        .reply(&create_user)
        .await;

    assert_eq!(res.status(), 200, "Should return 200 'OK'");
    println!("{:#?}", res.body());

    let user_with_incorrect_date_of_birth = NewUser {
        first_name: "steady".into(),
        last_name: "learner".into(),
        date_of_birth: "this is not valid date".into(),
    };

    let res = warp::test::request()
        .method("POST")
        .header("Content-Length", "0")
        .path("/api/user/v1")
        .json(&user_with_incorrect_date_of_birth)
        .reply(&create_user)
        .await;

    // warp/reject/index.html
    // Should be invalid data types instead.
    assert_eq!(res.status(), 404, "Should return 404 'Not Found'");
}