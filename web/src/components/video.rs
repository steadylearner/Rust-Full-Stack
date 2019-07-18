use yew::{html, Html};
use crate::Model;

// https://youtu.be/s7TVVyTyReU
// consider test with Self and Component method later?
pub fn view_video(value: &str) -> Html<Model> {
    html! {
        // wrapper with class here or modify width and height to minimum size
        <iframe
            allowfullscreen="1",
            title="YouTube video player",
            src={&value},
            width="100%",
            height="360",
            frameborder="0",
            class=("flex", "margin-top-one-and-a-half", "max-width-half"),
        />
    }
}
