use warp::{Reply, Rejection};

// type Reply = impl warp::Reply; `impl Trait` in type aliases is unstable
// type Rejection = warp::Rejection;

// pub async fn hello(name: String) -> Result<impl warp::Reply, warp::Rejection> {
pub async fn hello(name: String) -> Result<impl Reply, Rejection> {
    let reply = format!("Hello, {}!\n", name);
    print!("{}", &reply);
    Ok(warp::reply::html(reply))
}