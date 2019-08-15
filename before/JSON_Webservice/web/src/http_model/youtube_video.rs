
// https://serde.rs/attr-rename.html
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Video {
    pub items: Option<Vec<VideoItem>>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct VideoItem { // should be VideosItem
    pub id: String,
    pub snippet: Snippet,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Snippet {
    pub published_at: String,
    pub channel_id: String,
    pub title: String,
    pub description: Option<String>,
    pub thumbnails: Thumbnails,
    pub channel_title: String,
    pub tags: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Thumbnails {
    pub default: Thumbnail,
    pub high: Thumbnail,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Thumbnail {
    pub url: String,
    pub width: u16,
    pub height: u16,
}


