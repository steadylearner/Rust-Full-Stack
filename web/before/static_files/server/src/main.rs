#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] 
extern crate rocket;

mod routes;
use crate::routes::{ static_files, get };

fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .mount(
            "/",
            routes![
                static_files::file,
                get::index,
                get::single_page_app,
                get::favicon,
            ],
        )
}

fn main() {
    rocket().launch();
}


