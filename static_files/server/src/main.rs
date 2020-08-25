#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
// #[macro_use] extern crate serde_derive;

mod routes;
use crate::routes::{ static_files, get, error };
use rocket_contrib::templates::Template;

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
        .attach(Template::fairing())
        .register(catchers![error::not_found])
}

fn main() {
    rocket().launch();
}


