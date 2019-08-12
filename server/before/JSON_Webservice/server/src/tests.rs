use crate::rocket;
use crate::serde_json;
use crate::http_model::youtube_video::Video;

use rocket::local::Client;
use rocket::http::{Status, ContentType};


// Use this or just test it with curl
// Start your Rust Rocket App => $cargo run --release
// invalid_youtube_id => $curl http://localhost:8000/video_search_by_id/invalid_youtube_id
// valid_youtube_id => $curl http://localhost:8000/video_search_by_id/s7TVVyTyReU

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

    let valid_youtube_id = "s7TVVyTyReU";
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
    assert_eq!(payload.snippet.title, "Your Love - The Outfield");
}


