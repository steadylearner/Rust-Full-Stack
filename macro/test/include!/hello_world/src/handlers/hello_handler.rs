use warp::{
    Reply, Rejection,
    reply,
};

pub async fn hello(name: String) -> Result<impl Reply, Rejection> {
    let reply = format!("Hello, {}!", name);
    println!("{}", &reply);
    Ok(reply::html(reply))
}
