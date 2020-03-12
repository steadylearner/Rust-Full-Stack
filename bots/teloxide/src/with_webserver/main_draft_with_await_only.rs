#[macro_use]
extern crate fstrings;

use teloxide::prelude::*;
use teloxide::types::ParseMode::HTML;
use teloxide::utils::html::link;

use warp::Filter;

// What comes first, blocks the other.
#[tokio::main]
async fn main() {
    let hello = warp::path!("hello" / String)
        .map(|name| format!("Hello, {}!", name));

    warp::serve(hello)
        .run(([127, 0, 0, 1], 3030))
        .await;

    run().await;
}

async fn run() {
    teloxide::enable_logging!();
    log::info!("Starting reqwest proxy GET bot!");

    let bot = Bot::from_env();

    Dispatcher::new(bot)
        .messages_handler(|rx: DispatcherHandlerRx<Message>| {
            rx.text_messages()
                .for_each_concurrent(None, |(ctx, _raw_text_from_user)| async move {

                    // Start with the examples here.
                    // https://docs.rs/teloxide/0.2.0/teloxide/types/enum.ParseMode.html#html-style

                    let href = "https://www.steadylearner.com";
                    let text = "Steadylearner Website";
                    let website = link(&href, &text);

                    // let website = f!("<a href='{href}'>{text}</a>");
                    // let website = format!("<a href='{}'>{}</a>", href, text);

                    // Inline style doesn't work.
                    // let website = f!("<a style='color: 'use whatever you want and test' href='{href}'>{text}</a>");

                    ctx.answer(website)
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
