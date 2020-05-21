// Of course, we need to import the `doc_comment` macro:
#[macro_use]
extern crate doc_comment;

fn main() {
    doctest!("../README.md");
}
