// https://docs.rs/postgres/0.15.2/postgres/
extern crate postgres;
extern crate dotenv;

extern crate chrono;

pub mod user {
    tonic::include_proto!("user");
}

use tonic::{transport::Server};

use user::{
    server::{CrudServer},
};

extern crate uuid;

extern crate console;
use console::Style;

mod db_connection;

mod service;
use crate::service::User;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse().unwrap();
    let user = User::default();

    let blue = Style::new()
        .blue();

    println!("\nRust gRPC Server ready at {}", blue.apply_to(addr));

    // $curl [::1]:50051
    // Should show this.
    // Warning: Binary output can mess up your terminal. Use "--output -" to tell
    // Warning: curl to output it to your terminal anyway, or consider "--output
    // Warning: <FILE>" to save to a file.

    // Use [::1]:50051 for everywhere. https://github.com/uw-labs/bloomrpc

    Server::builder().serve(addr, CrudServer::new(user)).await?;
    Ok(())
}
