#[macro_use]
extern crate fstrings;

use teloxide::prelude::*;
use teloxide::types::ParseMode::HTML;
use teloxide::utils::html::link;

use warp::Filter;
use futures::{prelude::*, stream::futures_unordered::FuturesUnordered};

// 1. With thread in none async function

// https://github.com/steadylearner/Rust-Full-Stack/blob/master/server/src/main.rs
// thread::Builder::new()
// .name("Thread for Rust Chat with ws-rs".into())
// // .stack_size(83886 * 1024) // 80mib in killobytes
// .spawn(|| {
//     ws_rs::websocket();
// })
// .unwrap();

// 2. https://www.reddit.com/r/rust/comments/dh99xn/help_multiple_http_requests_on_a_singlethread/

// A simple type alias so as to DRY.
type MainResult<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

#[tokio::main]
async fn main() -> MainResult<()> {
    // https://github.com/seanmonstar/warp#example
    let hello = warp::path!("hello" / String)
        .map(|name| format!("Hello, {}!", name));

    // expected opaque type, found a different opaque type

    let mut list_of_futures = FuturesUnordered::new();
    list_of_futures.push(run());
    list_of_futures.push(warp::serve(hello).run(([127, 0, 0, 1], 3030)));

    Ok(())
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
