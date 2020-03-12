// #[macro_use]
extern crate actix_web;
extern crate console;

use actix_web::{web, App, HttpServer, Responder};
use console::Style;

const PORT: &str = "127.0.0.1:8080";

fn index(info: web::Path<(u32, String)>) -> impl Responder {
    format!("Hello {}! id:{}", info.1, info.0)
}

fn main() -> std::io::Result<()> {

    let blue = Style::new()
        .blue();

    println!("\nServer ready at {}", blue.apply_to(format!("http://{}", PORT)));

    HttpServer::new(
        || App::new().service(
              web::resource("/{id}/{name}/index.html").to(index)))
        .bind(&PORT)?
        .run()
}
