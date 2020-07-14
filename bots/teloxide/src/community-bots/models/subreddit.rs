use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Post {
    pub title: String,
    pub url: String, // Just include more fields here if you want to use them.
                     // selftext: String
}

#[derive(Deserialize)]
pub struct Child {
    pub data: Post,
}

#[derive(Deserialize)]
pub struct Children {
    pub children: Vec<Child>,
}

#[derive(Deserialize)]
pub struct Response {
    pub data: Children,
}
