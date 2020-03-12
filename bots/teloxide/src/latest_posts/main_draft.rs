#[macro_use]
extern crate fstrings;

// https://docs.rs/teloxide/0.2.0/teloxide/utils/markdown/fn.link.html
use teloxide::prelude::*;
// use teloxide::utils::markdown::link;
use teloxide::types::ParseMode::Markdown;

use reqwest;
// use std::collections::HashMap;

use serde_json::Value;

use console::Style;

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
            rx.text_messages().for_each_concurrent(None, |(ctx, _raw_text_from_user)| async move {
                let blue = Style::new()
                    .blue();

                // https://www.reddit.com/r/rust/new/.json?count=2
                let subreddit = "rust";
                let sort = "new";
                let limit = "10"; // Max is 100, Use 2 here to ease the development.

                let complete_target = f!("https://www.reddit.com/r/{subreddit}/{sort}/.json?limit={limit}");
                // println!("{}", &complete_target);
                let body_text = reqwest::get(&complete_target).await.unwrap().text().await.unwrap();

                // println!("{:#?}", &body_text);

                // Only extract what you want.
                // https://www.google.com/search?&q=extract+only+json+parts+I+want+serde

                let json_value: Value = serde_json::from_str(&body_text).unwrap();
                // println!("{:#?}", &json_value);
                // let list_of_posts = &json_value["data"]["children"]; // array
                // println!("List of posts\n {:#?}", &list_of_posts);

                // https://docs.serde.rs/serde_json/enum.Value.html#method.as_array
                // It becomes vector in Rust. Then, you can use its built in methods.
                let list_of_posts = json_value["data"]["children"].as_array().unwrap();
                // println!("{:#?}", list_of_posts);

                let mut markdown = String::new();
                let mut index: u8 = 1;

                for post in list_of_posts {
                    let title = &post["data"]["title"];
                    let url = &post["data"]["url"];

                    let double_quote = '"';
                    let empty_str = "";

                    // https://users.rust-lang.org/t/fast-removing-chars-from-string/24554
                    let complete_text = format!("{}. {}({})", &index, &title, &blue.apply_to(&url)).replace(double_quote, empty_str);
                    // let complete_text = f!("{index}. {title}({blue.apply_to(&url)})"); // It doesn't work.
                    println!("{}", &complete_text);

                    let for_markdown = f!("{index}. [{title}]({url})\n").replace(double_quote, empty_str);
                    // let for_markdown = link(url.as_str().unwrap(), title.as_str().unwrap());
                    markdown.push_str(&for_markdown);
                    index = index + 1;
                }

                // https://docs.rs/teloxide/0.2.0/teloxide/dispatching/struct.DispatcherHandlerCx.html
                // https://docs.rs/teloxide/0.2.0/teloxide/requests/struct.SendMessage.html
                ctx.answer(markdown).parse_mode(Markdown).send().await.log_on_error().await;
            })
        })
        .dispatch()
        .await;
}
