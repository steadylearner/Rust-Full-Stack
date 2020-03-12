// https://serde.rs/derive.html
use serde_derive::{Deserialize, Serialize};

// Into WebSocket - Message
#[derive(Serialize, Deserialize, Debug)]
pub struct WebSocketRequest {
    pub value: String,
    pub message_type: String,
    pub client: Option<String>,
}

// From WebSocket - Broadcast
#[derive(Serialize, Deserialize, Debug)]
pub struct WebSocketResponse {
    pub value: String,
    pub message_type: String,
    pub client: Option<String>,
    
    // from websocket
    pub number_of_connection: u32,
}
