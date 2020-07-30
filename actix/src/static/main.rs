extern crate actix_web;
extern crate actix_files;
extern crate console;

use actix_files as fs;
use actix_web::{App, HttpServer, middleware};

use console::Style;

const PORT: &str = "0.0.0.0:8000";

pub fn main() {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    let blue = Style::new()
        .blue();

    println!("\nServer ready at {}", blue.apply_to(format!("http://{}", PORT)));

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .service(fs::Files::new("/static", "./src/static/static").show_files_listing())
            // .service(fs::Files::new("/static", "./src/static/static")) // Unable to render directory without index file
    })
    .bind(&PORT)
    .unwrap()
    .run()
    .unwrap();
}
