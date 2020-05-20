use warp::{
    filters::BoxedFilter,
    Filter,
};

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
