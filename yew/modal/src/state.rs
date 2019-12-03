// similar to WebSocketRequest in http_model/websocket_json.rs
// should make folder for state?

#[derive(Debug)]
pub struct State {
  pub ws_responses: Vec<Option<String>>, // should be Vec<String> or Vec<Option<String>> to save messages
  pub message_type: String,
  pub client: Option<String>,
}

impl State {
    pub fn lost(&mut self) {
        self.client = None;
    }
}