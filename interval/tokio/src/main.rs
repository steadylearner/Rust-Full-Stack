extern crate tokio;

use tokio::time;
use std::time::Duration;

async fn renew_sitemaps_each_day() {
    // Refer to this and make a microservice for simtemaps of your website.
    // https://github.com/steadylearner/Rust-Full-Stack/tree/master/sitemap

    let day_in_secs = 1; // Use 1 for testing, 86400 for production.
 
    let mut interval_day = time::interval(Duration::from_secs(day_in_secs));
    loop {
        // let now = interval_day.tick().await; // Return Instant
        // println!("Renew sitemaps each day. (Time now = {:?})", now);

        interval_day.tick().await;
        println!("Renew sitemaps each day.");            
    }
}

fn main() {
    let mut rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(renew_sitemaps_each_day());
}
