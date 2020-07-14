use teloxide::prelude::*;
use teloxide::types::ParseMode::HTML;

use reqwest;
use serde_json;

mod models;

mod utils;
use self::utils::numbered_html_link;

// type ResponseResult = Result<Response, serde_json::error::Error>

#[tokio::main]
async fn main() {
    run().await;
}

async fn run() {
    teloxide::enable_logging!();
    log::info!("Starting Rust subreddit bot to find the latest posts!");

    let bot = Bot::from_env();

    Dispatcher::new(bot)
        .messages_handler(|rx: DispatcherHandlerRx<Message>| {
            rx.text_messages()
                .for_each_concurrent(None, |(ctx, _raw_text_from_user)| async move {
                    let end_point = "https://www.reddit.com/r/rust/new/.json?limit=10";
                    let body_text = reqwest::get(end_point).await.unwrap().text().await.unwrap();

                    // ResponseResult
                    let html: String = match serde_json::from_str(&body_text) {
                        Ok(json_value) => {
                            numbered_html_link(json_value)
                        },
                        Err(_e) => {
                            // println!("{:?}", e)
                            "Something went wrong. There were errors while reading the subreddit."
                                .to_string()
                        }
                    };

                    // https://docs.rs/teloxide/0.2.0/teloxide/dispatching/struct.DispatcherHandlerCx.html
                    // https://docs.rs/teloxide/0.2.0/teloxide/requests/struct.SendMessage.html
                    // Each method(parse_mode and send) returns SendMessage so you can chain them.
                    ctx.answer(html)
                        .parse_mode(HTML)
                        .send()
                        .await
                        .log_on_error()
                        .await;
                })
        })
        .dispatch()
        .await;
}
