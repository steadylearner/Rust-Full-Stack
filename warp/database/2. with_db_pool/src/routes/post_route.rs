// Refer to this.
// https://github.com/steadylearner/Rust-Full-Stack/blob/master/microservices_with_docker/warp_client/src/routes/user_route.rs

use warp::{
    filters::BoxedFilter,
    path,
    Filter,
};

use crate::{
    models::{
        post::{
            NewPost,
        }
    },
    json_body,
};

// It won't work easily. Just import it from crate(main.rs) and make it global
// If you don't want to you see the errors here.
// https://github.com/rust-lang/rust/issues/57966
// use super::reusable::{
//     json_body,
// };

// Use this to debug and verify API chaining work or not.
// pub fn repeat() -> BoxedFilter<(String, )> {
//     warp::get()
//         .and(path!("repeat" / String))
//         .and(warp::path::end())
//         .boxed()
// }

// Is this was the reason of all this problem?
// https://github.com/seanmonstar/warp/pull/359
// Use .. at the end when you want to make prefix with path! macro.
fn path_prefix() -> BoxedFilter<()> {
    // path! macro to assume a path::end() by default, with explicit / .. to allow building a prefix
    // path!("api" / "post" / "v1" ) // With v2.0 It won't work
    path!("api" / "post" / "v1" / ..)
        .boxed()
}

pub fn list() -> BoxedFilter<()> {
    warp::get()
        .and(path_prefix())
        .and(warp::path::end())
        .boxed()
}

pub fn get() -> BoxedFilter<(i32, )> {
    warp::get()
        .and(path_prefix())
        .and(warp::path::param::<i32>())
        .boxed()
}

pub fn create() -> warp::filters::BoxedFilter<(NewPost,)> {
    warp::post()
        .and(path_prefix())
        .and(warp::path::end())
        .and(json_body!())
        .boxed()
}

pub fn update() -> warp::filters::BoxedFilter<(i32, NewPost,)> {
    warp::put()
        .and(path_prefix())
        .and(warp::path::param::<i32>())
        .and(json_body!())
        .boxed()
}

pub fn delete() -> BoxedFilter<(i32, )> {
    warp::delete()
        .and(path_prefix())
        .and(warp::path::param::<i32>())
        .boxed()
}