use warp;

// Return type of handler functions will be Result<impl warp::Reply, warp::Rejection>
// So, just copy and paste.

// Compare it to .map(|name| format!("Hello, {}!", name)); in main.rs
pub async fn hello(name: String) -> Result<impl warp::Reply, warp::Rejection> {
    let reply = format!("Hello, {}!", name);
    println!("{}", &reply);
    Ok(warp::reply::html(reply))
}
