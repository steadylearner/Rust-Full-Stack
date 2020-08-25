use warp::{
    filters::BoxedFilter,
    // path,
    Filter,
};

fn path_prefix() -> BoxedFilter<()> {
    warp::path("hello")
        .boxed()
}

pub fn hello() -> BoxedFilter<(String, )> {
    warp::get()
        .and(path_prefix())
        .and(warp::path::param::<String>())
        .boxed()
}
