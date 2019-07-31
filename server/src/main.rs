#![feature(proc_macro_hygiene, decl_macro, custom_attribute, rustc_private, type_ascription, async_await)]
#[macro_use] extern crate rocket;

extern crate ws;

extern crate serde_derive;

mod route;
use crate::route::{static_files, get, web};

use std::thread;

mod chat;
use crate::chat::ws_rs;

mod http_model; // should be here to use super::super::http_model in chat/ws_rs.rs

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
        web::markdown_css,
        web::modal_css,
        // npm
        web::browserify,
        web::npm,
    ];

    rocket::ignite()
        .mount("/", rocket_routes)
}

fn main() {
        thread::Builder::new()
        .name("Thread for Rust Chat with ws-rs".into())
        // .stack_size(83886 * 1024) // 80mib in killobytes
        .spawn(|| {
            ws_rs::websocket();
        })
        .unwrap();

    rocket().launch();
}

