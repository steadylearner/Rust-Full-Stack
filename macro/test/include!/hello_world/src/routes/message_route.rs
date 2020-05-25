use warp::{
    filters::BoxedFilter,
    Filter,
    path,
    body::{content_length_limit, json},
};

use crate::{
    models::MessageCreateRequest,
};

pub fn message() -> BoxedFilter<(MessageCreateRequest,)> {
    warp::post()
        .and(include!("includes/v1.include"))
        .and(path("message"))
        .and(warp::path::end())
        .and(include!("includes/json_body.include"))
        .boxed()
}

