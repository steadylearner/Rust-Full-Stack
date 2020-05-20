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
                // https://stackoverflow.com/questions/58373663/cannot-use-the-operator-in-a-function-that-returns
                let resp = reqwest::get(&target_webpage)
                    .await
                    .unwrap();

                ctx.answer(resp.text().await.unwrap()).send().await.log_on_error().await;
            })
        })
        .dispatch()
        .await;
}
