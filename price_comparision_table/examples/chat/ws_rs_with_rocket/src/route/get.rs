use std::io;
use rocket::response::{NamedFile};

#[get("/")]
pub fn index() -> &'static str {
   "Build a Rust chat app with Steadylearner(Visit http://localhost:8000/chat)"
}

#[get("/chat")]
pub fn chat() -> io::Result<NamedFile> {
    NamedFile::open("static/chat/index.html")
}



