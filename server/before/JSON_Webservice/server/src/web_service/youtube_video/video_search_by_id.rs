use dotenv::dotenv;
use std::env;

use rocket_contrib::json::{
    Json,
    // JsonValue
};

use crate::http_model::youtube_video::Video; // This is relevant to mod http_model; in main.rs

#[get("/video_search_by_id/<q>")] // 8EPsnf_ZYU0
pub fn webservice(q: String) -> Result<Json<Video>, reqwest::Error> {
    dotenv().ok();
    let youtube_key = env::var("YOUTUBE_KEY").expect("Should be set already");

    let request_url = format!("https://www.googleapis.com/youtube/v3/videos?part=snippet&id={}&key={}",
        q, youtube_key
    );

    println!("{}", request_url);

    let video_search_by_id: Video = reqwest::get(&request_url)?.json()?;

    println!("{:?}", &video_search_by_id);

    Ok(Json(video_search_by_id))
}
