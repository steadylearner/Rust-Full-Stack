use std::io;
use std::path::{Path, PathBuf};
use rocket::response::{NamedFile};

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

#[get("/markdown.css")]
pub fn markdown_css() -> io::Result<NamedFile> {
    NamedFile::open("web/markdown.css")
}

// For browserify and NPM to work and it is optional

#[get("/bundle.js")]
pub fn browserify() -> io::Result<NamedFile> {
    NamedFile::open("web/bundle.js")
}

#[get("/node_modules/<file..>")]
pub fn npm(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("web/node_modules/").join(file)).ok()
}



