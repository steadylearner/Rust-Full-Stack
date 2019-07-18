use yew::{html, Html};
use crate::Model;

// s7TVVyTyReU - verified and need CSS
// consider test with Self and Component method later?
pub fn view_video(value: &str) -> Html<Model> {
    let src = format!("https://www.youtube.com/embed/{}", &value); // conditional with payload(videoid) later
    html! {
        // wrapper with class here or modify width and height to minimum size
        <iframe
            allowfullscreen="1",
            title="YouTube video player",
            src={src},
            width="100%",
            height="360",
            frameborder="0",
            class=("flex", "margin-top-one-and-a-half", "max-width-half"),
        />
    }
}
