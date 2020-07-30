#[macro_use]
extern crate fstrings;

use teloxide::prelude::*;

// https://core.telegram.org/bots/api#markdownv2-style
// It fails with MarkdownV2 or HTML so use Markdown instead.
// ERROR teloxide::error_handlers > Error: ApiError { status_code: 400, kind: Other }
// When they are used indepedently, there is no problem.
// It is likely that some code used here affect the result?
// Refer to https://github.com/steadylearner/Rust-Full-Stack/blob/master/bots/teloxide/src/html/main.rs

use teloxide::types::ParseMode::Markdown;
use teloxide::utils::markdown::link;

use reqwest;
use serde_json::Value;

use console::Style;

#[tokio::main]
async fn main() {
    run().await;
}

// https://users.rust-lang.org/t/fast-removing-chars-from-string/24554
// "" is included in some JSON data so use this.
fn remove_double_quote(text: String) -> String {
    let double_quote = '"';
    let empty_str = "";

    text.replace(double_quote, empty_str)
}

async fn run() {
    teloxide::enable_logging!();
    log::info!("Starting Rust subreddit bot for the latest posts!");

    let bot = Bot::from_env();

    Dispatcher::new(bot)
        .messages_handler(|rx: DispatcherHandlerRx<Message>| {
            rx.text_messages()
                .for_each_concurrent(None, |(ctx, _raw_text_from_user)| async move {
                    let blue = Style::new().blue();

                    // Max is 100. Use 10 here to see the data better.
                    // Refer to this. https://www.reddit.com/dev/api/
                    // Visit it at the browser first.
                    let end_point = "https://www.reddit.com/r/rust/new/.json?limit=10";

                    let body_text = reqwest::get(end_point).await.unwrap().text().await.unwrap();

                    // Only extract what you want.
                    // https://www.google.com/search?&q=extract+only+json+parts+I+want+serde
                    let json_value: Value = serde_json::from_str(&body_text).unwrap();

                    // https://docs.serde.rs/serde_json/enum.Value.html#method.as_array
                    // It becomes vector in Rust. Then, you can use its built in methods.
                    let list_of_posts = json_value["data"]["children"].as_array().unwrap();

                    let mut markdown = String::new();

                    // https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.enumerate
                    for (index, post) in list_of_posts.iter().enumerate() {
                        let title = &post["data"]["title"];
                        let url = &post["data"]["url"];

                        let num = index + 1;

                        let stdout_text = remove_double_quote(format!(
                            "{}. {}({})",
                            &num,
                            &title,
                            &blue.apply_to(&url)
                        ));
                        println!("{}", &stdout_text);

                        let post_link = link(url.as_str().unwrap(), title.as_str().unwrap());

                        let for_markdown = remove_double_quote(f!("{num}. {post_link}\n"));
                        markdown.push_str(&for_markdown);
                    }

                    // https://docs.rs/teloxide/0.2.0/teloxide/dispatching/struct.DispatcherHandlerCx.html
                    // https://docs.rs/teloxide/0.2.0/teloxide/requests/struct.SendMessage.html
                    // Each method(parse_mode and send) returns SendMessage so you can chain them.
                    ctx.answer(markdown)
                        .parse_mode(Markdown)
                        .send()
                        .await
                        .log_on_error()
                        .await;
                })
        })
        .dispatch()
        .await;
}
