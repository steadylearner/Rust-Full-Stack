use yew::{html, Html};

use crate::Model;

// consider test with Self and Component method later?
// pub fn steadylarner_blog() -> Html<Model> {
//     let class = "width-two-and-a-half theme-white border-round margin-right-half";
//     html! {
//         <a
//             class=("flex", "no-text-decoration", "hover", "cursor-pointer", "transition-half", "right-auto"),
//             href="https://www.steadylearner.com/blog/search/Rust",
//             title="Click it to learn how to code this.",
//             target="_blank",
//             rel="noopener noreferrer",
//         >
//             <span 
//                 class=("white", "bold", "flex", "center"), 
//             >
//                 <img 
//                     src="https://www.steadylearner.com/static/images/code/Rust.svg",
//                     class=class,
//                 />
//                 { "Full Stack Rust Chat App" }
//             </span>
//         </a>
//     }
// }

// https://github.com/steadylearner/code/blob/master/src/metatag/index.js
// Compared to React, it does work without dependency
// pub fn social(_title: &str, _description: &str, _image: &str) -> Html<Model> {
pub fn social() -> Html<Model> {
    html! {
        <>
            // index.html value(Full Stack Rust Chat App by Steadylearner) will used when there are common parts 
            <title>{ "Full Stack Rust Chat App" }</title>
            <meta name="description", content="Rust Full Stack Website by Steadylearner", />
            <meta name="thumbnail", content="https://avatars0.githubusercontent.com/u/32325099?s=460&v=4", />
            <meta property="og:title", content="Full Stack Rust Chat App by Steadylearner", />
            <meta property="og:description", content="Rust Full Stack Website by Steadylearner", />
            <meta property="og:image", content="https://avatars0.githubusercontent.com/u/32325099?s=460&v=4", />
            <meta property="og:locale", content="en_US", />
            <meta property="og:type", content="website", />
            <meta property="og:site_name", content="Full Stack Rust Chat App by Steadylearner", />
            <meta property="og:url", content="https://steadylearner.com/", />
            <meta name="twitter:title", content="Full Stack Rust Chat App by Steadylearner", />
            <meta name="twitter:description", content="Rust Full Stack Website by Steadylearner", />
            <meta name="twitter:image", content="https://avatars0.githubusercontent.com/u/32325099?s=460&v=4", />
            <meta name="twitter:card", content="summary", />
            <meta name="twitter:site", content="www.steadylearner.com", />
            <meta name="twitter:creator", content="@steadylearner_p", />
        </>
    }
}

// <title>Full Stack Rust Chat App by Steadylearner</title> 
// <meta name="description" content="Yew example made by https://www.steadylearner.com" />
// <meta name="thumbnail" content="https://avatars0.githubusercontent.com/u/32325099?s=460&v=4" />

// <meta property="og:title" content="Web by Steadylearner" />
// <meta property="og:description" content="Yew example made by https://www.steadylearner.com" />
// <meta property="og:image" content="https://avatars0.githubusercontent.com/u/32325099?s=460&v=4" />

// <meta property="og:locale" content="en_US" />
// <meta property="og:type" content="website" />
// <meta property="og:site_name" content="Steadylearner" />
// <meta property="og:url" content="https://steadylearner.com/" />

// <meta name="twitter:title" content="Web by Steadylearner" />
// <meta name="twitter:description" content="Yew example made by https://www.steadylearner.com" />
// <meta name="twitter:image" content="https://avatars0.githubusercontent.com/u/32325099?s=460&v=4" />

// <meta name="twitter:card" content="summary" />
// <meta name="twitter:site" content="www.steadylearner.com" />
// <meta name="twitter:creator" content="@steadylearner_p" />