// Why use this?
// You can save your resource
// if you already have a website deployed with Rust web framework.

use teloxide::prelude::*;
use teloxide::types::ParseMode::HTML;
use teloxide::utils::html::link;

use warp::Filter;
use futures::join;

// https://rust-lang.github.io/async-book/06_multiple_futures/02_join.html
// Use this inside the main functions to use async functions oncurrently that return nothing.

// https://rust-lang.github.io/async-book/06_multiple_futures/02_join.html#try_join
// Use this inside the main functions to use async functions oncurrently that return Result.

// https://docs.rs/futures/0.2.0/futures/future/fn.join_all.html
// Read this also.

#[tokio::main]
async fn main() {
    // Verify this with $curl 0.0.0.0:8000/framework/Teleoxide
    let hello = warp::path!("framework" / String)
        .map(|name| format!("I want to use Warp with {}!", name));

    // Use this code not to return it from main function.
    let _ = join!(
        run(),
        warp::serve(hello)
        .run(([0, 0, 0, 0], 8000))
    );
}

async fn run() {
    teloxide::enable_logging!();
    log::info!("Use Teleoxide with another Rust web framework.");

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
