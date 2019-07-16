use yew::{html, Html};

use crate::Model;

// https://www.steadylearner.com/static/images/brand/code.png - verified and need CSS
// consider test with Self and Component method later?
pub fn view_image(value: &str) -> Html<Model> {
    html! {
        <img class=("flex", "margin-top-one-and-a-half", "max-width-half"), src=value, ></img>
    }
}

