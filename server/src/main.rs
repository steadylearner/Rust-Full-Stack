#![feature(proc_macro_hygiene, decl_macro, custom_attribute, rustc_private, type_ascription, async_await)]
#[macro_use] extern crate rocket;

extern crate ws;

extern crate serde_derive;

mod route;
use crate::route::{static_files, get, web};

fn rocket() -> rocket::Rocket {
    let rocket_routes = routes![
        static_files::file,
        //
        get::index,
        //
        web::web,
        web::web_index_js,
        web::web_wasm,
        web::web_index_css,
        web::web_favicon,
        web::steadylearner_css,
        web::normalize_css,
        // npm
        web::browserify,
        web::npm,
    ];

    rocket::ignite()
        .mount("/", rocket_routes)
}

fn main() {
    rocket().launch();
}

