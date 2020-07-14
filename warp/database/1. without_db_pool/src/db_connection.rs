// https://github.com/ManifoldFR/rustfullstack/blob/master/src/main.rs

// Refer to this if you want to include r2d2 and use connection pool also
// when you ready to handle the Result with Warp seriously.
// https://github.com/steadylearner/Rust-Full-Stack/blob/master/actix/src/database/db_connection.rs

use diesel::{
    Connection,
    pg::PgConnection,
};
use dotenv::dotenv;
use std::env;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let db_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set.");
    PgConnection::establish(&db_url)
        .expect(&format!("Error connecting to {}", db_url))
}