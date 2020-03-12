// https://docs.rs/postgres/0.15.2/postgres/
extern crate postgres;
extern crate redis;
extern crate dotenv;

extern crate chrono;

pub mod user {
    tonic::include_proto!("user");
}

use tonic::{transport::Server};

use user::{
    user_service_server::{UserServiceServer},
};

extern crate uuid;

extern crate console;
use console::Style;

mod db_connection;

mod service;
use crate::service::User;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "0.0.0.0:50051".parse().unwrap();
    let user = User::default();

    let blue = Style::new()
        .blue();

    println!("\nRust gRPC Server ready at {}", blue.apply_to(addr));

    Server::builder()
        .add_service(UserServiceServer::new(user))
        .serve(addr)
        .await?;

    Ok(())
}


