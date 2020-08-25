// 1. First approach with lazy_static fail with private type.

// The type becomes very complicated.
// The author must have his own reason to make it(warp::filter) private.

// module `filter` is private
// error: module `filter` is private

// `impl Trait` in type aliases is unstable

// note: for more information, see https://github.com/rust-lang/rust/issues/63063
// help: add `#![feature(type_alias_impl_trait)]` to the crate attributes to enablerustc(E0658)
// `impl Trait` not allowed outside of function and inherent method return typesrustc(E0562)
// `impl Trait` not allowed outside of function and inherent method return types

// help: add `#![feature(impl_trait_in_bindings)]` to the crate attributes to

// use warp::{
//     filters::BoxedFilter,
//     path,
//     Filter,
// };

// pub type JsonBody = warp::filter::and::And<impl warp::filter::Filter+std::marker::Copy, impl warp::filter::Filter+std::marker::Copy>;
// lazy_static! {
//     pub static ref JSONBODY: JsonBody = {
//         let json_body = warp::body::content_length_limit(1024 * 16).and(warp::body::json());
//         json_body
//     };
// }

// 2. Use function but fail again with type problems.

// note: expected struct `warp::filter::boxed::BoxedFilter<()>`
//   found struct `warp::filter::boxed::BoxedFilter<(_,)>`

// `_` is not allowed within types on item signatures
// not allowed in type signatures

// fn json_body() -> BoxedFilter<(_,)> {
//     // path! macro to assume a path::end() by default, with explicit / .. to allow building a prefix
//     // path!("api" / "post" / "v1" ) // With v2.0 It won't work
//     warp::body::content_length_limit(1024 * 16).and(warp::body::json())
//         .boxed()
// }

// 3. Use macro then to ignore it.

#[macro_export]
macro_rules! json_body {
    () => {
        warp::body::content_length_limit(1024 * 16).and(warp::body::json())
    }
}