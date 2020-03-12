// https://github.com/markedjs/marked
// marked('# Marked in the browser\n\nRendered by **marked**.'); or test with pulldown-cmark in Rust
// test it with https://cdn.jsdelivr.net/npm

use stdweb::web::Node;
use stdweb::unstable::TryFrom;
use yew::{html, Html};
use yew::virtual_dom::VNode;

use stdweb::js;
// use stdweb::Value;
// use stdweb::unstable::TryInto;

use crate::Model;

pub fn view_code(value: &str) -> Html<Model> {
    //  or use code from pulldown-cmark here
    // and pass it with @{} ?
    // https://github.com/steadylearner/react-easy-md/blob/master/src/MarkdownPreview.js
    // Test it with [Steadylearner Blog](https://www.steadylearner.com/blog)

    // You can not approach variable defined here in your cosnole
    // but you can do it with what you define with browserify or unpkg etc.
    // It is not a problem of only this specific usage.
    let markdown = js! {
        const div = document.createElement("div");
        div.className = "markdown";
        const code = @{&value};

        const options = {
            gfm: true,
            tables: true,
            breaks: false,
            pedantic: false,
            sanitize: false, // true when you don't want to allow HTML
            smartLists: true,
            smartypants: false,

            // https://highlightjs.org/download/
            // https://github.com/steadylearner/react-easy-md/blob/master/src/MarkdownPreview.js

            // /static/index.html
            // <script src="https://cdnjs.cloudflare.com/ajax/libs/highlight.js/9.15.9/highlight.min.js"></script> 

            langPrefix: "hljs ",
            highlight: (code, lang) => {
                if (!!(lang && hljs.getLanguage(lang))) {
                    return hljs.highlight(lang, code).value;
                }

                return code;
            }
        };

        marked.setOptions(options);

        div.innerHTML = marked(code); // Dosen't parse the contents in code snippets with this in Rust
        console.log(marked(code));
        return div;
    };
    eprintln!("markdown: {:?}", markdown);
    let node = Node::try_from(markdown).expect("convert markdown");
    let vnode = VNode::VRef(node);
    eprintln!("div: {:?}", vnode);

    html! {
       { vnode }
    }
}


