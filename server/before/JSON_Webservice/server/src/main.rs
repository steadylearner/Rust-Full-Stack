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
use rocket_cors::{
    AllowedHeaders, AllowedOrigins, Error,
    Cors, CorsOptions
};

// https://doc.rust-lang.org/rust-by-example/attribute/cfg.html
#[cfg(test)] mod tests;

mod http_model;
mod web_service;
mod route;

use self::{
    web_service::youtube_video::video_search_by_id,
    route::web,
};

use rocket::{routes};

fn make_cors() -> Cors {
    let allowed_origins = AllowedOrigins::some_exact(&[
        "http://localhost:8080",
        "http://127.0.0.1:8080",
        "http://localhost:8000",
        "http://0.0.0.0:8000",
    ]);

    // You can also deserialize this
    // https://lawliet89.github.io/rocket_cors/rocket_cors/struct.CorsOptions.html
    // https://www.w3.org/TR/cors/#resource-processing-model
    CorsOptions {
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
    .to_cors()
    .expect("error while building cors")
}

fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .mount(
            "/",
            routes![
                // Rust JSON Web Service

                video_search_by_id::webservice,

                // Rust Frontend Production Files

                web::web,
                web::web_index_js,
                web::web_wasm,
                web::web_index_css,
                web::web_favicon,
                web::steadylearner_css,
                web::normalize_css,
                web::markdown_css,
                web::modal_css,
                // npm
                web::browserify,
                web::npm,
            ],
        )
        .attach(make_cors())
        // Error: No matching routes for OPTIONS /video_search_by_id/8EPsnf_ZYU0.
        // Warning: Responding with 404 Not Found catcher.
        // then faring with attach handle the route and return data
}

fn main() -> Result<(), Error> {
    rocket().launch();

    Ok(())
}
