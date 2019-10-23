// https://dev.to/werner/practical-rust-web-development-api-rest-29g1
// https://dev.to/werner/practical-rust-web-development-connection-pool-46f4
// The author should have included 
/// ```
/// use db_connection::establish_connection; 
/// .data(establish_connection()) 
/// ```

// #[macro_use]
extern crate actix_web;
// #[macro_use]
extern crate json;
extern crate console;
#[macro_use]
extern crate diesel;
extern crate dotenv;

use actix_web::{
    // error,
    middleware, web, App,
    // Error,
    HttpServer,
};

use console::Style;

mod schema;
mod models;
mod db_connection;

use db_connection::establish_connection;

pub mod handlers; // This goes to the top to load the next handlers module 

extern crate serde;
extern crate serde_json;
#[macro_use] 
extern crate serde_derive;

const PORT: &str = "127.0.0.1:8080";

fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
  
    let blue = Style::new()
        .blue();

    println!("\nServer ready at {}", blue.apply_to(format!("http://{}", PORT)));

    HttpServer::new(|| {
        App::new()
            // enable logger
            .wrap(middleware::Logger::default())
            .data(establish_connection())
            .service(
                web::resource("/products")
                    .route(web::get().to(handlers::products::index))
                    .route(web::post().to(handlers::products::create))
            )
             .service(
                web::resource("/products/{id}")
                    .route(web::get().to(handlers::products::show))
                    .route(web::delete().to(handlers::products::destroy))
                    .route(web::patch().to(handlers::products::update))
            )
    })
    .bind(&PORT)?
    .run()
}

