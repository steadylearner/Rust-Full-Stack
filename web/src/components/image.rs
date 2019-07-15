use yew::{html, Html};

use crate::Model;

// https://www.steadylearner.com/static/images/brand/code.png - verified and need CSS
pub fn view_image(value: &str) -> Html<Model> {
    html! {
        <img class=("flex", "margin-top-two-and-a-half"), src=value, ></img>
    }
}

