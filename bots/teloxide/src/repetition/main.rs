// This bot just answers "pong" to each incoming UpdateKind::Message.
// Inside your messages_handler
// rx.text_messages().filter((|ctx, message| future::ready(message == "BTCUSD")))
// And then call .for_each or .for_each_concurrent to handle each request

// $cargo watch -x 'run --release'

// rx.text_messages().for_each(|(ctx, text)| ...)

// So inside for_each you'll get only text messages
// And also consider using for_each_concurrent 
// instead of for_each to make you bot being able to talk with multiple users at the same time

use teloxide::prelude::*;

#[tokio::main]
async fn main() {
    run().await;
}

async fn run() {
    teloxide::enable_logging!();
    log::info!("Starting first_telegram_repetition_bot!");

    let bot = Bot::from_env();

    Dispatcher::new(bot)
        .messages_handler(|rx: DispatcherHandlerRx<Message>| {
            // https://github.com/teloxide/teloxide/blob/master/examples/simple_commands_bot/src/main.rs
            // Execute all incoming commands concurrently without limit(None).
            // https://docs.rs/teloxide/0.2.0/teloxide/prelude/trait.StreamExt.html#method.for_each_concurrent
            rx.text_messages().for_each_concurrent(None, |(ctx, text)| async move {
                ctx.answer(text).send().await.log_on_error().await;
            })
        })
        .dispatch()
        .await;
}
