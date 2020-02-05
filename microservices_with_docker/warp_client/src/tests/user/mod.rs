// You can use all of this in each test moduels with use super::*;
use pretty_env_logger;
use warp::Filter;

use std::str;

use crate::{
    crypto::hash::verify_with_argon2,
    handlers::user_handler,
    models::user::{
        NewUser,
        UpdateUser,
        UserSuccess,
    },
    routes::user_route,
};

// Export the tests here.
pub mod list;
pub mod get;
pub mod create;
pub mod update;
pub mod delete;

// Refer to this before you test those functions.
// $cargo test -- --nocapture if you want to use println! etc.

// I think that there are async problems when the compiler tests various functions for Warp.
// When test all of them with $cargo test they all fails
// Is this the problem of this crate or tokio::test?
// failures:

// ---- tests::user::tests::get_user stdout ----
// thread 'tests::user::tests::get_user' panicked at 'called `Result::unwrap()` on an `Err` value: SetLoggerError(())', src/libcore/result.rs:1189:5
// note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.

// Should read more documenation and ask for the author if necessary.

// or test just one function each time.
// For example, $cargo test list_users and it passes.

// Before you test,
// 1. Tonic gRPC server should be ready first.(It is the client of tonic_server/src/service.rs)
// 2. You should have a record similar to this in your Postgresql data first

// {
//     id: "steadylearner",
//     first_name: "steady",
//     last_name: "learner",
//     date_of_birth: "2019-01-01",
// }

// I expect tests similar to what used in this post.
// https://www.steadylearner.com/blog/read/How-to-use-CORS-and-OPTIONS-HTTP-request-with-Rust-Rocket