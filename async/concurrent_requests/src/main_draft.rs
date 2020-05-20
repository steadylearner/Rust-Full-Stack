// Make my own version with it.(Done)

// Code from: http://patshaughnessy.net/2020/1/20/downloading-100000-files-using-async-rust
//
// Cargo.toml:
// [dependencies]
// tokio = { version = "0.2", features = ["full"] }
// reqwest = { version = "0.10", features = ["json"] }
// futures = "0.3"

use std::io::prelude::*;
use std::fs::File;
use std::io::BufReader;
use futures::stream::StreamExt;

fn read_lines(path: &str) -> std::io::Result<Vec<String>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    Ok(
        reader.lines().filter_map(Result::ok).collect()
    )
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let paths: Vec<String> = read_lines("urls.txt")?;
    // https://docs.rs/futures-preview/0.3.0-alpha.17/futures/stream/trait.StreamExt.html#method.buffer_unordered
    // https://docs.rs/futures-preview/0.3.0-alpha.17/futures/stream/fn.iter.html
    let fetches = futures::stream::iter(
    paths.into_iter().map(|path| {
        async move {
            match reqwest::get(&path).await {
                Ok(resp) => {
                    match resp.text().await {
                        Ok(text) => {
                            println!("RESPONSE: {} bytes from {}", text.len(), path);
                        }
                        Err(_) => println!("ERROR reading {}", path),
                    }
                }
                Err(_) => println!("ERROR downloading {}", path),
            }
        }
    })
    ).buffer_unordered(100).collect::<Vec<()>>();
    fetches.await;
    Ok(())
}