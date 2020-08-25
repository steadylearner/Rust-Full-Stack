use super::*;

// The main tests here are 7 and 8.

// 1. Update a user without method "PUT"
//      -> fail with "HTTP method not allowed"
// 2. Update a user without content-length header
//      -> fail with 'A content-length header is required''
// 3. Update a user with incorrect header key for example 'without-authorizaiton' with correct data
//      -> fail with 'Missing request header 'authorization''
// 4. Update a user without correct header value such as 'not-allowed'
//      -> fail with 'Invalid request header 'authorization''
// 5. Update a user with correct header with data and when there is no data(id) in database.
//      -> fail with 'Fail to update the user with id notregistered yet.'
// 6. Update a user with correct header with incorrect data.
//      -> fail with 'Fail to update the user with id steadylearner.'
// 7. Update a user with correct header with data when there is already data in database.
//      -> success with 'Update 1 user with id f2bd8139-5044-4526-89b8-1981d6220b4.'
// 8. Update a user with data before 6.
//      -> success with 'Update the user with id f2bd8139-5044-4526-89b8-1981d6220b4.'

#[tokio::test]
async fn update_user() {
    let _ = pretty_env_logger::init();

    let update_user = user_route::update()
        .and_then(user_handler::update);

    // You should verify this exist in your database.
    // For example, $SELECT * FROM users;
    // Or refer to list_users api. But it will be more complicated than this.
    let target = "steadylearner";

    // 1.
    let res = warp::test::request()
        .header("authorization", "steadylearner")
        .path(&format!("/api/user/v1/{}", &target))
        .reply(&update_user)
        .await;

    assert_eq!(res.status(), 405, "Should return 405 'HTTP method not allowed.'");
    let body_str_from_byte = str::from_utf8(&res.body()).unwrap();
    assert_eq!(body_str_from_byte, "HTTP method not allowed");

    // 2.
    let res = warp::test::request()
        .method("PUT")
        .path(&format!("/api/user/v1/{}", &target))
        .reply(&update_user)
        .await;

    assert_eq!(res.status(), 411, "Should return 411 'A content-length header is required'");
    let body_str_from_byte = str::from_utf8(&res.body()).unwrap();
    assert_eq!(body_str_from_byte, "A content-length header is required");

    let update_user_with_correct_data = UpdateUser {
        first_name: "full stack rust".into(),
        last_name: "developer".into(),
        date_of_birth: "2019-01-01".into(),
    };

    // 3.
    let res = warp::test::request()
        .method("PUT")
        .header("Content-Length", "0")
        .header("without-authorization", "steadylearner")
        .path(&format!("/api/user/v1/{}", &target))
        .json(&update_user_with_correct_data)
        .reply(&update_user)
        .await;

    assert_eq!(res.status(), 400, "Should return 400 'Bad Request'");
    let body_str_from_byte = str::from_utf8(&res.body()).unwrap();
    assert_eq!(body_str_from_byte, "Missing request header 'authorization'");

    // 4.
    let res = warp::test::request()
        .method("PUT")
        .header("Content-Length", "0")
        .header("authorization", "not-allowed")
        .path(&format!("/api/user/v1/{}", &target))
        .json(&update_user_with_correct_data)
        .reply(&update_user)
        .await;

    assert_eq!(res.status(), 400, "Should return 400 'Bad Request'");
    let body_str_from_byte = str::from_utf8(&res.body()).unwrap();
    assert_eq!(body_str_from_byte, "Invalid request header 'authorization'");

    let not_registered_yet = "notregisteredyet";

    // 5.
    let res = warp::test::request()
        .method("PUT")
        .header("Content-Length", "0")
        .header("authorization", "user")
        .path(&format!("/api/user/v1/{}", &not_registered_yet))
        .json(&update_user_with_correct_data)
        .reply(&update_user)
        .await;

    assert_eq!(res.status(), 200, "Should return 200 OK.");
    let body_str_from_byte = str::from_utf8(&res.body()).unwrap();
    assert_eq!(&body_str_from_byte, &format!("Fail to update the user with id {}.", &not_registered_yet));

    let update_user_with_incorrect_data = UpdateUser {
        first_name: "full stack rust".into(),
        last_name: "developer".into(),
        date_of_birth: "not a valid date".into(),
    };

    // 6.
    let res = warp::test::request()
        .method("PUT")
        .header("Content-Length", "0")
        .header("authorization", "user")
        .path(&format!("/api/user/v1/{}", &target))
        .json(&update_user_with_incorrect_data)
        .reply(&update_user)
        .await;

    // It means there was an error in gRPC Rust Tonic server parsing invalid type 'date_of_birth'.
    // Should make a better error response than this. But, I have to learn more Warp.
    // I also need a reason to invest more time with this.
    assert_eq!(res.status(), 404, "Should return 404 'Not Found'.");

    // 7.
    let res = warp::test::request()
        .method("PUT")
        .header("Content-Length", "0")
        .header("authorization", "user")
        .path(&format!("/api/user/v1/{}", &target))
        .json(&update_user_with_correct_data)
        .reply(&update_user)
        .await;

    assert_eq!(res.status(), 200, "Should return 200 'Ok'.");
    let body_str_from_byte = str::from_utf8(&res.body()).unwrap();
    assert_eq!(&body_str_from_byte, &format!("Update 1 user with id {}.", &target));

    let update_user_with_the_previous_data = UpdateUser {
        first_name: "steady".into(),
        last_name: "learner".into(),
        date_of_birth: "2019-01-01".into(),
    };

    // 8.
    let res = warp::test::request()
        .method("PUT")
        .header("Content-Length", "0")
        .header("authorization", "user")
        .path(&format!("/api/user/v1/{}", &target))
        .json(&update_user_with_the_previous_data)
        .reply(&update_user)
        .await;

    assert_eq!(res.status(), 200, "Should return 200 'Ok'.");
    let body_str_from_byte = str::from_utf8(&res.body()).unwrap();
    assert_eq!(&body_str_from_byte, &format!("Update 1 user with id {}.", &target));
}