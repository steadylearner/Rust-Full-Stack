// This bot just repeats what you send.
// $cargo watch -x 'run --release'

use teloxide::prelude::*;

#[tokio::main]
async fn main() {
    run().await;
}

async fn run() {
    teloxide::enable_logging!();
    log::info!("Starting ping_pong_bot!");

    let bot = Bot::from_env();

    Dispatcher::new(bot)
        .messages_handler(|rx: DispatcherHandlerRx<Message>| {
            rx.for_each(|message| async move {
                // println!("{:#?}", message.update.text());
                // message.answer(message.text()).send().await.log_on_error().await;
                match message.update.text() {
                    None => {
                        // println!("It will be none when users send other kind of input such as Image, Video etc")
                    }
                    Some(input) => {
                        message.answer(input).send().await.log_on_error().await;
                    },
                }
            })
        })
        .dispatch()
        .await;
}
