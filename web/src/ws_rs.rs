use yew::services::websocket::{WebSocketService, WebSocketTask};
use yew::services::Task;
use yew::format::{Json};

use super::{
    http_model::{
        websocket_json::{
            WebSocketRequest,
        }
    },
};

pub struct WebSocket {
    pub ws_service: WebSocketService,
    pub ws: Option<WebSocketTask>,
}


// impl WebSocket here later?

// https://github.com/DenisKolodin/yew/blob/master/examples/dashboard/src/lib.rs
// No Send here because we do not want to bidn our payload to it use whereverw we want
// It also shows type errors difficult to solve
pub enum WebSocketAction {
    Connect,
    Disconnect,
    Lost,
}

impl WebSocket {
    pub fn connect(&mut self, task: WebSocketTask) {
        self.ws = Some(task);
    }

    pub fn send(&mut self, test: Json<&WebSocketRequest>) {
        self.ws.as_mut().unwrap().send_binary(test);
    }

    pub fn disconnect(&mut self) {
        self.ws.take().unwrap().cancel(); // You can use it with Task
    }

    pub fn lost(&mut self) {
        self.ws = None;
    }
}

