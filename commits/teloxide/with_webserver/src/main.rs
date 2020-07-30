// Why use this? 
// You can save your resource if you already have a website deployed with a Rust web framework.

use teloxide::prelude::*;
use teloxide::types::ParseMode::HTML;
use teloxide::utils::html::link;

use warp::Filter;
use futures::join;

// https://rust-lang.github.io/async-book/06_multiple_futures/02_join.html
// Use this to use async functions concurrently that return nothing.

// https://rust-lang.github.io/async-book/06_multiple_futures/02_join.html#try_join
// Use this to use async functions concurrently that return Result.

// https://docs.rs/futures/0.2.0/futures/future/fn.join_all.html
// Read this also.

#[tokio::main]
async fn main() {
    // Verify this with $curl 0.0.0.0:8000/framework/Teloxide
    let hello = warp::path!("framework" / String)
        .map(|name| format!("I want to use Warp with {}!", name));

    // Use _ to return nothing from main function.
    let _ = join!(
        run(),
        warp::serve(hello)
        .run(([0, 0, 0, 0], 8000))
    );
}

async fn run() {
    teloxide::enable_logging!();
    log::info!("Use Teloxide with another Rust web framework.");

    let bot = Bot::from_env();

    Dispatcher::new(bot)
        .messages_handler(|rx: DispatcherHandlerRx<Message>| {
            rx.text_messages()
                .for_each_concurrent(None, |(ctx, _raw_text_from_user)| async move {

                    // Start with the examples here.
                    // https://docs.rs/teloxide/0.2.0/teloxide/types/enum.ParseMode.html#html-style

                    let href = "https://github.com/teloxide/teloxide";
                    let text = "Rust Teloxide";
                    let website = link(&href, &text);

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
