extern crate actix_web;
extern crate actix_files;
extern crate console;

use actix_files as fs;
use actix_web::{App, HttpServer, web, middleware, Result, Responder};

use std::path::PathBuf;

use console::Style;

fn single_page_app() -> Result<fs::NamedFile> {
    // https://doc.rust-lang.org/std/path/struct.PathBuf.html
    let path: PathBuf = PathBuf::from("./public/index.html");
    Ok(fs::NamedFile::open(path)?)
}

// Use this later 
// use std::env;

// target = match env::var("SERVER_HOST") {
//    Ok(host) => host,
//    Err(_e) => "0.0.0.0:8000".to_string(),
// };

// Use only 0.0.0.0 when you use Docker [localhost, 0.0.0.0, 127.0.0.1]
pub fn main() {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    let blue = Style::new()
        .blue();

    // https://github.com/alextanhongpin/rust-actix-docker/blob/master/src/main.rs
    // "0.0.0.0:8000" 
    let prefix = "0.0.0.0:"; // // Use 0.0.0.0 instead of localhost or 127.0.0.1 to use Actix with docker
    let port = 8000; // We will use 80 for aws with env variable.
    let target = format!("{}{}", prefix, port);

    println!("\nServer ready at {}", blue.apply_to(format!("http://{}",&target)));

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            // static files https://docs.rs/actix-files/0.1.7/actix_files/struct.Files.html
            // .service(fs::Files::new("/", "./public")).show_files_listing() // Use this to develop
            // Sequence here is important, you won't have file with / or /user, so it will work without problem
            .route("/", web::get().to(single_page_app))
            .route("/user", web::get().to(single_page_app))
            .service(fs::Files::new("/", "./public").index_file("index.html"))
    })
    .bind(&target) // Separate prefix, port, target, println! not to show "Not registered service error"
    .unwrap()
    .run()
    .unwrap();
}
