use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct WebSocketRequest {
    pub value: String,
    pub user_id: Option<String>,
    // pub type: Enum(text, image, video, code etc)
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WebSocketResponse {
    pub value: String,
    pub client: Option<String>,
    // pub type: Enum(text, image, video, code etc)
    pub number_of_connection: u32,
}

// Use them to send and receive data from websocket
