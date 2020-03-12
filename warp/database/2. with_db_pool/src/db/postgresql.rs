// Refer to https://github.com/mehmetsefabalik/rust-mongodb-example if you want to use mongodb.

// Let these codes to use PostgresqlPool and inside main.rs to make them reusable in the project.
use dotenv::dotenv;
use std::env;

// https://github.com/sfackler/r2d2https://github.com/sfackler/r2d2
// https://docs.diesel.rs/diesel/r2d2/struct.ConnectionManager.html
// https://docs.diesel.rs/diesel/pg/struct.PgConnection.html
use diesel::r2d2::ConnectionManager;
use diesel::PgConnection;

// https://docs.rs/lazy_static/1.4.0/lazy_static/
// https://www.google.com/search?&q=why+use+lazy_static+rust
// Use this instead of calling estabilish_connection everytime.
// Compare it with 1. wihout_db_pool example/
pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;
lazy_static! {
    pub static ref POOL: Pool = {
        // DATABASE_URL=postgres://postgres:postgres@localhost/warp
        dotenv().ok();

        let database_url = env::var("DATABASE_URL")
            .expect("DATABASE_URL must be set.");

        // create db connection pool
        let manager = ConnectionManager::<PgConnection>::new(database_url);
        let pool: Pool = r2d2::Pool::builder()
            .build(manager)
            .expect("Failed to create pool.");
        pool
    };
}
//
