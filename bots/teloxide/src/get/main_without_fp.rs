// $cargo watch -x 'run --release'
use teloxide::prelude::*;

use reqwest;
// use std::collections::HashMap;

#[tokio::main]
async fn main() {
    run().await;
}

async fn run() {
    teloxide::enable_logging!();
    log::info!("Starting reqwest proxy GET bot!");

    let bot = Bot::from_env();

    Dispatcher::new(bot)
        .messages_handler(|rx: DispatcherHandlerRx<Message>| {
            rx.text_messages().for_each_concurrent(None, |(ctx, target_webpage)| async move {
                let resp = reqwest::get(&target_webpage)
                    .await;

                match resp {
                    Ok(body) => {
                        let body_text_async = body.text().await;
                        match body_text_async {
                            Ok(payload) => {
                                println!("Payload(body) is {:#?}", &payload);
                                // Should handle when payload(body) is too long.
                                // Handle send().await part manually instead of log_on_error whenever it is necessary.
                                match ctx.answer(payload).send().await {
                                    Ok(_) => {
                                        println!("The body was sent safely to the user.");
                                    }
                                    Err(e) => {
                                        println!("Error from API limit of Telegram is {:#?}", e);
                                        ctx.answer("There was an error from Telegram API. The body part of your target maybe too long.").send().await.log_on_error().await;
                                    }
                                }
                            }
                            Err(e) => {
                                println!("Error from parsing body to text is {:#?}", e);
                                // Or here?
                                ctx.answer("There was an error parsing the body of your target webpage.").send().await.log_on_error().await;
                            }
                        }
                    }
                    Err(e) => {
                        println!("Error from GET taget_webpage is {:#?}", e);
                        ctx.answer("There was an error requesting your target webpage. Verify you entered the correct data.").send().await.log_on_error().await;
                    }
                }
            })
        })
        .dispatch()
        .await;
}

