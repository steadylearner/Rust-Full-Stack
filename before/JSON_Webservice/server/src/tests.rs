use crate::rocket;
use crate::serde_json;
use crate::reqwest;
use crate::http_model::youtube_video::Video;

use rocket::local::Client;
use rocket::http::{
    Status, 
    ContentType, 
    Header,
    hyper::header::{
        Origin
    },
};

// Use this or just test it with curl
// Start your Rust Rocket App => $cargo run --release

// invalid_youtube_id 
// $curl http://localhost:8000/video_search_by_id/invalid_youtube_id
// valid_youtube_id 
// $curl http://localhost:8000/video_search_by_id/8EPsnf_ZYU0

// invalid_origin 
// $curl -H "Origin: http://invalid_origin" --verbose \http://localhost:8000/video_search_by_id/8EPsnf_ZYU0
// valid_origin 
// $curl -H "Origin: http://localhost:8080" --verbose \http://localhost:8000/video_search_by_id/8EPsnf_ZYU0

// 1. Test JSON Webservice made with Rust and YouTube API

#[test]
fn invalid_youtube_id() {
    // https://api.rocket.rs/v0.4/rocket/local/struct.Client.html
    let client = Client::new(rocket()).unwrap();

    let invalid_youtube_id = "invalid_youtube_id";

    let request_url = format!(
        "/video_search_by_id/{}",
        invalid_youtube_id,
    );

    let mut res = client.get(&request_url).header(ContentType::JSON).dispatch(); // return Video { items: Some([]) }

    // https://api.rocket.rs/v0.4/rocket/http/struct.Status.html
    assert_eq!(res.status(), Status::Ok);

    let body = res.body_string().unwrap();

    let video: Video = serde_json::from_str(&body).unwrap();

    assert_eq!(video.items.unwrap(), [].to_vec()); // should include PartialEq, Clone for http_model::youtube_video::Video 
}

#[test]
fn valid_youtube_id() {
    let client = Client::new(rocket()).unwrap();

    let valid_youtube_id = "8EPsnf_ZYU0";
    // "use_valid_youtube_id"

    let request_url = format!(
        "/video_search_by_id/{}",
        &valid_youtube_id,
    );

    let mut res = client.get(&request_url).header(ContentType::JSON).dispatch();

    assert_eq!(res.status(), Status::Ok);

    let body = res.body_string().unwrap();
    let video: Video = serde_json::from_str(&body).unwrap();

    let payload = &video.items.unwrap()[0]; // video_item or instead of payload

    assert_eq!(payload.id, valid_youtube_id.to_string());
    assert!(payload.snippet.title.contains("Rust"));
}

// 2. Test CORS with rocket-cors 

#[test]
fn valid_youtube_id_and_invalid_origin() {
    let client = Client::new(rocket()).unwrap();

    let invalid_youtube_id = "invalid_youtube_id";

    let request_url = format!(
        "/video_search_by_id/{}",
        invalid_youtube_id,
    );

    let invalid_origin = Header::new("Access-Control-Allow-Origin", "invalid_origin");

    let res = client
        .get(&request_url)
        .header(Origin::new("http", "invalid_origin", None))
        .header(invalid_origin)
        .header(ContentType::JSON)
        .dispatch();

    assert_ne!(res.status(), Status::Ok);
}

#[test]
fn valid_youtube_id_and_origin() {
    // https://api.rocket.rs/v0.4/rocket/local/struct.Client.html
    let client = Client::new(rocket()).unwrap();

    let valid_youtube_id = "8EPsnf_ZYU0";

    let request_url = format!(
        "/video_search_by_id/{}",
        valid_youtube_id,
    );

    // Refer to the code in /web/src/lib.rs it will be from "http://127.0.0.1:8000"
    // let request = Request::builder()
    //                 .method("GET")
    //                 .uri("http://localhost:8000/video_search_by_id/8EPsnf_ZYU0")
    //                 .header("Access-Control-Allow-Origin", "http://127.0.0.1:8000")
    //                 .body(Nothing)
    //                 .unwrap();

    let valid_origin = Header::new("Access-Control-Allow-Origin", "http://127.0.0.1:8000");

    let mut res = client
        .get(&request_url)
        .header(Origin::new("http", "127.0.0.1:8080", None))
        .header(valid_origin)
        .header(ContentType::JSON)
        .dispatch();

    println!("{:#?}", res.headers()); // cargo test -- --nocapture

    // Uncased {
    //     string: "Content-Type",
    // }: [
    //     "application/json",
    // ],
    // Uncased {
    //     string: "Server",
    // }: [
    //     "Rocket",
    // ],
    // Uncased {
    //     string: "Access-Control-Allow-Origin",
    // }: [
    //     "http://127.0.0.1:8080",
    // ],
    // Uncased {
    //     string: "Access-Control-Allow-Credentials",
    // }: [
    //     "true",
    // ],

    assert_eq!(res.headers().len(), 4); // why you should use 4 instead of 3

    assert_eq!(res.status(), Status::Ok);

    let body = res.body_string().unwrap();
    let video: Video = serde_json::from_str(&body).unwrap();

    let payload = &video.items.unwrap()[0]; // video_item or instead of payload

    assert_eq!(payload.id, valid_youtube_id.to_string());
    assert!(payload.snippet.title.contains("Rust"));
}

// https://doc.rust-lang.org/beta/std/fs/fn.copy.html
// https://doc.rust-lang.org/stable/std/string/struct.String.html#method.from_utf16
// https://doc.rust-lang.org/stable/std/char/fn.from_u32.html

#[test]
pub fn github_article_request() -> Result<(), Box<dyn std::error::Error>> {

    let target = "https://raw.githubusercontent.com/steadylearner/Rust-Full-Stack/master/README.md";

    let mut res = reqwest::get(target)?;
    let mut body: Vec<u8> = vec![];

    std::io::copy(&mut res, &mut body)?;

    // let characters: Vec<char> = body.into_iter().map(|x| x as char).collect();
    // let result: String = characters.into_iter().collect();

    let result: String = String::from_utf8(body).unwrap();

    println!("{:#?}", &result); // cargo test -- --nocapture

    Ok(())
}



