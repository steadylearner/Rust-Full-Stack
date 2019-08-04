extern crate yew;
extern crate sl_lib;

use sl_lib::Model;
use yew::prelude::App;

fn main() {
  yew::initialize();
  let app: App<Model> = App::new();
  app.mount_to_body();
  yew::run_loop();
}

// <Before/Chat>

// WebSocket
// refer to dashboard example in Yew

// 1. Make a connection - Done

// In Rocket Server
// Accepted a new tcp connection from 127.0.0.1:39660.
// Browser Request from "http://127.0.0.1:8080"
// Client found is None
// 127.0.0.1:39660 entered and the number of live connections is 1

// In Yew Chat Client
// Enter and Exit work with
// console.log("Websocket is ready. Start to chat with others.");
// console.log("The user wants to leave this page.");

// In JavaScript Chat Client
// When Click Enter in Yew client, show message below
// 127.0.0.1:39768 entered and the number of live connections is 2

// 2. receive data from websocket and render it to app? - Done

// Yew(specific state management and its methods)

// 1. send data from Yew app with form in yew(similar to React) and ws_data part work or not- Done
// 2. Should find how to render a message from server in client - Done, work in two different browser and I do not need JavaScript Chat anymore
// 3. Show many messages - Done
// 4. Use JSON instead and id - Done

// Write more features
// 1. Write code for localStorage - Done
// 2. CSS for messages from the user, others and (server with id None) - Done(but use database and user later)
// 3. window to scroll when user type - Done with js! macro
// 4. Include NPM package - Done
// 5. Find how to remove value from input - Done with impl Model or instead use component with js!
// 6. !clear and !clearall for server and client, !exit for client - Done
// 7. Render image - Done

// Read the documentation and code from others

// </Before/Chat>

// Separate project for blog post and write it

// <Rust Full Stack>

// 1. Write post for "How to modulize Rust Frontend" - Done
// 2. Write code for server side and web for chat app with ws-rs - Done

// Payload to send message to web socket in JSON
// WebSocketResponse {
//     value: "This is test",
//     message_type: "text",
// }
// serialized = {"value":"This is test","message_type":"text", number_of_connection":1}

// use localStorage and wrtie code for server

// 4. Write blog post "How to write full stack - Not Yet
// 5. End the "Fullstack Rust Web App" series

// </Rust Full Stack>




// Separate it to side project 

// 1. Security with hashed messages
// 2. User with database
// 3. Edit server parts with database and fetch when user enter the chat
// 4. Login part, form and page for that
// 5. Before connection and after disconnection
// 6. enable wss for https with ws-rs documentation

