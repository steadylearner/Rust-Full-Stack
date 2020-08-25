// More Than One Attribute

extern crate reqwest;
extern crate select;

use select::document::Document;
use select::predicate::{Class, Name, Predicate};

fn main() {
    hacker_news("https://news.ycombinator.com");
}

fn hacker_news(url: &str) {

    let resp = reqwest::get(url).unwrap();
    assert!(resp.status().is_success());

    let document = Document::from_read(resp).unwrap();

    // finding all instances of our class of interest
    for node in document.find(Class("athing")) {
        // grabbing the story rank
        let rank = node.find(Class("rank")).next().unwrap();
        // finding class, then selecting article title
        let story = node.find(Class("title").descendant(Name("a")))
            .next()
            .unwrap()
            .text();
        // printing out | rank | story headline
        println!("\n | {} | {}\n", rank.text(), story);
        // same as above
        let url = node.find(Class("title").descendant(Name("a"))).next().unwrap();
        // however, we don't grab text
        // instead find the "href" attribute, which gives us the url
        println!("{:?}\n", url.attr("href").unwrap());
    }
}
