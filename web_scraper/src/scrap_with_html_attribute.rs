// https://kadekillary.work/post/webscraping-rust/

// Grabbing All Links with html attribute

extern crate reqwest;
extern crate select;

use select::document::Document;
use select::predicate::Name;

fn main() {
    hacker_news("https://news.ycombinator.com");
}

fn hacker_news(url: &str) {
    let resp = reqwest::get(url).unwrap();
    assert!(resp.status().is_success());

    Document::from_read(resp)
        .unwrap()
        .find(Name("a"))
        .filter_map(|n| n.attr("href"))
        .for_each(|x| println!("{}", x));
}