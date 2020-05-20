use super::*;

// The main tests here are 5 and 6.

// 1. Delete user without method "DELETE"
//      -> fail with "HTTP method not allowed"
// 2. Delete user without header
//      -> fail with 'Missing request header 'authorization''
// 3. Delete user with incorrect header key for example 'without-authorizaiton'
//      -> fail with 'Missing request header 'authorization''
// 4. Delete user without correct header value such as 'not-allowed'
//      -> fail with 'Invalid request header 'authorization''
// 5. Delete user with correct header and when there is already data in database.
//      -> success with 'Remove the user with id f2bd8139-5044-4526-89b8-1981d6220b4.'
// 6. Delete user with correct header but no data in database
//      -> fail with 'Fail to delete the user with id f2bd8139-5044-4526-89b8-1981d6220b4.' from gRPC server

#[tokio::test]
async fn delete_user() {
    let _ = pretty_env_logger::init();

    let delete_user = user_route::delete()
        .and_then(user_handler::delete);

    // You should verify this exist in your database.
    // For example, $SELECT * FROM users;
    // Or refer to list_users api and request the data here. But it will be more complicated than this.
    let target = "898067d8-5787-4939-bb3e-20025ae88d4e";
    // If it(target or id) doesn't exist, it will fail at 5.
    // else all tests will pass.

    // 1.
    let res = warp::test::request()
        .header("authorization", "steadylearner")
        .path(&format!("/api/user/v1/{}", &target))
        .reply(&delete_user)
        .await;

    assert_eq!(res.status(), 405, "Should return 405 'HTTP method not allowed.'");
    let body_str_from_byte = str::from_utf8(&res.body()).unwrap();
    assert_eq!(body_str_from_byte, "HTTP method not allowed");

    // 2.
    let res = warp::test::request()
        .method("DELETE")
        .path(&format!("/api/user/v1/{}", &target))
        .reply(&delete_user)
        .await;

    assert_eq!(res.status(), 400, "Should return 400 'Bad Request'");
    let body_str_from_byte = str::from_utf8(&res.body()).unwrap();
    assert_eq!(body_str_from_byte, "Missing request header 'authorization'");

    // 3.
    let res = warp::test::request()
        .method("DELETE")
        .header("without-authorization", "steadylearner")
        .path(&format!("/api/user/v1/{}", &target))
        .reply(&delete_user)
        .await;

    assert_eq!(res.status(), 400, "Should return 400 'Bad Request'");
    let body_str_from_byte = str::from_utf8(&res.body()).unwrap();
    assert_eq!(body_str_from_byte, "Missing request header 'authorization'");

    // 4.
    let res = warp::test::request()
        .method("DELETE")
        .header("authorization", "not-allowed")
        .path(&format!("/api/user/v1/{}", &target))
        .reply(&delete_user)
        .await;

    assert_eq!(res.status(), 400, "Should return 400 'Bad Request'");
    let body_str_from_byte = str::from_utf8(&res.body()).unwrap();
    assert_eq!(body_str_from_byte, "Invalid request header 'authorization'");

    // 5.
    let res = warp::test::request()
        .method("DELETE")
        .header("authorization", "steadylearner")
        .path(&format!("/api/user/v1/{}", &target))
        .reply(&delete_user)
        .await;

    assert_eq!(res.status(), 200, "Should return 200 OK.");
    let body_str_from_byte = str::from_utf8(&res.body()).unwrap();
    assert_eq!(&body_str_from_byte, &format!("Remove the user with id {}.", &target));

    // 6.
    let res = warp::test::request()
        .method("DELETE")
        .header("authorization", "steadylearner")
        .path(&format!("/api/user/v1/{}", &target))
        .reply(&delete_user)
        .await;

    assert_eq!(res.status(), 200, "Should return 200 OK.");
    let body_str_from_byte = str::from_utf8(&res.body()).unwrap();
    assert_eq!(&body_str_from_byte, &format!("Fail to delete the user with id {}.", &target));

    // No records in Postgresql when 6. passes.
    // \c grpc;
    // $SELECT * FROM users;
}