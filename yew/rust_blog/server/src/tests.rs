use crate::rocket;
use crate::serde_json;
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
// Start your Rust Rocket App with $cargo run --release

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
        .get(&request_url)

    let mut res = client
        .header(Origin::new("http", "127.0.0.1:8080", None))
        .header(valid_origin)
        .header(ContentType::JSON)
        .dispatch();

    println!("{:#?}", res.headers()); // cargo test -- --nocapture

    assert_eq!(res.headers().len(), 4); 

    // why you should use 4 instead of 3
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

    assert_eq!(res.status(), Status::Ok);

    let body = res.body_string().unwrap();
    let video: Video = serde_json::from_str(&body).unwrap();

    let payload = &video.items.unwrap()[0]; // video_item or instead of payload

    assert_eq!(payload.id, valid_youtube_id.to_string());
    assert!(payload.snippet.title.contains("Rust"));
}

// #[test]
// pub fn post() -> Result<(), Box<dyn std::error::Error>> {

//     let client = Client::new(rocket()).unwrap();

//     let request_url = "/post";

//     let mut res = client.get(request_url).dispatch();

//     assert_eq!(res.status(), Status::Ok);

//     let body = res.body_string().unwrap();

//     println!("{:#?}", &body);

//     Ok(())
// }

// #[test]
// pub fn author() -> Result<(), Box<(dyn std::error::Error + 'static)>> {
 
//     let client = Client::new(rocket()).unwrap();

//     let request_url = "/author";

//     let mut res = client.get(request_url).dispatch();

//     assert_eq!(res.status(), Status::Ok);

//     let body = res.body_string().unwrap();

//     println!("{:#?}", &body);

//     Ok(())
// }



