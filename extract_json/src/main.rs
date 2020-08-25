// https://www.reddit.com/dev/api/#GET_subreddits_new

#[macro_use]
extern crate fstrings;

use reqwest;
use serde_json::Value;

use console::Style;

// #[derive(Debug)]
// struct Payload {
//    title: String,
//    url: String,
// }

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
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

    let mut index = 0;
    for post in list_of_posts {
        index = index + 1;
        let title = &post["data"]["title"];
        let url = &post["data"]["url"];
        let double_quote = '"';
        let empty_str = "";

        // https://users.rust-lang.org/t/fast-removing-chars-from-string/24554
        let complete_text = format!("{}. {}({})", &index, &title, &blue.apply_to(&url)).replace(double_quote, empty_str);
        // let complete_text = f!("{index}. {title}({blue.apply_to(&url)})"); // It doesn't work.
        println!("{}", &complete_text);        

        // let for_md = f!("{&index}. [{&title}]({&url})");
        // println!("{}", &for_md);
    }

    Ok(())
}

