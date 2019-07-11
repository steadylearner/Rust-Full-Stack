use std::io;
use rocket::response::NamedFile;

#[get("/web")]
pub fn web() -> io::Result<NamedFile> {
    NamedFile::open("web/index.html")
}

#[get("/index.js")]
pub fn web_index_js() -> io::Result<NamedFile> {
    NamedFile::open("web/index.js")
}

#[get("/index.wasm")]
pub fn web_wasm() -> io::Result<NamedFile> {
    NamedFile::open("web/index.wasm")
}

#[get("/index.css")]
pub fn web_index_css() -> io::Result<NamedFile> {
    NamedFile::open("web/index.css")
}

#[get("/favicon.ico")]
pub fn web_favicon() -> io::Result<NamedFile> {
    NamedFile::open("web/favicon.ico")
}

#[get("/steadylearner.css")]
pub fn steadylearner_css() -> io::Result<NamedFile> {
    NamedFile::open("web/steadylearner.css")
}

#[get("/normalize.css")]
pub fn normalize_css() -> io::Result<NamedFile> {
    NamedFile::open("web/normalize.css")
}




