use yew::{html, Html};

use crate::Model;

// consider test with Self and Component method later?
pub fn steadylarner_blog() -> Html<Model> {
    let class = "width-two-and-a-half theme-white border-round margin-right-half";
    html! {
        <a
            class=("flex", "no-text-decoration", "hover", "cursor-pointer", "transition-half", "right-auto"),
            href="https://www.steadylearner.com/blog/search/Rust",
            title="Click it to learn how to code this.",
            target="_blank",
            rel="noopener noreferrer",
        >
            <span 
                class=("white", "bold", "flex", "center"), 
            >
                <img 
                    src="https://www.steadylearner.com/static/images/code/Rust.svg",
                    class=class,
                />
                { "Full Stack Rust Chat App" }
            </span>
        </a>
    }
}
