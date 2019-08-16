// https://github.com/SergioBenitez/Rocket/tree/master/examples/json
#![feature(proc_macro_hygiene, decl_macro)] // should include decl_macro different from the current example

#[macro_use]
extern crate rocket;
// #[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

extern crate dotenv;
extern crate reqwest;

// https://doc.rust-lang.org/rust-by-example/attribute/cfg.html
#[cfg(test)] mod tests;
mod http_model;
mod web_service;

use self::{
    web_service::{
        youtube_video::video_search_by_id,
        post,
    },
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

fn main() {
    rocket().launch();
}

