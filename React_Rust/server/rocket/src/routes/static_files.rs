use std::path::{Path, PathBuf};
use rocket::response::NamedFile;

// Follow the structure of Express
// It is whether you ignore the serve shows it couldn't find / or /user
// or you edit manually index.html and other paths for images etc.
#[get("/<file..>")]
pub fn file(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("public/").join(file)).ok()
}
