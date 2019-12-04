use std::io;
use rocket::response::{NamedFile};

// #[get("/"), get("/user")], would be decent have API similar to this
// Then, we could use pub fn single_page_app().
// It will help this framework be more compatible with Single Page Apps
// Compare it with Actix.
#[get("/")]
pub fn index() -> io::Result<NamedFile> {
    NamedFile::open("public/index.html")
}

#[get("/user")]
pub fn user() -> io::Result<NamedFile> {
    NamedFile::open("public/index.html")
}



