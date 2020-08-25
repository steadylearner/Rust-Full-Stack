use warp;

// Return type of handler functions will be Result<impl warp::Reply, warp::Rejection>
// So, just copy and paste.

// Compare it to .map(|name| format!("Hello, {}!", name)); in main.rs
pub async fn hello(name: String) -> Result<impl warp::Reply, warp::Rejection> {
    let reply = format!("Hello, {}!", name);
    println!("{}", &reply);
    Ok(warp::reply::html(reply))
}

// If you have to serve a single_page_app to /bar, /hello etc
// Refer to this.
// pub async fn single_page_app(name: String) -> Result<impl warp::Reply, warp::Rejection> {
//     match name {
//          "bar" || "hello", => {
//                 Ok(warp::reply::html("Use single page app here instead"))
//           },
//           _ => {
//                 let reply = format!("{} are not defined at the single page app.", name);
//                 Ok(warp::reply::html(reply))
//           }
//     }
// }

// or

// pub async fn single_page_app(name: String) -> Result<impl warp::Reply, warp::Rejection> {
//     let list = ["bar", "hello"]
//     if list.contains(&name) {
//         Ok(warp::reply::html("Use single page app here instead"))
//     } else {
//         let reply = format!("{} are not defined at the single page app.", name);
//         Ok(warp::reply::html(reply)
//     }
// }
