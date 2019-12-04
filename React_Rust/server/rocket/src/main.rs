#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

mod routes;
use crate::routes::{ static_files, get };

fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .mount(
            "/",
            routes![
                get::index,
                get::user,
                static_files::file,
            ],
        )
}

fn main() {
    rocket().launch();
}


