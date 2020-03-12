use ws::{
    listen,
    CloseCode,
    Error,
    Handler,
    Handshake,
    Message,
    Request,
    Response,
    Result,
    Sender,
};

use std::{
    cell::Cell,
    rc::Rc
};

// Websocket handler
struct WebSocket {
    out: Sender,
    count: Rc<Cell<u32>>,
}

use super::super::http_model;

use self::{
    http_model::{
        websocket_json::{
            WebSocketRequest,
            WebSocketResponse,
        },
    },
};

impl Handler for WebSocket {
    // Edit here only when you are an expert with this.
    // used once for const socket = new WebSocket("ws://" + window.location.host + "/ws");
    // https://blog.stanko.io/do-you-really-need-websockets-343aed40aa9b
    // and no need for reconnect later

    // https://ws-rs.org/api_docs/ws/struct.Request.html
    fn on_request(&mut self, req: &Request) -> Result<(Response)> {
        match req.resource() {
            "/ws" => {

                println!("Browser Request from {:?}", req.origin().unwrap().unwrap());
                println!("Client found is {:?}", req.client_addr().unwrap());
                let resp = Response::from_request(req);
                resp
            }

            _ => Ok(Response::new(404, "Not Found", b"404 - Not Found".to_vec())),
        }
    }

    //

    fn on_open(&mut self, handshake: Handshake) -> Result<()> {
        // We have a new connection, so we increment the connection counter
        self.count.set(self.count.get() + 1);
        let number_of_connection = self.count.get();

        // if number_of_connection > 5 {
        //     println!("Use client instead")
        // }

        let client = handshake.peer_addr.expect("Websocket failed to locate the client").to_string();

        let ws_resp = WebSocketResponse {
            value: format!("I want to use a chat app completely built in Rust.").to_string(),
            message_type: "text".to_string(), 
            // message_type: "server".to_string(), or manipulate at client 
            client: Some(client), // we need this at this point to differenciate users
            number_of_connection,
        };

        println!("{:#?}", &ws_resp);
        let serialized = serde_json::to_string(&ws_resp).unwrap();
        println!("serialized = {}", serialized);

        self.out.broadcast(serialized)
    }

    // Handle messages recieved in the websocket (in this case, only on /ws)
    fn on_message(&mut self, message: Message) -> Result<()> {
        let number_of_connection = self.count.get();

        let raw_message = message.into_text()?;
        println!("The message from the client is {:#?}", &raw_message);

        let ws_request: WebSocketRequest = serde_json::from_str(&raw_message).unwrap();
        println!("ws_request = {:?}", &ws_request);
        let WebSocketRequest { value, message_type, client } = ws_request;

        let client = if value == "!clearall" { // remove "" with value.is_empty() later
            None // "WebSocket" instead?
        } else {
            client
        };

        let ws_resp = WebSocketResponse {
            value,
            message_type,
            client,
            number_of_connection,
        };

        println!("{:#?}", &ws_resp);
        let serialized = serde_json::to_string(&ws_resp).unwrap();
        println!("serialized = {}", serialized);

        self.out.broadcast(serialized)
    }

    // Edit on_close and on_error after you complete your chat app

    fn on_close(&mut self, code: CloseCode, reason: &str) {
        match code {
            CloseCode::Normal => println!("The client is done with the connection."),
            CloseCode::Away => println!("The client is leaving the site."),
            CloseCode::Abnormal => {
                println!("Closing handshake failed! Unable to obtain closing status from client.")
            },
            _ => println!("The client encountered an error: {}", reason),
        }
        self.count.set(self.count.get() - 1);
    }

    fn on_error(&mut self, err: Error) {
        println!("The WebSocket encountered an error: {:?}", err);
        self.count.set(self.count.get() - 1);
    }
}

// Rc is a reference-counted box for sharing the count between handlers
// since each handler needs to own its contents.
// Cell gives us interior mutability so we can increment
// or decrement the count between handlers.

// Listen on an address and call the closure for each connection

pub fn websocket() -> () {
  println!("Web Socket WebSocket is ready at ws://127.0.0.1:7777/ws");
  println!("WebSocket is ready at http://127.0.0.1:7777/");

  let count = Rc::new(Cell::new(0));
  listen("127.0.0.1:7777", |out| { WebSocket { out: out, count: count.clone(), } }).unwrap()
}
