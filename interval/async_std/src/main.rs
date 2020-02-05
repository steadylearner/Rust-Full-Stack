extern crate async_std;

use async_std::prelude::*;
use async_std::stream;
use async_std::task;

use std::time::Duration;

async fn renew_sitemaps_each_day() {
    // Refer to this and make a microservice for simtemaps of your website.
    // https://github.com/steadylearner/Rust-Full-Stack/tree/master/sitemap

    let day_in_secs = 1; // Use 1 for testing, 86400 for production.
    let mut interval_day = stream::interval(Duration::from_secs(day_in_secs));
    while let Some(_) = interval_day.next().await {
        println!("Renew sitemaps each day.");
    }
}

fn main() {
    task::block_on(renew_sitemaps_each_day())
}
