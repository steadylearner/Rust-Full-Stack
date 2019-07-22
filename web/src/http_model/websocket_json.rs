use serde_derive::{Deserialize, Serialize};

// Option<String> for 'client' because before a client connect to WebSocket server there is no information for that(None)

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