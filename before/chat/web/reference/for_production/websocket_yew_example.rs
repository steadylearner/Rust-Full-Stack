#![recursion_limit = "256"]

// https://github.com/DenisKolodin/yew/blob/master/src/services/websocket.rs and crates/shared/src/callback.rs
// https://github.com/koute/stdweb/blob/master/src/webapi/web_socket.rs

use failure::Error;

use yew::{html, Component, ComponentLink, Html, Renderable, ShouldRender};
use yew::services::Task; // cancelable
use yew::services::websocket::{WebSocketService, WebSocketTask, WebSocketStatus};

type UseBinary = bool; // You can send either binary or text itself and ws_rs receive binary data

pub struct Model {
    link: ComponentLink<Model>,
    fetching: bool,
    ws_service: WebSocketService,
    ws: Option<WebSocketTask>,
}

pub enum WebSocket {
    Connect,
    SendData(UseBinary),
    Disconnect,
    Lost,
}

pub enum Msg {
    WebSocket(WebSocket),
    WebSocketReady(Result<WebSocketResponse, Error>),
    Ignore,
}

impl From<WebSocket> for Msg {
    fn from(action: WebSocket) -> Self {
        Msg::WebSocket(action)
    }
}

/// This type is used as a request which sent to websocket connection.
#[derive(Serialize, Debug)]
struct WebSocketRequest {
    value: u32,
}

/// This type is an expected response from a websocket connection.
#[derive(Deserialize, Debug)]
pub struct WebSocketResponse {
    value: u32,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Model {
            WebSocket_service: WebSocketService::new(),
            WebSocket: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::WebSocket(action) => {
                match action {
                    WebSocket::Connect => {
                        let callback = self.link.send_back(|Json(data)| Msg::WebSocketReady(data));
                        let notification = self.link.send_back(|status| {
                            match status {
                                WebSocketStatus::Opened => Msg::Ignore,
                                WebSocketStatus::Closed | WebSocketStatus::Error => WebSocket::Lost.into(),
                            }
                        });
                        let task = self.WebSocket_service.connect("WebSocket://localhost:9001/", callback, notification);
                        self.WebSocket = Some(task);
                    }
                    WebSocket::SendData(binary) => {
                        let request = WebSocketRequest {
                            // value: 321, payload?
                        };
                        if binary {
                            self.WebSocket.as_mut().unwrap().send_binary(Json(&request));
                        } else {
                            // self.WebSocket.as_mut().unwrap().send(Json(&request));
                        }
                    }
                    WebSocket::Disconnect => {
                        self.WebSocket.take().unwrap().cancel();
                    }
                    WebSocket::Lost => {
                        self.WebSocket = None;
                    }
                }
            }
            Msg::FetchReady(response) => {
                self.fetching = false;
                self.data = response.map(|data| data.value).ok();
            }
            Msg::WebSocketReady(response) => {
                self.data = response.map(|data| data.value).ok();
            }
            Msg::Ignore => {
                return false;
            }
        }
        true
    }
}

impl Renderable<Model> for Model {
    fn view(&self) -> Html<Self> {
        html! {
            <div>
                <nav class="menu",>
                    <button disabled=self.WebSocket.is_some(),
                            onclick=|_| WebSocket::Connect.into(),>{ "Connect To WebSocket" }</button>
                    <button disabled=self.WebSocket.is_none(),
                            onclick=|_| WebSocket::SendData(false).into(),>{ "Send To WebSocket" }</button>
                    <button disabled=self.WebSocket.is_none(),
                            onclick=|_| WebSocket::SendData(true).into(),>{ "Send To WebSocket [binary]" }</button>
                    <button disabled=self.WebSocket.is_none(),
                            onclick=|_| WebSocket::Disconnect.into(),>{ "Close WebSocket connection" }</button>
                </nav>
            </div>
        }
    }

}

// impl Model {
//     fn view_data(&self) -> Html<Model> {
//         if let Some(value) = self.data {
//             html! {
//                 <p>{ value }</p>
//             }
//         } else {
//             html! {
//                 <p>{ "Data hasn't fetched yet." }</p>
//             }
//         }
//     }
// }