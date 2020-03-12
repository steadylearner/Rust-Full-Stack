use teloxide::prelude::*;
use teloxide::types::ParseMode::HTML;
use teloxide::utils::html::link;

use reqwest;
use serde_json; // 1.0.48
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Post {
    title: String,
    url: String
    // Just include more fields here if you want to use them.
    // selftext: String
}

#[derive(Deserialize)]
struct Child {
    data: Post
}

#[derive(Deserialize)]
struct Children {
    children: Vec<Child>
}

#[derive(Deserialize)]
struct Response {
    data: Children
}

#[tokio::main]
async fn main() {
    run().await;
}

async fn run() {
    teloxide::enable_logging!();
    log::info!("Starting Rust subreddit bot for the latest posts!");

    let bot = Bot::from_env();

    Dispatcher::new(bot)
        .messages_handler(|rx: DispatcherHandlerRx<Message>| {
            rx.text_messages()
                .for_each_concurrent(None, |(ctx, _raw_text_from_user)| async move {
                    let end_point = "https://www.reddit.com/r/rust/new/.json?limit=10";
                    let body_text = reqwest::get(end_point).await.unwrap().text().await.unwrap();

                    // It return Result here, So it will be better to hanlde it with map and map_err or match Ok or Err.
                    // https://docs.serde.rs/serde_json/fn.from_str.html
                    let json_value: Response = serde_json::from_str(&body_text).unwrap();

                    let mut html = String::new();
                    json_value.data.children.iter().enumerate().for_each(|(index, ch)| {
                        let Post { title, url } = &ch.data;
                        let post_link = link(url, title);
                        let num = index + 1;

                        let for_html = format!("{}. {}\n", &num, &post_link);
                        html.push_str(&for_html);
                    });

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
