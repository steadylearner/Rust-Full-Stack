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

use std::cell::Cell;
use std::rc::Rc;

// Server web application handler
struct Server {
    out: Sender,
    count: Rc<Cell<u32>>,
}

impl Handler for Server {
    fn on_request(&mut self, req: &Request) -> Result<(Response)> {
        match req.resource() {
            "/ws" => {
                // used once for const socket = new WebSocket("ws://" + window.location.host + "/ws");
                // https://blog.stanko.io/do-you-really-need-websockets-343aed40aa9b
                // and no need for reconnect later

                // https://ws-rs.org/api_docs/ws/struct.Request.html
                println!("Browser Request from {:?}", req.origin().unwrap().unwrap());
                println!("Client found is {:?}", req.client_addr().unwrap());
                let resp = Response::from_request(req);
                // println!("{:?} \n", &resp);
                resp
            }

            _ => Ok(Response::new(404, "Not Found", b"404 - Not Found".to_vec())),
        }
    }

    fn on_open(&mut self, handshake: Handshake) -> Result<()> {
        // We have a new connection, so we increment the connection counter
        self.count.set(self.count.get() + 1);
        let number_of_connection = self.count.get();

        if number_of_connection > 5 {
            // panic!("There are more user connection than expected.");
        }
        
        // The most important part and used to assign id for clients
        // println!("{}", &handshake.local_addr.unwrap());
        let open_message = format!("{} entered and the number of live connections is {}", &handshake.peer_addr.unwrap(), &number_of_connection);

        println!("{}", &open_message);
        self.out.broadcast(open_message);

        Ok(())
    }

    // Handle messages recieved in the websocket (in this case, only on /ws)
    fn on_message(&mut self, message: Message) -> Result<()> {
        let raw_message = message.into_text()?;
        println!("The message from the client is {:#?}", &raw_message);

        let message = if raw_message.contains("!warn") {
            let warn_message = "One of the clients sent warning to the server.";
            println!("{}", &warn_message);
            Message::Text("There was warning from another user.".to_string())
        } else {
            Message::Text(raw_message)
        };

        // Broadcast to all connections
        self.out.broadcast(message)
    }

    fn on_close(&mut self, code: CloseCode, reason: &str) {
        match code {
            CloseCode::Normal => println!("The client is done with the connection."),
            CloseCode::Away => println!("The client is leaving the site."),
            CloseCode::Abnormal => {
                println!("Closing handshake failed! Unable to obtain closing status from client.")
            },
            _ => println!("The client encountered an error: {}", reason),
        }
        self.count.set(self.count.get() - 1)
    }

    fn on_error(&mut self, err: Error) {
        println!("The server encountered an error: {:?}", err);
    }
}

pub fn websocket() -> () {
  println!("Web Socket Server is ready at ws://127.0.0.1:7777/ws");
  println!("Server is ready at http://127.0.0.1:7777/");

  // Rc is a reference-counted box for sharing the count between handlers
  // since each handler needs to own its contents.
  // Cell gives us interior mutability so we can increment
  // or decrement the count between handlers.

  // Listen on an address and call the closure for each connection
  let count = Rc::new(Cell::new(0));
  listen("127.0.0.1:7777", |out| { Server { out: out, count: count.clone(), } }).unwrap()
}