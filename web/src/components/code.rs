// https://github.com/markedjs/marked
// marked('# Marked in the browser\n\nRendered by **marked**.'); or test with pulldown-cmark in Rust
// test it with https://cdn.jsdelivr.net/npm

use stdweb::web::Node;
use stdweb::unstable::TryFrom;
use yew::{html, Html};
use yew::virtual_dom::VNode;

use crate::Model;

pub fn view_code(value: &str) -> Html<Model> {
    //  or use code from pulldown-cmark here
    // and pass it with @{} ?
    // https://github.com/steadylearner/react-easy-md/blob/master/src/MarkdownPreview.js
    // Test it with [Steadylearner Blog](https://www.steadylearner.com/blog)
    let markdown = js! {
        var div = document.createElement("div");
        div.className = "markdown";
        var code = @{&value};

        var options = {
            gfm: true,
            tables: true,
            breaks: false,
            pedantic: false,
            sanitize: true,
            smartLists: true,
            smartypants: false,
            langPrefix: "hljs ",
        };

        marked.setOptions(options);

        div.innerHTML = marked(code);
        // console.log(div);
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

