use std::io;
use rocket::response::{NamedFile};

#[get("/")]
pub fn index() -> &'static str {
   "Rust Webassembly at /web and equivalent JavaScript at /chat"
}

#[get("/chat")]
pub fn chat() -> io::Result<NamedFile> {
    NamedFile::open("static/chat/index.html")
}


