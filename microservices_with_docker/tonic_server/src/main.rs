// https://docs.rs/postgres/0.15.2/postgres/
extern crate postgres;
extern crate redis;
extern crate serde;
extern crate serde_json;
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
use crate::service::user::postgresql_grpc::{
    User,
};

// Read all the documenation of redis crate.($cargo doc -p redis --open)

// Refer to these
// 1. http://zsiciarz.github.io/24daysofrust/book/vol1/day18.html
// 2. https://github.com/actix/examples/blob/master/redis-session/src/main.rs
// 3. https://github.com/gabisurita/fullstack-rust/blob/master/server/src/repository.rs
// 4. Test this function https://docs.rs/redis/0.13.0/redis/trait.Commands.html#method.getset

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

// 1. Read serde documenation. Find how to use macro instead of manual implementation at server/user/redis.
// 2. How to compare the response time from Redis and Postgresql programmatically with Rust and not CURL.
// 3. Read more redis documentation.
// 4. Deploy it.

