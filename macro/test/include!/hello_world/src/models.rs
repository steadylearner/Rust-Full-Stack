use serde::{Deserialize, Serialize};

// Use base here later.

#[derive(Debug, Serialize, Deserialize)]
pub struct MessageCreateRequest {
    pub text: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MessageCreateReply {
    pub text: String
}
