use yew::{html, Html};
use crate::Model;

pub fn view_video(value: &str) -> Html<Model> {
    let src = format!("https://www.youtube.com/embed/{}", &value);
    html! {
        // wrapper with class here or modify width and height to minimum size
        <iframe
            allowfullscreen="1",
            title="YouTube video player",
            src={src},
            width="100%",
            height="360",
            frameborder="0",
            class="margin-left-a-quarter",
        />
    }
}
