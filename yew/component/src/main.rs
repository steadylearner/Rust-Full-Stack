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

// Separate project for blog post and write it

// 1. Write post for "How to modulize Rust Frontend" - Not Yet
// 2. Write code for server side and web for chat app with ws-rs - already have code for it
// 3. Write blog post "Fullstack Rust Chat App" - Not Yet
// 4. End the "Fullstack Rust Web App" series

// Separate it to side project 

// 1. Type for messages - Done
// 2. Before connection and after close connection
// 3. Edit server parts with database and local storage
// 4. enable wss for https with ws-rs documentation
// 5. Security with hashed messages
// 6. Login part, form and page for that
// 7. Use every variables(no unused variable) - Done

