// https://github.com/SergioBenitez/Rocket/tree/master/examples/json
#![feature(proc_macro_hygiene, decl_macro)] // should include decl_macro different from the current example

#[macro_use]
extern crate rocket;
// #[macro_use]
extern crate rocket_contrib;
// https://github.com/lawliet89/rocket_cors/blob/master/examples/fairing.rs#L13
extern crate rocket_cors; // need this to Yew Fetch API OPTIONS CORS routes to work with JSON Web Service

#[macro_use]
extern crate serde_derive;
extern crate serde_json;

extern crate dotenv;
extern crate reqwest;

use rocket::http::Method;
use rocket_cors::{AllowedHeaders, AllowedOrigins, Error};

// https://doc.rust-lang.org/rust-by-example/attribute/cfg.html
#[cfg(test)] mod tests;
mod http_model;
mod web_service;

use self::{
    web_service::youtube_video::video_search_by_id,
};

use rocket::{get, routes};

#[get("/")]
fn hello() -> &'static str {
    "Hello from www.steadylearner.com"
}

fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .mount(
            "/",
            routes![
                hello,
                video_search_by_id::webservice,
            ],
        )
}

fn main() -> Result<(), Error> {
    let allowed_origins = AllowedOrigins::some_exact(&["http://localhost:8080"]);

    // You can also deserialize this
    // https://lawliet89.github.io/rocket_cors/rocket_cors/struct.CorsOptions.html
    // https://www.w3.org/TR/cors/#resource-processing-model
    let cors = rocket_cors::CorsOptions {
        allowed_origins,
        allowed_methods: vec![Method::Get].into_iter().map(From::from).collect(),
        allowed_headers: AllowedHeaders::some(&[
            "Authorization",
            "Accept",
            "Access-Control-Allow-Origin", // copy and paste what you used in web folder
        ]),
        allow_credentials: true,
        ..Default::default()
    }
    .to_cors()?;

    rocket()
        .attach(cors)
        .launch();

    Ok(())
}
