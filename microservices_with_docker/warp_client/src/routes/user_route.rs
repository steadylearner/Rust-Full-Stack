use warp::Filter;
use crate::{
    models::{
        user::{
            NewUser,
            UpdateUser,
        }
    },
};

// It is equal to use it in main.
// let todos = path!("api" / "user" / "v1");

// When I made this, it was before v2.0
// You should use .. in v2.0
// https://github.com/seanmonstar/warp/pull/359
fn path_prefix() -> warp::filters::BoxedFilter<()> {
    path!("api" / "user" / "v1")
        .boxed()
}

// fn admin_only() -> warp::filters::BoxedFilter<()> {
//     path!("api" / "user" / "v1")
//         .boxed()
// }

// fn json_body() -> warp::filters::BoxedFilter<()> {
//     path!("api" / "user" / "v1")
//         .boxed()
// }

pub fn list() -> warp::filters::BoxedFilter<()> {
    warp::get()
        .and(path_prefix())
        .and(warp::path::end())
        .boxed()
}

// It is equal to use it in main.
// let get_user = path!("api" / "user" / "v1")
//     .and(warp::path::param::<String>())
pub fn get() -> warp::filters::BoxedFilter<(String, )> {
    warp::get()
        .and(path_prefix())
        .and(warp::path::param::<String>())
        .boxed()
}

pub fn create() -> warp::filters::BoxedFilter<(NewUser,)> {
    let json_body = warp::body::content_length_limit(1024 * 16).and(warp::body::json());

    warp::post()
        .and(path_prefix())
        .and(warp::path::end())
        .and(json_body)
        .boxed()
}

// // `PUT /todos/:id`
// let update = warp::put()
//     .and(todos_id)
//     .and(json_body)
//     .and(db.clone())
//     .and_then(update_todo);

pub fn update() -> warp::filters::BoxedFilter<(String, UpdateUser,)> {
    let user_only = warp::header::exact("authorization", "user");
    let json_body = warp::body::content_length_limit(1024 * 16).and(warp::body::json());

    warp::put()
        .and(path_prefix())
        .and(warp::path::param::<String>())
        .and(json_body)
        .and(user_only.clone())
        .boxed()
}

pub fn delete() -> warp::filters::BoxedFilter<(String, )> {
    // Move it to function later if necessary.
    // pub fn exact(
    //     name: &'static str,
    //     value: &'static str
    // ) -> impl Filter<Extract = (), Error = Rejection> + Copy
    let admin_only = warp::header::exact("authorization", "steadylearner");

    warp::delete()
        .and(path_prefix())
        .and(warp::path::param::<String>())
        .and(admin_only.clone())
        .boxed()
}
