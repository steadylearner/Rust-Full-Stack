use yew::{html, Html};
use crate::Model;

// s7TVVyTyReU - verified and need CSS
pub fn view_video(value: &str) -> Html<Model> {
    let src = format!("https://www.youtube.com/embed/{}", &value);
    html! {
        <iframe
            allowfullscreen="1",
            title="YouTube video player",
            src={src},
            width="100%",
            height="360",
            frameborder="0",
        />
    }
}
