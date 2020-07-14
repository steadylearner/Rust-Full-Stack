use warp::{
    filters::BoxedFilter,
    // path,
    Filter,
};

// They are equal to use
// let todos = path!("hello" / String);
// in main.rs file.

// 1. When you want to separate the prefix parts.
// BoxedFilter and .boxed is similar to glue that help you use and API.
// Make filters and use it inside and() to connect with other filters.
// Read them.
// https://docs.rs/warp/0.2.0/warp/filters/path/index.html#routing
// https://github.com/steadylearner/Rust-Full-Stack/blob/master/microservices_with_docker/warp_client/src/routes/user_route.rs

fn path_prefix() -> BoxedFilter<()> {
    warp::path("hello")
        .boxed()
}

pub fn hello() -> BoxedFilter<(String, )> {
    warp::get() // 1. Only accept GET
        .and(path_prefix()) // 2. That starts with /hello
        .and(warp::path::param::<String>()) // 3. /hello/:String and the server will use <Stirng> param from the path.
        .boxed()
}

// 2. Simpler one
// pub fn hello() -> BoxedFilter<(String, )> {
//     warp::get()
//         .and(path!("hello" / String))
//         .boxed()
// }

