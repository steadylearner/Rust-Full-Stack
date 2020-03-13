use teloxide::utils::html::link;

use crate::models::subreddit::{
    Response, Post,
};

pub fn numbered_html_link(json_value: Response) -> String {
    let mut draft = String::new();
    json_value
        .data
        .children
        .iter()
        .enumerate()
        .for_each(|(index, ch)| {
            let Post { title, url } = &ch.data;
            let post_link = link(url, title);
            let num = index + 1;

            let for_draft = format!("{}. {}\n", &num, &post_link);
            draft.push_str(&for_draft);
        });
    draft
}