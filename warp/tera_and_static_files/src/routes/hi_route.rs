use warp::{
    filters::BoxedFilter,
    // path,
    Filter,
};

fn path_prefix() -> BoxedFilter<()> {
    warp::path("hi")
        .boxed()
}

pub fn hi() -> BoxedFilter<(String, )> {
    warp::get()
        .and(path_prefix())
        .and(warp::path::param::<String>())
        .boxed()
}
