use yew::services::websocket::{WebSocketService, WebSocketTask};

pub struct WebSocket {
    pub ws_service: WebSocketService,
    pub ws: Option<WebSocketTask>,
    // link: ComponentLink<Model>,
}

// impl WebSocket here later?

pub enum WebSocketAction {
    Connect,
    Disconnect,
    Lost,
}

