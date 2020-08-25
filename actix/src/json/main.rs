// #[macro_use]
extern crate actix_web;
// #[macro_use]
extern crate json;
extern crate console;

use actix_web::{
    // error,
    middleware, web, App,
    // Error,
    HttpServer,
};

use console::Style;

mod handlers;
use crate::handlers::{json_test::index};

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
            .data(web::JsonConfig::default().limit(4096)) // <- limit size of the payload (global configuration)
            .service(
                web::resource("/")
                    .data(web::JsonConfig::default().limit(1024)) // <- limit size of the payload (resource level)
                    .route(web::post().to_async(index)),
            )
    })
    .bind(&PORT)?
    .run()
}

