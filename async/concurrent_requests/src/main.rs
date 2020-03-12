// Code here is not beautiful at all. But, I could make a prototype of what I wanted.
// Organize here and use less code for the production file.

// Code from: http://patshaughnessy.net/2020/1/20/downloading-100000-files-using-async-rust

use std::io::prelude::*;
use std::fs::File;
use std::io::BufReader;
use futures::stream::StreamExt;

use serde_json::Value;

use std::f64;

trait FloatIterExt {
    fn float_min(&mut self) -> f64;
    fn float_max(&mut self) -> f64;
}

// Floating point type doesn't implement Ord.
// So you write more code to make it work with NAN.
impl<T> FloatIterExt for T where T: Iterator<Item=f64> {
    // https://doc.rust-lang.org/std/primitive.f64.html#method.max
    // Recursively compare it.
    fn float_max(&mut self) -> f64 {
        self.fold(f64::NAN, f64::max)
    }

    // https://doc.rust-lang.org/std/primitive.f64.html#method.min
    // The same happens here
    fn float_min(&mut self) -> f64 {
        self.fold(f64::NAN, f64::min)
    }
}

fn read_lines(file_name: &str) -> std::io::Result<Vec<String>> {
    let file = File::open(file_name)?;
    let reader = BufReader::new(file);
    Ok(
        reader.lines().filter_map(Result::ok).collect()
    )
}

#[derive(Debug)]
struct Payload {
    target: String,
    price: Option<f64>,
}

#[derive(Debug, Clone)]
struct End {
    target: String,
    price: f64,
}

fn option_price(text: &str) -> Option<f64> {
    let json_value: Value = serde_json::from_str(&text).unwrap();
    let json_price = &json_value["result"]["price"];
    let option_price = json_price.as_f64();
    option_price
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let targets: Vec<String> = read_lines("targets.txt")?;
    let number_of_targets = targets.len();

    // https://docs.rs/futures-preview/0.3.0-alpha.17/futures/stream/trait.StreamExt.html#method.buffer_unordered
    // https://docs.rs/futures-preview/0.3.0-alpha.17/futures/stream/fn.iter.html
    let data = futures::stream::iter(
    targets.into_iter().map(|target| {
        async move {
            match reqwest::get(&target).await {
                Ok(body) => {
                    match body.text().await {
                        Ok(text) => {
                            // println!("From {}\n\n {}", &target, &text);
                            // let json_value: Value = serde_json::from_str(&text).unwrap();
                            // let json_price = &json_value["result"]["price"];
                            // let price = json_price.as_f64();
                            // let payload = Payload { target, price };

                            let price = option_price(&text);
                            let payload = Payload { target, price };
                            payload
                        }
                        Err(_) => {
                            println!("ERROR reading {}", &target);
                            let payload = Payload { target, price: None };
                            payload
                        }
                    }
                }
                Err(_) => {
                    println!("ERROR downloading {}", &target);
                    let payload = Payload { target, price: None };
                    payload
                }
            }
        }
    })
    ).buffer_unordered(number_of_targets).collect::<Vec<Payload>>();

    let payloads: Vec<Payload> = data.await;
    // Only extract market name here
    let ends: Vec<End> = payloads
        .into_iter()
        .filter(|payload| payload.price.is_some())
        .map(|end| End { target: end.target, price: end.price.unwrap() })
        .collect::<Vec<End>>();

    // println!("{:#?}", &ends);

    let prices: Vec<f64> = ends.clone().into_iter().map(|end| end.price).collect::<Vec<f64>>();
    // println!("{:#?}", &prices);

    let p_max = prices.iter().cloned().float_max();
    let p_min = prices.iter().cloned().float_min();

    // println!("Buy at {}", &p_min);
    // println!("Sell at {}", &p_max);
    
    let target_max: Vec<End> = ends.clone().into_iter().filter(|end| end.price == p_max).collect();
    let target_min: Vec<End> = ends.clone().into_iter().filter(|end| end.price == p_min).collect();
    
    println!("\nBuy at {:#?}", &target_min[0].target); // Use vector or tuple here to indicate where to buy
    println!("Sell at {:#?}", &target_max[0].target); 
    
    let margin = p_max - p_min;
    // Should buy and sell automatically here with question to a user.
    println!("You will get ${} in return.\n", margin.round());
    
    Ok(())
}

// Find the max and min value
// https://stackoverflow.com/questions/28247990/how-to-do-a-binary-search-on-a-vec-of-floats/28248065#28248065
