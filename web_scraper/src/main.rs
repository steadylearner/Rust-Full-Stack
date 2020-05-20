// specifying we'll be using a macro from
// the prettytable crate (ex: row!())
#[macro_use]
extern crate prettytable;
extern crate reqwest;
extern crate select;

use select::document::Document;
use select::predicate::{Class, Name, Predicate};
use prettytable::Table;

fn main() {
    hacker_news("https://news.ycombinator.com");
}

fn hacker_news(url: &str) {

    let resp = reqwest::get(url).unwrap();
    assert!(resp.status().is_success());

    let document = Document::from_read(resp).unwrap();

    let mut table = Table::new();

    for node in document.find(Class("athing")) {
        let rank = node.find(Class("rank")).next().unwrap();
        let story = node.find(Class("title").descendant(Name("a")))
            .next()
            .unwrap()
            .text();
        let url = node.find(Class("title").descendant(Name("a")))
            .next()
            .unwrap();
        let url_txt = url.attr("href").unwrap();
        // shorten strings to make table aesthetically appealing
        // otherwise table will look mangled by long URLs
        let url_trim = url_txt.trim_left_matches('/');
        let rank_story = format!(" | {} | {}", rank.text(), story);
        // [FdBybl->] specifies row formatting
        // F (foreground) d (black text)
        // B (background) y (yellow text) l (left-align)
        table.add_row(row![FdBybl->rank_story]);
        table.add_row(row![Fy->url_trim]);
    }
    // print table to stdout
    table.printstd();
}