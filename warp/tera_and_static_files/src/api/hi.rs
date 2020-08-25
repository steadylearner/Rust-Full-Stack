#[macro_export]
macro_rules! hi {
    () => {
        hi_route::hi()
        .and_then(hi_handler::hi)
    }
}