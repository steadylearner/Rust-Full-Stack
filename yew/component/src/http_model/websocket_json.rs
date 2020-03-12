use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct WebSocketRequest {
    pub value: String,
    pub message_type: String, // copy self.state.message_type of client here and send
}
