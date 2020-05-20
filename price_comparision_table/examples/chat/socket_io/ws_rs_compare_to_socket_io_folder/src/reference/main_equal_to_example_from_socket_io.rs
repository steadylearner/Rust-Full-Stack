/// An example of a chat web application server
extern crate ws;
use ws::{listen, Handler, Message, Request, Response, Result, Sender, CloseCode, Error};

// This can be read from a file(index.html)
// move this role for Rocket application or extract this part to .html file

// should read more at https://developer.mozilla.org/en-US/docs/Web/API/WebSocket
// and https://developer.mozilla.org/en-US/docs/Web/API/WebSocket/send
 
static INDEX_HTML: &'static [u8] = br#"
<!doctype html>
<html>

<head>
  <title>Websocket Rust chat</title>
  <!-- <link rel="stylesheet" href="/main.css"> -->
  <style>
    * {
      margin: 0;
      padding: 0;
      box-sizing: border-box;
    }

    body {
      font: 13px Helvetica, Arial;
    }

    form {
      background: #000;
      padding: 3px;
      position: fixed;
      bottom: 0;
      width: 100%;
    }

    form input {
      border: 0;
      padding: 10px;
      width: 90%;
      margin-right: .5%;
    }

    form button {
      width: 9%;
      background: rgb(130, 224, 255);
      border: none;
      padding: 10px;
    }

    #messages {
      list-style-type: none;
      margin: 0;
      padding: 0;
    }

    #messages li {
      padding: 5px 10px;
    }

    #messages li:nth-child(odd) {
      background: #eee;
    }

  </style>
</head>

<body>
  <ul id="messages"></ul>
  <form id="form">
    <input type="text" id="msg" autocomplete="off" >
    <button>Send</button>
  </form>
</body>

<script>
  const socket = new WebSocket("ws://" + window.location.host + "/ws");

  const form = document.getElementById("form");
  form.addEventListener('submit', function (event) {
    event.preventDefault();
    const input = document.getElementById("msg");
    socket.send(input.value);
    input.value = "";
  });

  socket.addEventListener('open', function (event) {
    socket.send('Hello Server. Please send this message back to me!');
  });

  socket.onmessage = function (event) {
    console.log(`${event.data} from ${event.origin}`);
    const messages = document.getElementById("messages");
    const li = document.createElement("li");
    li.append(event.data)
    messages.append(li);
  };

  // verify it work
  socket.onclose = function(event) {
    console.log("WebSocket is closed now.");
  };

</script>

</html>
    "#;

// <script src="https://code.jquery.com/jquery-1.11.1.js"></script>

// Server web application handler
struct Server {
    out: Sender,
}

impl Handler for Server {
    //
    fn on_request(&mut self, req: &Request) -> Result<(Response)> {
        // Using multiple handlers is better (see router example)
        match req.resource() {
            // The default trait implementation
            "/ws" => {
              // used once for const socket = new WebSocket("ws://" + window.location.host + "/ws");
              // https://blog.stanko.io/do-you-really-need-websockets-343aed40aa9b
              // and no need for reconnet later
              // println!("{:?} \n", req);
              let resp = Response::from_request(req);
              // println!("{:?} \n", &resp);
              resp
            },

            // Create a custom response
            "/" => Ok(Response::new(200, "OK", INDEX_HTML.to_vec())), // move this for Rocket application

            _ => Ok(Response::new(404, "Not Found", b"404 - Not Found".to_vec())), // move this for Rocket application
        }
    }

    // Handle messages recieved in the websocket (in this case, only on /ws)
    fn on_message(&mut self, msg: Message) -> Result<()> {
        println!("The message from the client is {:?}", &msg);
        // Broadcast to all connections
        self.out.broadcast(msg)
    }

    // fn on_close(&mut self, code: CloseCode, reason: &str) {
    //    println!("Socket Closed. Code Type was : {:?}. Reason was: {:?}.", code, reason);
    // }

    // verify it work
    fn on_close(&mut self, code: CloseCode, reason: &str) {
       match code {
            CloseCode::Normal => println!("The client is done with the connection."),
            CloseCode::Away   => println!("The client is leaving the site."),
            _ => println!("The client encountered an error: {}", reason),
      }
    }

    fn on_error(&mut self, err: Error) {
        println!("The server encountered an error: {:?}", err);
    }
}

fn main() {
    // Listen on an address and call the closure for each connection
    println!("Web Socket Server is ready at ws://127.0.0.1:8000/ws");
    println!("Server is ready at http://127.0.0.1:8000/");
    listen("127.0.0.1:8000", |out| Server { out }).unwrap();
}

    // to use WS with rocket you have to include it inside thread and use different port from Rocket

    // thread::spawn(|| {
    // println!("Web Socket Server is ready at ws://127.0.0.1:8000/ws");
    // println!("Server is ready at http://127.0.0.1:8000/");
    // listen("127.0.0.1:8000", |out| Server { out }).unwrap();
    // })
