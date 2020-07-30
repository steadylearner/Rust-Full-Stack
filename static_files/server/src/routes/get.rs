use std::io;
use rocket::response::{NamedFile, Redirect};

#[get("/")]
pub fn index() -> io::Result<NamedFile> {
    NamedFile::open("static/index.html")
}

#[get("/single_page_app")]
pub fn single_page_app() -> io::Result<NamedFile> {
    NamedFile::open("static/single_page_app/index.html")
}

#[get("/favicon.ico")]
pub fn favicon() -> Redirect {
    Redirect::to("/static/steadylearner_favicon.png")
}


